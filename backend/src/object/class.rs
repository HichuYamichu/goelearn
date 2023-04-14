use async_graphql::{SimpleObject, ID};
use uuid::Uuid;

#[derive(Clone, Debug, SimpleObject)]
pub struct ClassObject {
    pub id: ID,
    pub name: String,
}

impl From<::entity::class::Model> for ClassObject {
    fn from(c: ::entity::class::Model) -> Self {
        Self {
            id: ID::from(c.id),
            name: c.name,
        }
    }
}
