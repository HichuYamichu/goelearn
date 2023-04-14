use async_graphql::{dataloader::DataLoader, Context, Object};
use uuid::Uuid;

use crate::{
    core::{repo::user::UserRepo, AppError, Claims, LoggedInGuard},
    object::UserObject,
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "LoggedInGuard")]
    async fn me(&self, ctx: &Context<'_>) -> Result<UserObject, AppError> {
        let user_repo = ctx.data::<DataLoader<UserRepo>>().unwrap();
        let claims = ctx.data::<Option<Claims>>().unwrap();
        let id = Uuid::parse_str(&claims.as_ref().expect("Guard ensures claims exist").sub)?;
        let u = user_repo
            .loader()
            .user_by_id(id)
            .await?
            .expect("User id cannot be invalid here");
        Ok(u.into())
    }
}
