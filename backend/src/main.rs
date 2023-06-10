mod api;
mod core;
mod ws;

use crate::api::{FileHandler, Mutation, Query, Subscription, UserRest};
use crate::core::Claims;
use api::AppSchema;
use async_graphql::extensions::Tracing;
use async_graphql::http::GraphiQLSource;
use async_graphql::{dataloader::DataLoader, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use awscreds::Credentials;
use axum::routing::post;
use axum::{
    extract::{FromRef, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use migration::{Migrator, MigratorTrait};
use std::env;
use tower_http::cors::CorsLayer;

use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

#[cfg(debug_assertions)]
use dotenvy::dotenv;
use lazy_static::lazy_static;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

lazy_static! {
    static ref ADDR: String = env::var("URL").unwrap_or("0.0.0.0:3000".into());
    static ref DATABASE_URL: String = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:changeme@localhost:5432/goelearn".into());
    static ref SECRET: String = env::var("SECRET").expect("SECRET is not set");
    static ref REDIS_URL: String = env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".into());
    static ref MAIL_USERNAME: String = env::var("MAIL_USERNAME").expect("MAIL_USERNAME is not set");
    static ref MAIL_PASSWORD: String = env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD is not set");
    static ref HOST_URL: String = env::var("HOST_URL").expect("HOST_URL is not set");
}

#[derive(FromRef, Clone)]
pub struct AppState {
    schema: AppSchema,
    conn: DatabaseConnection,
    s3_bucket: s3::Bucket,
}

#[tokio::main]
pub async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let filter = filter::Targets::new()
        // .with_target("tower_http::trace::on_response", Level::WARN)
        // .with_target("tower_http::trace::on_request", Level::WARN)
        .with_target("backend", Level::TRACE)
        // .with_target("sqlx", Level::WARN)
        // .with_target("hyper", Level::WARN)
        // .with_target("async_graphql", Level::INFO)
        .with_default(Level::WARN);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(filter)
        .init();

    let config = argon2_async::Config::default();
    argon2_async::set_config(config).await;

    let redis_client = redis::Client::open(REDIS_URL.to_string()).unwrap();
    let conn: DatabaseConnection = Database::connect(ConnectOptions::new(DATABASE_URL.to_string()))
        .await
        .unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let s3_credentials = Credentials::new(None, None, None, None, None).unwrap();
    let s3_bucket = s3::Bucket::new(
        "goelearn",
        "eu-north-1".parse().expect("Region should be valid"),
        s3_credentials,
    )
    .unwrap();

    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(redis_client)
    .data(s3_bucket.clone())
    .extension(Tracing)
    .finish();

    let state = AppState {
        schema: schema.clone(),
        conn,
        s3_bucket,
    };

    let user_routes = Router::new().route("/activate/:user_id", get(UserRest::activate));
    let file_routes = Router::new()
        .route("/user-avatar/:user_id", get(FileHandler::get_user_avatar))
        .route("/class-image/:class_id", get(FileHandler::get_class_image))
        .route(
            "/class-files/:class_id/:file_id",
            get(FileHandler::get_class_file),
        )
        .route(
            "/class-files/:class_id/zip",
            post(FileHandler::get_class_files),
        );

    let app = Router::new()
        .route(
            "/api/v1/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .route_service("/ws", ws::GraphQLSubscription::new(schema, state.clone()))
        .nest("/api/v1/user", user_routes)
        .nest("/files", file_routes)
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(tower_http::trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    tracing::info!("Started on http://localhost:3000/api/v1/graphql");
    axum::Server::bind(&ADDR.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(
    State(schema): State<AppSchema>,
    State(conn): State<DatabaseConnection>,
    claims: Option<Claims>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let conn_dataloader = DataLoader::new(conn, tokio::spawn);

    schema
        .execute(req.into_inner().data(claims).data(conn_dataloader))
        .await
        .into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/api/v1/graphql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}
