use crate::core::AppError;
use async_graphql::{Enum, InputObject, Result, SimpleObject, Upload, ID};
use entity::sea_orm_active_enums;
use partialdebug::placeholder::PartialDebug;
use sea_orm::{Set, Unchanged};
use uuid::Uuid;

#[derive(Clone, Debug, SimpleObject)]
#[graphql(name = "File")]
pub struct FileObject {
    pub id: ID,
    pub name: String,
    pub public: bool,
    pub file_type: FileType,
    pub parent: Option<ID>,
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq)]
pub enum FileType {
    File,
    Directory,
}

impl From<sea_orm_active_enums::FileType> for FileType {
    fn from(e: sea_orm_active_enums::FileType) -> Self {
        match e {
            sea_orm_active_enums::FileType::File => Self::File,
            sea_orm_active_enums::FileType::Directory => Self::Directory,
        }
    }
}

impl From<::entity::file::Model> for FileObject {
    fn from(f: ::entity::file::Model) -> Self {
        Self {
            id: ID::from(f.id),
            name: f.name,
            public: f.public,
            file_type: f.file_type.into(),
            parent: f.parent_id.map(ID::from),
        }
    }
}

#[derive(InputObject, PartialDebug)]
pub struct UploadFileInput {
    pub public: bool,
    pub parent_id: Option<ID>,
    pub class_id: ID,
    pub files: Vec<Upload>,
}

impl UploadFileInput {
    pub fn try_into_active_model(
        self,
        name: String,
    ) -> Result<::entity::file::ActiveModel, AppError> {
        let id = match self.parent_id {
            Some(id) => Some(Uuid::parse_str(id.as_str())?),
            None => None,
        };

        Ok(::entity::file::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name),
            public: Set(self.public),
            file_type: Set(sea_orm_active_enums::FileType::File),
            parent_id: Set(id),
            class_id: Set(Uuid::parse_str(self.class_id.as_str())?),
            message_id: Set(None),
        })
    }
}

#[derive(InputObject, Debug)]
pub struct CreateDirectoryInput {
    pub name: String,
    pub parent_id: Option<ID>,
    pub class_id: ID,
}

impl CreateDirectoryInput {
    pub fn try_into_active_model(self) -> Result<::entity::file::ActiveModel, AppError> {
        let id = match self.parent_id {
            Some(id) => Some(Uuid::parse_str(id.as_str())?),
            None => None,
        };

        Ok(::entity::file::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(self.name),
            public: Set(false),
            file_type: Set(sea_orm_active_enums::FileType::Directory),
            parent_id: Set(id),
            class_id: Set(Uuid::parse_str(self.class_id.as_str())?),
            message_id: Set(None),
        })
    }
}

#[derive(InputObject, Debug)]
pub struct UpdateFileInput {
    pub id: ID,
    pub name: Option<String>,
    pub public: Option<bool>,
}

impl UpdateFileInput {
    pub fn try_into_active_model(self) -> Result<::entity::file::ActiveModel, AppError> {
        let id = Uuid::parse_str(self.id.as_str())?;
        Ok(::entity::file::ActiveModel {
            id: Set(id),
            name: match self.name {
                Some(name) => Set(name),
                None => Unchanged("".to_string()),
            },
            public: match self.public {
                Some(public) => Set(public),
                None => Unchanged(false),
            },
            file_type: Unchanged(sea_orm_active_enums::FileType::File),
            parent_id: Unchanged(None),
            class_id: Unchanged(Uuid::new_v4()),
            message_id: Unchanged(None),
        })
    }
}
