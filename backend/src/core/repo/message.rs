use ::entity::{message, message::Entity as Message};



use chrono::NaiveDate;

use sea_orm::DatabaseConnection;
use sea_orm::*;


use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MessageRepo {
    conn: DatabaseConnection,
}

impl MessageRepo {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn create_message(
        &self,
        model: message::ActiveModel,
    ) -> Result<message::Model, DbErr> {
        let msg = model.insert(&self.conn).await?;

        Ok(msg)
    }

    pub async fn load_messages(
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
            .all(&self.conn)
            .await?;

        Ok(messages)
    }
}
