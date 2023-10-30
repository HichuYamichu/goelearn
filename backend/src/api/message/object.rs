use crate::core::LoggedInGuard;
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject, ID,
};
use chrono::{NaiveDateTime, Utc};
use deadpool_redis::redis;
use redis::{FromRedisValue, RedisResult, RedisWrite, ToRedisArgs};
use sea_orm::{DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::api::user::{UserObject, UserRepo};

use crate::core::AppError;

#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
#[graphql(name = "Message")]
pub struct MessageObject {
    pub id: ID,
    pub content: String,
    pub author_id: ID,
    pub created_at: NaiveDateTime,
}

#[ComplexObject]
impl MessageObject {
    #[graphql(guard = "LoggedInGuard")]
    #[instrument(skip(self, ctx), err(Debug))]
    async fn author(&self, ctx: &Context<'_>) -> Result<UserObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let author_id = Uuid::parse_str(&self.author_id)?;
        let user = UserRepo::find_by_id(data_loader, author_id)
            .await?
            .expect("AuthorId should be valid");

        Ok(user.into())
    }
}

impl From<::entity::message::Model> for MessageObject {
    fn from(c: ::entity::message::Model) -> Self {
        Self {
            id: ID::from(c.id),
            content: c.content,
            author_id: ID::from(c.author_id),
            created_at: c.created_at,
        }
    }
}

impl ToRedisArgs for MessageObject {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let timestamp = self.created_at.to_string();
        let vec = vec![
            self.id.as_str(),
            &self.content,
            self.author_id.as_str(),
            &timestamp,
        ];
        vec.write_redis_args(out);
    }
}

impl FromRedisValue for MessageObject {
    fn from_redis_value(v: &redis::Value) -> RedisResult<Self> {
        let vec: Vec<String> = FromRedisValue::from_redis_value(v)?;
        Ok(Self {
            id: ID::from(vec[0].clone()),
            content: vec[1].clone(),
            author_id: ID::from(vec[2].clone()),
            created_at: NaiveDateTime::parse_from_str(&vec[3], "%Y-%m-%d %H:%M:%S%.f").unwrap(),
        })
    }
}

#[derive(InputObject, Debug)]
pub struct CreateMessageInput {
    #[graphql(validator(min_length = 1, max_length = 2000))]
    pub content: String,
    pub channel_id: ID,
    pub class_id: ID,
}

impl CreateMessageInput {
    pub fn try_into_active_model(
        self,
        author_id: Uuid,
    ) -> Result<::entity::message::ActiveModel, AppError> {
        Ok(::entity::message::ActiveModel {
            id: Set(Uuid::new_v4()),
            content: Set(self.content),
            channel_id: Set(Uuid::parse_str(self.channel_id.as_str())?),
            author_id: Set(author_id),
            created_at: Set(Utc::now().naive_utc()),
        })
    }
}
