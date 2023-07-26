use ::entity::{
    assignment_file, assignment_file::Entity as AssignmentFile, file, file::Entity as File,
};

use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;

use migration::ArrayType;
use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct FilesByClassId(Uuid);

#[async_trait]
impl Loader<FilesByClassId> for DatabaseConnection {
    type Value = Vec<file::Model>;
    type Error = Arc<DbErr>;

    #[instrument(skip(self), err)]
    async fn load(
        &self,
        keys: &[FilesByClassId],
    ) -> Result<HashMap<FilesByClassId, Self::Value>, Self::Error> {
        let files = file::Entity::find()
            .filter(file::Column::ClassId.is_in(keys.iter().map(|k| k.0)))
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct FilesByAssignmentId(Uuid);

#[async_trait]
impl Loader<FilesByAssignmentId> for DatabaseConnection {
    type Value = Vec<file::Model>;
    type Error = Arc<DbErr>;

    #[instrument(skip(self), err)]
    async fn load(
        &self,
        keys: &[FilesByAssignmentId],
    ) -> Result<HashMap<FilesByAssignmentId, Self::Value>, Self::Error> {
        let files = AssignmentFile::find()
            .filter(assignment_file::Column::AssignmentId.is_in(keys.iter().map(|k| k.0)))
            .find_also_related(File)
            .all(self)
            .await
            .map_err(Arc::new)?
            .into_iter()
            .map(|(af, f)| (af, f.expect("AssignmentFile to File is not optional")))
            .collect::<Vec<_>>();

        let mut res = HashMap::<_, _>::new();
        for key in keys.iter() {
            let e = res.entry(*key).or_insert_with(Vec::new);
            e.extend(
                files
                    .iter()
                    .filter(|f| f.0.assignment_id == key.0)
                    .map(|(_, f)| f)
                    .cloned(),
            );
        }

        Ok(res)
    }
}

#[async_trait]
pub trait FileRepo {
    async fn save_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr>;
    async fn save_files(&self, models: Vec<file::ActiveModel>) -> Result<(), DbErr>;
    async fn find_many(&self, file_ids: Vec<Uuid>) -> Result<Vec<file::Model>, DbErr>;
    async fn find_many_with_nested(&self, file_ids: Vec<Uuid>) -> Result<Vec<file::Model>, DbErr>;
    async fn delete_many_with_nested(&self, file_ids: Vec<Uuid>) -> Result<(), DbErr>;
    async fn update_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr>;

    async fn find_by_class_id(
        &self,
        class_id: Uuid,
    ) -> Result<Option<Vec<file::Model>>, Arc<DbErr>>;

    async fn find_by_assignment_id(
        &self,
        assignment_id: Uuid,
    ) -> Result<Option<Vec<file::Model>>, Arc<DbErr>>;
}

#[async_trait]
impl FileRepo for DataLoader<DatabaseConnection> {
    #[instrument(skip(self), err)]
    async fn save_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr> {
        model.insert(self.loader()).await
    }

    #[instrument(skip(self), err)]
    async fn save_files(&self, models: Vec<file::ActiveModel>) -> Result<(), DbErr> {
        File::insert_many(models).exec(self.loader()).await?;

        Ok(())
    }

    #[instrument(skip(self), err)]
    async fn find_many(&self, file_ids: Vec<Uuid>) -> Result<Vec<file::Model>, DbErr> {
        File::find()
            .filter(file::Column::Id.is_in(file_ids))
            .all(self.loader())
            .await
    }

    #[instrument(skip(self), err)]
    async fn find_many_with_nested(&self, ids: Vec<Uuid>) -> Result<Vec<file::Model>, DbErr> {
        let tuple = Value::Array(
            ArrayType::Uuid,
            Some(Box::new(
                ids.into_iter().map(|id| id.into()).collect::<Vec<_>>(),
            )),
        );

        let files = File::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
                WITH RECURSIVE file_hierarchy AS (
                    SELECT "id", "name", "public", CAST(file_type AS TEXT), parent_id, message_id, class_id
                    FROM public.file
                    WHERE id = ANY($1::uuid[])
    
                    UNION ALL
    
                    SELECT f.id, f.name, f.public, CAST(f.file_type AS TEXT), f.parent_id, f.message_id, f.class_id
                    FROM public.file f
                    INNER JOIN file_hierarchy fh ON f.parent_id = fh.id
                )
                SELECT *
                FROM file_hierarchy;
            "#,
            [tuple],
            ))
            .all(self.loader())
            .await?;

        dbg!(&files);

        Ok(files)
    }

    #[instrument(skip(self), err)]
    async fn delete_many_with_nested(&self, file_ids: Vec<Uuid>) -> Result<(), DbErr> {
        let ids = file_ids.into_iter().map(|id| id.into()).collect::<Vec<_>>();

        self.loader().execute(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            WITH RECURSIVE file_hierarchy AS (
                SELECT "id", "name", "public", CAST(file_type AS TEXT), parent_id, message_id, class_id
                FROM public.file
                WHERE id IN ($1)

                UNION ALL

                SELECT f.id, f.name, f.public, CAST(f.file_type AS TEXT), f.parent_id, f.message_id, f.class_id
                FROM public.file f
                INNER JOIN file_hierarchy fh ON f.parent_id = fh.id
            )
            DELETE FROM file_hierarchy;
            "#,
            ids,
        ))
        .await?;

        Ok(())
    }

    #[instrument(skip(self), err)]
    async fn update_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr> {
        model.update(self.loader()).await
    }

    #[instrument(skip(self), err)]
    async fn find_by_class_id(
        &self,
        class_id: Uuid,
    ) -> Result<Option<Vec<file::Model>>, Arc<DbErr>> {
        let files = self.load_one(FilesByClassId(class_id)).await?;
        Ok(files)
    }

    #[instrument(skip(self), err)]
    async fn find_by_assignment_id(
        &self,
        assignment_id: Uuid,
    ) -> Result<Option<Vec<file::Model>>, Arc<DbErr>> {
        let files = self.load_one(FilesByAssignmentId(assignment_id)).await?;
        Ok(files)
    }
}
