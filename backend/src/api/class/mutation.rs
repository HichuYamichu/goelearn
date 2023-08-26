use crate::api::user::UserRepo;
use crate::core::LoggedInGuard;
use crate::core::{auth, AppError, UserError};
use async_graphql::{dataloader::DataLoader, Context, Object, ID};
use auth::Claims;
use redis::AsyncCommands;
use redis::Client;
use sea_orm::DatabaseConnection;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing::instrument;
use uuid::Uuid;

use super::object::{CreateClassInput, UpdateClassInput};
use super::{ClassObject, ClassRepo};

#[derive(Default)]
pub struct ClassMutation;

#[Object]
impl ClassMutation {
    #[instrument(skip(self, ctx), err(Debug))]
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
                return Err(AppError::user(
                    "Image must be a jpeg",
                    UserError::BadInput {
                        parameter: "image",
                        given_value: "non jpeg image".into(),
                    },
                ));
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

    #[instrument(skip(self, ctx), err(Debug))]
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
                return Err(AppError::not_found(
                    "Class not found".into(),
                    "class",
                    "id",
                    class_id.to_string(),
                ))
            }
        };

        if !class.public {
            return Err(AppError::user(
                "You cannot join private class without explicit invite",
                UserError::BadInput {
                    parameter: "class_id",
                    given_value: class_id.to_string(),
                },
            ));
        }

        let members = UserRepo::find_by_class_id(data_loader, class_id)
            .await?
            .expect("class id is valid");

        if members.iter().any(|m| m.id == user_id) {
            return Err(AppError::user(
                "You cannot join class your're already member of",
                UserError::BadInput {
                    parameter: "class_id",
                    given_value: class_id.to_string(),
                },
            ));
        }

        ClassRepo::join_user_to_class(data_loader, user_id, class_id).await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn delete_class(&self, ctx: &Context<'_>, class_id: ID) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let client = ctx.data_unchecked::<Client>();
        let mut conn = client.get_async_connection().await?;

        let class_id = Uuid::parse_str(class_id.as_str())?;
        ClassRepo::delete_class(data_loader, class_id).await?;
        conn.publish(
            format!("class_deleted:{}", class_id.to_string()),
            serde_json::to_string(&class_id).expect("ClassID should serialize"),
        )
        .await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn update_class(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
        class_input: UpdateClassInput,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let client = ctx.data_unchecked::<Client>();
        let mut conn = client.get_async_connection().await?;

        let class_id = Uuid::parse_str(class_id.as_str())?;
        let update_data = class_input.into_active_model();
        let updated = ClassRepo::update_class(data_loader, class_id, update_data).await?;
        let updated = ClassObject::from(updated);

        conn.publish(
            format!("class_updated:{}", class_id.to_string()),
            serde_json::to_string(&updated).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }
}
