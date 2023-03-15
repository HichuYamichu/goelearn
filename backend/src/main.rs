mod mutation;
mod query;

use std::env;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use migration::{Migrator, MigratorTrait};

#[cfg(debug_assertions)]
use dotenvy::dotenv;
use lazy_static::lazy_static;
use mutation::Mutation;
use query::Query;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

lazy_static! {
    static ref ADDR: String = env::var("URL").unwrap_or("0.0.0.0:8000".into());
    static ref DATABASE_URI: String = env::var("DATABASE_URI")
        .unwrap_or("postgres://postgres:changeme@localhost:5432/BandsDB".into());
    // static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
    //     data.parse().expect("DEPTH_LIMIT is not a number")
    // ));
    // static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
    //     .map_or(None, |data| {
    //         Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
    //     });
}

#[tokio::main]
pub async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let db: DatabaseConnection = Database::connect(ConnectOptions::new(DATABASE_URI.to_string()))
        .await
        .unwrap();

    Migrator::up(&db, None).await.unwrap();

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish();

    let app = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .layer(Extension(schema));

    println!("Playground: http://localhost:3000/api/graphql");

    axum::Server::bind(&ADDR.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}
