use ::entity::channel;

use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct ChannelsByClassId(Uuid);

#[async_trait]
impl Loader<ChannelsByClassId> for DatabaseConnection {
    type Value = Vec<channel::Model>;
    type Error = Arc<DbErr>;

    #[instrument(skip(self), err(Debug))]
    async fn load(
        &self,
        keys: &[ChannelsByClassId],
    ) -> Result<HashMap<ChannelsByClassId, Self::Value>, Self::Error> {
        let channels = channel::Entity::find()
            .filter(channel::Column::ClassId.is_in(keys.iter().map(|k| k.0)))
            .all(self)
            .await
            .map_err(Arc::new)?;

        let mut res = HashMap::<_, _>::new();
        for key in keys.iter() {
            let e = res.entry(*key).or_insert_with(Vec::new);
            e.extend(channels.iter().filter(|c| c.class_id == key.0).cloned());
        }

        Ok(res)
    }
}

#[async_trait]
pub trait ChannelRepo {
    async fn create_channel(&self, model: channel::ActiveModel) -> Result<channel::Model, DbErr>;
    async fn find_by_class_id(
        &self,
        class_id: Uuid,
    ) -> Result<Option<Vec<channel::Model>>, Arc<DbErr>>;
}

#[async_trait]
impl ChannelRepo for DataLoader<DatabaseConnection> {
    #[instrument(skip(self), err(Debug))]
    async fn create_channel(&self, model: channel::ActiveModel) -> Result<channel::Model, DbErr> {
        model.insert(self.loader()).await
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_by_class_id(
        &self,
        class_id: Uuid,
    ) -> Result<Option<Vec<channel::Model>>, Arc<DbErr>> {
        let channels = self.load_one(ChannelsByClassId(class_id)).await?;
        Ok(channels)
    }
}
