use std::str::FromStr;

use crate::api::message::MessageObject;
use crate::api::message::MessageRepo;
use crate::core::AppError;
use crate::core::LoggedInGuard;
use async_graphql::connection::Connection;
use async_graphql::InputObject;
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, SimpleObject, ID};

use async_graphql::connection::{self, Edge, EmptyFields};
use base64::{decode, encode};
use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use chrono::DateTime;
use chrono::Utc;
use chrono::{NaiveDateTime, Timelike};
use sea_orm::DatabaseConnection;
use sea_orm::Set;
use tracing::instrument;
use uuid::Uuid;

#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
#[graphql(name = "Channel")]
pub struct ChannelObject {
    pub id: ID,
    pub name: String,
    pub description: Option<String>,
}

impl From<::entity::channel::Model> for ChannelObject {
    fn from(c: ::entity::channel::Model) -> Self {
        Self {
            id: ID::from(c.id),
            name: c.name,
            description: c.description,
        }
    }
}

#[ComplexObject]
impl ChannelObject {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn messages(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, MessageObject>, async_graphql::Error> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        make_messages_connection(
            data_loader,
            Uuid::parse_str(&self.id)?,
            after,
            before,
            first,
            last,
        )
        .await
    }
}

pub async fn make_messages_connection(
    data_loader: &DataLoader<DatabaseConnection>,
    channel_id: Uuid,
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
) -> Result<Connection<String, MessageObject>, async_graphql::Error> {
    connection::query(
        after,
        before,
        first,
        last,
        |after: Option<String>, before, first, last| async move {
            let start = after
                .as_ref()
                .map(|after| {
                    parse_cursor(after).unwrap_or(NaiveDateTime::from_timestamp_opt(0, 0).unwrap())
                })
                .unwrap_or(NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
            let end = before
                .as_ref()
                .map(|before| parse_cursor(before).unwrap_or(DateTime::<Utc>::MAX_UTC.naive_utc()))
                .unwrap_or(DateTime::<Utc>::MAX_UTC.naive_utc());

            let mut connection = Connection::new(false, false);
            let mut edges = Vec::new();

            let messages = MessageRepo::load_messages(data_loader, channel_id, start, end).await?;

            if let Some(first) = first {
                edges.extend(messages.iter().take(first as usize).map(|item| {
                    let cursor = create_cursor(&item.created_at);
                    Edge::new(cursor, MessageObject::from(item.clone()))
                }));
            } else if let Some(last) = last {
                edges.extend(messages.iter().rev().take(last as usize).rev().map(|item| {
                    let cursor = create_cursor(&item.created_at);
                    Edge::new(cursor, MessageObject::from(item.clone()))
                }));
            }

            connection.edges = edges;
            connection.has_previous_page = after.is_some();
            connection.has_next_page = before.is_some() && messages.len() > last.unwrap_or(0);

            Ok::<_, AppError>(connection)
        },
    )
    .await
}

fn parse_cursor(cursor: &str) -> Option<NaiveDateTime> {
    // Parse and return the timestamp from the cursor
    let decoded_bytes = base64::decode(cursor).ok()?;
    let decoded_str = String::from_utf8(decoded_bytes).ok()?;
    let timestamp = NaiveDateTime::parse_from_str(&decoded_str, "%Y-%m-%d %H:%M:%S").ok()?;
    Some(timestamp)
}

fn create_cursor(timestamp: &NaiveDateTime) -> String {
    // Create a cursor based on the given timestamp
    let timestamp_str = timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
    let encoded_bytes = base64::encode(timestamp_str);
    encoded_bytes
}

#[derive(Clone, Debug, InputObject)]
pub struct CreateChannelInput {
    pub name: String,
    pub description: Option<String>,
    pub class_id: ID,
    pub allow_members_to_post: bool,
}

impl CreateChannelInput {
    pub fn try_into_active_model(self) -> Result<::entity::channel::ActiveModel, AppError> {
        Ok(::entity::channel::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(self.name),
            description: Set(self.description),
            class_id: Set(Uuid::parse_str(self.class_id.as_str())?),
            allow_members_to_post: Set(self.allow_members_to_post),
        })
    }
}
