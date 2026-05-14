use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    AppState,
    handler::{
        login::login,
        user::{get_user_back_info, get_user_menu},
    },
    utils::middle::auth_middleware,
};

// 子路由模块
pub fn login_routes() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

pub fn user_routers() -> Router<AppState> {
    Router::new()
        .route("/admin/user/getUserInfo", get(get_user_back_info))
        .route("/admin/user/getUserMenu", get(get_user_menu))
        .route_layer(middleware::from_fn(auth_middleware))
}
