use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::error::DbErr;
use serde_json::json;

pub enum AppError {
    Auth,
    NotFound {
        what: &'static str,
        with: &'static str,
        why: String,
    },
    InternalError(InternalError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Auth => write!(f, "Authentication error"),
            AppError::NotFound { what, with, why } => {
                write!(f, "`{}` with `{}` = `{}` was not found", what, with, why)
            }
            AppError::InternalError(err) => write!(f, "Internal server error: {}", err),
        }
    }
}

#[derive(Debug)]
pub enum InternalError {
    DB(DbErr),
    JWT(jsonwebtoken::errors::Error),
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalError::DB(err) => write!(f, "Database error: {}", err),
            InternalError::JWT(err) => write!(f, "JWT error: {}", err),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(inner: DbErr) -> Self {
        AppError::InternalError(InternalError::DB(inner))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(inner: jsonwebtoken::errors::Error) -> Self {
        AppError::InternalError(InternalError::JWT(inner))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalError(err) => {
                tracing::error!("{}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".into(),
                )
            }
            AppError::NotFound { what, with, why } => (
                StatusCode::NOT_FOUND,
                format!("`{}` with `{}` = `{}` was not found", what, with, why),
            ),
            AppError::Auth => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
