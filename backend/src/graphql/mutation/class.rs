use crate::{
    core::{
        auth,
        repo::{
            class::{ClassById, ClassRepo},
            membership::{MembershipRepo, MembershipsByClassId},
        },
        AppError,
    },
    object::{ClassObject, CreateClassInput},
};
use async_graphql::{dataloader::DataLoader, Context, Object, ID};
use auth::Claims;
use uuid::Uuid;

use crate::core::LoggedInGuard;

#[derive(Default)]
pub struct ClassMutation;

#[Object]
impl ClassMutation {
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_class(
        &self,
        ctx: &Context<'_>,
        input: CreateClassInput,
    ) -> Result<ClassObject, AppError> {
        let class_repo = ctx.data_unchecked::<DataLoader<ClassRepo>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();

        let id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let model = input.into_active_model(id);

        let class = class_repo.loader().create_class(model).await?;
        Ok(class.into())
    }

    #[graphql(guard = "LoggedInGuard")]
    pub async fn join_class(&self, ctx: &Context<'_>, class_id: ID) -> Result<bool, AppError> {
        let class_repo = ctx.data_unchecked::<DataLoader<ClassRepo>>();
        let membership_repo = ctx.data_unchecked::<DataLoader<MembershipRepo>>();

        let claims = ctx.data_unchecked::<Option<Claims>>();
        let user_id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let class_id = Uuid::parse_str(&class_id.to_string())?;

        let class = class_repo.load_one(ClassById(class_id)).await?;
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

        let memberships = membership_repo
            .load_one(MembershipsByClassId(class_id))
            .await?
            .expect("Membership relation is not optional");

        if memberships.iter().any(|m| m.user_id == user_id) {
            return Err(AppError::UserError(crate::core::UserError::BadInput {
                simple: "already joined",
                detailed: "already joined".into(),
            }));
        }

        class_repo
            .loader()
            .join_user_to_class(user_id, class_id)
            .await?;

        Ok(true)
    }
}
