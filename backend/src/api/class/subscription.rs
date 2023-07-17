use crate::api::class::ClassObject;
use crate::api::message::MessageObject;
use crate::core::AppError;
use async_graphql::futures_util::StreamExt;
use async_graphql::ID;
use async_graphql::{futures_util::Stream, Context, Subscription};
use redis::Client;
use tracing::instrument;

#[derive(Default)]
pub struct ClassSubscription;

#[Subscription]
impl ClassSubscription {
    #[instrument(skip(self, ctx), err)]
    async fn class_updated(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
    ) -> Result<impl Stream<Item = ClassObject>, AppError> {
        let client = ctx.data_unchecked::<Client>();
        let mut conn = client.get_async_connection().await?.into_pubsub();
        conn.subscribe(format!("class_updated:{}", class_id.as_str()))
            .await?;
        Ok(conn.into_on_message().filter_map(|msg| async move {
            msg.get_payload()
                .ok()
                .and_then(|s: String| serde_json::from_str(s.as_str()).ok())
        }))
    }

    async fn class_deleted(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
    ) -> Result<impl Stream<Item = bool>, AppError> {
        let client = ctx.data_unchecked::<Client>();
        let mut conn = client.get_async_connection().await?.into_pubsub();
        conn.subscribe(format!("class_deleted:{}", class_id.as_str()))
            .await?;
        Ok(conn
            .into_on_message()
            .filter_map(|_msg| async move { Some(true) }))
    }
}
