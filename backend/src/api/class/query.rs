use crate::{api::UserRepo, core::ClassOwnerGuard};
use async_graphql::{dataloader::DataLoader, Context, Object, ID};

use entity::sea_orm_active_enums::UserType;
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    api::user::UserObject,
    core::{AppError, Claims, ClassMemberGuard, LoggedInGuard},
};

use super::{object::InviteObject, ClassObject, ClassRepo};

#[derive(Default)]
pub struct ClassQuery;

#[Object]
impl ClassQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassMemberGuard::new(id.clone()))")]
    async fn class_by_id(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<Option<ClassObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = Uuid::parse_str(id.as_str())?;
        let c = ClassRepo::find_by_id(data_loader, id).await?;
        Ok(c.map(|c| c.into()))
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn random_classes(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();

        let user_id = Uuid::parse_str(
            claims
                .as_ref()
                .expect("Guard ensures claims exist")
                .sub
                .as_str(),
        )?;
        let banned_in = ClassRepo::get_user_bans(data_loader, user_id).await?;
        let c = ClassRepo::find_random(data_loader, 10, banned_in).await?;

        Ok(c.into_iter().map(|c| c.into()).collect())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn classes_by_search(
        &self,
        ctx: &Context<'_>,
        query: String,
    ) -> Result<Vec<ClassObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let claims = ctx.data_unchecked::<Option<Claims>>();

        let user_id = Uuid::parse_str(
            claims
                .as_ref()
                .expect("Guard ensures claims exist")
                .sub
                .as_str(),
        )?;

        let user = UserRepo::find_by_id(data_loader, user_id)
            .await?
            .expect("User must exist");

        match user.user_type {
            UserType::Admin => {
                let c = ClassRepo::find_all(data_loader).await?;
                return Ok(c.into_iter().map(|c| c.into()).collect());
            }
            _ => {
                let banned_in = ClassRepo::get_user_bans(data_loader, user_id).await?;
                let c = match query.as_str() {
                    "" => ClassRepo::find_random(data_loader, 10, banned_in).await?,
                    _ => ClassRepo::find_by_query(data_loader, query, banned_in).await?,
                };

                Ok(c.into_iter().map(|c| c.into()).collect())
            }
        }
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassOwnerGuard::new(class_id.clone()))")]
    pub async fn banned_members(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
    ) -> Result<Vec<UserObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let users =
            ClassRepo::get_class_bans(data_loader, Uuid::parse_str(class_id.as_str())?).await?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard.and(ClassOwnerGuard::new(class_id.clone()))")]
    pub async fn invites(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
    ) -> Result<Vec<InviteObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let invites =
            ClassRepo::get_invites(data_loader, Uuid::parse_str(class_id.as_str())?).await?;
        Ok(invites.into_iter().map(|i| i.into()).collect())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    pub async fn class_by_invite_id(
        &self,
        ctx: &Context<'_>,
        invite_id: ID,
    ) -> Result<Option<ClassObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let invite_id = Uuid::parse_str(invite_id.as_str())?;
        let class = ClassRepo::find_by_invite_id(data_loader, invite_id).await?;
        Ok(class.map(|c| c.into()))
    }
}
