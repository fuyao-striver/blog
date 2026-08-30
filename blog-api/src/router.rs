use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    handler::{
        site_config_handler::get_site_config,
        user_handler::{get_user_info, get_user_menu, update_password},
    },
};

pub fn admin_user_router() -> Router<AppState> {
    Router::new()
        .route("/getUserInfo", get(get_user_info))
        .route("/getUserMenu", get(get_user_menu))
        .route("/password", post(update_password))
}

pub fn admin_site_config() -> Router<AppState> {
    Router::new().route("/list", get(get_site_config))
}
