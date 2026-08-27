use axum::{Json, extract::State};

use crate::{
    AppState,
    modal::{
        request::{login::LoginRequest, user::UpdatePassword},
        response::{
            AppResponse, AppResult,
            user_reponse::{RouterResp, UserBackInfo},
        },
    },
    utils::{error::AppError, jwt::Claims},
};

pub async fn login(
    State(state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<AppResponse<String>>, AppError> {
    tracing::info!("请求参数为:{:?}", login_request);
    let result = state.user_service.login(login_request).await?;
    Ok(Json(AppResponse::ok("登录成功!", Some(result))))
}

pub async fn get_user_info(
    State(state): State<AppState>,
    claims: Claims,
) -> AppResult<UserBackInfo> {
    let user_info = state.user_service.get_user_back_info(claims).await?;
    Ok(Json(AppResponse::<UserBackInfo>::ok(
        "获取用户信息成功",
        Some(user_info),
    )))
}

// 获取登录用户的菜单
pub async fn get_user_menu(
    claims: Claims,
    State(state): State<AppState>,
) -> AppResult<Vec<RouterResp>> {
    let route = state.user_service.get_user_menu(claims.sub).await?;
    Ok(Json(AppResponse::<Vec<RouterResp>>::ok(
        "获取登录用户的菜单成功!",
        Some(route),
    )))
}

// 更换用户密码
pub async fn update_password(
    claims: Claims,
    State(state): State<AppState>,
    Json(passowrd): Json<UpdatePassword>,
) -> AppResult<()> {
    let _ = state
        .user_service
        .update_password(passowrd, claims.sub)
        .await?;
    Ok(Json(AppResponse::<()>::ok_msg("更新密码成功!")))
}
