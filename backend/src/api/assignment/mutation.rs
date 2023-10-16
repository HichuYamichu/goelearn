use crate::api::class;
use crate::api::class::AssignmentDeleteInfo;
use crate::api::class::ClassResourceCreate;
use crate::api::class::ClassResourceDelete;
use crate::api::class::ClassResourceUpdate;
use crate::api::class::CLASS_RESOURCE_CREATED;
use crate::api::class::CLASS_RESOURCE_DELETED;
use crate::api::class::CLASS_RESOURCE_UPDATED;
use crate::core::AppError;
use crate::core::Claims;
use crate::core::LoggedInGuard;
use async_graphql::ID;
use async_graphql::{dataloader::DataLoader, Context, Object};
use deadpool_redis::redis::AsyncCommands;
use entity::assignment;
use sea_orm::DatabaseConnection;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing::instrument;
use uuid::Uuid;

use super::object::CreateAssignmanetSubmissionFeedbackInput;
use super::object::UpdateAssignmanetSubmissionFeedbackInput;
use super::object::UpdateAssignmentSubmissionInput;
use super::object::{CreateAssignmentInput, SubmitAssignmentInput, UpdateAssignmentInput};
use super::repo::DeleteSubmissionResult;
use super::{AssignmentObject, AssignmentRepo};

#[derive(Default)]
pub struct AssignmentMutation;

#[Object]
impl AssignmentMutation {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_assignment(
        &self,
        ctx: &Context<'_>,
        input: CreateAssignmentInput,
    ) -> Result<AssignmentObject, async_graphql::Error> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;

