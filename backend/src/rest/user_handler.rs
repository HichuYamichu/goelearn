use axum::extract::{Path, State};
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use crate::core::AppError;

#[instrument(err)]
pub async fn activate(
    Path(user_id): Path<Uuid>,
    State(conn): State<DatabaseConnection>,
) -> Result<(), AppError> {
    use crate::core::repo::user::UserRepoExt;
    conn.activate_user(user_id).await?;
    Ok(())
}
