use crate::core::auth;
use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;

use crate::objects::{LoginInput, LoginResult, SignupInput};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn signup(&self, ctx: &Context<'_>, input: SignupInput) -> Result<String> {
        let res = auth::user_register(input, ctx.data::<DatabaseConnection>().unwrap()).await?;
        Ok("User created".to_string())
    }

    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<LoginResult> {
        let res = auth::user_login(input, ctx.data::<DatabaseConnection>().unwrap()).await?;
        Ok(res)
    }
}
