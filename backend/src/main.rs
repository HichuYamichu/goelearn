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
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

#[cfg(debug_assertions)]
use dotenvy::dotenv;
use graphql::Mutation;
use graphql::Query;
use lazy_static::lazy_static;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{core::UserRepo, rest::user_handler};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

lazy_static! {
    static ref ADDR: String = env::var("URL").unwrap_or("0.0.0.0:3000".into());
    static ref DATABASE_URL: String = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:changeme@localhost:5432/goelearn".into());
    static ref SECRET: String = env::var("SECRET").expect("SECRET is not set");
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
    user_repo: UserRepo,
}

#[tokio::main]
pub async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", Level::TRACE)
        .with_target("tower_http::trace::on_request", Level::TRACE)
        .with_target("backend", Level::DEBUG)
        .with_default(Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let config = argon2_async::Config::default();
    argon2_async::set_config(config).await;

    let db: DatabaseConnection = Database::connect(ConnectOptions::new(DATABASE_URL.to_string()))
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    let user_repo = UserRepo::new(db.clone());
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(user_repo.clone())
        .finish();
    let state = AppState { schema, user_repo };

    let user_routes = Router::new().route("/activate/:user_id", get(user_handler::activate));
    let app = Router::new()
        .route(
            "/api/v1/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .nest("/api/v1/user", user_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    tracing::info!("Started on http://localhost:3000/api/v1/graphql");
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
