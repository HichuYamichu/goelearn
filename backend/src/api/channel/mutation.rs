use crate::api::class::{ChannelDeleteInfo, CLASS_RESOURCE_DELETED};
use crate::api::class::{
    ClassResourceCreate, ClassResourceDelete, CLASS_RESOURCE_CREATED, CLASS_RESOURCE_UPDATED,
};
use crate::core::AppError;
use crate::core::LoggedInGuard;
use async_graphql::ID;
use async_graphql::{dataloader::DataLoader, Context, Object};
use deadpool_redis::redis::AsyncCommands;
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use super::object::{CreateChannelInput, UpdateChannelInput};
use super::ChannelObject;
use crate::api::channel::repo::ChannelRepo;

#[derive(Default)]
pub struct ChannelMutation;

#[Object]
impl ChannelMutation {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn create_channel(
        &self,
        ctx: &Context<'_>,
        input: CreateChannelInput,
    ) -> Result<ChannelObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;

        let class_id = input.class_id.to_string();
        let model = input.try_into_active_model()?;
        let channel = ChannelRepo::create_channel(data_loader, model).await?;
        let channel = ChannelObject::from(channel);

        let update_data = ClassResourceCreate::Channel(channel.clone());
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_CREATED, class_id.as_str()),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(channel)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn update_channel(
        &self,
        ctx: &Context<'_>,
        input: UpdateChannelInput,
    ) -> Result<ChannelObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;

        let class_id = input.class_id.to_string();
        let model = input.try_into_active_model()?;
        let channel = ChannelRepo::update_channel(data_loader, model).await?;
        let channel = ChannelObject::from(channel);

        Ok(channel)
    }

    pub async fn delete_channel(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
        channel_id: ID,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let redis_pool = ctx.data_unchecked::<deadpool_redis::Pool>();
        let mut conn = redis_pool.get().await?;

        let original_id = channel_id.clone();
        let channel_id = channel_id.parse::<Uuid>()?;
        ChannelRepo::delete_channel(data_loader, channel_id).await?;

        let update_data = ClassResourceDelete::Channel(ChannelDeleteInfo { id: original_id });
        tracing::debug!(
            "Publishing to {}",
            format!("{}:{}", CLASS_RESOURCE_DELETED, class_id.to_string())
        );
        conn.publish(
            format!("{}:{}", CLASS_RESOURCE_DELETED, class_id.to_string()),
            serde_json::to_string(&update_data).expect("Class should serialize"),
        )
        .await?;

        Ok(true)
    }
}
