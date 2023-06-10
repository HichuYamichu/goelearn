use crate::{
    core::AppError,
    object::{CreateDirectoryInput, FileObject, UpdateFileInput, UploadFileInput},
};
use async_graphql::{dataloader::DataLoader, Context, Object, ID};
use tracing::instrument;

use crate::core::repo::file::FileRepoExt;
use entity::sea_orm_active_enums;
use sea_orm::{DatabaseConnection, Set};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use uuid::Uuid;

use crate::core::LoggedInGuard;

#[derive(Default)]
pub struct FileMutation;

#[Object]
impl FileMutation {
    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn upload_files(
        &self,
        ctx: &Context<'_>,
        input: UploadFileInput,
    ) -> Result<bool, AppError> {
        let file_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();

        let class_id = Uuid::parse_str(&input.class_id.to_string())?;

        let mut files = input
            .files
            .iter()
            .map(|f| f.value(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        let file_models = files
            .iter()
            .map(|file| ::entity::file::ActiveModel {
                id: Set(Uuid::new_v4()),
                name: Set(file.filename.clone()),
                public: Set(input.public),
                file_type: Set(sea_orm_active_enums::FileType::File),
                parent_id: Set(input
                    .parent_id
                    .clone()
                    .map(|id| Uuid::parse_str(id.as_str()).unwrap())),
                class_id: Set(class_id),
                message_id: Set(None),
            })
            .collect::<Vec<_>>();

        let file_ids = file_models
            .iter()
            .map(|f| f.id.clone().unwrap())
            .collect::<Vec<_>>();

        file_repo.loader().save_files(file_models).await?;

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
                .put_object_stream_with_content_type(&mut reader, &s3_path, ct)
                .await?;
        }

        Ok(true)
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_direcotry(
        &self,
        ctx: &Context<'_>,
        input: CreateDirectoryInput,
    ) -> Result<FileObject, AppError> {
        let file_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let file_model = file_repo
            .loader()
            .save_file(input.try_into_active_model()?)
            .await?;

        Ok(file_model.into())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn delete_files(
        &self,
        ctx: &Context<'_>,
        file_ids: Vec<ID>,
    ) -> Result<bool, AppError> {
        let file_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();

        let file_ids = file_ids
            .iter()
            .map(|id| Uuid::parse_str(id))
            .collect::<Result<Vec<_>, _>>()?;

        let files = file_repo
            .loader()
            .find_many_with_nested(file_ids.clone())
            .await?;

        for file in files {
            let s3_path = format!(
                "class-files/{class_id}/{file_id}",
                class_id = file.class_id,
                file_id = file.id
            );

            s3_bucket.delete_object(&s3_path).await?;
        }

        file_repo.loader().delete_many_with_nested(file_ids).await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn update_file(
        &self,
        ctx: &Context<'_>,
        input: UpdateFileInput,
    ) -> Result<FileObject, AppError> {
        let file_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let file_model = file_repo
            .loader()
            .update_file(input.try_into_active_model()?)
            .await?;

        Ok(file_model.into())
    }
}
