use ::entity::{user, user::Entity as User};
use sea_orm::DatabaseConnection;
use sea_orm::*;
use uuid::Uuid;

pub async fn user_by_username(
    username: String,
    db: &DatabaseConnection,
) -> Result<Option<user::Model>, DbErr> {
    let user = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await?;

    Ok(user)
}

pub async fn user_by_id(id: Uuid, db: &DatabaseConnection) -> Result<Option<user::Model>, DbErr> {
    let user = User::find().filter(user::Column::Id.eq(id)).one(db).await?;
    Ok(user)
}
