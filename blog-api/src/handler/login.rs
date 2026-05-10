use axum::{Json, extract::State};

use crate::{
    AppState, error::AppError, modal::api_result::ApiResponse, request::login::LoginRequest,
};

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<ApiResponse<String>, AppError> {
    let result = state.user_service.login(login_request).await;
    match result {
        Ok(token) => Ok(ApiResponse::success(token)),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}
