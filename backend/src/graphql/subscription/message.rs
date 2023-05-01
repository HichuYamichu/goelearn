use crate::core::AppError;
use crate::object::MessageObject;
use async_graphql::futures_util::StreamExt;
use async_graphql::ID;
use async_graphql::{futures_util::Stream, Context, Subscription};
use redis::Client;

#[derive(Default)]
pub struct MessageSubscription;

#[Subscription]
impl MessageSubscription {
    async fn message_created(
        &self,
        ctx: &Context<'_>,
        channel_id: ID,
    ) -> Result<impl Stream<Item = MessageObject>, AppError> {
        let client = ctx.data_unchecked::<Client>();
        let mut conn = client.get_async_connection().await?.into_pubsub();
        conn.subscribe(format!("channel_message:{}", channel_id.as_str()))
            .await?;
        Ok(conn.into_on_message().filter_map(|msg| async move {
            msg.get_payload()
                .ok()
                .and_then(|s: String| serde_json::from_str(s.as_str()).ok())
        }))
    }
}
