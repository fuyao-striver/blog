use axum::{Router, routing::get};

use crate::{
    AppState,
    handler::user_handler::{get_user_info, get_user_menu},
};

pub fn admin_user_router() -> Router<AppState> {
    Router::new()
        .route("/getUserInfo", get(get_user_info))
        .route("/getUserMenu", get(get_user_menu))
}
