use crate::core::LoggedInGuard;
use crate::core::{
    repo::channel::{self, ChannelRepo},
    AppError,
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject, ID,
};
use sea_orm::Set;
use uuid::Uuid;

use super::ChannelObject;

#[derive(InputObject)]
pub struct CreateClassInput {
    pub name: String,
    pub description: String,
    pub public: bool,
}

impl CreateClassInput {
    pub fn into_active_model(self, owner_id: Uuid) -> ::entity::class::ActiveModel {
        ::entity::class::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(self.name),
            description: Set(self.description),
            owner_id: Set(owner_id),
            public: Set(self.public),
        }
    }
}

#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
pub struct ClassObject {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub owner_id: ID,
    pub public: bool,
}

impl From<::entity::class::Model> for ClassObject {
    fn from(c: ::entity::class::Model) -> Self {
        Self {
            id: ID::from(c.id),
            name: c.name,
            description: c.description,
            owner_id: ID::from(c.owner_id),
            public: c.public,
        }
    }
}

#[ComplexObject]
impl ClassObject {
    #[graphql(guard = "LoggedInGuard")]
    async fn channels(&self, ctx: &Context<'_>) -> Result<Vec<ChannelObject>, AppError> {
        let channel_repo = ctx.data::<DataLoader<ChannelRepo>>().unwrap();

        let id = channel::ChannelByClassId(Uuid::parse_str(&self.id)?);
        let channels = channel_repo.load_many([id].into_iter()).await?;

        Ok(channels.into_values().map(|c| c.into()).collect())
    }
}
