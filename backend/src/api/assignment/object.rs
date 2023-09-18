use crate::api::file::FileRepo;
use crate::api::user::{UserObject, UserRepo};
use crate::core::option_to_active_value;
use crate::{api::file::FileObject, core::AppError};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, SimpleObject, Upload, ID,
};
use chrono::{NaiveDateTime, Utc};
use deadpool_redis::redis::{self, FromRedisValue, RedisResult, RedisWrite, ToRedisArgs};
use partialdebug::placeholder::PartialDebug;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use super::AssignmentRepo;

#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
#[graphql(name = "Assignment")]
pub struct AssignmentObject {
    pub id: ID,
    pub name: String,
    pub content: String,
    pub due_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl ToRedisArgs for AssignmentObject {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let vec = vec![
            self.id.to_string(),
            self.name.clone(),
            self.content.clone(),
            self.due_at.map(|o| o.to_string()).unwrap_or("".to_string()),
            self.created_at.to_string(),
        ];
        vec.write_redis_args(out)
    }
}

impl FromRedisValue for AssignmentObject {
    fn from_redis_value(v: &redis::Value) -> RedisResult<Self> {
        let vec = Vec::<String>::from_redis_value(v)?;
        Ok(Self {
            id: ID::from(vec[0].clone()),
            name: vec[1].clone(),
            content: vec[2].clone(),
            due_at: match vec[3].as_str() {
                "" => None,
                _ => Some(
                    NaiveDateTime::parse_from_str(vec[3].as_str(), "%Y-%m-%d %H:%M:%S")
                        .expect("Date should be valid"),
                ),
            },
            created_at: NaiveDateTime::parse_from_str(vec[4].as_str(), "%Y-%m-%d %H:%M:%S")
                .expect("Date should be valid"),
        })
    }
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

    #[instrument(skip(self, ctx), err(Debug))]
    async fn submissions<'ctx>(
        &self,
        ctx: &'ctx Context<'_>,
    ) -> Result<Vec<AssignmentSubmission>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let assignment_id = Uuid::parse_str(self.id.as_str())?;
        let submissions =
            AssignmentRepo::find_submissions_by_assignment_id(data_loader, assignment_id)
                .await?
                .expect("Id should be valid");
        Ok(submissions.into_iter().map(|f| f.into()).collect())
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
    pub due_at: Option<NaiveDateTime>,
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
pub struct UpdateAssignmentInput {
    pub id: ID,
    pub name: Option<String>,
    pub content: Option<String>,
    pub due_at: Option<NaiveDateTime>,
    pub new_files: Vec<Upload>,
    pub delete_files: Vec<ID>,
}

impl UpdateAssignmentInput {
    pub fn try_into_active_model(
        self,
    ) -> Result<(::entity::assignment::ActiveModel, Vec<Upload>, Vec<Uuid>), AppError> {
        let id = Uuid::parse_str(self.id.as_str())?;
        Ok((
            ::entity::assignment::ActiveModel {
                id: Set(id),
                name: option_to_active_value(self.name),
                content: option_to_active_value(self.content),
                due_at: Set(self.due_at),
                ..Default::default()
            },
            self.new_files,
            self.delete_files
                .into_iter()
                .map(|id| Uuid::parse_str(id.as_str()))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
#[graphql(name = "AssignmentSubmission")]
pub struct AssignmentSubmission {
    pub id: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub user_id: ID,
}

#[ComplexObject]
impl AssignmentSubmission {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn user<'ctx>(&self, ctx: &'ctx Context<'_>) -> Result<UserObject, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let user_id = Uuid::parse_str(self.user_id.as_str())?;
        let user = UserRepo::find_by_id(data_loader, user_id)
            .await?
            .expect("Id should be valid");
        Ok(user.into())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn files<'ctx>(&self, ctx: &'ctx Context<'_>) -> Result<Vec<FileObject>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let submission_id = Uuid::parse_str(self.id.as_str())?;
        let files = FileRepo::find_by_assignment_submission_id(data_loader, submission_id).await?;
        Ok(files.into_iter().map(|f| f.into()).collect())
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn feedback<'ctx>(
        &self,
        ctx: &'ctx Context<'_>,
    ) -> Result<Option<AssignmentSubmissionFeedback>, AppError> {
        let data_loader = ctx.data_unchecked::<DataLoader<DatabaseConnection>>();

        let submission_id = Uuid::parse_str(self.id.as_str())?;
        let feedback =
            AssignmentRepo::find_feedback_by_assignment_submission_id(data_loader, submission_id)
                .await?;
        Ok(feedback.map(|f| f.into()))
    }
}

impl From<::entity::assignment_submission::Model> for AssignmentSubmission {
    fn from(c: ::entity::assignment_submission::Model) -> Self {
        Self {
            id: ID::from(c.id),
            created_at: c.created_at,
            updated_at: c.updated_at,
            user_id: ID::from(c.user_id),
        }
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
                updated_at: Set(None),
                user_id: Set(user_id),
            },
            self.files,
        ))
    }
}

#[derive(InputObject, Debug)]
pub struct CreateAssignmanetSubmissionFeedbackInput {
    pub id: Option<ID>,
    pub assignment_submission_id: ID,
    pub feedback: String,
}

impl CreateAssignmanetSubmissionFeedbackInput {
    pub fn try_into_active_model(
        self,
    ) -> Result<::entity::assignment_submission_feedback::ActiveModel, AppError> {
        Ok(::entity::assignment_submission_feedback::ActiveModel {
            id: match self.id {
                Some(id) => Set(Uuid::parse_str(id.as_str())?),
                None => Set(Uuid::new_v4()),
            },
            assignment_submission_id: Set(Uuid::parse_str(self.assignment_submission_id.as_str())?),
            feedback: Set(self.feedback),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(None),
        })
    }
}

#[derive(Clone, Debug, SimpleObject)]
pub struct AssignmentSubmissionFeedback {
    pub id: ID,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<::entity::assignment_submission_feedback::Model> for AssignmentSubmissionFeedback {
    fn from(c: ::entity::assignment_submission_feedback::Model) -> Self {
        Self {
            id: ID::from(c.id),
            content: c.feedback,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

#[derive(InputObject, PartialDebug)]
pub struct UpdateAssignmentSubmissionInput {
    pub id: ID,
    pub files: Vec<Upload>,
    pub delete_files: Vec<ID>,
}

impl UpdateAssignmentSubmissionInput {
    pub fn try_into_active_model(
        self,
    ) -> Result<
        (
            ::entity::assignment_submission::ActiveModel,
            Vec<Upload>,
            Vec<Uuid>,
        ),
        AppError,
    > {
        let id = Uuid::parse_str(self.id.as_str())?;
        Ok((
            ::entity::assignment_submission::ActiveModel {
                id: Set(id),
                ..Default::default()
            },
            self.files,
            self.delete_files
                .into_iter()
                .map(|id| Uuid::parse_str(id.as_str()))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
