use axum::{Json, extract::State};

use crate::{
    AppState,
    modal::{request::login::LoginRequest, response::AppResponse},
    utils::error::AppError,
};

pub async fn login(
    State(state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<AppResponse<String>>, AppError> {
    tracing::info!("请求参数为:{:?}", login_request);
    let result = state.user_service.login(login_request).await?;
    Ok(Json(AppResponse::ok("登录成功!".to_string(), Some(result))))
}
