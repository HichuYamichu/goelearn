use async_graphql::{dataloader::DataLoader, Context, Object, ID};
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use crate::core::{AppError, LoggedInGuard};

use super::{ClassObject, ClassRepo};

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
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = Uuid::parse_str(id.as_str())?;
        let c = ClassRepo::find_by_id(data_loader, id).await?;
        Ok(c.map(|c| c.into()))
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn random_classes(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let c = ClassRepo::find_random(data_loader, 10).await?;
        Ok(c.into_iter().map(|c| c.into()).collect())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn classes_by_search(
        &self,
        ctx: &Context<'_>,
        query: String,
    ) -> Result<Vec<ClassObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let c = match query.as_str() {
            "" => ClassRepo::find_random(data_loader, 10).await?,
            _ => ClassRepo::find_by_query(data_loader, query).await?,
        };
        Ok(c.into_iter().map(|c| c.into()).collect())
    }
}
