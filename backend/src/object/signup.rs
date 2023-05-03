use ::entity::sea_orm_active_enums::*;
use ::entity::user;
use async_graphql::InputObject;
use async_graphql::Upload;
use sea_orm::*;
use uuid::Uuid;

#[derive(InputObject)]
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
