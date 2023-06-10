use crate::api::user::UserRepo;
use crate::core::{auth, AppError, UserError};
use async_graphql::{dataloader::DataLoader, Context, Object, ID};
use auth::Claims;
use sea_orm::DatabaseConnection;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing::instrument;
use uuid::Uuid;

use crate::core::LoggedInGuard;

use super::object::CreateClassInput;
use super::{ClassObject, ClassRepo};

#[derive(Default)]
pub struct ClassMutation;

#[Object]
impl ClassMutation {
    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_class(
        &self,
        ctx: &Context<'_>,
        mut input: CreateClassInput,
    ) -> Result<ClassObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();

        let id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let image = input.image.take();
        let model = input.into_active_model(id, image.is_some());
        let class_id = model.id.clone().into_value().expect("id was just set");

        if let Some(image) = image {
            let image = image.value(ctx)?;
            if image.content_type.is_none() || image.content_type.as_ref().unwrap() != "image/jpeg"
            {
                return Err(AppError::UserError(UserError::BadInput {
                    simple: "image must be a jpeg image",
                    detailed: "image must be a jpeg image".into(),
                }));
            }

            let s3_path = format!("class-images/{}", class_id.as_ref_uuid().unwrap());
            let mut reader = image.into_async_read().compat();
            s3_bucket
                .put_object_stream_with_content_type(&mut reader, s3_path, "image/jpeg")
                .await?;
        }

        let class = ClassRepo::create_class(data_loader, model).await?;
        Ok(class.into())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn join_class(&self, ctx: &Context<'_>, class_id: ID) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();

        let user_id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let class_id = Uuid::parse_str(class_id.as_str())?;

        let class = ClassRepo::find_by_id(data_loader, class_id).await?;
        let class = match class {
            Some(class) => class,
            None => {
                return Err(AppError::NotFound {
                    what: "class",
                    with: "id",
                    why: class_id.to_string(),
                })
            }
        };

        if !class.public {
            return Err(AppError::UserError(crate::core::UserError::BadInput {
                simple: "class is private",
                detailed: "class is private".into(),
            }));
        }

        let members = UserRepo::find_by_class_id(data_loader, class_id)
            .await?
            .expect("class id is valid");

        if members.iter().any(|m| m.id == user_id) {
            return Err(AppError::UserError(crate::core::UserError::BadInput {
                simple: "already joined",
                detailed: "already joined".into(),
            }));
        }

        ClassRepo::join_user_to_class(data_loader, user_id, class_id).await?;

        Ok(true)
    }
}
