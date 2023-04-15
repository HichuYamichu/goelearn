use crate::core::{auth, repo::user::UserRepo, AppError};
use async_graphql::{dataloader::DataLoader, Context, Object};

use crate::object::{LoginInput, LoginResult, SignupInput};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn signup(&self, ctx: &Context<'_>, input: SignupInput) -> Result<String, AppError> {
        let user_repo = ctx.data_unchecked::<DataLoader<UserRepo>>();
        let id = auth::register_user(input, user_repo.loader()).await?;
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
