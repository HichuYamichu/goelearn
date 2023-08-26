use async_graphql::dataloader::DataLoader;
use async_graphql::{
    ComplexObject, Context, InputObject, Object, Result, SimpleObject, Upload, ID,
};

use entity::sea_orm_active_enums::UserType;
use entity::user;
use partialdebug::placeholder::PartialDebug;
use sea_orm::{DatabaseConnection, Set};
use tracing::instrument;
use uuid::Uuid;

use crate::api::class::{ClassObject, ClassRepo};
use crate::core::{AppError, LoggedInGuard};

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

#[ComplexObject]
impl UserObject {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn clesses(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let user_id = Uuid::parse_str(&self.id)?;
        let classes = ClassRepo::find_by_user_id(data_loader, user_id)
            .await?
            .expect("user id is valid");

        Ok(classes.into_iter().map(|c| c.into()).collect())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn owned_classes(&self, ctx: &Context<'_>) -> Result<Vec<ClassObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let owner_id = Uuid::parse_str(&self.id)?;
        let classes = ClassRepo::find_by_owner_id(data_loader, owner_id)
            .await?
            .expect("owner id is valid");

        Ok(classes.into_iter().map(|c| c.into()).collect())
    }
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

#[derive(InputObject, PartialDebug)]
pub struct SignupInput {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<Upload>,
}

impl SignupInput {
    pub fn into_active_model(self, has_avatar: bool) -> user::ActiveModel {
        user::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(self.username),
            first_name: Set(self.first_name),
            last_name: Set(self.last_name),
            has_avatar: Set(has_avatar),
            email: Set(self.email),
            password: Set(self.password),
            created_at: Set(chrono::offset::Utc::now().naive_utc()),
            deleted_at: Set(None),
            active: Set(false),
            user_type: Set(UserType::Regular),
            ..Default::default()
        }
    }
}

#[derive(InputObject, Clone, Debug)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

pub struct LoginResult {
    pub token: String,
}

#[Object]
impl LoginResult {
    pub async fn token(&self) -> &str {
        &self.token
    }
}
