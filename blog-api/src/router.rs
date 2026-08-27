use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    handler::user_handler::{get_user_info, get_user_menu, update_password},
};

pub fn admin_user_router() -> Router<AppState> {
    Router::new()
        .route("/getUserInfo", get(get_user_info))
        .route("/getUserMenu", get(get_user_menu))
        .route("/password", post(update_password))
}