        let (model, files) = input.try_into_active_model()?;
        let files = files
            .iter()
            .map(|file| file.value(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        let file_names = files
            .iter()
            .map(|file| file.filename.clone())
            .collect::<Vec<_>>();
        let (assignment, file_ids) =
            AssignmentRepo::create_assignment(data_loader, model, file_names).await?;

        let class_id = assignment.class_id;
        for (mut file, file_id) in files.into_iter().zip(file_ids) {
            let s3_path = format!("class-files/{class_id}/{file_id}");
            let ct = file
                .content_type
                .take()
                .unwrap_or("application/octet-stream".to_string());
            let mut reader = file.into_async_read().compat();
            s3_bucket
                .put_object_stream_with_content_type(&mut reader, s3_path, ct)
                .await?;
        }

        let update_data = ClassResourceCreate::Assignment(assignment.clone().into());
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_CREATED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(assignment.into())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn update_assignment(
        &self,
        ctx: &Context<'_>,
        input: UpdateAssignmentInput,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;

        let (model, new_files, old_files) = input.try_into_active_model()?;

        let new_files = new_files
            .iter()
            .map(|file| file.value(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        let new_file_names = new_files
            .iter()
            .map(|file| file.filename.clone())
            .collect::<Vec<_>>();

        let (updated_assignment, file_ids) = AssignmentRepo::update_assignment(
            data_loader,
            model,
            new_file_names,
            old_files.clone(),
        )
        .await?;
        let class_id = updated_assignment.class_id;

        for file_id in old_files {
            let s3_path = format!("class-files/{class_id}/{file_id}");
            s3_bucket.delete_object(s3_path).await?;
        }

        for (mut file, file_id) in new_files.into_iter().zip(file_ids) {
            let s3_path = format!("class-files/{class_id}/{file_id}");
            let ct = file
                .content_type
                .take()
                .unwrap_or("application/octet-stream".to_string());
            let mut reader = file.into_async_read().compat();
            s3_bucket
                .put_object_stream_with_content_type(&mut reader, s3_path, ct)
                .await?;
        }

        let update_data = ClassResourceUpdate::Assignment(updated_assignment.into());
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_UPDATED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn delete_assignment(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
        assignment_id: ID,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;

        let original_id = assignment_id.clone();
        let class_id = class_id.parse::<Uuid>()?;
        let assigmnent_id = assignment_id.parse::<Uuid>()?;
        let file_ids = AssignmentRepo::delete_assignment(data_loader, assigmnent_id).await?;
        for file_id in file_ids {
            let s3_path = format!("class-files/{class_id}/{file_id}");
            s3_bucket.delete_object(s3_path).await?;
        }

        let update_data = ClassResourceDelete::Assignment(AssignmentDeleteInfo { id: original_id });
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_DELETED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_assignment_submission(
        &self,
        ctx: &Context<'_>,
        input: SubmitAssignmentInput,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;
        let claims = ctx.data_unchecked::<Option<Claims>>();

        let user_id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let (model, files) = input.try_into_active_model(user_id)?;
        let assignment_id = model.assignment_id.clone().unwrap();
        let files = files
            .iter()
            .map(|file| file.value(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        let file_names = files
            .iter()
            .map(|file| file.filename.clone())
            .collect::<Vec<_>>();
        let (class_id, file_ids) =
            AssignmentRepo::submit_assignment(data_loader, model, file_names).await?;

        for (mut file, file_id) in files.into_iter().zip(file_ids) {
            let s3_path = format!("class-files/{class_id}/{file_id}");
            let ct = file
                .content_type
                .take()
                .unwrap_or("application/octet-stream".to_string());
            let mut reader = file.into_async_read().compat();
            s3_bucket
                .put_object_stream_with_content_type(&mut reader, s3_path, ct)
                .await?;
        }

        let updated_assignment = AssignmentRepo::find_by_id(data_loader, assignment_id)
            .await?
            .expect("Assignment must exist");
        let update_data = ClassResourceUpdate::Assignment(updated_assignment.into());
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_UPDATED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn update_assignment_submission(
        &self,
        ctx: &Context<'_>,
        input: UpdateAssignmentSubmissionInput,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;

        let (model, new_files, old_files) = input.try_into_active_model()?;
        let assignment_id = model.assignment_id.clone().unwrap();
        let new_files = new_files
            .iter()
            .map(|file| file.value(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        let new_file_names = new_files
            .iter()
            .map(|file| file.filename.clone())
            .collect::<Vec<_>>();

        let (class_id, file_ids) = AssignmentRepo::update_assignment_submission(
            data_loader,
            model,
            new_file_names,
            old_files.clone(),
        )
        .await?;

        for file_id in old_files {
            let s3_path = format!("class-files/{class_id}/{file_id}");
            s3_bucket.delete_object(s3_path).await?;
        }

        for (mut file, file_id) in new_files.into_iter().zip(file_ids) {
            let s3_path = format!("class-files/{class_id}/{file_id}");
            let ct = file
                .content_type
                .take()
                .unwrap_or("application/octet-stream".to_string());
            let mut reader = file.into_async_read().compat();
            s3_bucket
                .put_object_stream_with_content_type(&mut reader, s3_path, ct)
                .await?;
        }

        let updated_assignment = AssignmentRepo::find_by_id(data_loader, assignment_id)
            .await?
            .expect("Assignment must exist");
        let update_data = ClassResourceUpdate::Assignment(updated_assignment.into());
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_UPDATED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn delete_assignment_submission(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
        assignment_id: ID,
        assignment_submission_id: ID,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;

        let class_id = class_id.parse::<Uuid>()?;
        let assignment_submission_id = assignment_submission_id.parse::<Uuid>()?;

        let res =
            AssignmentRepo::delete_assignment_submission(data_loader, assignment_submission_id)
                .await?;
        let res = match res {
            DeleteSubmissionResult::NotDeleted => return Ok(false),
            DeleteSubmissionResult::Deleted(file_ids) => {
                for file_id in file_ids {
                    let s3_path = format!("class-files/{class_id}/{file_id}");
                    s3_bucket.delete_object(s3_path).await?;
                }
                Ok(true)
            }
        };

        let assignment_id = Uuid::parse_str(assignment_id.as_str())?;
        let updated_assignment = AssignmentRepo::find_by_id(data_loader, assignment_id)
            .await?
            .expect("Assignment must exist");
        let class_id = updated_assignment.class_id;
        let update_data = ClassResourceUpdate::Assignment(updated_assignment.into());
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_UPDATED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;
        res
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_assignment_submission_feedback(
        &self,
        ctx: &Context<'_>,
        input: CreateAssignmanetSubmissionFeedbackInput,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;

        let assignment_id = Uuid::parse_str(input.assignment_id.clone().as_str())?;
        let model = input.try_into_active_model()?;

        AssignmentRepo::create_assignment_submission_feedback(data_loader, model).await?;

        let updated_assignment = AssignmentRepo::find_by_id(data_loader, assignment_id)
            .await?
            .expect("Assignment must exist");
        let class_id = updated_assignment.class_id;
        let update_data = ClassResourceUpdate::Assignment(updated_assignment.into());
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_UPDATED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn delete_assignment_submission_feedback(
        &self,
        ctx: &Context<'_>,
        assignment_id: ID,
        assignment_submission_feedback_id: ID,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;

        let assignment_submission_feedback_id =
            assignment_submission_feedback_id.parse::<Uuid>()?;
        AssignmentRepo::delete_assignment_feedback(data_loader, assignment_submission_feedback_id)
            .await?;

        let assignment_id = Uuid::parse_str(assignment_id.as_str())?;
        let updated_assignment = AssignmentRepo::find_by_id(data_loader, assignment_id)
            .await?
            .expect("Assignment must exist");
        let class_id = updated_assignment.class_id;
        let update_data = ClassResourceUpdate::Assignment(updated_assignment.into());
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_UPDATED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn update_assignment_submission_feedback(
        &self,
        ctx: &Context<'_>,
        input: UpdateAssignmanetSubmissionFeedbackInput,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;

        let assignment_id = Uuid::parse_str(input.assignment_id.clone().as_str())?;

        AssignmentRepo::update_assignment_submission_feedback(
            data_loader,
            input.try_into_active_model()?,
        )
        .await?;

        let updated_assignment = AssignmentRepo::find_by_id(data_loader, assignment_id)
            .await?
            .expect("Assignment must exist");
        let class_id = updated_assignment.class_id;
        let update_data = ClassResourceUpdate::Assignment(updated_assignment.into());
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_UPDATED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }
}
