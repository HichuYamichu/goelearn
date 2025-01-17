use crate::core::{AdminGuard, AppError, Claims, LoggedInGuard};

use async_graphql::{dataloader::DataLoader, Context, Object, ID};
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use super::{UserObject, UserRepo};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn me(&self, ctx: &Context<'_>) -> Result<UserObject, async_graphql::Error> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();
        let id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let u = UserRepo::find_by_id(data_loader, id)
            .await?
            .expect("User id cannot be invalid here");
        Ok(u.into())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(AdminGuard)")]
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserObject>, async_graphql::Error> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let users = UserRepo::find_all(data_loader).await?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }
}
