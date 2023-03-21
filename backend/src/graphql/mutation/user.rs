use crate::core::{auth, UserRepo};
use async_graphql::{Context, Object, Result};

use crate::objects::{LoginInput, LoginResult, SignupInput};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn signup(&self, ctx: &Context<'_>, input: SignupInput) -> Result<String> {
        let user_repo = ctx.data::<UserRepo>().unwrap();
        let id = auth::register_user(input, user_repo).await?;
        Ok(id.to_string())
    }

    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<LoginResult> {
        let user_repo = ctx.data::<UserRepo>().unwrap();
        let res = auth::login_user(input, user_repo).await?;
        Ok(res)
    }
}
