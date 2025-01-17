pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230319_120845_create_relation;
mod m20230504_111801_create_tsvector;
mod m20231115_161824_create_admin;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230319_120845_create_relation::Migration),
            Box::new(m20230504_111801_create_tsvector::Migration),
            Box::new(m20231115_161824_create_admin::Migration),
        ]
    }
}
