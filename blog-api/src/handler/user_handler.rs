use axum::{Json, extract::State};

use crate::{
    AppState,
    modal::{
        request::login::LoginRequest,
        response::{AppResponse, user_reponse::UserBackInfo},
    },
    utils::{error::AppError, jwt::Claims},
};

pub async fn login(
    State(state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<AppResponse<String>>, AppError> {
    tracing::info!("请求参数为:{:?}", login_request);
    let result = state.user_service.login(login_request).await?;
    Ok(Json(AppResponse::ok("登录成功!".to_string(), Some(result))))
}

pub async fn get_user_info(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<AppResponse<UserBackInfo>>, AppError> {
    let user_info = state.user_service.get_user_back_info(claims).await?;
    Ok(Json(AppResponse::<UserBackInfo>::ok(
        "获取用户信息成功".to_string(),
        Some(user_info),
    )))
}
