use crate::core::AppError;
use crate::core::LoggedInGuard;
use async_graphql::{dataloader::DataLoader, Context, Object};
use sea_orm::DatabaseConnection;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing::instrument;

use super::object::{CreateAssignmentInput, SubmitAssignmentInput};
use super::{AssignmentObject, AssignmentRepo};

#[derive(Default)]
pub struct AssignmentMutation;

#[Object]
impl AssignmentMutation {
    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_assignment(
        &self,
        ctx: &Context<'_>,
        input: CreateAssignmentInput,
    ) -> Result<AssignmentObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();

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

        Ok(assignment.into())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn submit_assignment(
        &self,
        ctx: &Context<'_>,
        input: SubmitAssignmentInput,
    ) -> Result<bool, AppError> {
        todo!();
        Ok(true)
    }
}
