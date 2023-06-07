use super::repo::user::UserRepoExt;
use crate::{
    object::{LoginInput, LoginResult, SignupInput},
    HOST_URL, MAIL_PASSWORD, MAIL_USERNAME, SECRET,
};
use async_graphql::{Context, Guard};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use lettre::AsyncSmtpTransport;
use lettre::AsyncTransport;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    Tokio1Executor,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub async fn login_user(
    creadentials: LoginInput,
    user_repo: &DatabaseConnection,
) -> Result<LoginResult, AppError> {
    let user = user_repo
        .user_by_username(creadentials.username.clone())
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

    if !user.active {
        return Err(AppError::Auth);
    }

    if user.deleted_at.is_some() {
        return Err(AppError::Auth);
    }

    let is_match = argon2_async::verify(creadentials.password, user.password).await?;
    if !is_match {
        return Err(AppError::Auth);
    }

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            sub: user.id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp(),
        },
        &jsonwebtoken::EncodingKey::from_secret(SECRET.as_ref()),
    )?;

    Ok(LoginResult { token })
}

pub async fn register_user(
    mut creadentials: SignupInput,
    user_repo: &DatabaseConnection,
    has_avatar: bool,
) -> Result<Uuid, AppError> {
    let hash = argon2_async::hash(creadentials.password).await?;
    creadentials.password = hash;
    let addr = creadentials.email.clone();
    let username = creadentials.username.clone();
    let id = user_repo
        .create_user(creadentials.into_active_model(has_avatar))
        .await?;

    let host = HOST_URL.to_string();
    let body = format!(
        r#"Hello, {username}! Please, follow the link to activate your account: <a href="{host}/api/v1/user/activate/{id}<a>">{host}/api/v1/user/activate/{id}<a>"#
    );

    let email = Message::builder()
        .from(
            MAIL_USERNAME
                .parse()
                .expect("Service username should be valid email"),
        )
        .to(addr.parse().expect("User email should be valid email"))
        .subject("Account activation")
        .header(ContentType::TEXT_HTML)
        .body(body)
        .expect("Email should be valid");

    let creds = Credentials::new(MAIL_USERNAME.to_string(), MAIL_PASSWORD.to_string());

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .expect("Relay should be valid")
            .credentials(creds)
            .build();

    mailer.send(email).await?;

    Ok(id)
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(req, state)
                .await
                .map_err(|_| AppError::Auth)?;

        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(SECRET.as_ref()),
            &validation,
        )
        .map_err(|_| AppError::Auth)?;

        Ok(token_data.claims)
    }
}

pub struct LoggedInGuard;

#[async_trait]
impl Guard for LoggedInGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), async_graphql::Error> {
        let claims = ctx.data_unchecked::<Option<Claims>>();

        match claims {
            Some(_) => Ok(()),
            None => Err(AppError::Auth.into()),
        }
    }
}
