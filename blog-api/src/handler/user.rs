use axum::{Extension, Json, extract::State};

use crate::modal::request::user_req::PasswordReq;
use crate::{
    AppState,
    error::AppError,
    modal::{
        api_result::ApiResponse,
        response::{router_resp::RouterResp, user_back::UserBackInfoResp},
    },
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

pub async fn get_user_menu(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<Vec<RouterResp>>, AppError> {
    let result = state.user_service.get_user_menu(claims.sub).await;
    match result {
        Ok(value) => Ok(ApiResponse::success(value)),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

#[axum::debug_handler]
pub async fn logout(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<()>, AppError> {
    let result = state.user_service.logout(&claims.jti, claims.exp).await;
    match result {
        Ok(_) => Ok(ApiResponse::ok_message("登出成功")),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

#[axum::debug_handler]
pub async fn update_password(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(password_req): Json<PasswordReq>,
) -> Result<ApiResponse<()>, AppError> {
    let result = state
        .user_service
        .update_password(password_req.check_password.as_str(), claims.sub)
        .await;
    match result {
        Ok(_) => Ok(ApiResponse::ok_message("更新密码成功！")),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}
