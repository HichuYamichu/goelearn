use self::user::UserQuery;

mod user;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery);
