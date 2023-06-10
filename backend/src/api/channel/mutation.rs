use crate::core::AppError;
use async_graphql::{dataloader::DataLoader, Context, Object};
use sea_orm::DatabaseConnection;
use tracing::instrument;

use crate::core::LoggedInGuard;

use super::object::CreateChannelInput;
use super::ChannelObject;
use crate::api::channel::repo::ChannelRepo;

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
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let model = input.try_into_active_model()?;
        let channel = ChannelRepo::create_channel(data_loader, model).await?;
        Ok(channel.into())
    }
}
