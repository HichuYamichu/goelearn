use ::entity::{channel, file, membership, membership::Entity as Membership, sea_orm_active_enums};
use ::entity::{class, class::Entity as Class};
use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct ClassById(Uuid);

#[async_trait]
impl Loader<ClassById> for DatabaseConnection {
    type Value = class::Model;
    type Error = Arc<DbErr>;

    #[instrument(skip(self), err)]
    async fn load(
        &self,
        keys: &[ClassById],
    ) -> Result<HashMap<ClassById, Self::Value>, Self::Error> {
        let classes = Class::find()
            .filter(class::Column::Id.is_in(keys.iter().map(|k| k.0)))
            .all(self)
            .await
            .map_err(Arc::new)?;

        Ok(classes.into_iter().map(|c| (ClassById(c.id), c)).collect())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct ClassesByOwnerId(Uuid);

#[async_trait]
impl Loader<ClassesByOwnerId> for DatabaseConnection {
    type Value = Vec<class::Model>;
    type Error = Arc<DbErr>;

    #[instrument(skip(self), err)]
    async fn load(
        &self,
        keys: &[ClassesByOwnerId],
    ) -> Result<HashMap<ClassesByOwnerId, Self::Value>, Self::Error> {
        let classes = Class::find()
            .filter(class::Column::OwnerId.is_in(keys.iter().map(|k| k.0)))
            .all(self)
            .await
            .map_err(Arc::new)?;

        let mut res = HashMap::<_, _>::new();
        for key in keys.iter() {
            let e = res.entry(*key).or_insert_with(Vec::new);
            e.extend(classes.iter().filter(|c| c.owner_id == key.0).cloned());
        }

        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct ClassesByUserId(Uuid);

#[async_trait]
impl Loader<ClassesByUserId> for DatabaseConnection {
    type Value = Vec<class::Model>;
    type Error = Arc<DbErr>;

    #[instrument(skip(self), err)]
    async fn load(
        &self,
        keys: &[ClassesByUserId],
    ) -> Result<HashMap<ClassesByUserId, Self::Value>, Self::Error> {
        // TODO: untested / use join
        let memberships = Membership::find()
            .filter(membership::Column::UserId.is_in(keys.iter().map(|k| k.0)))
            .all(self)
            .await
            .map_err(Arc::new)?;

        let classes_ids = memberships
            .iter()
            .map(|m| m.class_id)
            .collect::<Vec<Uuid>>();

        let classes = Class::find()
            .filter(class::Column::Id.is_in(classes_ids.iter().copied()))
            .all(self)
            .await
            .map_err(Arc::new)?;

        let mut res = HashMap::<_, _>::new();
        for key in keys.iter() {
            let e = res.entry(*key).or_insert_with(Vec::new);
            e.extend(
                classes
                    .iter()
                    .filter(|c| memberships.iter().any(|m| m.class_id == c.id))
                    .cloned(),
            );
        }

        Ok(res)
    }
}

#[async_trait]
pub trait ClassRepo {
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
    async fn find_by_id(&self, id: Uuid) -> Result<Option<class::Model>, DbErr>;

    async fn find_by_user_id(&self, user_id: Uuid)
        -> Result<Option<Vec<class::Model>>, Arc<DbErr>>;
    async fn find_by_owner_id(
        &self,
        owner_id: Uuid,
    ) -> Result<Option<Vec<class::Model>>, Arc<DbErr>>;
}

#[async_trait]
impl ClassRepo for DataLoader<DatabaseConnection> {
    #[instrument(skip(self), err)]
    async fn create_class(
        &self,
        model: class::ActiveModel,
    ) -> Result<class::Model, TransactionError<DbErr>> {
        let class = self
            .loader()
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

    #[instrument(skip(self), err)]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<class::Model>, DbErr> {
        let class = Class::find_by_id(id).one(self.loader()).await?;
        Ok(class)
    }

    #[instrument(skip(self), err)]
    async fn find_random(&self, limit: u64) -> Result<Vec<class::Model>, TransactionError<DbErr>> {
        let classes = Class::find()
            .order_by_asc(class::Column::Id)
            .limit(Some(limit))
            .all(self.loader())
            .await?;

        Ok(classes)
    }

    #[instrument(skip(self), err)]
    async fn join_user_to_class(
        &self,
        user_id: Uuid,
        class_id: Uuid,
    ) -> Result<membership::Model, DbErr> {
        let member = membership::ActiveModel {
            user_id: Set(user_id),
            class_id: Set(class_id),
        };

        let member = member.insert(self.loader()).await?;

        Ok(member)
    }

    #[instrument(skip(self), err)]
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
            .all(self.loader())
            .await?;

        Ok(classes)
    }

    #[instrument(skip(self), err)]
    async fn find_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<Vec<class::Model>>, Arc<DbErr>> {
        let classes = self.load_one(ClassesByUserId(user_id)).await?;
        Ok(classes)
    }

    #[instrument(skip(self), err)]
    async fn find_by_owner_id(
        &self,
        owner_id: Uuid,
    ) -> Result<Option<Vec<class::Model>>, Arc<DbErr>> {
        let classes = self.load_one(ClassesByOwnerId(owner_id)).await?;
        Ok(classes)
    }
}
