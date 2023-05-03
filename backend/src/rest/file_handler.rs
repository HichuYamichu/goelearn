use axum::body::HttpBody;
use axum::{
    body::Body,
    debug_handler,
    extract::{Path, State},
    http::Response,
    response::IntoResponse,
};
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use crate::core::AppError;

#[debug_handler]
pub async fn get_user_avatar(
    Path(user_id): Path<Uuid>,
    State(s3_bucker): State<s3::Bucket>,
) -> Result<impl IntoResponse, AppError> {
    let s3_path = format!("user-avatars/{}", user_id);
    let object = s3_bucker.get_object(s3_path).await;
    match object {
        Ok(object) => {
            let response = Response::builder()
                .header("Content-Type", "image/jpeg")
                .body(Body::from(object.to_vec()))
                .unwrap();

            return Ok(response);
        }
        Err(s3::error::S3Error::Http(404, _)) => {
            return Err(AppError::NotFound {
                what: "user avatar",
                with: "user id",
                why: user_id.to_string(),
            }
            .into())
        }
        Err(e) => return Err(e.into()),
    }
}
