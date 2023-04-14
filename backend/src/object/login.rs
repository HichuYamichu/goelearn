use crate::{
    core::{repo::user::UserRepo, AppError, Claims},
    SECRET,
};
use async_graphql::{dataloader::DataLoader, *};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use uuid::Uuid;

use super::UserObject;

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
}
