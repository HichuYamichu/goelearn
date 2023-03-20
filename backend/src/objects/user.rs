use async_graphql::SimpleObject;

#[derive(Clone, Debug, SimpleObject)]
pub struct User {
    pub username: String,
    pub email: String,
    // pub user_type: UserType,
}

impl From<::entity::user::Model> for User {
    fn from(u: ::entity::user::Model) -> Self {
        Self {
            username: u.username,
            email: u.email,
            // user_type: u.user_type,
        }
    }
}
