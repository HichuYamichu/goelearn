use crate::{
    object::{LoginInput, LoginResult, SignupInput},
    SECRET,
};
use async_graphql::{Context, Guard};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{repo::user::UserRepo, AppError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub async fn login_user(
    creadentials: LoginInput,
    user_repo: &UserRepo,
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
    user_repo: &UserRepo,
) -> Result<Uuid, AppError> {
    let hash = argon2_async::hash(creadentials.password).await?;
    creadentials.password = hash;
    let id = user_repo.create_user(creadentials).await?;
    Ok(id)
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(req, state)
                .await
                .map_err(|_| AppError::Auth)?;

        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(SECRET.as_ref()),
            &validation,
        )
        .map_err(|_| AppError::Auth)?;

        Ok(token_data.claims)
    }
}

pub struct LoggedInGuard;

#[async_trait]
impl Guard for LoggedInGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        let claims = ctx.data::<Option<Claims>>().unwrap();

        match claims {
            Some(_) => Ok(()),
            None => Err(AppError::Auth.into()),
        }
    }
}
