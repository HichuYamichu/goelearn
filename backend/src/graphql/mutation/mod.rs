mod channel;
mod class;
mod file;
mod message;
mod user;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    user::UserMutation,
    class::ClassMutation,
    message::MessageMutation,
    channel::ChannelMutation,
    file::FileMutation,
);
