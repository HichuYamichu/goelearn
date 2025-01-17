use std::collections::HashMap;

use ::entity::membership;
use ::entity::sea_orm_active_enums::UserType;
use ::entity::{
    membership::Entity as Membership, password_reset_token,
    password_reset_token::Entity as PasswordResetToken, user, user::Entity as User,
};
use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::core::{option_to_active_value, AppError};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct UsersByClassId(Uuid);

#[async_trait]
impl Loader<UsersByClassId> for DatabaseConnection {
    type Value = Vec<user::Model>;
    type Error = Arc<DbErr>;

    #[instrument(skip(self), err(Debug))]
    async fn load(
        &self,
        keys: &[UsersByClassId],
    ) -> Result<HashMap<UsersByClassId, Self::Value>, Self::Error> {
        let memberships = Membership::find()
            .filter(membership::Column::ClassId.is_in(keys.iter().map(|k| k.0)))
            .all(self)
            .await?;

        let users = memberships.load_one(User, self).await?;

        let mut res = HashMap::<_, _>::new();
        for (m, u) in memberships
            .into_iter()
            .zip(users.into_iter())
            .filter_map(|(m, u)| Some((m, u.expect("Membership to User is not optional"))))
        {
            res.entry(*keys.iter().find(|k| k.0 == m.class_id).unwrap())
                .or_insert_with(Vec::new)
                .push(u);
        }

        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct UserByAuthorId(Uuid);

#[async_trait]
impl Loader<UserByAuthorId> for DatabaseConnection {
    type Value = user::Model;
    type Error = Arc<DbErr>;

    #[instrument(skip(self), err(Debug))]
    async fn load(
        &self,
        keys: &[UserByAuthorId],
    ) -> Result<HashMap<UserByAuthorId, Self::Value>, Self::Error> {
        let users = User::find()
            .filter(user::Column::Id.is_in(keys.iter().map(|k| k.0)))
            .all(self)
            .await?;

        let mut res = HashMap::<_, _>::new();
        for key in keys.iter() {
            res.entry(*key)
                .or_insert(users.iter().find(|u| u.id == key.0).unwrap().clone());
        }

        Ok(res)
    }
}

#[async_trait]
pub trait UserRepo {
    async fn find_by_username(&self, username: String) -> Result<Option<user::Model>, DbErr>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<user::Model>, DbErr>;
    async fn create_user(&self, si: user::ActiveModel) -> Result<Uuid, DbErr>;
    async fn activate_user(&self, id: Uuid) -> Result<(), DbErr>;

    async fn find_by_class_id(
        &self,
        class_id: Uuid,
    ) -> Result<Option<Vec<user::Model>>, Arc<DbErr>>;

    async fn update(
        &self,
        user_id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        password: String,
    ) -> Result<user::Model, AppError>;

    async fn change_password(
        &self,
        user_id: Uuid,
        old_password: String,
        new_password: String,
    ) -> Result<user::Model, AppError>;

    async fn find_all(&self) -> Result<Vec<user::Model>, DbErr>;
    async fn admin_user_update(
        &self,
        user_id: Uuid,
        user_type: UserType,
        deleted_at: Option<NaiveDateTime>,
    ) -> Result<user::Model, AppError>;

    async fn emergency_change_password(
        &self,
        token: Uuid,
        password: String,
    ) -> Result<user::Model, AppError>;

    async fn find_user_by_reset_token(&self, token: Uuid) -> Result<Option<user::Model>, DbErr>;
    async fn create_password_change_token(
        &self,
        user_email: String,
    ) -> Result<(Uuid, user::Model), AppError>;
}

#[async_trait]
impl UserRepo for DataLoader<DatabaseConnection> {
    #[instrument(skip(self), err(Debug))]
    async fn find_by_username(&self, username: String) -> Result<Option<user::Model>, DbErr> {
        let user = User::find()
            .filter(user::Column::Username.eq(username))
            .one(self.loader())
            .await?;

        Ok(user)
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        let user = User::find()
            .filter(user::Column::Id.eq(id))
            .one(self.loader())
            .await?;
        Ok(user)
    }

    #[instrument(skip(self), err(Debug))]
    async fn create_user(&self, mut si: user::ActiveModel) -> Result<Uuid, DbErr> {
        if cfg!(debug_assertions) {
            si.active = Set(true);
        }
        let u = User::insert(si).exec(self.loader()).await?;
        Ok(u.last_insert_id)
    }

    #[instrument(skip(self), err(Debug))]
    async fn activate_user(&self, id: Uuid) -> Result<(), DbErr> {
        User::update(user::ActiveModel {
            id: Set(id),
            active: Set(true),
            ..Default::default()
        })
        .exec(self.loader())
        .await?;

        Ok(())
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_by_class_id(
        &self,
        class_id: Uuid,
    ) -> Result<Option<Vec<user::Model>>, Arc<DbErr>> {
        let users = self.load_one(UsersByClassId(class_id)).await?;
        Ok(users)
    }

    #[instrument(skip(self), err(Debug))]
    async fn update(
        &self,
        user_id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        password: String,
    ) -> Result<user::Model, AppError> {
        let user = User::find()
            .filter(user::Column::Id.eq(user_id))
            .one(self.loader())
            .await?;

        let user = match user {
            Some(user) => user,
            None => {
                return Err(AppError::not_found(
                    "user not found",
                    "user",
                    "id",
                    &user_id.to_string(),
                ));
            }
        };

        let is_match = argon2_async::verify(password, user.password).await?;
        if !is_match {
            return Err(AppError::auth("Bad credentials"));
        }

        let user = User::update(user::ActiveModel {
            id: Set(user_id),
            first_name: option_to_active_value(first_name),
            last_name: option_to_active_value(last_name),
            ..Default::default()
        })
        .exec(self.loader())
        .await?;

        Ok(user)
    }

    #[instrument(skip(self), err(Debug))]
    async fn change_password(
        &self,
        user_id: Uuid,
        old_password: String,
        new_password: String,
    ) -> Result<user::Model, AppError> {
        let user = User::find()
            .filter(user::Column::Id.eq(user_id))
            .one(self.loader())
            .await?;

        let user = match user {
            Some(user) => user,
            None => {
                return Err(AppError::not_found(
                    "user not found",
                    "user",
                    "id",
                    &user_id.to_string(),
                ));
            }
        };
        let is_match = argon2_async::verify(old_password, user.password).await?;
        if !is_match {
            return Err(AppError::auth("Bad credentials"));
        }

        let hash = argon2_async::hash(new_password).await?;

        let user_update = user::ActiveModel {
            id: Set(user_id),
            password: Set(hash),
            ..Default::default()
        };

        let user = User::update(user_update).exec(self.loader()).await?;

        Ok(user)
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_all(&self) -> Result<Vec<user::Model>, DbErr> {
        let users = User::find()
            .filter(user::Column::UserType.ne(UserType::Admin))
            .all(self.loader())
            .await?;
        Ok(users)
    }

    #[instrument(skip(self), err(Debug))]
    async fn admin_user_update(
        &self,
        user_id: Uuid,
        user_type: UserType,
        deleted_at: Option<NaiveDateTime>,
    ) -> Result<user::Model, AppError> {
        let user = User::find()
            .filter(user::Column::Id.eq(user_id))
            .one(self.loader())
            .await?;

        let user = match user {
            Some(user) => user,
            None => {
                return Err(AppError::not_found(
                    "user not found",
                    "user",
                    "id",
                    &user_id.to_string(),
                ));
            }
        };

        let user_update = user::ActiveModel {
            id: Set(user_id),
            user_type: Set(user_type),
            deleted_at: Set(deleted_at),
            ..Default::default()
        };

        let user = User::update(user_update).exec(self.loader()).await?;

        Ok(user)
    }

    #[instrument(skip(self), err(Debug))]
    async fn emergency_change_password(
        &self,
        token: Uuid,
        password: String,
    ) -> Result<user::Model, AppError> {
        let user = PasswordResetToken::find()
            .filter(password_reset_token::Column::Id.eq(token))
            .find_also_related(User)
            .one(self.loader())
            .await?
            .map(|(_, u)| u.expect("PasswordResetToken to User is not optional"));

        let user = match user {
            Some(user) => user,
            None => {
                return Err(AppError::not_found(
                    "user not found",
                    "user",
                    "id",
                    &token.to_string(),
                ));
            }
        };

        let hash = argon2_async::hash(password).await?;

        let user_update = user::ActiveModel {
            id: Set(user.id),
            password: Set(hash),
            ..Default::default()
        };

        let user = User::update(user_update).exec(self.loader()).await?;
        PasswordResetToken::delete_by_id(token)
            .exec(self.loader())
            .await?;

        Ok(user)
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_user_by_reset_token(&self, token: Uuid) -> Result<Option<user::Model>, DbErr> {
        let user = PasswordResetToken::find()
            .filter(password_reset_token::Column::Id.eq(token))
            .find_also_related(User)
            .one(self.loader())
            .await?
            .map(|(_, u)| u.expect("PasswordResetToken to User is not optional"));

        Ok(user)
    }

    #[instrument(skip(self), err(Debug))]
    async fn create_password_change_token(
        &self,
        user_email: String,
    ) -> Result<(Uuid, user::Model), AppError> {
        let user = User::find()
            .filter(user::Column::Email.eq(user_email.clone()))
            .one(self.loader())
            .await?;

        let user = match user {
            Some(user) => user,
            None => {
                return Err(AppError::not_found(
                    "user not found".to_string(),
                    "user",
                    "email",
                    user_email,
                ));
            }
        };

        let active_token = password_reset_token::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user.id),
            ..Default::default()
        };
        let token_id = PasswordResetToken::insert(active_token)
            .exec(self.loader())
            .await?
            .last_insert_id;

        Ok((token_id, user))
    }
}
