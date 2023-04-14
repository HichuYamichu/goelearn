use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};

use uuid::Uuid;

use crate::core::repo::class::ClassRepo;
use crate::core::repo::membership::{self, MembershipRepo};
use crate::core::{repo::class, AppError};

use super::ClassObject;

#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
pub struct UserObject {
    pub id: ID,
    pub username: String,
    pub email: String,
    // pub user_type: UserType,
}

impl From<::entity::user::Model> for UserObject {
    fn from(u: ::entity::user::Model) -> Self {
        Self {
            id: ID::from(u.id),
            username: u.username,
            email: u.email,
        }
    }
}

#[ComplexObject]
impl UserObject {
    async fn clesses(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let membership_repo = ctx.data::<DataLoader<MembershipRepo>>().unwrap();
        let class_repo = ctx.data::<DataLoader<ClassRepo>>().unwrap();

        let id = membership::MembershipByUserId(Uuid::parse_str(&self.id)?);
        let memberships = membership_repo.load_many([id].into_iter()).await?;
        let class_ids = memberships.values().map(|m| class::ClassById(m.class_id));
        let classes = class_repo.load_many(class_ids).await?;

        Ok(classes.into_iter().map(|(_, c)| c.into()).collect())
    }

    async fn owned_classes(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let class_repo = ctx.data::<DataLoader<ClassRepo>>().unwrap();

        let id = class::ClassByOwnerId(Uuid::parse_str(&self.id)?);
        let classes = class_repo.load_many([id].into_iter()).await?;

        Ok(classes.into_iter().map(|(_, c)| c.into()).collect())
    }
}
