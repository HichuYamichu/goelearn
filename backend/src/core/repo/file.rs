use ::entity::{file};

use async_graphql::dataloader::Loader;
use async_trait::async_trait;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FileRepo {
    conn: DatabaseConnection,
}

impl FileRepo {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn save_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr> {
        model.insert(&self.conn).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FilesByClassId(pub Uuid);

#[async_trait]
impl Loader<FilesByClassId> for FileRepo {
    type Value = Vec<file::Model>;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[FilesByClassId],
    ) -> Result<HashMap<FilesByClassId, Self::Value>, Self::Error> {
        let files = file::Entity::find()
            .filter(file::Column::ClassId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(&self.conn)
            .await
            .map_err(Arc::new)?;

        let mut res = HashMap::<_, _>::new();
        for f in files {
            res.entry(*keys.iter().find(|k| k.0 == f.class_id).unwrap())
                .or_insert_with(Vec::new)
                .push(f);
        }

        Ok(res)
    }
}
