use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        todo!()
    }
}
