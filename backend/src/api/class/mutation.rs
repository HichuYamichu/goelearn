use crate::api::user::UserRepo;
use crate::api::MAX_FILE_SIZE;
use crate::core::{auth, AppError, UserError};
use crate::core::{ClassMemberGuard, ClassOwnerGuard, LoggedInGuard};
use async_graphql::{dataloader::DataLoader, Context, Object, ID};
use auth::Claims;

use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::Pool;
use sea_orm::DatabaseConnection;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing::instrument;
use uuid::Uuid;

use super::object::{CreateClassInput, CreateInviteInput, InviteObject, UpdateClassInput};
use super::{
    ClassObject, ClassRepo, ClassResourceCreate, ClassResourceDelete, ClassResourceUpdate,
    CLASS_RESOURCE_CREATED, CLASS_RESOURCE_DELETED, CLASS_RESOURCE_UPDATED,
};

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
            let exeeds_limit = image.size()? > MAX_FILE_SIZE;
            if exeeds_limit {
                return Err(AppError::user("file too large", UserError::FileTooLarge));
            }

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
    pub async fn join_class(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
        invite_id: Option<ID>,
    ) -> Result<ID, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();
        let redis_pool = ctx.data_unchecked::<Pool>();
        let mut conn = redis_pool.get().await?;

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
            if invite_id.is_none() {
                return Err(AppError::user(
                    "You cannot join private class without explicit invite",
                    UserError::BadInput {
                        parameter: "class_id",
                        given_value: class_id.to_string(),
                    },
                ));
            }

            let invite_id = Uuid::parse_str(invite_id.unwrap().as_str())?;
            let is_valid = ClassRepo::is_valid_invite(data_loader, invite_id, class_id).await?;

            if !is_valid {
                return Err(AppError::user(
                    "You cannot join class with invalid invite",
                    UserError::BadInput {
                        parameter: "invite_id",
                        given_value: invite_id.to_string(),
                    },
                ));
            }
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

        let bans = ClassRepo::get_user_bans(data_loader, user_id).await?;
        if bans.iter().any(|b| *b == class_id) {
            return Err(AppError::user(
                "You cannot join class you're banned from",
                UserError::BadInput {
                    parameter: "class_id",
                    given_value: class_id.to_string(),
                },
            ));
        }

        let user = UserRepo::find_by_id(data_loader, user_id)
            .await?
            .expect("user exists");
        ClassRepo::join_user_to_class(data_loader, user_id, class_id).await?;

        let update_data = ClassResourceCreate::Member(user.into());
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_CREATED, class_id),
            serde_json::to_string(&update_data).expect("User should serialize"),
        )
        .await?;

        Ok(ID::from(class_id))
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassMemberGuard::new(class_id.clone()))")]
    pub async fn leave_class(&self, ctx: &Context<'_>, class_id: ID) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();
        let redis_pool = ctx.data_unchecked::<Pool>();
        let mut conn = redis_pool.get().await?;

        let user_id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let original_id = ID::from(user_id);
        let class_id = Uuid::parse_str(class_id.as_str())?;
        ClassRepo::remove_memeber(data_loader, class_id, user_id).await?;

        let update_data = ClassResourceDelete::Member(super::MemberDeleteInfo { id: original_id });
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_DELETED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassOwnerGuard::new(class_id.clone()))")]
    pub async fn ban_member(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
        user_id: ID,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let redis_pool = ctx.data_unchecked::<Pool>();
        let mut conn = redis_pool.get().await?;

        let original_id = user_id.clone();
        let class_id = Uuid::parse_str(class_id.as_str())?;
        let user_id = Uuid::parse_str(user_id.as_str())?;
        ClassRepo::ban_member(data_loader, class_id, user_id).await?;

        let update_data = ClassResourceDelete::Member(super::MemberDeleteInfo { id: original_id });
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_DELETED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassOwnerGuard::new(class_id.clone()))")]
    pub async fn unban_member(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
        user_id: ID,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let class_id = Uuid::parse_str(class_id.as_str())?;
        let user_id = Uuid::parse_str(user_id.as_str())?;
        ClassRepo::unban_member(data_loader, class_id, user_id).await?;
        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassOwnerGuard::new(class_id.clone()))")]
    pub async fn delete_class(&self, ctx: &Context<'_>, class_id: ID) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let redis_pool = ctx.data_unchecked::<Pool>();
        let mut conn = redis_pool.get().await?;

        let class_id = Uuid::parse_str(class_id.as_str())?;
        ClassRepo::delete_class(data_loader, class_id).await?;
        conn.publish(
            format!("class_deleted:{}", class_id),
            serde_json::to_string(&class_id).expect("ClassID should serialize"),
        )
        .await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassOwnerGuard::new(class_id.clone()))")]
    pub async fn update_class(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
        class_input: UpdateClassInput,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let redis_pool = ctx.data_unchecked::<Pool>();
        let mut conn = redis_pool.get().await?;

        let class_id = Uuid::parse_str(class_id.as_str())?;
        let update_data = class_input.into_active_model();
        let updated = ClassRepo::update_class(data_loader, class_id, update_data).await?;
        let updated = ClassObject::from(updated);

        let update_data = ClassResourceUpdate::Class(updated.clone());
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_UPDATED, class_id),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassOwnerGuard::new(input.class_id.clone()))")]
    pub async fn create_invite(
        &self,
        ctx: &Context<'_>,
        input: CreateInviteInput,
    ) -> Result<InviteObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let model = input.try_into_active_model()?;
        let invite = ClassRepo::create_invite(data_loader, model).await?;

        Ok(invite.into())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassOwnerGuard::new(class_id.clone()))")]
    pub async fn delete_invite(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
        invite_id: ID,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let invite_id = Uuid::parse_str(invite_id.as_str())?;
        ClassRepo::delete_invite(data_loader, invite_id).await?;

        Ok(true)
    }
}
