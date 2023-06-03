use crate::{
    core::{
        repo::{file::FileRepo},
        AppError,
    },
    object::{
        CreateDirectoryInput,
        FileObject, UploadFileInput,
    },
};
use async_graphql::{dataloader::DataLoader, Context, Object};

use tokio_util::compat::FuturesAsyncReadCompatExt;


use crate::core::LoggedInGuard;

#[derive(Default)]
pub struct FileMutation;

#[Object]
impl FileMutation {
    #[graphql(guard = "LoggedInGuard")]
    pub async fn upload_file(
        &self,
        ctx: &Context<'_>,
        input: UploadFileInput,
    ) -> Result<FileObject, AppError> {
        let file_repo = ctx.data_unchecked::<DataLoader<FileRepo>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();

        let (model, file) = input.try_into_active_model()?;

        let file_model = file_repo.loader().save_file(model).await?;
        let s3_path = format!(
            "class-files/{class_id}/{file_id}",
            class_id = file_model.class_id,
            file_id = file_model.id
        );
        let mut file = file.value(ctx)?;

        let ct = file
            .content_type
            .take()
            .unwrap_or("application/octet-stream".to_string());
        let mut reader = file.into_async_read().compat();
        s3_bucket
            .put_object_stream_with_content_type(&mut reader, s3_path, ct)
            .await?;

        Ok(file_model.into())
    }

    pub async fn create_direcotry(
        &self,
        ctx: &Context<'_>,
        input: CreateDirectoryInput,
    ) -> Result<FileObject, AppError> {
        let file_repo = ctx.data_unchecked::<DataLoader<FileRepo>>();

        let file_model = file_repo
            .loader()
            .save_file(input.try_into_active_model()?)
            .await?;

        Ok(file_model.into())
    }
}
