use axum::{Json, extract::State};

use crate::{
    AppState,
    entity::site_config::SiteConfig,
    modal::response::{AppResponse, AppResult},
    utils::jwt::Claims,
};

pub async fn get_site_config(
    State(state): State<AppState>,
    _claims: Claims,
) -> AppResult<SiteConfig> {
    let site_config = state.site_config_service.get_site_config().await?;
    Ok(Json(AppResponse::<SiteConfig>::ok(
        "获取网站配置成功!",
        Some(site_config),
    )))
}
