mod core;
mod graphql;
mod objects;
mod rest;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::{FromRef, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use migration::{Migrator, MigratorTrait};
use std::env;

#[cfg(debug_assertions)]
use dotenvy::dotenv;
use graphql::Mutation;
use graphql::Query;
use lazy_static::lazy_static;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

lazy_static! {
    static ref ADDR: String = env::var("URL").unwrap_or("0.0.0.0:3000".into());
    static ref DATABASE_URL: String = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:changeme@localhost:5432/goelearn".into());
    // static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
    //     data.parse().expect("DEPTH_LIMIT is not a number")
    // ));
    // static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
    //     .map_or(None, |data| {
    //         Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
    //     });
}

#[derive(FromRef, Clone)]
struct AppState {
    schema: AppSchema,
}

#[tokio::main]
pub async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db: DatabaseConnection = Database::connect(ConnectOptions::new(DATABASE_URL.to_string()))
        .await
        .unwrap();

    Migrator::up(&db, None).await.unwrap();

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish();

    let state = AppState { schema };

    let app = Router::new()
        .route(
            "/api/v1/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(state);

    println!("Playground: http://localhost:3000/api/v1/graphql");

    axum::Server::bind(&ADDR.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(State(schema): State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/v1/graphql",
    )))
}
