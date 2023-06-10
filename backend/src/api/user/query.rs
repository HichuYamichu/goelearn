use async_graphql::{dataloader::DataLoader, Context, Object};
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use crate::core::{AppError, Claims, LoggedInGuard};

use super::{UserObject, UserRepo};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn me(&self, ctx: &Context<'_>) -> Result<UserObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();
        let id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let u = UserRepo::find_by_id(data_loader, id)
            .await?
            .expect("User id cannot be invalid here");
        Ok(u.into())
    }
}