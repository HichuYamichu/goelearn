use crate::core::LoggedInGuard;

use crate::core::{auth, AppError};
use async_graphql::{dataloader::DataLoader, Context, Object};
use auth::Claims;
use deadpool_redis::{redis, Pool};
use redis::AsyncCommands;
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use super::object::CreateMessageInput;
use super::{MessageObject, MessageRepo};

#[derive(Default)]
pub struct MessageMutation;

#[Object]
impl MessageMutation {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_message(
        &self,
        ctx: &Context<'_>,
        input: CreateMessageInput,
    ) -> Result<MessageObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();
        let redis_pool = ctx.data_unchecked::<Pool>();
        let mut conn = redis_pool.get().await?;

        let id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let channel_id = input.channel_id.clone();
        let model = input.try_into_active_model(id)?;
        let message: MessageObject = MessageRepo::create_message(data_loader, model)
            .await?
            .into();
        conn.publish(
            format!("channel_message:{}", channel_id.as_str()),
            serde_json::to_string(&message).expect("Message should serialize"),
        )
        .await?;

        Ok(message)
    }
}
