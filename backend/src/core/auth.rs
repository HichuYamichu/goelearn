use crate::SECRET;
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

use super::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
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
                .map_err(|_| AppError::auth("Malformed JWT"))?;

        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(SECRET.as_ref()),
            &validation,
        )
        .map_err(|_| AppError::auth("Malformed JWT"))?;

        Ok(token_data.claims)
    }
}

pub struct LoggedInGuard;

#[async_trait]
impl Guard for LoggedInGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        let claims = ctx.data_unchecked::<Option<Claims>>();

        match claims {
            Some(_) => Ok(()),
            None => Err(AppError::auth("Missing JWT claims").into()),
        }
    }
}
