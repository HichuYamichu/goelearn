use std::str::FromStr;

use crate::core::repo::message::MessageRepoExt;
use async_graphql::{
    connection::{self, Connection, Edge, EmptyFields},
    dataloader::DataLoader,
    Context, Object, ID,
};
use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use chrono::NaiveDate;
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    core::{AppError, LoggedInGuard},
    object::MessageObject,
};

#[derive(Default)]
pub struct MessageQuery;

#[Object]
impl MessageQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn messages(
        &self,
        ctx: &Context<'_>,
        _class_id: ID,
        channel_id: ID,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, MessageObject>, async_graphql::Error> {
        let message_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        make_messages_connection(
            message_repo,
            Uuid::parse_str(channel_id.as_str())?,
            after,
            before,
            first,
            last,
        )
        .await
    }
}

pub async fn make_messages_connection(
    message_repo: &DataLoader<DatabaseConnection>,
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

            let messages = message_repo
                .loader()
                .load_messages(channel_id, after, before, first, last)
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
