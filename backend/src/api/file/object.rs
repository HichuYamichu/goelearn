use std::fmt::Display;

use crate::core::AppError;
use async_graphql::{Enum, InputObject, Result, SimpleObject, Upload, ID};
use deadpool_redis::redis::{self, FromRedisValue, RedisResult, RedisWrite, ToRedisArgs};
use entity::sea_orm_active_enums;
use partialdebug::placeholder::PartialDebug;
use sea_orm::{Set, Unchanged};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
#[graphql(name = "File")]
pub struct FileObject {
    pub id: ID,
    pub name: String,
    pub public: bool,
    pub file_type: FileType,
    pub parent: Option<ID>,
}

impl ToRedisArgs for FileObject {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let vec = vec![
            self.id.to_string(),
            self.name.clone(),
            self.public.to_string(),
            self.file_type.to_string(),
            self.parent
                .clone()
                .map(|o| o.to_string())
                .unwrap_or("".to_string()),
        ];
        vec.write_redis_args(out)
    }
}

impl FromRedisValue for FileObject {
    fn from_redis_value(v: &redis::Value) -> RedisResult<Self> {
        let vec = Vec::<String>::from_redis_value(v)?;
        Ok(Self {
            id: ID::from(vec[0].clone()),
            name: vec[1].clone(),
            public: vec[2].parse::<bool>().unwrap(),
            file_type: vec[3].parse::<FileType>().unwrap(),
            parent: match vec[4].as_str() {
                "" => None,
                _ => Some(ID::from(vec[4].clone())),
            },
        })
    }
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum FileType {
    File,
    Directory,
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::File => write!(f, "File"),
            FileType::Directory => write!(f, "Directory"),
        }
    }
}

impl std::str::FromStr for FileType {
    type Err = ();
    fn from_str(input: &str) -> Result<FileType, Self::Err> {
        match input {
            "File" => Ok(Self::File),
            "Directory" => Ok(Self::Directory),
            _ => Err(()),
        }
    }
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

impl From<::entity::file::ActiveModel> for FileObject {
    fn from(f: ::entity::file::ActiveModel) -> Self {
        Self {
            id: ID::from(f.id.unwrap()),
            name: f.name.unwrap(),
            public: f.public.unwrap(),
            file_type: f.file_type.unwrap().into(),
            parent: f.parent_id.unwrap().map(ID::from),
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
    #[graphql(validator(min_length = 1, max_length = 35))]
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
            public: Set(true),
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
    pub class_id: ID,
    #[graphql(validator(min_length = 1, max_length = 35))]
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
