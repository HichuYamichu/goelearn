use crate::core::{ClassOwnerGuard, LoggedInGuard};
use async_graphql::{connection::Connection, dataloader::DataLoader, Context, Object, ID};
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use crate::api::channel::make_messages_connection;

use super::MessageObject;

#[derive(Default)]
pub struct MessageQuery;

#[Object]
impl MessageQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassOwnerGuard::new(class_id.clone()))")]
    async fn messages(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
        channel_id: ID,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, MessageObject>, async_graphql::Error> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        make_messages_connection(
            data_loader,
            Uuid::parse_str(&channel_id)?,
            after,
            before,
            first,
            last,
        )
        .await
    }
}
