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
    // 根据用户名查询用户
    let user = match user_repo::find_by_username(&state.db, &form.username).await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("查询用户失败: {}", e);
            return Result::fail(500, "服务器内部错误");
        }
    };

    // 检查用户是否存在
    let user = match user {
        Some(u) => u,
        None => return Result::fail(400, "用户名或密码错误"),
    };

    // 检查账号是否被禁用
    if user.is_disable == 1 {
        return Result::fail(403, "账号已被禁用");
    }

    // 解析密码哈希
    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("密码哈希解析失败: {}", e);
            return Result::fail(500, "服务器内部错误");
        }
    };

    // 验证密码
    if Argon2::default()
        .verify_password(form.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Result::fail(400, "用户名或密码错误");
    }

    // 生成JWT Token
    let token = match jwt::generate_token(user.id) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("生成Token失败: {}", e);
            return Result::fail(500, "服务器内部错误");
        }
    };

    Result::success_with_msg("登录成功", token)
}

/// 用户登出
///
/// # 返回
/// 成功返回退出成功消息
pub async fn logout() -> Result<()> {
    Result::success_with_msg("退出成功", ())
}
