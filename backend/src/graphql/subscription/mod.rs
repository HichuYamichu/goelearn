use self::message::MessageSubscription;

mod message;

#[derive(async_graphql::MergedSubscription, Default)]
pub struct Subscription(MessageSubscription);
