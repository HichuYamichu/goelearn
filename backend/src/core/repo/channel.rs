use ::entity::channel;

use async_graphql::dataloader::Loader;
use async_trait::async_trait;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ChannelsByClassId(pub Uuid);

#[async_trait]
impl Loader<ChannelsByClassId> for DatabaseConnection {
    type Value = Vec<channel::Model>;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[ChannelsByClassId],
    ) -> Result<HashMap<ChannelsByClassId, Self::Value>, Self::Error> {
        let channels = channel::Entity::find()
            .filter(channel::Column::ClassId.is_in(keys.iter().map(|k| k.0).into_iter()))
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
pub trait ChannelRepoExt {
    async fn create_channel(&self, model: channel::ActiveModel) -> Result<channel::Model, DbErr>;
}

#[async_trait]
impl ChannelRepoExt for DatabaseConnection {
    async fn create_channel(&self, model: channel::ActiveModel) -> Result<channel::Model, DbErr> {
        model.insert(self).await
    }
}
