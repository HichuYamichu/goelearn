use ::entity::{channel};

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
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ChannelByClassId(pub Uuid);

#[async_trait]
impl Loader<ChannelByClassId> for ChannelRepo {
    type Value = channel::Model;
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

        Ok(channels
            .into_iter()
            .map(|c| (ChannelByClassId(c.class_id), c))
            .collect())
    }
}
