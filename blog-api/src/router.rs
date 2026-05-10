use axum::{Router, routing::post};

use crate::{AppState, handler::login::login};

// 子路由模块
pub fn login_routes() -> Router<AppState> {
    Router::new().route("/login", post(login))
}
