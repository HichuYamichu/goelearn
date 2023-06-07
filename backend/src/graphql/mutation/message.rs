use crate::core::LoggedInGuard;

use crate::core::repo::message::MessageRepoExt;
use crate::{
    core::{auth, AppError},
    object::{CreateMessageInput, MessageObject},
};
use async_graphql::{dataloader::DataLoader, Context, Object};
use auth::Claims;
use redis::{AsyncCommands, Client};
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

#[derive(Default)]
pub struct MessageMutation;

#[Object]
impl MessageMutation {
    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_message(
        &self,
        ctx: &Context<'_>,
        input: CreateMessageInput,
    ) -> Result<MessageObject, AppError> {
        let message_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();
        let client = ctx.data_unchecked::<Client>();
        let mut conn = client.get_async_connection().await?;

        let id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let channel_id = input.channel_id.clone();
        let model = input.try_into_active_model(id)?;
        let message: MessageObject = message_repo.loader().create_message(model).await?.into();
        conn.publish(
            format!("channel_message:{}", channel_id.as_str()),
            serde_json::to_string(&message).expect("Message should serialize"),
        )
        .await?;

        Ok(message)
    }
}
