use std::str::FromStr;

use crate::api::message::MessageObject;
use crate::api::message::MessageRepo;
use crate::core::AppError;
use crate::core::LoggedInGuard;
use async_graphql::connection::Connection;
use async_graphql::InputObject;
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, SimpleObject, ID};

use async_graphql::connection::{self, Edge, EmptyFields};
use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use chrono::NaiveDate;
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
            // TODO: support last

            // let after = OpaqueCursor::decode_cursor(after.as_deref())?;
            let after = after
                .map(|after| {
                    let bytes = BASE64_STANDARD_NO_PAD.decode(after.as_bytes())?;
                    let date = String::from_utf8(bytes)?;
                    Ok::<_, AppError>(NaiveDate::from_str(&date)?)
                })
                .transpose()?;

            let before = before
                .map(|before| {
                    let bytes = BASE64_STANDARD_NO_PAD.decode(before.as_bytes())?;
                    let date = String::from_utf8(bytes)?;
                    Ok::<_, AppError>(NaiveDate::from_str(&date)?)
                })
                .transpose()?;

            let first = first.unwrap_or(20);
            let last = last.unwrap_or(20);

            let messages =
                MessageRepo::load_messages(data_loader, channel_id, after, before, first, last)
                    .await?;

            let edges: Vec<Edge<String, MessageObject, EmptyFields>> = messages
                .iter()
                .map(|m| {
                    Edge::new(
                        BASE64_STANDARD_NO_PAD.encode(m.created_at.to_string().as_bytes()),
                        MessageObject::from(m.clone()),
                    )
                })
                .collect();

            // TODO: impl has next page
            let mut connection = Connection::new(false, false);
            connection.edges.extend(edges);

            Ok::<_, AppError>(connection)
        },
    )
    .await
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
