use super::{object::AssignmentSubmission, AssignmentObject, AssignmentRepo};
use crate::core::ClassOwnerGuard;
use async_graphql::{dataloader::DataLoader, Context, Object, ID};

use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    api::user::UserObject,
    core::{AppError, Claims, ClassMemberGuard, LoggedInGuard},
};

#[derive(Default)]
pub struct AssignmentQuery;

#[Object]
impl AssignmentQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    #[graphql(guard = "LoggedInGuard")]
    async fn assignment_submissions(
        &self,
        ctx: &Context<'_>,
        assignment_id: ID,
    ) -> Result<Option<Vec<AssignmentSubmission>>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let id = Uuid::parse_str(assignment_id.as_str())?;
        let a = AssignmentRepo::find_submissions_by_assignment_id(data_loader, id).await?;
        Ok(a.map(|a| a.into_iter().map(|a| a.into()).collect::<Vec<_>>()))
    }
}
