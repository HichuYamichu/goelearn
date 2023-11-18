use crate::api::MAX_FILE_SIZE;
use crate::core::auth::Claims;
use crate::core::AppError;
use crate::core::UserError;
use crate::{HOST_URL, MAIL_PASSWORD, MAIL_USERNAME, SECRET};
use async_graphql::Upload;
use async_graphql::ID;
use async_graphql::{dataloader::DataLoader, Context, Object};
use chrono::NaiveDateTime;
use entity::user;
use lettre::AsyncTransport;
use tracing::instrument;

use sea_orm::DatabaseConnection;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use uuid::Uuid;

use super::object::UserType;
use super::object::{LoginInput, LoginResult, SignupInput};
use super::UserObject;
use super::UserRepo;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[instrument(skip(self, ctx), err(Debug))]
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
            let exeeds_limit = avatar.size()? > MAX_FILE_SIZE;
            if exeeds_limit {
                return Err(AppError::user("file too large", UserError::FileTooLarge));
            }

            if avatar.content_type.is_none()
                || avatar.content_type.as_ref().unwrap() != "image/jpeg"
            {
                return Err(AppError::user(
                    "Avatar must be a jpeg",
                    UserError::BadInput {
                        parameter: "avatar",
                        given_value: "non jpeg image".into(),
                    },
                ));
            }

            let s3_path = format!("user-avatars/{id}");
            let mut reader = avatar.into_async_read().compat();
            s3_bucket
                .put_object_stream_with_content_type(&mut reader, s3_path, "image/jpeg")
                .await?;
        }

        Ok(id.to_string())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    pub async fn login(
        &self,
        ctx: &Context<'_>,
        input: LoginInput,
    ) -> Result<LoginResult, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let res = login_user(input, data_loader).await?;
        Ok(res)
    }

    #[instrument(skip(self, ctx, avatar), err(Debug))]
    pub async fn update_user(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
        first_name: Option<String>,
        last_name: Option<String>,
        avatar: Option<Upload>,
        password: String,
    ) -> Result<UserObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let s3_bucket = ctx.data_unchecked::<s3::Bucket>();

        let user_id = Uuid::parse_str(&user_id)?;
        if let Some(avatar) = avatar {
            let avatar = avatar.value(ctx)?;
            let exeeds_limit = avatar.size()? > MAX_FILE_SIZE;
            if exeeds_limit {
                return Err(AppError::user("file too large", UserError::FileTooLarge));
            }

            if avatar.content_type.is_none()
                || avatar.content_type.as_ref().unwrap() != "image/jpeg"
            {
                return Err(AppError::user(
                    "Avatar must be a jpeg",
                    UserError::BadInput {
                        parameter: "avatar",
                        given_value: "non jpeg image".into(),
                    },
                ));
            }

            let s3_path = format!("user-avatars/{user_id}");
            let mut reader = avatar.into_async_read().compat();
            s3_bucket
                .put_object_stream_with_content_type(&mut reader, s3_path, "image/jpeg")
                .await?;
        }

        let user = UserRepo::update(data_loader, user_id, first_name, last_name, password).await?;

        Ok(user.into())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn change_password(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
        old_password: String,
        new_password: String,
    ) -> Result<UserObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let user_id = Uuid::parse_str(&user_id)?;
        let user =
            UserRepo::change_password(data_loader, user_id, old_password, new_password).await?;

        Ok(user.into())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn emergency_change_password(
        &self,
        ctx: &Context<'_>,
        token: ID,
        password: String,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let token = Uuid::parse_str(&token)?;

        UserRepo::emergency_change_password(data_loader, token, password).await?;

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn create_password_change_token(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let (token, user) = UserRepo::create_password_change_token(data_loader, email).await?;
        let host = HOST_URL.to_string();
        let username = user.username;
        let addr = user.email;
        let body = format!(
            r#"Hello, {username}! Please, follow the link to change your password: <a href="{host}/password-reset/{token}<a>">{host}/password-reset/{token}<a>"#
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

        Ok(true)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn admin_user_update(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
        user_type: UserType,
        deleted_at: Option<NaiveDateTime>,
    ) -> Result<bool, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();
        let user_id = Uuid::parse_str(&user_id)?;
        UserRepo::admin_user_update(data_loader, user_id, user_type.into(), deleted_at).await?;

        Ok(true)
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
            return Err(AppError::auth("Bad credentials"));
        }
    };

    if !user.active {
        return Err(AppError::auth("User is not active"));
    }

    if user.deleted_at.is_some() {
        return Err(AppError::auth("User is deleted"));
    }

    let is_match = argon2_async::verify(creadentials.password, user.password).await?;
    if !is_match {
        return Err(AppError::auth("Bad credentials"));
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

#[instrument(skip(creadentials, data_loader, has_avatar), err(Debug))]
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

    // skip email sending in debug mode
    if cfg!(debug_assertions) {
        return Ok(id);
    }

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
