mod assignment;
mod channel;
mod class;
mod file;
mod message;
mod user;

use assignment::AssignmentMutation;
use async_graphql::Schema;
use channel::ChannelMutation;
use class::{ClassMutation, ClassQuery};
use file::FileMutation;
use message::MessageMutation;
use user::{UserMutation, UserQuery};

pub use class::ClassRepo;
pub use file::FileHandler;
pub use user::UserRepo;
pub use user::UserRest;

use self::assignment::AssignmentQuery;
use self::{
    class::ClassSubscription,
    message::{MessageQuery, MessageSubscription},
};

pub const MAX_FILE_SIZE: u64 = 1 * 1024 * 1024;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(ClassQuery, UserQuery, MessageQuery, AssignmentQuery);

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    UserMutation,
    ClassMutation,
    MessageMutation,
    ChannelMutation,
    FileMutation,
    AssignmentMutation,
);

#[derive(async_graphql::MergedSubscription, Default)]
pub struct Subscription(MessageSubscription, ClassSubscription);

pub type AppSchema = Schema<Query, Mutation, Subscription>;
