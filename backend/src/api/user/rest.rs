use async_graphql::dataloader::DataLoader;
use axum::extract::{Path, State};
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use crate::core::AppError;

use super::UserRepo;

pub struct UserRest;

impl UserRest {
    #[instrument(err, skip(conn))]
    pub async fn activate(
        Path(user_id): Path<Uuid>,
        State(conn): State<DatabaseConnection>,
    ) -> Result<(), AppError> {
        let data_loader = DataLoader::new(conn, tokio::spawn);
        UserRepo::activate_user(&data_loader, user_id).await?;
        Ok(())
    }
}
