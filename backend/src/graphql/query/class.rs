use async_graphql::{dataloader::DataLoader, Context, Object, ID};
use uuid::Uuid;

use crate::{
    core::{
        repo::{
            class::{ClassById, ClassRepo},
            user::UserRepo,
        },
        AppError, Claims, LoggedInGuard,
    },
    object::{ClassObject, UserObject},
};

#[derive(Default)]
pub struct ClassQuery;

#[Object]
impl ClassQuery {
    #[graphql(guard = "LoggedInGuard")]
    async fn class_by_id(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<Option<ClassObject>, AppError> {
        let class_repo = ctx.data_unchecked::<DataLoader<ClassRepo>>();
        let c = class_repo
            .load_one(ClassById(Uuid::parse_str(id.as_str())?))
            .await?;
        Ok(c.map(|c| c.into()))
    }

    #[graphql(guard = "LoggedInGuard")]
    async fn random_classes(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let class_repo = ctx.data_unchecked::<DataLoader<ClassRepo>>();
        let c = class_repo.loader().find_random(10).await?;
        Ok(c.into_iter().map(|c| c.into()).collect())
    }

    #[graphql(guard = "LoggedInGuard")]
    async fn classes_by_search(
        &self,
        ctx: &Context<'_>,
        query: String,
    ) -> Result<Vec<ClassObject>, AppError> {
        let class_repo = ctx.data_unchecked::<DataLoader<ClassRepo>>();
        let c = class_repo.loader().find_by_query(query).await?;
        Ok(c.into_iter().map(|c| c.into()).collect())
    }
}
