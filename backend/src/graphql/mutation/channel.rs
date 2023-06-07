use crate::{
    core::AppError,
    object::{ChannelObject, CreateChannelInput},
};
use async_graphql::{dataloader::DataLoader, Context, Object};
use sea_orm::DatabaseConnection;
use tracing::instrument;

use crate::core::repo::channel::ChannelRepoExt;
use crate::core::LoggedInGuard;

#[derive(Default)]
pub struct ChannelMutation;

#[Object]
impl ChannelMutation {
    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_channel(
        &self,
        ctx: &Context<'_>,
        input: CreateChannelInput,
    ) -> Result<ChannelObject, AppError> {
        let channel_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let model = input.try_into_active_model()?;
        let channel = channel_repo.loader().create_channel(model).await?;
        Ok(channel.into())
    }
}
