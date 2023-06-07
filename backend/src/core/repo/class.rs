use ::entity::{channel, file, membership, sea_orm_active_enums};
use ::entity::{class, class::Entity as Class};
use async_graphql::dataloader::Loader;
use async_trait::async_trait;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ClassById(pub Uuid);

#[async_trait]
impl Loader<ClassById> for DatabaseConnection {
    type Value = class::Model;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[ClassById],
    ) -> Result<HashMap<ClassById, Self::Value>, Self::Error> {
        let classes = Class::find()
            .filter(class::Column::Id.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(self)
            .await
            .map_err(Arc::new)?;

        Ok(classes.into_iter().map(|c| (ClassById(c.id), c)).collect())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ClassByOwnerId(pub Uuid);

#[async_trait]
impl Loader<ClassByOwnerId> for DatabaseConnection {
    type Value = class::Model;
    type Error = Arc<DbErr>;

    async fn load(
        &self,
        keys: &[ClassByOwnerId],
    ) -> Result<HashMap<ClassByOwnerId, Self::Value>, Self::Error> {
        let classes = Class::find()
            .filter(class::Column::OwnerId.is_in(keys.iter().map(|k| k.0).into_iter()))
            .all(self)
            .await
            .map_err(Arc::new)?;

        Ok(classes
            .into_iter()
            .map(|c| (ClassByOwnerId(c.owner_id), c))
            .collect())
    }
}

#[async_trait]
pub trait ClassRepoExt {
    async fn join_user_to_class(
        &self,
        user_id: Uuid,
        class_id: Uuid,
    ) -> Result<membership::Model, DbErr>;
    async fn find_random(&self, limit: u64) -> Result<Vec<class::Model>, TransactionError<DbErr>>;
    async fn find_by_query(&self, query: String) -> Result<Vec<class::Model>, DbErr>;
    async fn create_class(
        &self,
        model: class::ActiveModel,
    ) -> Result<class::Model, TransactionError<DbErr>>;
}

#[async_trait]
impl ClassRepoExt for DatabaseConnection {
    async fn create_class(
        &self,
        model: class::ActiveModel,
    ) -> Result<class::Model, TransactionError<DbErr>> {
        let class = self
            .transaction::<_, class::Model, DbErr>(|txn| {
                Box::pin(async move {
                    let class = model.insert(txn).await?;
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

                    let assignment_files = file::ActiveModel {
                        id: Set(Uuid::new_v4()),
                        name: Set("Assignment files".to_string()),
                        class_id: Set(class.id),
                        file_type: Set(sea_orm_active_enums::FileType::Directory),
                        public: Set(true),
                        ..Default::default()
                    };
                    assignment_files.insert(txn).await?;

                    let chat_files = file::ActiveModel {
                        id: Set(Uuid::new_v4()),
                        name: Set("Chat files".to_string()),
                        class_id: Set(class.id),
                        file_type: Set(sea_orm_active_enums::FileType::Directory),
                        public: Set(true),
                        ..Default::default()
                    };
                    chat_files.insert(txn).await?;

                    Ok(class)
                })
            })
            .await?;

        Ok(class)
    }

    async fn find_random(&self, limit: u64) -> Result<Vec<class::Model>, TransactionError<DbErr>> {
        let classes = Class::find()
            .order_by_asc(class::Column::Id)
            .limit(Some(limit))
            .all(self)
            .await?;

        Ok(classes)
    }

    async fn join_user_to_class(
        &self,
        user_id: Uuid,
        class_id: Uuid,
    ) -> Result<membership::Model, DbErr> {
        let member = membership::ActiveModel {
            user_id: Set(user_id),
            class_id: Set(class_id),
        };

        let member = member.insert(self).await?;

        Ok(member)
    }

    async fn find_by_query(&self, query: String) -> Result<Vec<class::Model>, DbErr> {
        let classes = Class::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
                select *,
                    ts_rank(search, websearch_to_tsquery('english', $1)) + 
                    ts_rank(search, websearch_to_tsquery('simple', $1)) as rank
                from "class"
                where search @@ websearch_to_tsquery('english', $1)
                or search @@ websearch_to_tsquery('simple', $1)
                order by rank desc;
                "#,
                [query.into()],
            ))
            .all(self)
            .await?;

        Ok(classes)
    }
}
