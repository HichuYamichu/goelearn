use ::entity::sea_orm_active_enums::*;
use ::entity::user;
use async_graphql::InputObject;
use sea_orm::*;
use uuid::Uuid;

#[derive(InputObject)]
pub struct SignupInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl SignupInput {
    pub fn into_active_model(self) -> user::ActiveModel {
        user::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(self.username.into()),
            email: Set(self.email.into()),
            password: Set(self.password.into()),
            created_at: Set(chrono::offset::Utc::now().naive_utc()),
            deleted_at: Set(None),
            active: Set(false),
            user_type: Set(UserType::Regular),
            ..Default::default()
        }
    }
}
