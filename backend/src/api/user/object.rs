use std::fmt::Display;

use async_graphql::dataloader::DataLoader;
use async_graphql::{
    ComplexObject, Context, InputObject, Object, Result, SimpleObject, Upload, ID,
};

use async_graphql::Enum;
use deadpool_redis::redis::{self, FromRedisValue, RedisResult, RedisWrite, ToRedisArgs};
use entity::{sea_orm_active_enums, user};
use partialdebug::placeholder::PartialDebug;
use sea_orm::{DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::api::assignment::{AssignmentObject, AssignmentRepo};
use crate::api::class::{ClassObject, ClassRepo};
use crate::core::{AppError, LoggedInGuard};

#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
#[graphql(name = "User")]
pub struct UserObject {
    pub id: ID,
    pub username: String,
    pub email: String,
    pub has_avatar: bool,
    pub first_name: String,
    pub last_name: String,
    pub user_type: UserType,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl ToRedisArgs for UserObject {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let vec = vec![
            self.id.to_string(),
            self.username.clone(),
            self.email.clone(),
            self.has_avatar.to_string(),
            self.first_name.clone(),
            self.last_name.clone(),
            self.user_type.to_string(),
            self.deleted_at
                .map(|d| d.timestamp().to_string())
                .unwrap_or("".into()),
        ];
        vec.write_redis_args(out)
    }
}

impl FromRedisValue for UserObject {
    fn from_redis_value(v: &redis::Value) -> RedisResult<Self> {
        let vec = Vec::<String>::from_redis_value(v)?;
        Ok(Self {
            id: ID::from(vec[0].clone()),
            username: vec[1].clone(),
            email: vec[2].clone(),
            has_avatar: vec[3].parse::<bool>().unwrap(),
            first_name: vec[4].clone(),
            last_name: vec[5].clone(),
            user_type: vec[6].parse::<UserType>().unwrap(),
            deleted_at: if vec[7].is_empty() {
                None
            } else {
                Some(chrono::NaiveDateTime::from_timestamp(
                    vec[7].parse::<i64>().unwrap(),
                    0,
                ))
            },
        })
    }
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum UserType {
    Regular,
    Mod,
    Admin,
}

impl Display for UserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserType::Regular => write!(f, "Regular"),
            UserType::Mod => write!(f, "Mod"),
            UserType::Admin => write!(f, "Admin"),
        }
    }
}

impl std::str::FromStr for UserType {
    type Err = ();
    fn from_str(input: &str) -> Result<UserType, Self::Err> {
        match input {
            "Regular" => Ok(Self::Regular),
            "Mod" => Ok(Self::Mod),
            "Admin" => Ok(Self::Admin),
            _ => Err(()),
        }
    }
}

impl From<sea_orm_active_enums::UserType> for UserType {
    fn from(e: sea_orm_active_enums::UserType) -> Self {
        match e {
            sea_orm_active_enums::UserType::Regular => Self::Regular,
            sea_orm_active_enums::UserType::Mod => Self::Mod,
            sea_orm_active_enums::UserType::Admin => Self::Admin,
        }
    }
}

impl From<UserType> for sea_orm_active_enums::UserType {
    fn from(e: UserType) -> Self {
        match e {
            UserType::Regular => Self::Regular,
            UserType::Mod => Self::Mod,
            UserType::Admin => Self::Admin,
        }
    }
}

#[ComplexObject]
impl UserObject {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn clesses(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let user_id = Uuid::parse_str(&self.id)?;
        let classes = ClassRepo::find_by_user_id(data_loader, user_id)
            .await?
            .expect("user id is valid");

        Ok(classes.into_iter().map(|c| c.into()).collect())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn owned_classes(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let owner_id = Uuid::parse_str(&self.id)?;
        let classes = ClassRepo::find_by_owner_id(data_loader, owner_id)
            .await?
            .expect("owner id is valid");

        Ok(classes.into_iter().map(|c| c.into()).collect())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn assignments(&self, ctx: &Context<'_>) -> Result<Vec<AssignmentObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let user_id = Uuid::parse_str(&self.id)?;
        let assignments = AssignmentRepo::find_by_user_id(data_loader, user_id).await?;

        Ok(assignments.into_iter().map(|c| c.into()).collect())
    }
}

impl From<::entity::user::Model> for UserObject {
    fn from(u: ::entity::user::Model) -> Self {
        Self {
            id: ID::from(u.id),
            username: u.username,
            email: u.email,
            has_avatar: u.has_avatar,
            first_name: u.first_name,
            last_name: u.last_name,
            user_type: u.user_type.into(),
            deleted_at: u.deleted_at,
        }
    }
}

#[derive(InputObject, PartialDebug)]
pub struct SignupInput {
    #[graphql(validator(min_length = 5, max_length = 20))]
    pub username: String,
    #[graphql(validator(min_length = 2, max_length = 40))]
    pub first_name: String,
    #[graphql(validator(min_length = 2, max_length = 60))]
    pub last_name: String,
    #[graphql(validator(email))]
    pub email: String,
    #[graphql(validator(min_length = 8, max_length = 100))]
    pub password: String,
    pub avatar: Option<Upload>,
}

impl SignupInput {
    pub fn into_active_model(self, has_avatar: bool) -> user::ActiveModel {
        user::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(self.username),
            first_name: Set(self.first_name),
            last_name: Set(self.last_name),
            has_avatar: Set(has_avatar),
            email: Set(self.email),
            password: Set(self.password),
            created_at: Set(chrono::offset::Utc::now().naive_utc()),
            deleted_at: Set(None),
            active: Set(false),
            user_type: Set(sea_orm_active_enums::UserType::Regular),
            ..Default::default()
        }
    }
}

#[derive(InputObject, Clone, Debug)]
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
