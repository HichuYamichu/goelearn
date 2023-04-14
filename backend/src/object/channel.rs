use std::str::FromStr;

use crate::core::LoggedInGuard;
use crate::core::{repo::message::MessageRepo, AppError};
use async_graphql::connection::{self, Connection, DefaultConnectionName, Edge, EmptyFields};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject, ID,
};
use base64::engine::general_purpose;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use chrono::NaiveDate;
use sea_orm::Set;
use uuid::Uuid;

use super::MessageObject;

#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
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
        let message_repo = ctx.data::<DataLoader<MessageRepo>>().unwrap();

        let wtf = connection::query(
            after,
            before,
            first,
            last,
            |after: Option<String>, before, first, last| async move {
                // TODO: support last
                // TODO: unwrap
                let after = after
                    .map(|after| BASE64_STANDARD_NO_PAD.decode(after.as_bytes()).unwrap())
                    .map(|after| String::from_utf8(after).unwrap())
                    .map(|after| NaiveDate::from_str(after.as_ref()).unwrap())
                    .unwrap_or(NaiveDate::MIN);

                let before = before
                    .map(|before| BASE64_STANDARD_NO_PAD.decode(before.as_bytes()).unwrap())
                    .map(|before| String::from_utf8(before).unwrap())
                    .map(|before| NaiveDate::from_str(before.as_ref()))
                    .map(|a| a)
                    .unwrap_or(Ok(NaiveDate::MAX))?;

                let first = first.unwrap_or(20);
                let last = last.unwrap_or(20);

                let messages = message_repo
                    .loader()
                    .load_messages(Uuid::parse_str(&self.id)?, None, None, first, last)
                    .await?;

                let edges: Vec<Edge<String, MessageObject, EmptyFields>> = messages
                    .iter()
                    .map(|m| {
                        Edge::new(
                            BASE64_STANDARD_NO_PAD
                                .encode(m.created_at.to_string().as_bytes())
                                .into(),
                            MessageObject::from(m.clone()),
                        )
                    })
                    .collect();

                // TODO: impl has next page
                let mut connection = Connection::new(false, false);
                connection.edges.extend(edges);

                Ok::<_, AppError>(connection)
            },
        )
        .await;

        wtf
    }
}
