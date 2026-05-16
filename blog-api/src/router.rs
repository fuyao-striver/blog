use axum::{
    Router, middleware,
    routing::{get, post, put},
};

use crate::handler::friend::get_friends;
use crate::{
    AppState,
    handler::{
        login::login,
        user::{get_user_back_info, get_user_menu, logout, update_password},
    },
    utils::middle::auth_middleware,
};
// 子路由模块

/// 公开路由（无需鉴权）
pub fn login_routes() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

/// 需鉴权的路由
pub fn user_routers(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/admin/user/getUserInfo", get(get_user_back_info))
        .route("/admin/user/getUserMenu", get(get_user_menu))
        .route("/admin/password", put(update_password))
        .route("/logout", get(logout))
        // 使用 from_fn_with_state 使中间件能访问 AppState（黑名单检查）
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

pub fn friend_routers(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/admin/friend/list", get(get_friends))
        // 使用 from_fn_with_state 使中间件能访问 AppState（黑名单检查）
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}
