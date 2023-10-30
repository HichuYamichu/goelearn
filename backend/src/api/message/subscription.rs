use crate::api::class;
use crate::api::message::MessageObject;
use crate::core::AppError;
use crate::core::{ClassMemberGuard, ClassOwnerGuard, LoggedInGuard};
use async_graphql::futures_util::StreamExt;
use async_graphql::ID;
use async_graphql::{futures_util::Stream, Context, Subscription};
use deadpool_redis::Pool;
use tracing::instrument;

#[derive(Default)]
pub struct MessageSubscription;

#[Subscription]
impl MessageSubscription {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassMemberGuard::new(class_id.clone()))")]
    async fn message_created(
        &self,
        ctx: &Context<'_>,
        channel_id: ID,
        class_id: ID,
    ) -> Result<impl Stream<Item = MessageObject>, AppError> {
        let redis_pool = ctx.data_unchecked::<Pool>();
        let conn = deadpool_redis::Connection::take(redis_pool.get().await?);
        let mut conn = conn.into_pubsub();
        conn.subscribe(format!("channel_message:{}", channel_id.as_str()))
            .await?;
        Ok(conn.into_on_message().filter_map(|msg| async move {
            msg.get_payload()
                .ok()
                .and_then(|s: String| serde_json::from_str(s.as_str()).ok())
        }))
    }
}
