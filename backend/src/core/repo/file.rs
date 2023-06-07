use ::entity::{file, file::Entity as File};

use async_graphql::dataloader::Loader;
use async_trait::async_trait;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FilesByClassId(pub Uuid);

#[async_trait]
impl Loader<FilesByClassId> for DatabaseConnection {
    type Value = Vec<file::Model>;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[FilesByClassId],
    ) -> Result<HashMap<FilesByClassId, Self::Value>, Self::Error> {
        let files = file::Entity::find()
            .filter(file::Column::ClassId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(self)
            .await
            .map_err(Arc::new)?;

        let mut res = HashMap::<_, _>::new();
        for key in keys.iter() {
            let e = res.entry(*key).or_insert_with(Vec::new);
            e.extend(files.iter().filter(|f| f.class_id == key.0).cloned());
        }

        Ok(res)
    }
}

#[async_trait]
pub trait FileRepoExt {
    async fn save_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr>;
    async fn save_files(&self, models: Vec<file::ActiveModel>) -> Result<(), DbErr>;
    async fn find_many(&self, file_ids: Vec<Uuid>) -> Result<Vec<file::Model>, DbErr>;
    async fn delete_many(&self, file_ids: Vec<Uuid>) -> Result<(), DbErr>;
    async fn update_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr>;
}

#[async_trait]
impl FileRepoExt for DatabaseConnection {
    async fn save_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr> {
        model.insert(self).await
    }

    async fn save_files(&self, models: Vec<file::ActiveModel>) -> Result<(), DbErr> {
        File::insert_many(models).exec(self).await?;

        Ok(())
    }

    async fn find_many(&self, file_ids: Vec<Uuid>) -> Result<Vec<file::Model>, DbErr> {
        File::find()
            .filter(file::Column::Id.is_in(file_ids))
            .all(self)
            .await
    }
    async fn delete_many(&self, file_ids: Vec<Uuid>) -> Result<(), DbErr> {
        File::delete_many()
            .filter(file::Column::Id.is_in(file_ids))
            .exec(self)
            .await?;

        Ok(())
    }

    async fn update_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr> {
        model.update(self).await
    }
}
