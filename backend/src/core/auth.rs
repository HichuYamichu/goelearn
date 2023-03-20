use crate::objects::{LoginInput, LoginResult, SignupInput};
use ::entity::{user, user::Entity as User};
use sea_orm::DatabaseConnection;
use sea_orm::*;
use serde::{Deserialize, Serialize};

use super::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub async fn user_login(
    creadentials: LoginInput,
    db: &DatabaseConnection,
) -> Result<LoginResult, AppError> {
    let user = User::find()
        .filter(user::Column::Username.eq(creadentials.username.clone()))
        .one(db)
        .await?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err(AppError::NotFound {
                what: "User",
                with: "username",
                why: creadentials.username,
            })
        }
    };

    if user.password != creadentials.password {
        return Err(AppError::Auth);
    }

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            sub: user.id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp(),
        },
        &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()),
    )?;

    Ok(LoginResult { token })
}

pub async fn user_register(
    creadentials: SignupInput,
    db: &DatabaseConnection,
) -> Result<User, DbErr> {
    todo!()
}
