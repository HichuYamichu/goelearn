use crate::core::UserError;
use crate::core::{auth, repo::user::UserRepo, AppError};
use crate::object::{LoginInput, LoginResult, SignupInput};
use async_graphql::futures_util;
use async_graphql::{dataloader::DataLoader, Context, Object};
use tokio::fs::File;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing_subscriber::fmt::format;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn signup(
        &self,
        ctx: &Context<'_>,
        mut input: SignupInput,
    ) -> Result<String, AppError> {
        let user_repo = ctx.data_unchecked::<DataLoader<UserRepo>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();

        let avatar = input.avatar.take();
        let id = auth::register_user(input, user_repo.loader(), avatar.is_some()).await?;

        if let Some(avatar) = avatar {
            let avatar = avatar.value(ctx)?;
            if avatar.content_type.is_none()
                || avatar.content_type.as_ref().unwrap() != "image/jpeg"
            {
                return Err(AppError::UserError(UserError::BadInput {
                    simple: "avatar must be a jpeg image",
                    detailed: "avatar must be a jpeg image".into(),
                }));
            }

            let s3_path = format!("user-avatars/{}", id);
            let mut reader = avatar.into_async_read().compat();
            s3_bucket
                .put_object_stream_with_content_type(&mut reader, s3_path, "image/jpeg")
                .await?;
        }

        Ok(id.to_string())
    }

    pub async fn login(
        &self,
        ctx: &Context<'_>,
        input: LoginInput,
    ) -> Result<LoginResult, AppError> {
        let user_repo = ctx.data_unchecked::<DataLoader<UserRepo>>();
        let res = auth::login_user(input, user_repo.loader()).await?;
        Ok(res)
    }
}
