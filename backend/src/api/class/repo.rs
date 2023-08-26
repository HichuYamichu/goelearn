use ::entity::{channel, file, membership, membership::Entity as Membership, sea_orm_active_enums};
use ::entity::{class, class::Entity as Class};
use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;
use chrono::Utc;

use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::core::{AppError, UserError};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct ClassById(Uuid);

#[async_trait]
impl Loader<ClassById> for DatabaseConnection {
    type Value = class::Model;
    type Error = Arc<DbErr>;

    #[instrument(skip(self), err(Debug))]
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

    #[instrument(skip(self), err(Debug))]
    async fn load(
        &self,
        keys: &[ClassesByOwnerId],
    ) -> Result<HashMap<ClassesByOwnerId, Self::Value>, Self::Error> {
        let condition = Condition::all()
            .add(class::Column::OwnerId.is_in(keys.iter().map(|k| k.0)))
            .add(class::Column::DeletedAt.is_null());

        let classes = Class::find()
            .filter(condition)
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

    #[instrument(skip(self), err(Debug))]
    async fn load(
        &self,
        keys: &[ClassesByUserId],
    ) -> Result<HashMap<ClassesByUserId, Self::Value>, Self::Error> {
        let condition = Condition::all()
            .add(membership::Column::UserId.is_in(keys.iter().map(|k| k.0)))
            .add(class::Column::DeletedAt.is_null());
        let classes = Membership::find()
            .filter(condition)
            .find_also_related(Class)
            .all(self)
            .await
            .map_err(Arc::new)?
            .into_iter()
            .map(|(m, c)| (m, c.expect("class should be present")))
            .collect::<Vec<_>>();

        let mut res = HashMap::<_, _>::new();
        for key in keys.iter() {
            let e = res.entry(*key).or_insert_with(Vec::new);
            let user_classes = classes
                .iter()
                .filter(|(m, _)| m.user_id == key.0)
                .map(|(_, c)| c.clone())
                .collect::<Vec<_>>();
            e.extend(user_classes);
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
    async fn delete_class(&self, class_id: Uuid) -> Result<(), DbErr>;
    async fn update_class(
        &self,
        class_id: Uuid,
        model: class::ActiveModel,
    ) -> Result<class::Model, DbErr>;

    async fn find_by_user_id(&self, user_id: Uuid)
        -> Result<Option<Vec<class::Model>>, Arc<DbErr>>;
    async fn find_by_owner_id(
        &self,
        owner_id: Uuid,
    ) -> Result<Option<Vec<class::Model>>, Arc<DbErr>>;
}

#[async_trait]
impl ClassRepo for DataLoader<DatabaseConnection> {
    #[instrument(skip(self), err(Debug))]
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

    #[instrument(skip(self), err(Debug))]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<class::Model>, DbErr> {
        let class = Class::find_by_id(id)
            .filter(class::Column::DeletedAt.is_null())
            .one(self.loader())
            .await?;
        Ok(class)
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_random(&self, limit: u64) -> Result<Vec<class::Model>, TransactionError<DbErr>> {
        let classes = Class::find()
            .order_by_asc(class::Column::Id)
            .filter(class::Column::DeletedAt.is_null())
            .limit(Some(limit))
            .all(self.loader())
            .await?;

        Ok(classes)
    }

    #[instrument(skip(self), err(Debug))]
    async fn join_user_to_class(
        &self,
        user_id: Uuid,
        class_id: Uuid,
    ) -> Result<membership::Model, DbErr> {
        let member = membership::ActiveModel {
            user_id: Set(user_id),
            class_id: Set(class_id),
        };
        // TODO: check if user is already a member
        // TODO: check if class exists
        // FIXME: User can join deleted classes
        let member = member.insert(self.loader()).await?;

        Ok(member)
    }

    #[instrument(skip(self), err(Debug))]
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
                and deleted_at is null
                order by rank desc;
                "#,
                [query.into()],
            ))
            .all(self.loader())
            .await?;

        Ok(classes)
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<Vec<class::Model>>, Arc<DbErr>> {
        let classes = self.load_one(ClassesByUserId(user_id)).await?;
        Ok(classes)
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_by_owner_id(
        &self,
        owner_id: Uuid,
    ) -> Result<Option<Vec<class::Model>>, Arc<DbErr>> {
        let classes = self.load_one(ClassesByOwnerId(owner_id)).await?;
        Ok(classes)
    }

    #[instrument(skip(self), err(Debug))]
    async fn delete_class(&self, class_id: Uuid) -> Result<(), DbErr> {
        let class = Class::find_by_id(class_id).one(self.loader()).await?;
        if let Some(class) = class {
            let mut active = class.into_active_model();
            active.deleted_at = Set(Some(Utc::now().naive_utc()));
            active.update(self.loader()).await?;
        }
        Ok(())
    }

    #[instrument(skip(self), err(Debug))]
    async fn update_class(
        &self,
        class_id: Uuid,
        model: class::ActiveModel,
    ) -> Result<class::Model, DbErr> {
        let class = Class::find_by_id(class_id).one(self.loader()).await?;
        if let Some(class) = class {
            let mut active = class.into_active_model();
            active.name = model.name;
            active.description = model.description;
            active.public = model.public;
            active.tags = model.tags;
            let updated = active.update(self.loader()).await?;
            Ok(updated)
        } else {
            Err(DbErr::RecordNotFound("class not found".into()))
        }
    }
}
