use crate::core::repo::class::ClassRepoExt;
use async_graphql::{dataloader::DataLoader, Context, Object, ID};
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    core::{repo::class::ClassById, AppError, LoggedInGuard},
    object::ClassObject,
};

#[derive(Default)]
pub struct ClassQuery;

#[Object]
impl ClassQuery {
    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn class_by_id(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<Option<ClassObject>, AppError> {
        let class_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let c = class_repo
            .load_one(ClassById(Uuid::parse_str(id.as_str())?))
            .await?;
        Ok(c.map(|c| c.into()))
    }

    #[graphql(guard = "LoggedInGuard")]
    async fn random_classes(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let class_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let c = class_repo.loader().find_random(10).await?;
        Ok(c.into_iter().map(|c| c.into()).collect())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn classes_by_search(
        &self,
        ctx: &Context<'_>,
        query: String,
    ) -> Result<Vec<ClassObject>, AppError> {
        let class_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let c = match query.as_str() {
            "" => class_repo.loader().find_random(10).await?,
            _ => class_repo.loader().find_by_query(query).await?,
        };
        Ok(c.into_iter().map(|c| c.into()).collect())
    }
}
