use ::entity::{channel, membership};
use ::entity::{class, class::Entity as Class};
use async_graphql::dataloader::Loader;
use async_trait::async_trait;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ClassRepo {
    conn: DatabaseConnection,
}

impl ClassRepo {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn create_class(
        &self,
        model: class::ActiveModel,
    ) -> Result<class::Model, TransactionError<DbErr>> {
        let class = self
            .conn
            .transaction::<_, class::Model, DbErr>(|txn| {
                Box::pin(async move {
                    tracing::info!("Before");

                    let class = model.insert(txn).await?;
                    tracing::info!("Created class {:?}", class);
                    let main_channel = channel::ActiveModel {
                        id: Set(Uuid::new_v4()),
                        name: Set("Main".to_string()),
                        class_id: Set(class.id),
                        allow_members_to_post: Set(true),
                        ..Default::default()
                    };
                    main_channel.insert(txn).await?;
                    let member = membership::ActiveModel {
                        user_id: Set(class.owner_id),
                        class_id: Set(class.id),
                    };
                    member.insert(txn).await?;

                    Ok(class)
                })
            })
            .await?;

        Ok(class)
    }

    pub async fn find_random(
        &self,
        limit: u64,
    ) -> Result<Vec<class::Model>, TransactionError<DbErr>> {
        let classes = Class::find()
            .order_by_asc(class::Column::Id)
            .limit(Some(limit))
            .all(&self.conn)
            .await?;

        Ok(classes)
    }

    pub async fn join_user_to_class(
        &self,
        user_id: Uuid,
        class_id: Uuid,
    ) -> Result<membership::Model, DbErr> {
        let member = membership::ActiveModel {
            user_id: Set(user_id),
            class_id: Set(class_id),
        };

        let member = member.insert(&self.conn).await?;

        Ok(member)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ClassById(pub Uuid);

#[async_trait]
impl Loader<ClassById> for ClassRepo {
    type Value = class::Model;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[ClassById],
    ) -> Result<HashMap<ClassById, Self::Value>, Self::Error> {
        let classes = Class::find()
            .filter(class::Column::Id.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(&self.conn)
            .await
            .map_err(Arc::new)?;

        Ok(classes.into_iter().map(|c| (ClassById(c.id), c)).collect())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ClassByOwnerId(pub Uuid);

#[async_trait]
impl Loader<ClassByOwnerId> for ClassRepo {
    type Value = class::Model;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[ClassByOwnerId],
    ) -> Result<HashMap<ClassByOwnerId, Self::Value>, Self::Error> {
        let classes = Class::find()
            .filter(class::Column::OwnerId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(&self.conn)
            .await
            .map_err(Arc::new)?;

        Ok(classes
            .into_iter()
            .map(|c| (ClassByOwnerId(c.owner_id), c))
            .collect())
    }
}
