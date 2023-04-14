use crate::core::LoggedInGuard;
use crate::object::{LoginInput, LoginResult, SignupInput};
use crate::{
    core::{
        auth,
        repo::{class::ClassRepo, message::MessageRepo, user::UserRepo},
        AppError,
    },
    object::{ClassObject, CreateClassInput, CreateMessageInput, MessageObject},
};
use async_graphql::{dataloader::DataLoader, Context, Object};
use auth::Claims;
use redis::{AsyncCommands, Client};
use uuid::Uuid;

#[derive(Default)]
pub struct MessageMutation;

#[Object]
impl MessageMutation {
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_message(
        &self,
        ctx: &Context<'_>,
        input: CreateMessageInput,
    ) -> Result<MessageObject, AppError> {
        let message_repo = ctx.data::<DataLoader<MessageRepo>>().unwrap();
        let claims = ctx.data::<Option<Claims>>().unwrap();

        let id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let model = input.try_into_active_model(id)?;

        let message = message_repo.loader().create_message(model).await?;

        let client = ctx.data_unchecked::<Client>();
        let mut conn = client.get_async_connection().await.unwrap();
        conn.publish::<_, _, u32>("values", "aaaa".to_owned())
            .await
            .unwrap();

        Ok(message.into())
    }
}
