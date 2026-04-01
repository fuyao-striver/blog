//! 认证提取器模块
//!
//! 提供从请求头中提取和验证JWT Token的功能

use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use crate::utils::jwt;

/// 认证用户信息
///
/// 从请求头的Authorization中提取并验证JWT Token，获取用户ID
#[derive(Debug, Clone)]
pub struct AuthUser {
    /// 用户ID
    pub user_id: i32,
}

/// 认证错误响应
#[derive(Serialize)]
pub struct AuthError {
    code: i32,
    msg: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let body = Json(self);
        (axum::http::StatusCode::UNAUTHORIZED, body).into_response()
    }
}

/// Token前缀
const BEARER_PREFIX: &str = "Bearer ";

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    fn from_request_parts(parts: &mut Parts, _state: &S) -> impl std::future::Future<Output = std::result::Result<Self, Self::Rejection>> + Send {
        // 获取Authorization头
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok());

        async move {
            // 检查是否存在Authorization头
            let auth_header = match auth_header {
                Some(header) => header,
                None => return Err(AuthError { code: 401, msg: "未提供认证Token".to_string() }),
            };

            // 检查是否为Bearer Token格式
            if !auth_header.starts_with(BEARER_PREFIX) {
                return Err(AuthError { code: 401, msg: "Token格式错误".to_string() });
            }

            // 提取Token字符串
            let token = &auth_header[BEARER_PREFIX.len()..];

            // 验证Token并获取Claims
            let claims = match jwt::verify_token(token) {
                Ok(claims) => claims,
                Err(e) => {
                    tracing::warn!("Token验证失败: {}", e);
                    return Err(AuthError { code: 401, msg: "Token无效或已过期".to_string() });
                }
            };

            Ok(AuthUser {
                user_id: claims.sub,
            })
        }
    }
}
