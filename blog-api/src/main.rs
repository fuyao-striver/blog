mod entity;
mod error;
mod handler;
mod modal;
mod repo;
mod request;
mod router;
mod service;
mod utils;

use crate::router::login_routes;
use crate::service::user::UserService;
use crate::utils::sql::connect_postgres;
use axum::Router;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Clone)]
struct AppState {
    user_service: UserService,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self {
            user_service: UserService::new(db),
        }
    }
}

const DATABASE_URL: &str = "postgresql://postgres:651908384@localhost:5432/blog";

#[tokio::main]
async fn main() {
    // 初始化 tracing 订阅器
    // 优先使用 RUST_LOG 环境变量，如果没设置则默认使用 info 级别
    tracing_subscriber::registry()
        .with(fmt::layer().pretty())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();
    let pool = connect_postgres(DATABASE_URL).await;
    let state = AppState::new(pool);
    let app = Router::new()
        .merge(login_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
