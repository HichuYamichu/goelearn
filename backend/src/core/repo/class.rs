use ::entity::{class, class::Entity as Class, membership, membership::Entity as Membership};
use async_graphql::dataloader::Loader;
use async_trait::async_trait;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ClassRepo {
    conn: DatabaseConnection,
}

impl ClassRepo {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ClassById(pub Uuid);

#[async_trait]
impl Loader<ClassById> for ClassRepo {
    type Value = class::Model;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[ClassById],
    ) -> Result<HashMap<ClassById, Self::Value>, Self::Error> {
        let classes = Class::find()
            .filter(class::Column::Id.is_in(keys.into_iter().map(|k| k.0).into_iter()))
            .all(&self.conn)
            .await
            .map_err(|e| Arc::new(e.into()))?;

        Ok(classes.into_iter().map(|c| (ClassById(c.id), c)).collect())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ClassByOwnerId(pub Uuid);

#[async_trait]
impl Loader<ClassByOwnerId> for ClassRepo {
    type Value = class::Model;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[ClassByOwnerId],
    ) -> Result<HashMap<ClassByOwnerId, Self::Value>, Self::Error> {
        let classes = Class::find()
            .filter(class::Column::OwnerId.is_in(keys.into_iter().map(|k| k.0).into_iter()))
            .all(&self.conn)
            .await
            .map_err(|e| Arc::new(e.into()))?;

        Ok(classes
            .into_iter()
            .map(|c| (ClassByOwnerId(c.owner_id), c))
            .collect())
    }
}
