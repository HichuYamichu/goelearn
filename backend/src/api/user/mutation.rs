use crate::core::auth::Claims;
use crate::core::AppError;
use crate::core::UserError;
use crate::{HOST_URL, MAIL_PASSWORD, MAIL_USERNAME, SECRET};
use async_graphql::{dataloader::DataLoader, Context, Object};
use lettre::AsyncTransport;
use tracing::instrument;

use sea_orm::DatabaseConnection;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use uuid::Uuid;

use super::object::{LoginInput, LoginResult, SignupInput};
use super::UserRepo;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[instrument(skip(self, ctx), err)]
    pub async fn signup(
        &self,
        ctx: &Context<'_>,
        mut input: SignupInput,
    ) -> Result<String, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();

        let avatar = input.avatar.take();
        let id = register_user(input, data_loader, avatar.is_some()).await?;

        if let Some(avatar) = avatar {
            let avatar = avatar.value(ctx)?;
            if avatar.content_type.is_none()
                || avatar.content_type.as_ref().unwrap() != "image/jpeg"
            {
                return Err(AppError::UserError(UserError::BadInput {
                    simple: "avatar must be a jpeg image",
                    detailed: "avatar must be a jpeg image".into(),
                }));
            }

            let s3_path = format!("user-avatars/{id}");
            let mut reader = avatar.into_async_read().compat();
            s3_bucket
                .put_object_stream_with_content_type(&mut reader, s3_path, "image/jpeg")
                .await?;
        }

        Ok(id.to_string())
    }

    #[instrument(skip(self, ctx), err)]
    pub async fn login(
        &self,
        ctx: &Context<'_>,
        input: LoginInput,
    ) -> Result<LoginResult, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let res = login_user(input, data_loader).await?;
        Ok(res)
    }
}

pub async fn login_user(
    creadentials: LoginInput,
    data_loader: &DataLoader<DatabaseConnection>,
) -> Result<LoginResult, AppError> {
    let user = UserRepo::find_by_username(data_loader, creadentials.username.clone()).await?;

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
    data_loader: &DataLoader<DatabaseConnection>,
    has_avatar: bool,
) -> Result<Uuid, AppError> {
    let hash = argon2_async::hash(creadentials.password).await?;
    creadentials.password = hash;
    let addr = creadentials.email.clone();
    let username = creadentials.username.clone();
    let id = UserRepo::create_user(data_loader, creadentials.into_active_model(has_avatar)).await?;

    let host = HOST_URL.to_string();
    let body = format!(
        r#"Hello, {username}! Please, follow the link to activate your account: <a href="{host}/api/v1/user/activate/{id}<a>">{host}/api/v1/user/activate/{id}<a>"#
    );

    let email = lettre::Message::builder()
        .from(
            MAIL_USERNAME
                .parse()
                .expect("Service username should be valid email"),
        )
        .to(addr.parse().expect("User email should be valid email"))
        .subject("Account activation")
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(body)
        .expect("Email should be valid");

    let creds = lettre::transport::smtp::authentication::Credentials::new(
        MAIL_USERNAME.to_string(),
        MAIL_PASSWORD.to_string(),
    );

    let mailer: lettre::AsyncSmtpTransport<lettre::Tokio1Executor> =
        lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::relay("smtp.gmail.com")
            .expect("Relay should be valid")
            .credentials(creds)
            .build();

    mailer.send(email).await?;

    Ok(id)
}
