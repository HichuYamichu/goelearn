use async_graphql::{InputObject, SimpleObject, ID};
use chrono::Utc;
use redis::{FromRedisValue, RedisResult, RedisWrite, ToRedisArgs};
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::AppError;

#[derive(InputObject)]
pub struct CreateMessageInput {
    pub content: String,
    pub channel_id: ID,
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

#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct MessageObject {
    pub id: ID,
    pub content: String,
}

impl From<::entity::message::Model> for MessageObject {
    fn from(c: ::entity::message::Model) -> Self {
        Self {
            id: ID::from(c.id),
            content: c.content,
        }
    }
}

impl ToRedisArgs for MessageObject {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let mut vec = Vec::new();
        vec.push(self.id.as_str());
        vec.push(&self.content);
        vec.write_redis_args(out);
    }
}

impl FromRedisValue for MessageObject {
    fn from_redis_value(v: &redis::Value) -> RedisResult<Self> {
        let vec: Vec<String> = FromRedisValue::from_redis_value(v)?;
        Ok(Self {
            id: ID::from(vec[0].clone()),
            content: vec[1].clone(),
        })
    }
}
