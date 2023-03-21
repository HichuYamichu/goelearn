use crate::{
    objects::{LoginInput, LoginResult, SignupInput},
    SECRET,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{user_repo, AppError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub async fn login_user(
    creadentials: LoginInput,
    user_repo: &user_repo::UserRepo,
) -> Result<LoginResult, AppError> {
    let user = user_repo
        .user_by_username(creadentials.username.clone())
        .await?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err(AppError::NotFound {
                what: "User",
                with: "username",
                why: creadentials.username,
            })
        }
    };

    if !user.active {
        return Err(AppError::Auth);
    }

    if user.deleted_at.is_some() {
        return Err(AppError::Auth);
    }

    let is_match = argon2_async::verify(creadentials.password, user.password).await?;
    if !is_match {
        return Err(AppError::Auth);
    }

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            sub: user.id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp(),
        },
        &jsonwebtoken::EncodingKey::from_secret(SECRET.as_ref()),
    )?;

    Ok(LoginResult { token })
}

pub async fn register_user(
    mut creadentials: SignupInput,
    user_repo: &user_repo::UserRepo,
) -> Result<Uuid, AppError> {
    let hash = argon2_async::hash(creadentials.password).await?;
    creadentials.password = hash;
    let id = user_repo.create_user(creadentials).await?;
    Ok(id)
}
