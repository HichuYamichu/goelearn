use crate::{
    core::{
        auth,
        repo::{class::ClassRepo, user::UserRepo},
        AppError,
    },
    object::{ClassObject, CreateClassInput},
};
use async_graphql::{dataloader::DataLoader, Context, Object};
use auth::Claims;
use uuid::Uuid;

use crate::core::LoggedInGuard;
use crate::object::{LoginInput, LoginResult, SignupInput};

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
        let class_repo = ctx.data::<DataLoader<ClassRepo>>().unwrap();
        let claims = ctx.data::<Option<Claims>>().unwrap();

        let id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let model = input.into_active_model(id);

        let class = class_repo.loader().create_class(model).await?;
        Ok(class.into())
    }
}
