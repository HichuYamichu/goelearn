use crate::core::AppError;
use async_graphql::futures_util::StreamExt;
use async_graphql::{futures_util::Stream, Context, Subscription};

#[derive(Default)]
pub struct MessageSubscription;

#[Subscription]
impl MessageSubscription {
    async fn values(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = String>, AppError> {
        let client = ctx.data_unchecked::<redis::Client>();
        let mut conn = client.get_async_connection().await.unwrap().into_pubsub();
        conn.subscribe("values").await.unwrap();
        Ok(conn
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }
}
