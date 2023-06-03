use std::collections::HashMap;

use ::entity::membership;
use ::entity::{membership::Entity as Membership, user, user::Entity as User};
use async_graphql::dataloader::Loader;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;



#[derive(Debug, Clone)]
pub struct UserRepo {
    conn: DatabaseConnection,
}

impl UserRepo {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn user_by_username(&self, username: String) -> Result<Option<user::Model>, DbErr> {
        let user = User::find()
            .filter(user::Column::Username.eq(username))
            .one(&self.conn)
            .await?;

        Ok(user)
    }

    pub async fn user_by_id(&self, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        let user = User::find()
            .filter(user::Column::Id.eq(id))
            .one(&self.conn)
            .await?;
        Ok(user)
    }

    pub async fn create_user(&self, si: user::ActiveModel) -> Result<Uuid, DbErr> {
        let u = User::insert(si).exec(&self.conn).await?;
        Ok(u.last_insert_id)
    }

    pub async fn activate_user(&self, id: Uuid) -> Result<(), DbErr> {
        User::update(user::ActiveModel {
            id: Set(id),
            active: Set(true),
            ..Default::default()
        })
        .exec(&self.conn)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct UsersByClassId(pub Uuid);

#[async_trait]
impl Loader<UsersByClassId> for UserRepo {
    type Value = Vec<user::Model>;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[UsersByClassId],
    ) -> Result<HashMap<UsersByClassId, Self::Value>, Self::Error> {
        let memberships = Membership::find()
            .filter(membership::Column::ClassId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(&self.conn)
            .await?;

        let users = memberships.load_one(User, &self.conn).await?;

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
impl Loader<UserByAuthorId> for UserRepo {
    type Value = user::Model;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[UserByAuthorId],
    ) -> Result<HashMap<UserByAuthorId, Self::Value>, Self::Error> {
        let users = User::find()
            .filter(user::Column::Id.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(&self.conn)
            .await?;

        let mut res = HashMap::<_, _>::new();
        for u in users.into_iter() {
            res.entry(*keys.iter().find(|k| k.0 == u.id).unwrap())
                .or_insert(u);
        }

        Ok(res)
    }
}
