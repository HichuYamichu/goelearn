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
pub struct GuildQuery;

#[Object]
impl GuildQuery {
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
}
