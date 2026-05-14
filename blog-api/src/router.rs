use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    AppState,
    handler::{login::login, user::get_user_back_info},
    utils::middle::auth_middleware,
};

// 子路由模块
pub fn login_routes() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

pub fn user_routers() -> Router<AppState> {
    Router::new()
        .route("/admin/user/getUserInfo", get(get_user_back_info))
        .route_layer(middleware::from_fn(auth_middleware))
}
