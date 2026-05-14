use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::utils::jwt::Claims;

pub async fn auth_middleware(
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
    // 2. 解码并验证 token（检查有效期）
    let jwt_secret = b"651908384@qq.com"; // 建议放在配置中
    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => {
            // 额外手动检查是否过期（虽然 Validation::default() 也会检查，但显式检查更清晰）
            if data.claims.is_expired() {
                return Err(StatusCode::UNAUTHORIZED);
            }
            data
        }
        Err(err) => {
            // 根据错误类型返回不同的状态码
            match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    return Err(StatusCode::UNAUTHORIZED);
                }
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    return Err(StatusCode::UNAUTHORIZED);
                }
                _ => {
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        }
    };
    // 3. 将解析后的 Claims 注入到请求扩展中
    request.extensions_mut().insert(token_data.claims);

    // 4. 继续处理请求
    Ok(next.run(request).await)
}
