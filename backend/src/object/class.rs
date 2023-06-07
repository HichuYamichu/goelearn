use crate::core::repo::assignment::{self};
use crate::core::repo::file::{self};
use crate::core::repo::user::{self, UserRepoExt};
use crate::core::LoggedInGuard;
use crate::core::{
    repo::channel::{self},
    AppError,
};
use async_graphql::Upload;
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject, ID,
};
use partialdebug::placeholder::PartialDebug;
use sea_orm::{DatabaseConnection, Set};
use tracing::instrument;
use uuid::Uuid;

use super::{AssignmentObject, ChannelObject, FileObject, UserObject};

#[derive(InputObject, PartialDebug)]
pub struct CreateClassInput {
    pub name: String,
    pub description: String,
    pub public: bool,
    pub tags: String,
    pub image: Option<Upload>,
}

impl CreateClassInput {
    pub fn into_active_model(
        self,
        owner_id: Uuid,
        has_image: bool,
    ) -> ::entity::class::ActiveModel {
        ::entity::class::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(self.name),
            description: Set(self.description),
            owner_id: Set(owner_id),
            public: Set(self.public),
            tags: Set(self.tags),
            has_image: Set(has_image),
        }
    }
}

#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
#[graphql(name = "Class")]
pub struct ClassObject {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub owner_id: ID,
    pub public: bool,
    pub tags: String,
    pub has_image: bool,
}

impl From<::entity::class::Model> for ClassObject {
    fn from(c: ::entity::class::Model) -> Self {
        Self {
            id: ID::from(c.id),
            name: c.name,
            description: c.description,
            owner_id: ID::from(c.owner_id),
            public: c.public,
            tags: c.tags,
            has_image: c.has_image,
        }
    }
}

#[ComplexObject]
impl ClassObject {
    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn channels(&self, ctx: &Context<'_>) -> Result<Vec<ChannelObject>, AppError> {
        let channel_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = channel::ChannelsByClassId(Uuid::parse_str(&self.id)?);
        let channels = channel_repo
            .load_one(id)
            .await?
            .expect("Id should be valid");

        Ok(channels.into_iter().map(ChannelObject::from).collect())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<UserObject>, AppError> {
        let user_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = user::UsersByClassId(Uuid::parse_str(&self.id)?);
        let users = user_repo.load_one(id).await?.expect("Id should be valid");

        Ok(users.into_iter().map(UserObject::from).collect())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn files(&self, ctx: &Context<'_>) -> Result<Vec<FileObject>, AppError> {
        let file_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = file::FilesByClassId(Uuid::parse_str(&self.id)?);
        let files = file_repo.load_one(id).await?.expect("Id should be valid");

        Ok(files.into_iter().map(FileObject::from).collect())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn owner(&self, ctx: &Context<'_>) -> Result<UserObject, AppError> {
        let user_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = Uuid::parse_str(&self.owner_id)?;
        let user = user_repo
            .loader()
            .user_by_id(id)
            .await?
            .expect("Id should be valid");

        Ok(UserObject::from(user))
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn assignments(&self, ctx: &Context<'_>) -> Result<Vec<AssignmentObject>, AppError> {
        let assignment_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = assignment::AssignmentsByClassId(Uuid::parse_str(&self.id)?);
        let assignments = assignment_repo
            .load_one(id)
            .await?
            .expect("Id should be valid");

        Ok(assignments
            .into_iter()
            .map(AssignmentObject::from)
            .collect())
    }
}
