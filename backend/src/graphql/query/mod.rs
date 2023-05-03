mod class;
mod message;
mod user;

use self::{class::ClassQuery, message::MessageQuery, user::UserQuery};
pub use message::make_messages_connection;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery, ClassQuery, MessageQuery);
