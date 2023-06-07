use std::collections::HashMap;

use ::entity::membership;
use ::entity::{membership::Entity as Membership, user, user::Entity as User};
use async_graphql::dataloader::Loader;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct UsersByClassId(pub Uuid);

#[async_trait]
impl Loader<UsersByClassId> for DatabaseConnection {
    type Value = Vec<user::Model>;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[UsersByClassId],
    ) -> Result<HashMap<UsersByClassId, Self::Value>, Self::Error> {
        let memberships = Membership::find()
            .filter(membership::Column::ClassId.is_in(keys.iter().map(|k| k.0).into_iter()))
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
pub struct UserByAuthorId(pub Uuid);

#[async_trait]
impl Loader<UserByAuthorId> for DatabaseConnection {
    type Value = user::Model;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[UserByAuthorId],
    ) -> Result<HashMap<UserByAuthorId, Self::Value>, Self::Error> {
        let users = User::find()
            .filter(user::Column::Id.is_in(keys.iter().map(|k| k.0).into_iter()))
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
pub trait UserRepoExt {
    async fn user_by_username(&self, username: String) -> Result<Option<user::Model>, DbErr>;
    async fn user_by_id(&self, id: Uuid) -> Result<Option<user::Model>, DbErr>;
    async fn create_user(&self, si: user::ActiveModel) -> Result<Uuid, DbErr>;
    async fn activate_user(&self, id: Uuid) -> Result<(), DbErr>;
}

#[async_trait]
impl UserRepoExt for DatabaseConnection {
    async fn user_by_username(&self, username: String) -> Result<Option<user::Model>, DbErr> {
        let user = User::find()
            .filter(user::Column::Username.eq(username))
            .one(self)
            .await?;

        Ok(user)
    }

    async fn user_by_id(&self, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        let user = User::find()
            .filter(user::Column::Id.eq(id))
            .one(self)
            .await?;
        Ok(user)
    }

    async fn create_user(&self, si: user::ActiveModel) -> Result<Uuid, DbErr> {
        let u = User::insert(si).exec(self).await?;
        Ok(u.last_insert_id)
    }

    async fn activate_user(&self, id: Uuid) -> Result<(), DbErr> {
        User::update(user::ActiveModel {
            id: Set(id),
            active: Set(true),
            ..Default::default()
        })
        .exec(self)
        .await?;

        Ok(())
    }
}
