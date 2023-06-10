use ::entity::{
    assignment_file, assignment_file::Entity as AssignmentFile, file, file::Entity as File,
};

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FilesByAssignmentId(pub Uuid);

#[async_trait]
impl Loader<FilesByAssignmentId> for DatabaseConnection {
    type Value = Vec<file::Model>;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[FilesByAssignmentId],
    ) -> Result<HashMap<FilesByAssignmentId, Self::Value>, Self::Error> {
        let files = AssignmentFile::find()
            .filter(
                assignment_file::Column::AssignmentId.is_in(keys.iter().map(|k| k.0).into_iter()),
            )
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
pub trait FileRepoExt {
    async fn save_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr>;
    async fn save_files(&self, models: Vec<file::ActiveModel>) -> Result<(), DbErr>;
    async fn find_many(&self, file_ids: Vec<Uuid>) -> Result<Vec<file::Model>, DbErr>;
    async fn find_many_with_nested(&self, file_ids: Vec<Uuid>) -> Result<Vec<file::Model>, DbErr>;
    async fn delete_many_with_nested(&self, file_ids: Vec<Uuid>) -> Result<(), DbErr>;
    async fn update_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr>;
}

#[derive(Debug, FromQueryResult)]
pub struct TestFile {
    pub id: Uuid,
    pub name: String,
    pub public: bool,
    pub file_type: ::entity::sea_orm_active_enums::FileType,
    pub parent_id: Option<Uuid>,
    pub class_id: Uuid,
    pub message_id: Option<Uuid>,
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

    async fn find_many_with_nested(&self, file_ids: Vec<Uuid>) -> Result<Vec<file::Model>, DbErr> {
        let ids = file_ids.into_iter().map(|id| id.into()).collect::<Vec<_>>();

        // let files = File::find()
        //     .from_raw_sql(Statement::from_sql_and_values(
        //         DbBackend::Postgres,
        //         r#"
        //     SELECT *
        //     FROM file;
        //     "#,
        //         [],
        //     ))
        //     .into_model::<Test>()
        //     .all(self)
        //     .await?;
        let files = File::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
                WITH RECURSIVE file_hierarchy AS (
                    SELECT "id", "name", "public", CAST(file_type AS file_type), parent_id, message_id, class_id
                    FROM public.file
                    WHERE id IN ($1)
    
                    UNION ALL
    
                    SELECT f.id, f.name, f.public, CAST(f.file_type AS file_type), f.parent_id, f.message_id, f.class_id
                    FROM public.file f
                    INNER JOIN file_hierarchy fh ON f.parent_id = fh.id
                )
                SELECT *
                FROM file_hierarchy;
            "#,
                ids,
            ))
            .all(self)
            .await?;
        dbg!(files);

        tracing::error!("after query");

        let files = vec![];

        Ok(files)
    }

    async fn delete_many_with_nested(&self, file_ids: Vec<Uuid>) -> Result<(), DbErr> {
        let ids = file_ids.into_iter().map(|id| id.into()).collect::<Vec<_>>();

        self.execute(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            WITH RECURSIVE file_hierarchy AS (
                SELECT "id", "name", "public", CAST(file_type AS file_type), parent_id, message_id, class_id
                FROM public.file
                WHERE id IN ($1)

                UNION ALL

                SELECT f.id, f.name, f.public, CAST(f.file_type AS file_type), f.parent_id, f.message_id, f.class_id
                FROM public.file f
                INNER JOIN file_hierarchy fh ON f.parent_id = fh.id
            )
            SELECT "id", "name", "public", CAST(file_type AS file_type), parent_id, message_id, class_id
            FROM file_hierarchy;
            "#,
            ids,
        ))
        .await?;

        Ok(())
    }

    async fn update_file(&self, model: file::ActiveModel) -> Result<file::Model, DbErr> {
        model.update(self).await
    }
}

#[derive(Debug, FromQueryResult)]
struct Test {
    id: uuid::Uuid,
    file_type: ::entity::sea_orm_active_enums::FileType,
}

// impl FromQueryResult for Test {
//     fn from_query_result(result: &QueryResult, pre: &str) -> Result<Self, DbErr> {
//         // let t = result.try_get::<::entity::sea_orm_active_enums::FileType>(pre, "file_type")?;
//         // dbg!(t);
//         Ok(Self {
//             id: result.try_get(pre, "id")?,
//             file_type: ::entity::sea_orm_active_enums::FileType::File,
//         })
//     }
// }
