use crate::core::{user_repo, Claims};
use ::entity::sea_orm_active_enums::UserType;
use async_graphql::*;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use sea_orm::*;
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
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let claims = jsonwebtoken::decode::<Claims>(
            &self.token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        // TODO: remove unwrap?
        let id = Uuid::parse_str(&claims.claims.sub).unwrap();
        let u = user_repo::user_by_id(id, db).await?.unwrap();

        Ok(u.into())
    }
}
