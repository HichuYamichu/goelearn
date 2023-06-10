mod assignment;
mod channel;
mod class;
mod file;
mod message;
mod user;

use assignment::{AssignmentMutation};
use async_graphql::Schema;
use channel::{ChannelMutation};
use class::{ClassMutation, ClassQuery};
use file::{FileMutation};
use message::{MessageMutation};
use user::{UserMutation, UserQuery};

pub use file::FileHandler;
pub use user::UserRest;

use self::message::MessageSubscription;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(ClassQuery, UserQuery);

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
pub struct Subscription(MessageSubscription);

pub type AppSchema = Schema<Query, Mutation, Subscription>;
