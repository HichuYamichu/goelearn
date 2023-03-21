use crate::{
    core::{user_repo, Claims},
    SECRET,
};
use async_graphql::*;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use user_repo::UserRepo;
use uuid::Uuid;

use super::User;

#[derive(InputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

pub struct LoginResult {
    pub token: String,
}

#[Object]
impl LoginResult {
    pub async fn token(&self) -> &str {
        &self.token
    }

    pub async fn user(&self, ctx: &Context<'_>) -> Result<User> {
        let user_repo = ctx.data::<UserRepo>().unwrap();
        let claims = jsonwebtoken::decode::<Claims>(
            &self.token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        let id = Uuid::parse_str(&claims.claims.sub).unwrap();
        let u = user_repo.user_by_id(id).await?.unwrap();

        Ok(u.into())
    }
}
