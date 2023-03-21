use axum::extract::{Path, State};
use uuid::Uuid;

use crate::core::{user_repo::UserRepo, AppError};

pub async fn activate(
    Path(user_id): Path<Uuid>,
    State(user_repo): State<UserRepo>,
) -> Result<(), AppError> {
    user_repo.activate_user(user_id).await?;
    Ok(())
}
