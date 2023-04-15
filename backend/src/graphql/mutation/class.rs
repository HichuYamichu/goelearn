use crate::{
    core::{auth, repo::class::ClassRepo, AppError},
    object::{ClassObject, CreateClassInput},
};
use async_graphql::{dataloader::DataLoader, Context, Object};
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
}
