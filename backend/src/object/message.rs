use async_graphql::{InputObject, SimpleObject, ID};
use chrono::{NaiveDateTime, Utc};
use sea_orm::Set;
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

#[derive(Clone, Debug, SimpleObject)]
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
