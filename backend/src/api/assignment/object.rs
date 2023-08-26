use crate::api::file::FileRepo;
use crate::{api::file::FileObject, core::AppError};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject, Upload, ID,
};
use chrono::{NaiveDateTime, Utc};
use partialdebug::placeholder::PartialDebug;
use sea_orm::{DatabaseConnection, Set};
use tracing::instrument;
use uuid::Uuid;

#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
#[graphql(name = "Assignment")]
pub struct AssignmentObject {
    pub id: ID,
    pub name: String,
    pub content: String,
    pub due_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[ComplexObject]
impl AssignmentObject {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn files<'ctx>(&self, ctx: &'ctx Context<'_>) -> Result<Vec<FileObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let assignment_id = Uuid::parse_str(self.id.as_str())?;
        let files = FileRepo::find_by_assignment_id(data_loader, assignment_id)
            .await?
            .expect("Id should be valid");
        Ok(files.into_iter().map(|f| f.into()).collect())
    }
}

impl From<::entity::assignment::Model> for AssignmentObject {
    fn from(c: ::entity::assignment::Model) -> Self {
        Self {
            id: ID::from(c.id),
            name: c.name,
            content: c.content,
            due_at: c.due_at,
            created_at: c.created_at,
        }
    }
}

#[derive(InputObject, PartialDebug)]
pub struct CreateAssignmentInput {
    pub name: String,
    pub content: String,
    pub due_at: NaiveDateTime,
    pub class_id: ID,
    pub files: Vec<Upload>,
}

impl CreateAssignmentInput {
    pub fn try_into_active_model(
        self,
    ) -> Result<(::entity::assignment::ActiveModel, Vec<Upload>), AppError> {
        let class_id = Uuid::parse_str(self.class_id.as_str())?;
        Ok((
            ::entity::assignment::ActiveModel {
                id: Set(Uuid::new_v4()),
                name: Set(self.name),
                content: Set(self.content),
                created_at: Set(Utc::now().naive_utc()),
                due_at: Set(self.due_at),
                class_id: Set(class_id),
            },
            self.files,
        ))
    }
}

#[derive(InputObject, PartialDebug)]
pub struct SubmitAssignmentInput {
    pub assignment_id: ID,
    pub files: Vec<Upload>,
}

impl SubmitAssignmentInput {
    pub fn try_into_active_model(
        self,
        user_id: Uuid,
    ) -> Result<(::entity::assignment_submission::ActiveModel, Vec<Upload>), AppError> {
        let assignment_id = Uuid::parse_str(self.assignment_id.as_str())?;
        Ok((
            ::entity::assignment_submission::ActiveModel {
                id: Set(Uuid::new_v4()),
                assignment_id: Set(assignment_id),
                created_at: Set(Utc::now().naive_utc()),
                user_id: Set(user_id),
            },
            self.files,
        ))
    }
}
