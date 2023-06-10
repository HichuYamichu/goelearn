use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject, ID,
};
use chrono::Utc;
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
}

#[ComplexObject]
impl MessageObject {
    // TODO: Websocket authentication needed
    // #[graphql(guard = "LoggedInGuard")]
    #[instrument(skip(self, ctx), err)]
    async fn author(&self, ctx: &Context<'_>) -> Result<UserObject, AppError> {
        tracing::warn!("author_id: {:?}", self.author_id);
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
        }
    }
}

impl ToRedisArgs for MessageObject {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let vec = vec![self.id.as_str(), &self.content, self.author_id.as_str()];
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
        })
    }
}

#[derive(InputObject, Debug)]
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
