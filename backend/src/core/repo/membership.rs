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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MembershipByClassId(Uuid);

#[async_trait]
impl Loader<MembershipByClassId> for MembershipRepo {
    type Value = membership::Model;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[MembershipByClassId],
    ) -> Result<HashMap<MembershipByClassId, Self::Value>, Self::Error> {
        let memberships = Membership::find()
            .filter(membership::Column::ClassId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(&self.conn)
            .await?;

        Ok(memberships
            .into_iter()
            .map(|c| (MembershipByClassId(c.user_id), c))
            .collect())
    }
}
