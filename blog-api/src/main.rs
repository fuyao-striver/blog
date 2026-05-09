mod entity;
mod handler;
mod modal;
mod repo;
mod router;
mod utils;
mod service;

use crate::utils::sql::connect_postgres;
use axum::Router;
use axum::routing::get;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    // 初始化 tracing 订阅器
    // 优先使用 RUST_LOG 环境变量，如果没设置则默认使用 info 级别
    tracing_subscriber::registry()
        .with(fmt::layer().pretty())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();
    let pool = connect_postgres("postgresql://localhost:5432/blog").await;
    let state = AppState { db: pool };
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
