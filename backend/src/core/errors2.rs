use async_graphql::ErrorExtensions;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{error::DbErr, TransactionError};
use serde_json::json;
use std::sync::Arc;

const INTERNAL_ERROR_MSG: &'static str = "Something went wrong when processing your request.";

#[derive(Debug)]
pub struct AppError {
    message: String,
    kind: ErrorKind,
}

impl AppError {
    pub fn not_found(
        message: String,
        resource: &'static str,
        attribute: &'static str,
        value: String,
    ) -> Self {
        AppError {
            message,
            kind: ErrorKind::NotFound {
                resource,
                attribute,
                value,
            },
        }
    }

    pub fn user(message: String, user_err: UserError) -> Self {
        AppError {
            message,
            kind: ErrorKind::User(user_err),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
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

        async_graphql::Error::new(message).extend_with(|err, e| match kind {
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

#[derive(Debug)]
pub enum InternalError {
    DB(DbErr),
    DBTrans(TransactionError<DbErr>),
    DBArced(Arc<DbErr>),
    Jwt(jsonwebtoken::errors::Error),
    Argon2(argon2_async::Error),
    Redis(redis::RedisError),
    Email(lettre::transport::smtp::Error),
    Io(std::io::Error),
    S3(s3::error::S3Error),
}

#[derive(Debug)]
pub enum UserError {
    BadInput {
        parameter: &'static str,
        given_value: String,
    },
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
            kind: ErrorKind::Internal(InternalError::DBTrans(inner))
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(inner: jsonwebtoken::errors::Error) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::Jwt(inner))
        }
    }
}

impl From<argon2_async::Error> for AppError {
    fn from(inner: argon2_async::Error) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::Argon2(inner))
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
            })
        }
    }
}

impl From<redis::RedisError> for AppError {
    fn from(inner: redis::RedisError) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::Redis(inner))
        }
    }
}

impl From<lettre::transport::smtp::Error> for AppError {
    fn from(inner: lettre::transport::smtp::Error) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::Email(inner))
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(inner: std::io::Error) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::Io(inner))
        }
    }
}

impl From<s3::error::S3Error> for AppError {
    fn from(inner: s3::error::S3Error) -> Self {
        AppError {
            message: INTERNAL_ERROR_MSG.to_owned(),
            kind: ErrorKind::Internal(InternalError::S3(inner))
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        
        todo!()
        // (status, body).into_response()
    }
}
