use crate::api::assignment::AssignmentObject;
use crate::api::assignment::AssignmentRepo;
use crate::api::channel::ChannelObject;
use crate::api::channel::ChannelRepo;
use crate::api::file::FileObject;
use crate::api::file::FileRepo;
use crate::api::user::UserObject;
use crate::api::user::UserRepo;
use crate::core::AppError;
use crate::core::LoggedInGuard;
use async_graphql::Upload;
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject, ID,
};
use partialdebug::placeholder::PartialDebug;
use sea_orm::{DatabaseConnection, Set};
use tracing::instrument;
use uuid::Uuid;

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

#[ComplexObject]
impl ClassObject {
    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn channels(&self, ctx: &Context<'_>) -> Result<Vec<ChannelObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let channels = ChannelRepo::find_by_class_id(data_loader, Uuid::parse_str(&self.id)?)
            .await?
            .expect("Id should be valid");

        Ok(channels.into_iter().map(ChannelObject::from).collect())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<UserObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let class_id = Uuid::parse_str(&self.id)?;
        let users = UserRepo::find_by_class_id(data_loader, class_id)
            .await?
            .expect("Id should be valid");

        Ok(users.into_iter().map(UserObject::from).collect())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn files(&self, ctx: &Context<'_>) -> Result<Vec<FileObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let class_id = Uuid::parse_str(&self.id)?;
        let files = FileRepo::find_by_class_id(data_loader, class_id)
            .await?
            .expect("Id should be valid");

        Ok(files.into_iter().map(FileObject::from).collect())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn owner(&self, ctx: &Context<'_>) -> Result<UserObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = Uuid::parse_str(&self.owner_id)?;
        let user = UserRepo::find_by_id(data_loader, id)
            .await?
            .expect("Id should be valid");

        Ok(UserObject::from(user))
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn assignments(&self, ctx: &Context<'_>) -> Result<Vec<AssignmentObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let class_id = Uuid::parse_str(&self.id)?;
        let assignments = AssignmentRepo::find_by_class_id(data_loader, class_id)
            .await?
            .expect("Id should be valid");

        Ok(assignments
            .into_iter()
            .map(AssignmentObject::from)
            .collect())
    }
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
