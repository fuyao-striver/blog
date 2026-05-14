use axum::{Extension, extract::State};

use crate::{
    AppState,
    error::AppError,
    modal::{api_result::ApiResponse, response::user_back::UserBackInfoResp},
    utils::jwt::Claims,
};

pub async fn get_user_back_info(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<UserBackInfoResp>, AppError> {
    let result = state.user_service.get_user_back_info(claims.sub).await;
    match result {
        Ok(value) => Ok(ApiResponse::success(value)),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}
