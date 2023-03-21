use ::entity::{user, user::Entity as User};
use sea_orm::DatabaseConnection;
use sea_orm::*;
use uuid::Uuid;

use crate::objects::SignupInput;

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

    pub async fn create_user(&self, si: SignupInput) -> Result<Uuid, DbErr> {
        let u = User::insert(si.into_active_model())
            .exec(&self.conn)
            .await?;
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
