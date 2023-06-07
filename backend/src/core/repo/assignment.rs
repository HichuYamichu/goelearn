use async_graphql::dataloader::Loader;
use async_trait::async_trait;

use ::entity::sea_orm_active_enums::FileType;
use ::entity::{assignment, assignment::Entity as Assignment, file, file::Entity as File};
use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AssignmentsByClassId(pub Uuid);

#[async_trait]
impl Loader<AssignmentsByClassId> for DatabaseConnection {
    type Value = Vec<assignment::Model>;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[AssignmentsByClassId],
    ) -> Result<HashMap<AssignmentsByClassId, Self::Value>, Self::Error> {
        let assignments = assignment::Entity::find()
            .filter(assignment::Column::ClassId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(self)
            .await
            .map_err(Arc::new)?;

        let mut res = HashMap::<_, _>::new();
        for key in keys.iter() {
            let e = res.entry(*key).or_insert_with(Vec::new);
            e.extend(assignments.iter().filter(|a| a.class_id == key.0).cloned());
        }

        Ok(res)
    }
}

#[async_trait]
pub trait AssignmentRepoExt {
    async fn create_assignment(
        &self,
        model: assignment::ActiveModel,
        file_names: Vec<String>,
    ) -> Result<(assignment::Model, Vec<Uuid>), TransactionError<DbErr>>;
}

#[async_trait]
impl AssignmentRepoExt for DatabaseConnection {
    #[instrument(skip(self), err)]
    async fn create_assignment(
        &self,
        model: assignment::ActiveModel,
        file_names: Vec<String>,
    ) -> Result<(assignment::Model, Vec<Uuid>), TransactionError<DbErr>> {
        let (assignment, file_ids) = self
            .transaction::<_, (assignment::Model, Vec<Uuid>), DbErr>(|txn| {
                Box::pin(async move {
                    let assignment = model.insert(txn).await?;

                    let condition = Condition::all()
                        .add(file::Column::Name.eq("Assignment files"))
                        .add(file::Column::ClassId.eq(assignment.class_id));

                    let assignment_dir = File::find()
                        .filter(condition)
                        .one(txn)
                        .await?
                        .expect("Assignment files is always created when a class is created");

                    let files = file_names
                        .into_iter()
                        .map(|name| file::ActiveModel {
                            id: Set(Uuid::new_v4()),
                            name: Set(name),
                            class_id: Set(assignment.class_id),
                            parent_id: Set(Some(assignment_dir.id)),
                            public: Set(true),
                            file_type: Set(FileType::File),
                            message_id: Set(None),
                        })
                        .collect::<Vec<_>>();
                    let file_ids = files
                        .iter()
                        .map(|f| f.id.clone().unwrap())
                        .collect::<Vec<_>>();

                    if !file_ids.is_empty() {
                        File::insert_many(files).exec(txn).await?;
                    }

                    Ok((assignment, file_ids))
                })
            })
            .await?;

        Ok((assignment, file_ids))
    }
}
