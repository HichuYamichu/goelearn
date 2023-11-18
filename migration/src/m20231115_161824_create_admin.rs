use std::env;

use entity::sea_orm_active_enums::UserType;
use entity::user::{self, Entity as User};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ColumnTrait;
use sea_orm_migration::sea_orm::QueryFilter;
use sea_orm_migration::sea_orm::{EntityTrait, Set};
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let config = argon2_async::Config::default();
        argon2_async::set_config(config).await;

        let mut admins = Vec::new();
        for (key, value) in env::vars() {
            let has_prefix = key.starts_with("ADMIN_ACCOUNT_");
            if has_prefix {
                let values: Vec<_> = value.split(";").collect();

                let u = user::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    username: Set(values[0].to_owned()),
                    first_name: Set(values[1].to_owned()),
                    last_name: Set(values[2].to_owned()),
                    has_avatar: Set(false),
                    email: Set(values[3].to_owned()),
                    password: Set(values[4].to_owned()),
                    created_at: Set(chrono::offset::Utc::now().naive_utc()),
                    deleted_at: Set(None),
                    active: Set(true),
                    user_type: Set(UserType::Admin),
                    ..Default::default()
                };
                admins.push(u);
            }
        }

        for admin in &mut admins {
            admin.password = Set(argon2_async::hash(admin.password.clone().unwrap())
                .await
                .expect("hashing failed"));
        }

        let db = manager.get_connection();
        User::insert_many(admins).exec(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        User::delete_many()
            .filter(user::Column::UserType.eq(UserType::Admin))
            .exec(db)
            .await?;

        Ok(())
    }
}
