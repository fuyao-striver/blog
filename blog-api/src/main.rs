mod constant;
mod entity;
mod error;
mod handler;
mod modal;
mod registry;
mod repo;
mod router;
mod service;
mod utils;

use crate::registry::AppRegistry;
use crate::router::{friend_routers, login_routes, user_routers};
use crate::service::friend::FriendService;
use crate::service::user::UserService;
use crate::utils::middle::log_middleware;
use crate::utils::sql::connect_postgres;
use axum::{Router, middleware};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Clone)]
struct AppState {
    pub registry: AppRegistry,
    pub user_service: UserService,
    pub friend_service: FriendService,
}

impl AppState {
    pub fn new(registry: &AppRegistry) -> Self {
        Self {
            registry: registry.clone(),
            user_service: UserService::new(registry),
            friend_service: FriendService::new(registry),
        }
    }
}

const DATABASE_URL: &str = "postgresql://postgres:651908384@localhost:5432/blog";

#[tokio::main]
async fn main() {
    // 初始化 tracing 订阅器
    // 默认日志级别：
    //   - 应用日志：info
    //   - SQL 查询日志：debug（sqlx 的 tracing feature 在此级别输出 SQL 语句）
    tracing_subscriber::registry()
        .with(fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=debug")),
        )
        .init();

    let pool = connect_postgres(DATABASE_URL).await;
    let registry = AppRegistry::new(&pool);
    let state = AppState::new(&registry);
    let app = Router::new()
        .merge(login_routes())
        .merge(user_routers(state.clone()))
        .merge(friend_routers(state.clone()))
        // log_middleware 在最外层，才能记录完整耗时
        .layer(middleware::from_fn(log_middleware))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("服务已启动，监听 http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
