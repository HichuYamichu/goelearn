use crate::core::repo::assignment::AssignmentRepoExt;
use crate::core::LoggedInGuard;
use crate::{
    core::{
        auth,
        repo::{class::ClassById, membership::MembershipsByClassId},
        AppError, UserError,
    },
    object::{
        AssignmentObject, ClassObject, CreateAssignmentInput, CreateClassInput,
        SubmitAssignmentInput,
    },
};
use async_graphql::{dataloader::DataLoader, Context, Object, ID};
use auth::Claims;
use sea_orm::DatabaseConnection;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing::instrument;
use uuid::Uuid;

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
        let assignment_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();

        let id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let (model, files) = input.try_into_active_model()?;
        let files = files
            .iter()
            .map(|file| file.value(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        let file_names = files
            .iter()
            .map(|file| file.filename.clone())
            .collect::<Vec<_>>();
        let (assignment, file_ids) = assignment_repo
            .loader()
            .create_assignment(model, file_names)
            .await?;

        let class_id = assignment.class_id;
        for (mut file, file_id) in files.into_iter().zip(file_ids) {
            let s3_path = format!(
                "class-files/{class_id}/{file_id}",
                class_id = class_id,
                file_id = file_id
            );
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
        // let assignment_repo = ctx.data_unchecked::<DataLoader<AssignmentRepo>>();
        // let claims = ctx.data_unchecked::<Option<Claims>>();
        // let s3_bucket = ctx.data_unchecked::<s3::Bucket>();

        // let id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        // let (model, files) = input.try_into_active_model(id)?;
        // let files = files
        //     .iter()
        //     .map(|file| file.value(ctx))
        //     .collect::<Result<Vec<_>, _>>()?;

        // let file_names = files
        //     .iter()
        //     .map(|file| file.filename.clone())
        //     .collect::<Vec<_>>();
        // let (assignment, file_ids) = assignment_repo
        //     .loader()
        //     .create_assignment(model, file_names)
        //     .await?;

        // let class_id = assignment.class_id;
        // for (mut file, file_id) in files.into_iter().zip(file_ids) {
        //     let s3_path = format!(
        //         "class-files/{class_id}/{file_id}",
        //         class_id = class_id,
        //         file_id = file_id
        //     );
        //     let ct = file
        //         .content_type
        //         .take()
        //         .unwrap_or("application/octet-stream".to_string());
        //     let mut reader = file.into_async_read().compat();
        //     s3_bucket
        //         .put_object_stream_with_content_type(&mut reader, s3_path, ct)
        //         .await?;
        // }

        Ok(true)
    }
}
