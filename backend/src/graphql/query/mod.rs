mod guild;
mod message;
mod user;

use self::{guild::GuildQuery, message::MessageQuery, user::UserQuery};
pub use message::make_messages_connection;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery, GuildQuery, MessageQuery);
