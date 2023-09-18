use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;

use ::entity::assignment_submission_feedback;
use ::entity::sea_orm_active_enums::FileType;
use ::entity::{
    assignment, assignment::Entity as Assignment, assignment_file,
    assignment_file::Entity as AssignmentFile, assignment_submission,
    assignment_submission::Entity as AssignmentSubmission,
    assignment_submission_feedback::Entity as AssignmentSubmissionFeedback,
    assignment_submission_file, assignment_submission_file::Entity as AssignmentSubmissionFile,
    file, file::Entity as File,
};
use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::core::AppError;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct AssignmentsByClassId(Uuid);

#[async_trait]
impl Loader<AssignmentsByClassId> for DatabaseConnection {
    type Value = Vec<assignment::Model>;
    type Error = Arc<DbErr>;

    #[instrument(skip(self), err(Debug))]
    async fn load(
        &self,
        keys: &[AssignmentsByClassId],
    ) -> Result<HashMap<AssignmentsByClassId, Self::Value>, Self::Error> {
        let assignments = assignment::Entity::find()
            .filter(assignment::Column::ClassId.is_in(keys.iter().map(|k| k.0)))
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct AssignmentSubmissionsByAssignmentId(Uuid);

#[async_trait]
impl Loader<AssignmentSubmissionsByAssignmentId> for DatabaseConnection {
    type Value = Vec<assignment_submission::Model>;
    type Error = Arc<DbErr>;

    #[instrument(skip(self), err(Debug))]
    async fn load(
        &self,
        keys: &[AssignmentSubmissionsByAssignmentId],
    ) -> Result<HashMap<AssignmentSubmissionsByAssignmentId, Self::Value>, Self::Error> {
        let submissions = assignment_submission::Entity::find()
            .filter(assignment_submission::Column::AssignmentId.is_in(keys.iter().map(|k| k.0)))
            .all(self)
            .await
            .map_err(Arc::new)?;

        let mut res = HashMap::<_, _>::new();
        for key in keys.iter() {
            let e = res.entry(*key).or_insert_with(Vec::new);
            e.extend(
                submissions
                    .iter()
                    .filter(|a| a.assignment_id == key.0)
                    .cloned(),
            );
        }

        Ok(res)
    }
}

#[async_trait]
pub trait AssignmentRepo {
    async fn create_assignment(
        &self,
        model: assignment::ActiveModel,
        file_names: Vec<String>,
    ) -> Result<(assignment::Model, Vec<Uuid>), TransactionError<DbErr>>;

    async fn delete_assignment(
        &self,
        assignment_id: Uuid,
    ) -> Result<Vec<Uuid>, TransactionError<DbErr>>;

    async fn submit_assignment(
        &self,
        model: assignment_submission::ActiveModel,
        file_names: Vec<String>,
    ) -> Result<(uuid::Uuid, Vec<Uuid>), TransactionError<AppError>>;

    async fn create_assignment_submission_feedback(
        &self,
        model: assignment_submission_feedback::ActiveModel,
    ) -> Result<(), TransactionError<AppError>>;

    async fn update_assignment_submission(
        &self,
        model: assignment_submission::ActiveModel,
        new_file_names: Vec<String>,
        old_files: Vec<Uuid>,
    ) -> Result<(Uuid, Vec<Uuid>), TransactionError<AppError>>;

    async fn find_by_class_id(
        &self,
        class_id: Uuid,
    ) -> Result<Option<Vec<assignment::Model>>, Arc<DbErr>>;

    async fn find_submissions_by_assignment_id(
        &self,
        assignment_id: Uuid,
    ) -> Result<Option<Vec<assignment_submission::Model>>, Arc<DbErr>>;

    async fn find_feedback_by_assignment_submission_id(
        &self,
        assignment_submission_id: Uuid,
    ) -> Result<Option<assignment_submission_feedback::Model>, Arc<DbErr>>;

    async fn delete_assignment_feedback(
        &self,
        assignment_submission_feedback_id: Uuid,
    ) -> Result<(), TransactionError<AppError>>;

    async fn update_assignment(
        &self,
        model: assignment::ActiveModel,
        new_file_names: Vec<String>,
        old_files: Vec<Uuid>,
    ) -> Result<(assignment::Model, Vec<Uuid>), TransactionError<DbErr>>;
}

#[async_trait]
impl AssignmentRepo for DataLoader<DatabaseConnection> {
    #[instrument(skip(self), err(Debug))]
    async fn create_assignment(
        &self,
        model: assignment::ActiveModel,
        file_names: Vec<String>,
    ) -> Result<(assignment::Model, Vec<Uuid>), TransactionError<DbErr>> {
        let (assignment, file_ids) =
            self.loader()
                .transaction::<_, (assignment::Model, Vec<Uuid>), DbErr>(|txn| {
                    Box::pin(async move {
                        let assignment = model.insert(txn).await?;

                        let condition = Condition::all()
                            .add(file::Column::Name.eq("Assignment files"))
                            .add(file::Column::ClassId.eq(assignment.class_id));

                        let assignment_dir =
                            File::find().filter(condition).one(txn).await?.expect(
                                "Assignment files is always created when a class is created",
                            );

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

                        let assignment_files = file_ids
                            .iter()
                            .map(|id| assignment_file::ActiveModel {
                                assignment_id: Set(assignment.id),
                                file_id: Set(*id),
                            })
                            .collect::<Vec<_>>();

                        if !assignment_files.is_empty() {
                            AssignmentFile::insert_many(assignment_files)
                                .exec(txn)
                                .await?;
                        }

                        Ok((assignment, file_ids))
                    })
                })
                .await?;

        Ok((assignment, file_ids))
    }

    #[instrument(skip(self), err(Debug))]
    async fn delete_assignment(
        &self,
        assignment_id: Uuid,
    ) -> Result<Vec<Uuid>, TransactionError<DbErr>> {
        let file_ids = self
            .loader()
            .transaction::<_, Vec<Uuid>, DbErr>(|txn| {
                Box::pin(async move {
                    let files = AssignmentFile::find()
                        .filter(assignment_file::Column::AssignmentId.eq(assignment_id))
                        .all(txn)
                        .await?;

                    AssignmentFile::delete_many()
                        .filter(assignment_file::Column::AssignmentId.eq(assignment_id))
                        .exec(txn)
                        .await?;

                    let assignment_submissions = AssignmentSubmission::find()
                        .filter(assignment_submission::Column::AssignmentId.eq(assignment_id))
                        .all(txn)
                        .await?;

                    let assignment_submission_files = AssignmentSubmissionFile::find()
                        .filter(
                            assignment_submission_file::Column::AssignmentSubmissionId
                                .is_in(assignment_submissions.iter().map(|s| s.id)),
                        )
                        .all(txn)
                        .await?;

                    AssignmentSubmissionFile::delete_many()
                        .filter(
                            assignment_submission_file::Column::AssignmentSubmissionId
                                .is_in(assignment_submissions.iter().map(|s| s.id)),
                        )
                        .exec(txn)
                        .await?;

                    let assignment_submissions = AssignmentSubmission::find()
                        .filter(assignment_submission::Column::AssignmentId.eq(assignment_id))
                        .all(txn)
                        .await?;

                    AssignmentSubmissionFeedback::delete_many()
                        .filter(
                            assignment_submission_feedback::Column::AssignmentSubmissionId
                                .is_in(assignment_submissions.iter().map(|s| s.id)),
                        )
                        .exec(txn)
                        .await?;

                    AssignmentSubmission::delete_many()
                        .filter(assignment_submission::Column::AssignmentId.eq(assignment_id))
                        .exec(txn)
                        .await?;

                    let model = assignment::ActiveModel {
                        id: Set(assignment_id),
                        ..Default::default()
                    };

                    model.delete(txn).await?;

                    let mut ids = files.into_iter().map(|m| m.file_id).collect::<Vec<_>>();
                    ids.extend(assignment_submission_files.into_iter().map(|m| m.file_id));

                    Ok(ids)
                })
            })
            .await?;

        Ok(file_ids)
    }

    #[instrument(skip(self), err(Debug))]
    async fn submit_assignment(
        &self,
        model: assignment_submission::ActiveModel,
        file_names: Vec<String>,
    ) -> Result<(uuid::Uuid, Vec<Uuid>), TransactionError<AppError>> {
        let (class_id, file_ids) =
            self.loader()
                .transaction::<_, (uuid::Uuid, Vec<Uuid>), AppError>(|txn| {
                    Box::pin(async move {
                        let assignment_submission = model.insert(txn).await?;

                        let assignment_id = assignment_submission.assignment_id;
                        let assignment = Assignment::find_by_id(assignment_id)
                            .one(txn)
                            .await?
                            .ok_or(AppError::not_found(
                                "assignment not found",
                                "assignment",
                                "id",
                                assignment_id.to_string().as_str(),
                            ))?;

                        let condition = Condition::all()
                            .add(file::Column::Name.eq("Assignment submission files"))
                            .add(file::Column::ClassId.eq(assignment.class_id));

                        let assignment_dir =
                            File::find().filter(condition).one(txn).await?.expect(
                                "Assignment files is always created when a class is created",
                            );

                        let condition = Condition::all()
                            .add(file::Column::Name.eq(assignment.name.clone()))
                            .add(file::Column::ClassId.eq(assignment.class_id));

                        let this_assignment_dir = File::find().filter(condition).one(txn).await?;
                        let this_assignment_dir_id = match this_assignment_dir {
                            Some(dir) => dir.id,
                            None => {
                                let res = File::insert(file::ActiveModel {
                                    id: Set(Uuid::new_v4()),
                                    name: Set(assignment.name),
                                    class_id: Set(assignment.class_id),
                                    parent_id: Set(Some(assignment_dir.id)),
                                    public: Set(false),
                                    file_type: Set(FileType::Directory),
                                    message_id: Set(None),
                                })
                                .exec(txn)
                                .await?;
                                res.last_insert_id
                            }
                        };

                        // TODO: finish this. Submission should be put in a folder with the submitter name
                        let files = file_names
                            .into_iter()
                            .map(|name| file::ActiveModel {
                                id: Set(Uuid::new_v4()),
                                name: Set(name),
                                class_id: Set(assignment.class_id),
                                parent_id: Set(Some(this_assignment_dir_id)),
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

                        let assignment_submission_files = file_ids
                            .iter()
                            .map(|id| assignment_submission_file::ActiveModel {
                                id: Set(Uuid::new_v4()),
                                assignment_submission_id: Set(assignment_submission.id),
                                file_id: Set(*id),
                            })
                            .collect::<Vec<_>>();

                        if !assignment_submission_files.is_empty() {
                            AssignmentSubmissionFile::insert_many(assignment_submission_files)
                                .exec(txn)
                                .await?;
                        }

                        Ok((assignment.class_id, file_ids))
                    })
                })
                .await?;

        Ok((class_id, file_ids))
    }

    #[instrument(skip(self), err(Debug))]
    async fn update_assignment_submission(
        &self,
        model: assignment_submission::ActiveModel,
        new_file_names: Vec<String>,
        old_files: Vec<Uuid>,
    ) -> Result<(Uuid, Vec<Uuid>), TransactionError<AppError>> {
        let (class_id, file_ids) = self
            .loader()
            .transaction::<_, (uuid::Uuid, Vec<Uuid>), AppError>(|txn| {
                Box::pin(async move {
                    let assignment_submission = model.update(txn).await?;

                    AssignmentSubmissionFile::delete_many()
                        .filter(
                            assignment_file::Column::FileId.is_in(old_files.iter().map(|id| *id)),
                        )
                        .exec(txn)
                        .await?;

                    File::delete_many()
                        .filter(file::Column::Id.is_in(old_files.iter().map(|id| *id)))
                        .exec(txn)
                        .await?;

                    let assignment_id = assignment_submission.assignment_id;
                    let assignment = Assignment::find_by_id(assignment_id)
                        .one(txn)
                        .await?
                        .ok_or(AppError::not_found(
                            "assignment not found",
                            "assignment",
                            "id",
                            assignment_id.to_string().as_str(),
                        ))?;

                    let condition = Condition::all()
                        .add(file::Column::Name.eq("Assignment submission files"))
                        .add(file::Column::ClassId.eq(assignment.class_id));

                    let assignment_dir = File::find()
                        .filter(condition)
                        .one(txn)
                        .await?
                        .expect("Assignment files is always created when a class is created");

                    let condition = Condition::all()
                        .add(file::Column::Name.eq(assignment.name.clone()))
                        .add(file::Column::ClassId.eq(assignment.class_id));

                    let this_assignment_dir = File::find().filter(condition).one(txn).await?;
                    let this_assignment_dir_id = match this_assignment_dir {
                        Some(dir) => dir.id,
                        None => {
                            let res = File::insert(file::ActiveModel {
                                id: Set(Uuid::new_v4()),
                                name: Set(assignment.name),
                                class_id: Set(assignment.class_id),
                                parent_id: Set(Some(assignment_dir.id)),
                                public: Set(false),
                                file_type: Set(FileType::Directory),
                                message_id: Set(None),
                            })
                            .exec(txn)
                            .await?;
                            res.last_insert_id
                        }
                    };

                    // TODO: finish this. Submission should be put in a folder with the submitter name
                    let files = new_file_names
                        .into_iter()
                        .map(|name| file::ActiveModel {
                            id: Set(Uuid::new_v4()),
                            name: Set(name),
                            class_id: Set(assignment.class_id),
                            parent_id: Set(Some(this_assignment_dir_id)),
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

                    let assignment_submission_files = file_ids
                        .iter()
                        .map(|id| assignment_submission_file::ActiveModel {
                            id: Set(Uuid::new_v4()),
                            assignment_submission_id: Set(assignment_submission.id),
                            file_id: Set(*id),
                        })
                        .collect::<Vec<_>>();

                    if !assignment_submission_files.is_empty() {
                        AssignmentSubmissionFile::insert_many(assignment_submission_files)
                            .exec(txn)
                            .await?;
                    }

                    Ok((assignment.class_id, file_ids))
                })
            })
            .await?;

        Ok((class_id, file_ids))
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_by_class_id(
        &self,
        class_id: Uuid,
    ) -> Result<Option<Vec<assignment::Model>>, Arc<DbErr>> {
        let assignments = self.load_one(AssignmentsByClassId(class_id)).await?;
        Ok(assignments)
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_submissions_by_assignment_id(
        &self,
        assignment_id: Uuid,
    ) -> Result<Option<Vec<assignment_submission::Model>>, Arc<DbErr>> {
        let submissions = self
            .load_one(AssignmentSubmissionsByAssignmentId(assignment_id))
            .await?;

        Ok(submissions)
    }

    #[instrument(skip(self), err(Debug))]
    async fn create_assignment_submission_feedback(
        &self,
        model: assignment_submission_feedback::ActiveModel,
    ) -> Result<(), TransactionError<AppError>> {
        let feedback = model.save(self.loader()).await?;
        Ok(())
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_feedback_by_assignment_submission_id(
        &self,
        assignment_submission_id: Uuid,
    ) -> Result<Option<assignment_submission_feedback::Model>, Arc<DbErr>> {
        let feedback = assignment_submission_feedback::Entity::find()
            .filter(
                assignment_submission_feedback::Column::AssignmentSubmissionId
                    .eq(assignment_submission_id),
            )
            .one(self.loader())
            .await?;

        Ok(feedback)
    }

    #[instrument(skip(self), err(Debug))]
    async fn delete_assignment_feedback(
        &self,
        assignment_submission_feedback_id: Uuid,
    ) -> Result<(), TransactionError<AppError>> {
        let model = assignment_submission_feedback::ActiveModel {
            id: Set(assignment_submission_feedback_id),
            ..Default::default()
        };

        model.delete(self.loader()).await?;

        Ok(())
    }

    async fn update_assignment(
        &self,
        model: assignment::ActiveModel,
        new_file_names: Vec<String>,
        old_files: Vec<Uuid>,
    ) -> Result<(assignment::Model, Vec<Uuid>), TransactionError<DbErr>> {
        let (assignment, files) = self
            .loader()
            .transaction::<_, (assignment::Model, Vec<Uuid>), DbErr>(|txn| {
                Box::pin(async move {
                    let assignment = model.update(txn).await?;

                    AssignmentFile::delete_many()
                        .filter(
                            assignment_file::Column::FileId.is_in(old_files.iter().map(|id| *id)),
                        )
                        .exec(txn)
                        .await?;

                    File::delete_many()
                        .filter(file::Column::Id.is_in(old_files.iter().map(|id| *id)))
                        .exec(txn)
                        .await?;

                    let condition = Condition::all()
                        .add(file::Column::Name.eq("Assignment files"))
                        .add(file::Column::ClassId.eq(assignment.class_id));

                    let assignment_dir = File::find()
                        .filter(condition)
                        .one(txn)
                        .await?
                        .expect("Assignment files is always created when a class is created");

                    let files = new_file_names
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

                    let assignment_files = file_ids
                        .iter()
                        .map(|id| assignment_file::ActiveModel {
                            assignment_id: Set(assignment.id),
                            file_id: Set(*id),
                        })
                        .collect::<Vec<_>>();

                    if !assignment_files.is_empty() {
                        AssignmentFile::insert_many(assignment_files)
                            .exec(txn)
                            .await?;
                    }
                    Ok((assignment, file_ids))
                })
            })
            .await?;

        Ok((assignment, files))
    }
}
