use async_graphql::{Context, Object, Result};
// use entity::user;
use sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        todo!()
    }

    async fn get_note_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<String>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        todo!()
    }
}
