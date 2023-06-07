use ::entity::{membership, membership::Entity as Membership};
use async_graphql::dataloader::Loader;

use async_trait::async_trait;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MembershipsByUserId(pub Uuid);

#[async_trait]
impl Loader<MembershipsByUserId> for DatabaseConnection {
    type Value = Vec<membership::Model>;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[MembershipsByUserId],
    ) -> Result<HashMap<MembershipsByUserId, Self::Value>, Self::Error> {
        let memberships = Membership::find()
            .filter(membership::Column::UserId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(self)
            .await
            .map_err(Arc::new)?;

        let mut res = HashMap::<_, _>::new();
        for key in keys.iter() {
            let e = res.entry(*key).or_insert_with(Vec::new);
            e.extend(memberships.iter().filter(|m| m.user_id == key.0).cloned());
        }

        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MembershipsByClassId(pub Uuid);

#[async_trait]
impl Loader<MembershipsByClassId> for DatabaseConnection {
    type Value = Vec<membership::Model>;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[MembershipsByClassId],
    ) -> Result<HashMap<MembershipsByClassId, Self::Value>, Self::Error> {
        let memberships = Membership::find()
            .filter(membership::Column::ClassId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(self)
            .await
            .map_err(Arc::new)?;

        let mut res = HashMap::<_, _>::new();
        for key in keys.iter() {
            let e = res.entry(*key).or_insert_with(Vec::new);
            e.extend(memberships.iter().filter(|m| m.class_id == key.0).cloned());
        }

        Ok(res)
    }
}
