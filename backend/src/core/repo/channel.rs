use ::entity::channel;

use async_graphql::dataloader::Loader;
use async_trait::async_trait;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ChannelRepo {
    conn: DatabaseConnection,
}

impl ChannelRepo {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn create_channel(
        &self,
        model: channel::ActiveModel,
    ) -> Result<channel::Model, DbErr> {
        Ok(model.insert(&self.conn).await?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ChannelByClassId(pub Uuid);

#[async_trait]
impl Loader<ChannelByClassId> for ChannelRepo {
    type Value = Vec<channel::Model>;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[ChannelByClassId],
    ) -> Result<HashMap<ChannelByClassId, Self::Value>, Self::Error> {
        let channels = channel::Entity::find()
            .filter(channel::Column::ClassId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(&self.conn)
            .await
            .map_err(Arc::new)?;

        let mut res = HashMap::<_, _>::new();
        for c in channels {
            res.entry(*keys.iter().find(|k| k.0 == c.class_id).unwrap())
                .or_insert_with(Vec::new)
                .push(c);
        }

        Ok(res)
    }
}
