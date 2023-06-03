mod core;
mod graphql;
mod object;
mod rest;
mod ws;

use crate::core::repo::channel::ChannelRepo;
use crate::core::repo::class::ClassRepo;
use crate::core::repo::file::FileRepo;
use crate::core::repo::message::MessageRepo;
use crate::core::repo::{membership::MembershipRepo, user::UserRepo};
use crate::core::Claims;
use crate::graphql::Subscription;
use async_graphql::extensions::Tracing;
use async_graphql::http::GraphiQLSource;
use async_graphql::{dataloader::DataLoader, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use awscreds::Credentials;
use axum::{
    extract::{FromRef, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use migration::{Migrator, MigratorTrait};
use std::env;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
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

use crate::rest::user_handler;

pub type AppSchema = Schema<Query, Mutation, Subscription>;

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
    user_repo: UserRepo,
    membership_repo: MembershipRepo,
    class_repo: ClassRepo,
    message_repo: MessageRepo,
    channel_repo: ChannelRepo,
    file_repo: FileRepo,
    s3_bucket: s3::Bucket,
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

    let redis_client = redis::Client::open(REDIS_URL.to_string()).unwrap();
    let db: DatabaseConnection = Database::connect(ConnectOptions::new(DATABASE_URL.to_string()))
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    let s3_credentials = Credentials::new(None, None, None, None, None).unwrap();
    let s3_bucket = s3::Bucket::new(
        "goelearn",
        "eu-north-1".parse().expect("Region should be valid"),
        s3_credentials,
    )
    .unwrap();

    let user_repo = UserRepo::new(db.clone());
    let membership_repo = MembershipRepo::new(db.clone());
    let class_repo = ClassRepo::new(db.clone());
    let message_repo = MessageRepo::new(db.clone());
    let channel_repo = ChannelRepo::new(db.clone());
    let file_repo = FileRepo::new(db.clone());

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
        user_repo,
        membership_repo,
        class_repo,
        message_repo,
        channel_repo,
        file_repo,
        s3_bucket,
    };

    let user_routes = Router::new().route("/activate/:user_id", get(user_handler::activate));
    let app = Router::new()
        .route(
            "/api/v1/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .route_service("/ws", ws::GraphQLSubscription::new(schema, state.clone()))
        .nest("/api/v1/user", user_routes)
        .route(
            "/files/user-avatar/:user_id",
            get(rest::file_handler::get_user_avatar),
        )
        .route(
            "/files/class-image/:class_id",
            get(rest::file_handler::get_class_image),
        )
        .route(
            "/files/class-files/:class_id/:file_id",
            get(rest::file_handler::get_class_file),
        )
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    tracing::info!("Started on http://localhost:3000/api/v1/graphql");
    axum::Server::bind(&ADDR.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(
    State(schema): State<AppSchema>,
    State(membership_repo): State<MembershipRepo>,
    State(user_repo): State<UserRepo>,
    State(class_repo): State<ClassRepo>,
    State(message_repo): State<MessageRepo>,
    State(channel_repo): State<ChannelRepo>,
    State(file_repo): State<FileRepo>,
    claims: Option<Claims>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let membership_dataloader = DataLoader::new(membership_repo, tokio::spawn);
    let user_dataloader = DataLoader::new(user_repo, tokio::spawn);
    let class_dataloader = DataLoader::new(class_repo, tokio::spawn);
    let message_dataloader = DataLoader::new(message_repo, tokio::spawn);
    let channel_dataloader = DataLoader::new(channel_repo, tokio::spawn);
    let file_dataloader = DataLoader::new(file_repo, tokio::spawn);

    schema
        .execute(
            req.into_inner()
                .data(claims)
                .data(membership_dataloader)
                .data(user_dataloader)
                .data(class_dataloader)
                .data(message_dataloader)
                .data(channel_dataloader)
                .data(file_dataloader),
        )
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
