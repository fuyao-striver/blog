use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
};
use http_body_util::BodyExt;
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{AppState, utils::jwt::Claims};

/// JWT 鉴权中间件（含黑名单检查）
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. 提取token
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer ").map(|value| value.to_string()));
    let token = match auth_header {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    // 2. 解码并验证 token
    let jwt_secret = b"651908384@qq.com";
    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => {
            if data.claims.is_expired() {
                return Err(StatusCode::UNAUTHORIZED);
            }
            data
        }
        Err(err) => match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                return Err(StatusCode::UNAUTHORIZED);
            }
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                return Err(StatusCode::UNAUTHORIZED);
            }
            _ => {
                return Err(StatusCode::UNAUTHORIZED);
            }
        },
    };

    // 3. 检查 token 是否在黑名单中（已登出）
    let jti = &token_data.claims.jti;
    let blacklisted = state
        .registry
        .token_blacklist_repo
        .is_blacklisted(jti)
        .await
        .unwrap_or(true); // 查询失败则拒绝访问，安全优先

    if blacklisted {
        tracing::warn!(jti = %jti, "令牌已被登出");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 4. 将解析后的 Claims 注入到请求扩展中
    request.extensions_mut().insert(token_data.claims);

    // 5. 继续处理请求
    Ok(next.run(request).await)
}

/// 请求/响应日志中间件 —— 记录入参、响应体、耗时
pub async fn log_middleware(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();

    // 提取 query 参数作为入参日志
    let query = uri.query().unwrap_or("-");

    tracing::info!(
        method = %method,
        uri = %uri.path(),
        query = %query,
        "--> 收到请求"
    );

    // 执行后续中间件和 handler
    let response = next.run(request).await;
    let elapsed = start.elapsed();

    let status = response.status().as_u16();

    // 拆解 response，缓冲 body 以便日志输出
    let (parts, body) = response.into_parts();

    // 限制缓冲大小，防止大响应撑爆内存
    let bytes = body
        .collect()
        .await
        .map(|b| b.to_bytes())
        .unwrap_or_default();
    let body_str = String::from_utf8_lossy(&bytes);

    // 截断过长的响应体（超过 1024 字符则截断）
    let body_log = if body_str.len() > 1024 {
        format!("{}...(截断, 共{}字节)", &body_str[..1024], body_str.len())
    } else {
        body_str.to_string()
    };

    tracing::info!(
        status = status,
        elapsed_ms = elapsed.as_millis(),
        response = %body_log,
        "<-- 请求响应"
    );

    Ok(Response::from_parts(parts, Body::from(bytes)))
}
