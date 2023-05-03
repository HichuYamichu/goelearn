use ::entity::{membership, membership::Entity as Membership};
use async_graphql::dataloader::Loader;

use async_trait::async_trait;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MembershipRepo {
    conn: DatabaseConnection,
}

impl MembershipRepo {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MembershipByUserId(pub Uuid);

#[async_trait]
impl Loader<MembershipByUserId> for MembershipRepo {
    type Value = membership::Model;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[MembershipByUserId],
    ) -> Result<HashMap<MembershipByUserId, Self::Value>, Self::Error> {
        let memberships = Membership::find()
            .filter(membership::Column::UserId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(&self.conn)
            .await
            .map_err(Arc::new)?;

        Ok(memberships
            .into_iter()
            .map(|c| (MembershipByUserId(c.user_id), c))
            .collect())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MembershipsByClassId(pub Uuid);

#[async_trait]
impl Loader<MembershipsByClassId> for MembershipRepo {
    type Value = Vec<membership::Model>;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[MembershipsByClassId],
    ) -> Result<HashMap<MembershipsByClassId, Self::Value>, Self::Error> {
        let memberships = Membership::find()
            .filter(membership::Column::ClassId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(&self.conn)
            .await
            .map_err(Arc::new)?;

        let mut res = HashMap::<_, _>::new();
        for c in memberships {
            res.entry(*keys.iter().find(|k| k.0 == c.class_id).unwrap())
                .or_insert_with(Vec::new)
                .push(c);
        }
        Ok(res)
    }
}
