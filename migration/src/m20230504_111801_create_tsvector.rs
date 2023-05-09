use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "alter table class
                add search tsvector
                generated always as (
                setweight(to_tsvector('simple',tags), 'A')  || ' ' ||
                setweight(to_tsvector('english',name), 'B') || ' ' ||
                setweight(to_tsvector('english',description), 'C') :: tsvector
            ) stored;
            ",
        )
        .await?;

        db.execute_unprepared("create index class_search_idx on class using gin(search);")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("drop index class_search_idx;")
            .await?;

        db.execute_unprepared(
            "alter table class
                drop column search;
            ",
        )
        .await?;

        Ok(())
    }
}
