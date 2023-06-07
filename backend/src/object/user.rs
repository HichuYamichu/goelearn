use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};

use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use crate::core::repo::membership::{self};
use crate::core::{repo::class, AppError};

use super::ClassObject;
use crate::core::LoggedInGuard;

#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
#[graphql(name = "User")]
pub struct UserObject {
    pub id: ID,
    pub username: String,
    pub email: String,
    pub has_avatar: bool,
    pub first_name: String,
    pub last_name: String,
    // pub user_type: UserType,
}

impl From<::entity::user::Model> for UserObject {
    fn from(u: ::entity::user::Model) -> Self {
        Self {
            id: ID::from(u.id),
            username: u.username,
            email: u.email,
            has_avatar: u.has_avatar,
            first_name: u.first_name,
            last_name: u.last_name,
        }
    }
}

#[ComplexObject]
impl UserObject {
    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn clesses(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let conn = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = membership::MembershipsByUserId(Uuid::parse_str(&self.id)?);
        let memberships = conn
            .load_one(id)
            .await?
            .expect("No memberships will be empty vec");
        let class_ids = memberships.iter().map(|m| class::ClassById(m.class_id));
        let classes = conn.load_many(class_ids).await?;
        dbg!(&memberships);

        Ok(classes.into_values().map(|c| c.into()).collect())
    }

    #[instrument(skip(self, ctx), err)]
    #[graphql(guard = "LoggedInGuard")]
    async fn owned_classes(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let class_repo = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = class::ClassByOwnerId(Uuid::parse_str(&self.id)?);
        let classes = class_repo.load_many([id].into_iter()).await?;

        Ok(classes.into_values().map(|c| c.into()).collect())
    }
}
