use crate::{
    core::{
        repo::{channel::ChannelRepo},
        AppError,
    },
    object::{ChannelObject, CreateChannelInput},
};
use async_graphql::{dataloader::DataLoader, Context, Object};



use crate::core::LoggedInGuard;

#[derive(Default)]
pub struct ChannelMutation;

#[Object]
impl ChannelMutation {
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_channel(
        &self,
        ctx: &Context<'_>,
        input: CreateChannelInput,
    ) -> Result<ChannelObject, AppError> {
        let channel_repo = ctx.data_unchecked::<DataLoader<ChannelRepo>>();

        let model = input.try_into_active_model()?;
        let channel = channel_repo.loader().create_channel(model).await?;
        Ok(channel.into())
    }
}
