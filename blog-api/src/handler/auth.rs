//! 认证处理模块
//!
//! 处理用户登录、登出等认证相关请求

use std::sync::Arc;

use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHash};
use axum::{Json, extract::State};

use crate::{AppState, model::LoginForm, repository::user_repo, result::Result, utils::jwt};

/// 用户登录
///
/// # 参数
/// - `state`: 应用状态（包含数据库连接池）
/// - `form`: 登录表单（用户名和密码）
///
/// # 返回
/// 成功返回JWT Token，失败返回错误信息
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(form): Json<LoginForm>,
) -> Result<String> {
    let start = std::time::Instant::now();
    
    // 记录请求日志
    tracing::info!(
        path = "/api/login",
        method = "POST",
        username = %form.username,
        "用户登录请求"
    );

    // 根据用户名查询用户
    let user = match user_repo::find_by_username(&state.db, &form.username).await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("查询用户失败: {}", e);
            let response = Result::<String>::fail(500, "服务器内部错误");
            tracing::info!(
                path = "/api/login",
                method = "POST",
                status = 500,
                success = false,
                response = ?response,
                elapsed_ms = start.elapsed().as_millis(),
                "用户登录失败"
            );
            return response;
        }
    };

    // 检查用户是否存在
    let user = match user {
        Some(u) => u,
        None => {
            let response = Result::<String>::fail(400, "用户名或密码错误");
            tracing::info!(
                path = "/api/login",
                method = "POST",
                status = 400,
                success = false,
                response = ?response,
                elapsed_ms = start.elapsed().as_millis(),
                "用户登录失败：用户不存在"
            );
            return response;
        }
    };

    // 检查账号是否被禁用
    if user.is_disable == 1 {
        let response = Result::<String>::fail(403, "账号已被禁用");
        tracing::info!(
            path = "/api/login",
            method = "POST",
            status = 403,
            success = false,
            response = ?response,
            elapsed_ms = start.elapsed().as_millis(),
            "用户登录失败：账号已被禁用"
        );
        return response;
    }

    // 解析密码哈希
    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("密码哈希解析失败: {}", e);
            let response = Result::<String>::fail(500, "服务器内部错误");
            tracing::info!(
                path = "/api/login",
                method = "POST",
                status = 500,
                success = false,
                response = ?response,
                elapsed_ms = start.elapsed().as_millis(),
                "用户登录失败"
            );
            return response;
        }
    };

    // 验证密码
    if Argon2::default()
        .verify_password(form.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        let response = Result::<String>::fail(400, "用户名或密码错误");
        tracing::info!(
            path = "/api/login",
            method = "POST",
            status = 400,
            success = false,
            response = ?response,
            elapsed_ms = start.elapsed().as_millis(),
            "用户登录失败：密码错误"
        );
        return response;
    }

    // 生成JWT Token
    let token = match jwt::generate_token(user.id) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("生成Token失败: {}", e);
            let response = Result::<String>::fail(500, "服务器内部错误");
            tracing::info!(
                path = "/api/login",
                method = "POST",
                status = 500,
                success = false,
                response = ?response,
                elapsed_ms = start.elapsed().as_millis(),
                "用户登录失败"
            );
            return response;
        }
    };

    let response = Result::success_with_msg("登录成功", token.clone());
    
    // 记录响应日志
    tracing::info!(
        path = "/api/login",
        method = "POST",
        status = 200,
        success = true,
        response = ?response,
        elapsed_ms = start.elapsed().as_millis(),
        "用户登录成功"
    );
    
    response
}

/// 用户登出
///
/// # 返回
/// 成功返回退出成功消息
pub async fn logout() -> Result<()> {
    let start = std::time::Instant::now();
    
    // 记录请求日志
    tracing::info!(
        path = "/api/logout",
        method = "POST",
        "用户登出请求"
    );
    
    let response = Result::success_with_msg("退出成功", ());
    
    // 记录响应日志
    tracing::info!(
        path = "/api/logout",
        method = "POST",
        status = 200,
        success = true,
        response = ?response,
        elapsed_ms = start.elapsed().as_millis(),
        "用户登出成功"
    );
    
    response
}
