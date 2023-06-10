use ::entity::{message, message::Entity as Message};
use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;

use chrono::NaiveDate;

use sea_orm::DatabaseConnection;
use sea_orm::*;

use tracing::instrument;
use uuid::Uuid;

#[async_trait]
pub trait MessageRepo {
    async fn create_message(&self, model: message::ActiveModel) -> Result<message::Model, DbErr>;

    async fn load_messages(
        &self,
        channel_id: Uuid,
        after: Option<NaiveDate>,
        before: Option<NaiveDate>,
        first: usize,
        _last: usize,
    ) -> Result<Vec<message::Model>, DbErr>;
}

#[async_trait]
impl MessageRepo for DataLoader<DatabaseConnection> {
    #[instrument(skip(self), err)]
    async fn create_message(&self, model: message::ActiveModel) -> Result<message::Model, DbErr> {
        let msg = model.insert(self.loader()).await?;
        Ok(msg)
    }

    #[instrument(skip(self), err)]
    async fn load_messages(
        &self,
        channel_id: Uuid,
        after: Option<NaiveDate>,
        before: Option<NaiveDate>,
        first: usize,
        _last: usize,
    ) -> Result<Vec<message::Model>, DbErr> {
        let condition = Condition::all()
            .add(message::Column::ChannelId.eq(channel_id))
            .add_option(after.map(|after| message::Column::CreatedAt.gt(after)))
            .add_option(before.map(|before| message::Column::CreatedAt.lt(before)));

        let messages = Message::find()
            .filter(condition)
            .order_by(message::Column::CreatedAt, Order::Desc)
            .limit(first as u64)
            .all(self.loader())
            .await?;

        Ok(messages)
    }
}
