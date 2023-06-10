use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{error::DbErr, TransactionError};
use serde_json::json;
use std::sync::Arc;

#[derive(Debug)]
pub enum AppError {
    Auth,
    NotFound {
        what: &'static str,
        with: &'static str,
        why: String,
    },
    UserError(UserError),
    InternalError(InternalError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Auth => write!(f, "Authentication error"),
            AppError::NotFound { what, with, why } => {
                write!(f, "`{what}` with `{with}` = `{why}` was not found")
            }
            AppError::UserError(err) => write!(f, "User error: {err}"),
            AppError::InternalError(err) => write!(f, "Internal server error: {err}"),
        }
    }
}

#[derive(Debug)]
pub enum InternalError {
    DB(DbErr),
    DBTrans(TransactionError<DbErr>),
    DBArced(Arc<DbErr>),
    JWT(jsonwebtoken::errors::Error),
    Argon2(argon2_async::Error),
    Redis(redis::RedisError),
    Email(lettre::transport::smtp::Error),
    Io(std::io::Error),
    S3(s3::error::S3Error),
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalError::DB(err) => write!(f, "Database error: {err}"),
            InternalError::DBTrans(err) => write!(f, "Database error: {err}"),
            InternalError::DBArced(err) => write!(f, "Database error: {err}"),
            InternalError::JWT(err) => write!(f, "JWT error: {err}"),
            InternalError::Argon2(err) => write!(f, "Argon2 error: {err}"),
            InternalError::Redis(err) => write!(f, "Redis error: {err}"),
            InternalError::Email(err) => write!(f, "Email error: {err}"),
            InternalError::Io(err) => write!(f, "IO error: {err}"),
            InternalError::S3(err) => write!(f, "S3 error: {err}"),
        }
    }
}

#[derive(Debug)]
pub enum UserError {
    BadInput {
        simple: &'static str,
        detailed: String,
    },
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::BadInput {
                simple: _,
                detailed,
            } => write!(f, "Bad input: {detailed}"),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(inner: DbErr) -> Self {
        AppError::InternalError(InternalError::DB(inner))
    }
}

impl From<Arc<DbErr>> for AppError {
    fn from(inner: Arc<DbErr>) -> Self {
        AppError::InternalError(InternalError::DBArced(inner))
    }
}

impl From<TransactionError<DbErr>> for AppError {
    fn from(inner: TransactionError<DbErr>) -> Self {
        AppError::InternalError(InternalError::DBTrans(inner))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(inner: jsonwebtoken::errors::Error) -> Self {
        AppError::InternalError(InternalError::JWT(inner))
    }
}

impl From<argon2_async::Error> for AppError {
    fn from(inner: argon2_async::Error) -> Self {
        AppError::InternalError(InternalError::Argon2(inner))
    }
}

impl From<uuid::Error> for AppError {
    fn from(inner: uuid::Error) -> Self {
        AppError::UserError(UserError::BadInput {
            simple: "Invalid Uuid",
            detailed: inner.to_string(),
        })
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(_inner: chrono::ParseError) -> Self {
        AppError::UserError(UserError::BadInput {
            simple: "Invalid date",
            detailed: "Invalid date".into(),
        })
    }
}

impl From<redis::RedisError> for AppError {
    fn from(inner: redis::RedisError) -> Self {
        AppError::InternalError(InternalError::Redis(inner))
    }
}

impl From<lettre::transport::smtp::Error> for AppError {
    fn from(inner: lettre::transport::smtp::Error) -> Self {
        AppError::InternalError(InternalError::Email(inner))
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(_inner: base64::DecodeError) -> Self {
        AppError::UserError(UserError::BadInput {
            simple: "Invalid input",
            detailed: "Invalid input".into(),
        })
    }
}

impl From<std::string::FromUtf8Error> for AppError {
    fn from(_inner: std::string::FromUtf8Error) -> Self {
        AppError::UserError(UserError::BadInput {
            simple: "Invalid input",
            detailed: "Invalid input".into(),
        })
    }
}

impl From<std::io::Error> for AppError {
    fn from(inner: std::io::Error) -> Self {
        AppError::InternalError(InternalError::Io(inner))
    }
}

impl From<s3::error::S3Error> for AppError {
    fn from(inner: s3::error::S3Error) -> Self {
        AppError::InternalError(InternalError::S3(inner))
    }
}

impl std::error::Error for AppError {} // TODO: is this needed?

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
            AppError::NotFound { what, with, why } => (
                StatusCode::NOT_FOUND,
                format!("`{what}` with `{with}` = `{why}` was not found"),
            ),
            AppError::UserError(err) => match err {
                UserError::BadInput {
                    simple,
                    detailed: _,
                } => (StatusCode::BAD_REQUEST, simple.into()),
            },
            AppError::Auth => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
