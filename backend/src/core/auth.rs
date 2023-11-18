use crate::{
    api::{ClassRepo, UserRepo},
    SECRET,
};
use async_graphql::{dataloader::DataLoader, Context, Guard, ID};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};

use entity::sea_orm_active_enums::UserType;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
        validate_token(bearer.token())
    }
}

pub fn validate_token(token: &str) -> Result<Claims, AppError> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    )
    .map_err(|_| AppError::auth("Malformed JWT"))?;

    Ok(token_data.claims)
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

pub struct ClassMemberGuard {
    class_id: ID,
}

impl ClassMemberGuard {
    pub fn new(class_id: ID) -> Self {
        Self { class_id }
    }
}

#[async_trait]
impl Guard for ClassMemberGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();

        let user_id = Uuid::parse_str(claims.as_ref().expect("claims exist").sub.as_str())?;
        let class_id = self.class_id.parse::<Uuid>()?;

        if is_class_member(data_loader, user_id, class_id).await {
            return Ok(());
        }

        return Err(AppError::auth("User is not a member of this class").into());
    }
}

pub async fn is_class_member(
    data_loader: &DataLoader<DatabaseConnection>,
    user_id: Uuid,
    class_id: Uuid,
) -> bool {
    let members = match UserRepo::find_by_class_id(data_loader, class_id).await {
        Ok(members) => members,
        Err(_) => return false,
    };

    if let Some(members) = members {
        if members.iter().any(|m| m.id == user_id) {
            return true;
        }
    }
    return false;
}

pub struct ClassOwnerGuard {
    class_id: ID,
}

impl ClassOwnerGuard {
    pub fn new(class_id: ID) -> Self {
        Self { class_id }
    }
}

#[async_trait]
impl Guard for ClassOwnerGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();

        let user_id = Uuid::parse_str(claims.as_ref().expect("claims exist").sub.as_str())?;
        let class_id = self.class_id.parse::<Uuid>()?;

        let class = ClassRepo::find_by_id(data_loader, class_id).await?;
        if let Some(class) = class {
            if class.owner_id == user_id {
                return Ok(());
            }
        }

        return Err(AppError::auth("User is not the owner of this class").into());
    }
}

pub struct ResourceOwnerGuard;

#[async_trait]
impl Guard for ResourceOwnerGuard {
    async fn check(&self, _ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        todo!()
    }
}

pub struct AdminGuard;

#[async_trait]
impl Guard for AdminGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();

        let user_id = Uuid::parse_str(claims.as_ref().expect("claims exist").sub.as_str())?;
        let user = UserRepo::find_by_id(data_loader, user_id)
            .await?
            .expect("user must exist");

        match user.user_type {
            UserType::Admin => Ok(()),
            _ => Err(AppError::auth("User is not an admin").into()),
        }
    }
}

pub struct ModGuard;

#[async_trait]
impl Guard for ModGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();

        let user_id = Uuid::parse_str(claims.as_ref().expect("claims exist").sub.as_str())?;
        let user = UserRepo::find_by_id(data_loader, user_id)
            .await?
            .expect("user must exist");

        match user.user_type {
            UserType::Admin | UserType::Mod => Ok(()),
            _ => Err(AppError::auth("User is not a mod").into()),
        }
    }
}
