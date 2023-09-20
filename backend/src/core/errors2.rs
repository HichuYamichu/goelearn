use async_graphql::ErrorExtensions;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use deadpool_redis::{redis, PoolError};
use sea_orm::{error::DbErr, TransactionError};
use serde_json::json;
use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

const INTERNAL_ERROR_MSG: &str = "Something went wrong when processing your request.";

#[derive(Debug)]
pub struct AppError {
    message: String,
    kind: ErrorKind,
}

impl AppError {
    pub fn auth<T: Into<String>>(message: T) -> Self {
        AppError {
            message: message.into(),
            kind: ErrorKind::Auth,
        }
    }

    pub fn not_found<T: Into<String>>(
        message: T,
        resource: &'static str,
        attribute: &'static str,
        value: T,
    ) -> Self {
        AppError {
            message: message.into(),
            kind: ErrorKind::NotFound {
                resource,
                attribute,
                value: value.into(),
            },
        }
    }

    pub fn user<T: Into<String>>(message: T, user_err: UserError) -> Self {
        AppError {
            message: message.into(),
            kind: ErrorKind::User(user_err),
        }
    }
}

// INFO: https://github.com/async-graphql/async-graphql/issues/1265
impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Auth => write!(f, "Authentication error"),
            ErrorKind::NotFound {
                resource,
                attribute,
                value,
            } => write!(
                f,
                "`{resource}` with `{attribute}` = `{value}` was not found",
                resource = resource,
                attribute = attribute,
                value = value
            ),
            ErrorKind::User(user_err) => write!(f, "User error: {user_err}"),
            ErrorKind::Internal(internal_err) => write!(f, "Internal server error: {internal_err}"),
        }
    }
}

impl std::error::Error for AppError {}

#[derive(Debug)]
pub enum ErrorKind {
    Auth,
    NotFound {
        resource: &'static str,
        attribute: &'static str,
        value: String,
    },
    User(UserError),
    Internal(InternalError), // kept so that `err` macro from tracing creates useful report
}

// INFO: remove clone when `extend` teakes ownership if ever
impl ErrorExtensions for AppError {
    fn extend(&self) -> async_graphql::Error {
        let AppError { message, kind } = self;

        async_graphql::Error::new(message).extend_with(|_err, e| match kind {
            ErrorKind::Auth => {}
            ErrorKind::NotFound {
                resource,
                attribute,
                value,
            } => {
                e.set("resource", *resource);
                e.set("attribute", *attribute);
                e.set("value", value.clone());
            }
            ErrorKind::User(UserError::BadInput {
                parameter,
                given_value,
            }) => {
                e.set("parameter", *parameter);
                e.set("given_value", given_value.clone());
            }
            ErrorKind::Internal(_) => {}
        })
    }
}

// impl From<AppError> for async_graphql::Error {
//     fn from(inner: AppError) -> Self {
//         inner.extend()
//     }
// }

#[derive(Debug)]
pub enum InternalError {
    DB(DbErr),
    DBTrans(TransactionError<DbErr>),
    DBArced(Arc<DbErr>),
    Jwt(jsonwebtoken::errors::Error),
    Argon2(argon2_async::Error),
    Redis(redis::RedisError),
    RedisPool(PoolError),
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
            InternalError::Jwt(err) => write!(f, "Jwt error: {err}"),
            InternalError::Argon2(err) => write!(f, "Argon2 error: {err}"),
            InternalError::Redis(err) => write!(f, "Redis error: {err}"),
            InternalError::RedisPool(err) => write!(f, "Redis pool error: {err}"),
            InternalError::Email(err) => write!(f, "Email error: {err}"),
            InternalError::Io(err) => write!(f, "IO error: {err}"),
            InternalError::S3(err) => write!(f, "S3 error: {err}"),
        }
    }
}

#[derive(Debug)]
pub enum UserError {
    BadInput {
        parameter: &'static str,
        given_value: String,
    },
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::BadInput {
                parameter,
                given_value,
            } => write!(
                f,
                "Bad input for parameter `{}` with value `{}`",
                parameter, given_value
            ),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(inner: DbErr) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::DB(inner)),
        }
    }
}

impl From<Arc<DbErr>> for AppError {
    fn from(inner: Arc<DbErr>) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::DBArced(inner)),
        }
    }
}

impl From<TransactionError<DbErr>> for AppError {
    fn from(inner: TransactionError<DbErr>) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::DBTrans(inner)),
        }
    }
}

impl From<sea_orm::TransactionError<AppError>> for AppError {
    fn from(inner: sea_orm::TransactionError<AppError>) -> Self {
        match inner {
            TransactionError::Connection(err) => AppError::from(err),
            TransactionError::Transaction(err) => err,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(inner: jsonwebtoken::errors::Error) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::Jwt(inner)),
        }
    }
}

impl From<argon2_async::Error> for AppError {
    fn from(inner: argon2_async::Error) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::Argon2(inner)),
        }
    }
}

impl From<uuid::Error> for AppError {
    fn from(inner: uuid::Error) -> Self {
        AppError {
            message: "Invalid Uuid".to_owned(),
            kind: ErrorKind::User(UserError::BadInput {
                parameter: "uuid",
                given_value: inner.to_string(),
            }),
        }
    }
}

impl From<redis::RedisError> for AppError {
    fn from(inner: redis::RedisError) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::Redis(inner)),
        }
    }
}

impl From<PoolError> for AppError {
    fn from(inner: PoolError) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::RedisPool(inner)),
        }
    }
}

impl From<lettre::transport::smtp::Error> for AppError {
    fn from(inner: lettre::transport::smtp::Error) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::Email(inner)),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(inner: std::io::Error) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::Io(inner)),
        }
    }
}

impl From<s3::error::S3Error> for AppError {
    fn from(inner: s3::error::S3Error) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::S3(inner)),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let AppError { message, kind } = self;

        let status = match kind {
            ErrorKind::Auth => StatusCode::UNAUTHORIZED,
            ErrorKind::NotFound { .. } => StatusCode::NOT_FOUND,
            ErrorKind::User(_) => StatusCode::BAD_REQUEST,
            ErrorKind::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({
            "message": message,
        });

        (status, Json(body)).into_response()
    }
}
