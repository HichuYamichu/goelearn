//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "assignment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: DateTime,
    pub due_at: DateTime,
    pub class_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::assignment_submission::Entity")]
    AssignmentSubmission,
    #[sea_orm(
        belongs_to = "super::class::Entity",
        from = "Column::ClassId",
        to = "super::class::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Class,
}

impl Related<super::assignment_submission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssignmentSubmission.def()
    }
}

impl Related<super::class::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Class.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}