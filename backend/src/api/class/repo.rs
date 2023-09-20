use ::entity::{
    channel, class_blacklist, class_blacklist::Entity as ClassBlacklist, file,
    invite::Entity as Invite, membership, membership::Entity as Membership, sea_orm_active_enums,
    user::Entity as User,
};
use ::entity::{class, class::Entity as Class};
use ::entity::{invite, user};
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
    async fn find_random(
        &self,
        limit: u64,
        exceptions: Vec<Uuid>,
    ) -> Result<Vec<class::Model>, TransactionError<DbErr>>;
    async fn find_by_query(
        &self,
        query: String,
        exceptions: Vec<Uuid>,
    ) -> Result<Vec<class::Model>, DbErr>;
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

    async fn ban_member(
        &self,
        class_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), TransactionError<DbErr>>;

    async fn unban_member(
        &self,
        class_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), TransactionError<DbErr>>;

    async fn remove_memeber(
        &self,
        class_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), TransactionError<DbErr>>;

    async fn get_user_bans(&self, user_id: Uuid) -> Result<Vec<Uuid>, DbErr>;
    async fn get_class_bans(&self, class_id: Uuid) -> Result<Vec<user::Model>, DbErr>;
    async fn create_invite(&self, model: invite::ActiveModel) -> Result<invite::Model, DbErr>;
    async fn delete_invite(&self, invite_id: Uuid) -> Result<(), DbErr>;
    async fn get_invites(&self, class_id: Uuid) -> Result<Vec<invite::Model>, DbErr>;
    async fn is_valid_invite(&self, invite_id: Uuid, class_id: Uuid) -> Result<bool, DbErr>;
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

                    let submission_files = file::ActiveModel {
                        id: Set(Uuid::new_v4()),
                        name: Set("Assignment submission files".to_string()),
                        class_id: Set(class.id),
                        file_type: Set(sea_orm_active_enums::FileType::Directory),
                        public: Set(true),
                        ..Default::default()
                    };
                    submission_files.insert(txn).await?;

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
    async fn find_random(
        &self,
        limit: u64,
        exceptions: Vec<Uuid>,
    ) -> Result<Vec<class::Model>, TransactionError<DbErr>> {
        let condidion = Condition::all()
            .add(class::Column::DeletedAt.is_null())
            .add(class::Column::Public.eq(true))
            .add(class::Column::Id.is_not_in(exceptions));

        let classes = Class::find()
            .order_by_asc(class::Column::Id)
            .filter(condidion)
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
        let member = member.insert(self.loader()).await?;
        Ok(member)
    }

    #[instrument(skip(self), err(Debug))]
    async fn find_by_query(
        &self,
        query: String,
        exceptions: Vec<Uuid>,
    ) -> Result<Vec<class::Model>, DbErr> {
        // FIXME: `and id != ALL($2::uuid[])` is not working
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
                and public = true
                and id != ALL($2::uuid[])
                order by rank desc;
                "#,
                [query.into(), exceptions.into()],
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

    #[instrument(skip(self), err(Debug))]
    async fn ban_member(
        &self,
        class_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), TransactionError<DbErr>> {
        self.loader()
            .transaction::<_, (), DbErr>(|txn| {
                Box::pin(async move {
                    Membership::delete_by_id((user_id, class_id))
                        .exec(txn)
                        .await?;

                    let model = class_blacklist::ActiveModel {
                        user_id: Set(user_id),
                        class_id: Set(class_id),
                    };

                    model.insert(txn).await?;
                    Ok(())
                })
            })
            .await?;

        Ok(())
    }

    #[instrument(skip(self), err(Debug))]
    async fn unban_member(
        &self,
        class_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), TransactionError<DbErr>> {
        self.loader()
            .transaction::<_, (), DbErr>(|txn| {
                Box::pin(async move {
                    ClassBlacklist::delete_by_id((user_id, class_id))
                        .exec(txn)
                        .await?;
                    Ok(())
                })
            })
            .await?;

        Ok(())
    }

    #[instrument(skip(self), err(Debug))]
    async fn remove_memeber(
        &self,
        class_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), TransactionError<DbErr>> {
        self.loader()
            .transaction::<_, (), DbErr>(|txn| {
                Box::pin(async move {
                    Membership::delete_by_id((user_id, class_id))
                        .exec(txn)
                        .await?;
                    Ok(())
                })
            })
            .await?;

        Ok(())
    }

    async fn get_user_bans(&self, user_id: Uuid) -> Result<Vec<Uuid>, DbErr> {
        let found = ClassBlacklist::find()
            .filter(class_blacklist::Column::UserId.eq(user_id))
            .all(self.loader())
            .await?;

        Ok(found.into_iter().map(|b| b.class_id).collect())
    }

    async fn get_class_bans(&self, class_id: Uuid) -> Result<Vec<user::Model>, DbErr> {
        let found = ClassBlacklist::find()
            .filter(class_blacklist::Column::ClassId.eq(class_id))
            .find_also_related(User)
            .all(self.loader())
            .await?;

        Ok(found
            .into_iter()
            .map(|(_, u)| u.expect("relation is not optional"))
            .collect())
    }

    async fn create_invite(&self, model: invite::ActiveModel) -> Result<invite::Model, DbErr> {
        let invite = model.insert(self.loader()).await?;
        Ok(invite)
    }

    async fn delete_invite(&self, invite_id: Uuid) -> Result<(), DbErr> {
        Invite::delete_by_id(invite_id).exec(self.loader()).await?;
        Ok(())
    }

    async fn get_invites(&self, class_id: Uuid) -> Result<Vec<invite::Model>, DbErr> {
        let invites = Invite::find()
            .filter(invite::Column::ClassId.eq(class_id))
            .all(self.loader())
            .await?;
        Ok(invites)
    }

    async fn is_valid_invite(&self, invite_id: Uuid, class_id: Uuid) -> Result<bool, DbErr> {
        let invite = Invite::find_by_id(invite_id).one(self.loader()).await?;

        // TODO: invite cleanup
        if let Some(invite) = invite {
            if invite.class_id != class_id {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }

        Ok(true)
    }
}
