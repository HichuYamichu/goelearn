//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use super::sea_orm_active_enums::UserType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub has_avatar: bool,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
    pub deleted_at: Option<DateTime>,
    pub active: bool,
    pub user_type: UserType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::assignment_submission::Entity")]
    AssignmentSubmission,
    #[sea_orm(has_many = "super::class::Entity")]
    Class,
    #[sea_orm(has_many = "super::message::Entity")]
    Message,
    #[sea_orm(has_many = "super::report::Entity")]
    Report,
}

impl Related<super::assignment_submission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssignmentSubmission.def()
    }
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl Related<super::report::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Report.def()
    }
}

impl Related<super::class_blacklist::Entity> for Entity {
    fn to() -> RelationDef {
        super::class_blacklist::Relation::Class.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::class_blacklist::Relation::User.def().rev())
    }
}

impl Related<super::class::Entity> for Entity {
    fn to() -> RelationDef {
        super::membership::Relation::Class.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::membership::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
