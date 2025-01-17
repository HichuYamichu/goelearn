use crate::api::assignment::AssignmentObject;
use crate::api::assignment::AssignmentRepo;
use crate::api::channel::ChannelObject;
use crate::api::channel::ChannelRepo;
use crate::api::file::FileObject;
use crate::api::file::FileRepo;
use crate::api::user::UserObject;
use crate::api::user::UserRepo;
use crate::core::option_to_active_value;
use crate::core::AppError;
use crate::core::Claims;
use crate::core::LoggedInGuard;
use async_graphql::Upload;
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject, ID,
};

use chrono::NaiveDateTime;
use deadpool_redis::redis;
use partialdebug::placeholder::PartialDebug;
use redis::FromRedisValue;
use redis::RedisResult;
use redis::RedisWrite;
use redis::ToRedisArgs;
use sea_orm::{DatabaseConnection, Set};
use serde::Deserialize;
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
#[graphql(name = "Class")]
pub struct ClassObject {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub owner_id: ID,
    pub public: bool,
    pub tags: Vec<String>,
    pub has_image: bool,
    pub deleted_at: Option<NaiveDateTime>,
}

#[ComplexObject]
impl ClassObject {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn channels(&self, ctx: &Context<'_>) -> Result<Vec<ChannelObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let channels = ChannelRepo::find_by_class_id(data_loader, Uuid::parse_str(&self.id)?)
            .await?
            .expect("Id should be valid");

        Ok(channels.into_iter().map(ChannelObject::from).collect())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<UserObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let class_id = Uuid::parse_str(&self.id)?;
        let users = UserRepo::find_by_class_id(data_loader, class_id)
            .await?
            .expect("Id should be valid");

        Ok(users.into_iter().map(UserObject::from).collect())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn files(&self, ctx: &Context<'_>) -> Result<Vec<FileObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();
        let user_id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;

        let is_owner = user_id == Uuid::parse_str(&self.owner_id)?;
        let class_id = Uuid::parse_str(&self.id)?;
        let files = FileRepo::find_by_class_id(data_loader, class_id, is_owner)
            .await?
            .expect("Id should be valid");

        Ok(files.into_iter().map(FileObject::from).collect())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn owner(&self, ctx: &Context<'_>) -> Result<UserObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = Uuid::parse_str(&self.owner_id)?;
        let user = UserRepo::find_by_id(data_loader, id)
            .await?
            .expect("Id should be valid");

        Ok(UserObject::from(user))
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn assignments(&self, ctx: &Context<'_>) -> Result<Vec<AssignmentObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let class_id = Uuid::parse_str(&self.id)?;
        let assignments = AssignmentRepo::find_by_class_id(data_loader, class_id)
            .await?
            .expect("Id should be valid");

        Ok(assignments
            .into_iter()
            .map(AssignmentObject::from)
            .collect())
    }
}

impl From<::entity::class::Model> for ClassObject {
    fn from(c: ::entity::class::Model) -> Self {
        Self {
            id: ID::from(c.id),
            name: c.name,
            description: c.description,
            owner_id: ID::from(c.owner_id),
            public: c.public,
            tags: c.tags.split(' ').map(|s| s.to_string()).collect(),
            has_image: c.has_image,
            deleted_at: c.deleted_at,
        }
    }
}

impl ToRedisArgs for ClassObject {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let vec = vec![
            self.id.to_string(),
            self.name.clone(),
            self.description.clone(),
            self.owner_id.to_string(),
            self.public.to_string(),
            self.tags.join(" "),
            self.has_image.to_string(),
            self.deleted_at
                .map(|d| d.timestamp().to_string())
                .unwrap_or("".into()),
        ];
        vec.write_redis_args(out)
    }
}

impl FromRedisValue for ClassObject {
    fn from_redis_value(v: &redis::Value) -> RedisResult<Self> {
        let vec = Vec::<String>::from_redis_value(v)?;
        Ok(Self {
            id: ID::from(vec[0].clone()),
            name: vec[1].clone(),
            description: vec[2].clone(),
            owner_id: ID::from(vec[3].clone()),
            public: vec[4].parse::<bool>().unwrap(),
            tags: vec[5].split(' ').map(|s| s.to_string()).collect(),
            has_image: vec[6].parse::<bool>().unwrap(),
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

#[derive(InputObject, PartialDebug)]
pub struct CreateClassInput {
    #[graphql(validator(min_length = 1, max_length = 35))]
    pub name: String,
    #[graphql(validator(max_length = 200))]
    pub description: String,
    pub public: bool,
    pub tags: String,
    pub image: Option<Upload>,
}

impl CreateClassInput {
    pub fn into_active_model(
        self,
        owner_id: Uuid,
        has_image: bool,
    ) -> ::entity::class::ActiveModel {
        ::entity::class::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(self.name),
            description: Set(self.description),
            owner_id: Set(owner_id),
            public: Set(self.public),
            tags: Set(self.tags),
            has_image: Set(has_image),
            deleted_at: Set(None),
        }
    }
}

#[derive(PartialDebug, InputObject)]
pub struct UpdateClassInput {
    #[graphql(validator(min_length = 5, max_length = 35))]
    pub name: Option<String>,
    #[graphql(validator(max_length = 200))]
    pub description: Option<String>,
    pub public: Option<bool>,
    pub tags: Option<String>,
    pub image: Option<Upload>,
}

use sea_orm::NotSet;
impl UpdateClassInput {
    pub fn into_active_model(self) -> ::entity::class::ActiveModel {
        ::entity::class::ActiveModel {
            id: NotSet,
            name: option_to_active_value(self.name),
            description: option_to_active_value(self.description),
            owner_id: NotSet,
            public: option_to_active_value(self.public),
            tags: option_to_active_value(self.tags),
            has_image: NotSet,
            deleted_at: NotSet,
        }
    }
}

#[derive(Clone, Debug, SimpleObject)]
pub struct InviteObject {
    pub id: ID,
    pub class_id: ID,
    pub multiuse: bool,
    pub expires_at: Option<NaiveDateTime>,
}

impl From<::entity::invite::Model> for InviteObject {
    fn from(i: ::entity::invite::Model) -> Self {
        Self {
            id: ID::from(i.id),
            class_id: ID::from(i.class_id),
            multiuse: i.multiuse,
            expires_at: i.expires_at,
        }
    }
}

#[derive(Clone, Debug, InputObject)]
pub struct CreateInviteInput {
    pub class_id: ID,
    pub multiuse: bool,
    pub expires_at: Option<NaiveDateTime>,
}

impl CreateInviteInput {
    pub fn try_into_active_model(self) -> Result<::entity::invite::ActiveModel, AppError> {
        Ok(::entity::invite::ActiveModel {
            id: Set(Uuid::new_v4()),
            class_id: Set(Uuid::parse_str(&self.class_id)?),
            multiuse: Set(self.multiuse),
            expires_at: Set(self.expires_at),
            ..Default::default()
        })
    }
}
