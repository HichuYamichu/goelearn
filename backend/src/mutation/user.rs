use async_graphql::{Context, InputObject, Object, Result, SimpleObject};
use sea_orm::DatabaseConnection;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub title: String,
    pub text: String,
}

// impl CreateUserInput {
//     fn into_model_with_arbitrary_id(self) -> user::Model {
//         user::Model {
//             id: 0,
//             username: "aaa".into(),
//             email: "aaa".into(),
//             password: "aaa".into(),
//             is_active: false,
//         }
//     }
// }

#[derive(SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        todo!()
        // Ok(Mutation::create_user(conn, input.into_model_with_arbitrary_id()).await?)
    }

    pub async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        todo!()
    }
}
