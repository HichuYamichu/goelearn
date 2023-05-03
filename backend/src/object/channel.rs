use std::str::FromStr;

use crate::core::LoggedInGuard;
use crate::core::{repo::message::MessageRepo, AppError};
use crate::graphql::make_messages_connection;
use async_graphql::connection::{self, Connection, Edge, EmptyFields};
use async_graphql::InputObject;
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, SimpleObject, ID};

use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use chrono::NaiveDate;

use sea_orm::Set;
use uuid::Uuid;

use super::MessageObject;

#[derive(Clone, Debug, InputObject)]
pub struct CreateChannelInput {
    pub name: String,
    pub description: Option<String>,
    pub class_id: ID,
}

impl CreateChannelInput {
    pub fn try_into_active_model(self) -> Result<::entity::channel::ActiveModel, AppError> {
        Ok(::entity::channel::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(self.name),
            description: Set(self.description),
            class_id: Set(Uuid::parse_str(self.class_id.as_str())?),
        })
    }
}

#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
#[graphql(name = "Channel")]
pub struct ChannelObject {
    pub id: ID,
    pub name: String,
    pub description: Option<String>,
}

impl From<::entity::channel::Model> for ChannelObject {
    fn from(c: ::entity::channel::Model) -> Self {
        Self {
            id: ID::from(c.id),
            name: c.name,
            description: c.description,
        }
    }
}

#[ComplexObject]
impl ChannelObject {
    #[graphql(guard = "LoggedInGuard")]
    async fn messages(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, MessageObject>, async_graphql::Error> {
        let message_repo = ctx.data_unchecked::<DataLoader<MessageRepo>>();

        make_messages_connection(
            message_repo,
            Uuid::parse_str(&self.id)?,
            after,
            before,
            first,
            last,
        )
        .await
    }
}
