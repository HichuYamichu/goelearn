use ::entity::{message, message::Entity as Message};
use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;

use chrono::{NaiveDate, NaiveDateTime};

use sea_orm::DatabaseConnection;
use sea_orm::*;

use tracing::{info, instrument};
use uuid::Uuid;

#[async_trait]
pub trait MessageRepo {
    async fn create_message(&self, model: message::ActiveModel) -> Result<message::Model, DbErr>;

    async fn load_messages(
        &self,
        channel_id: Uuid,
        start: NaiveDateTime,
        end: NaiveDateTime,
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
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<message::Model>, DbErr> {
        info!(
            "Loading messages for channel {} start {} end {}",
            channel_id, start, end
        );
        let condition = Condition::all()
            .add(message::Column::ChannelId.eq(channel_id))
            .add(message::Column::CreatedAt.gt(start))
            .add(message::Column::CreatedAt.lte(end));

        let messages = Message::find()
            .filter(condition)
            .order_by(message::Column::CreatedAt, Order::Asc)
            .all(self.loader())
            .await?;

        info!("Loaded {} messages", messages.len());

        Ok(messages)
    }
}
