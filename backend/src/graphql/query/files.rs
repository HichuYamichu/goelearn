use async_graphql::{dataloader::DataLoader, Context, Object, ID};
use uuid::Uuid;

use crate::{
    core::{
        repo::{
            class::{ClassById, ClassRepo},
            user::UserRepo,
        },
        AppError, Claims, LoggedInGuard,
    },
    object::{ClassObject, UserObject},
};

#[derive(Default)]
pub struct FileQuery;

#[Object]
impl FileQuery {}
