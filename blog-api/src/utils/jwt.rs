use axum::{extract::FromRequestParts, http::header};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::utils::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,                     // 用户id
    pub role_list: Vec<String>,       // 用户角色
    pub permission_list: Vec<String>, // 用户权限id
    pub exp: usize,                   // 过期时间
    pub iat: usize,                   // 签发时间
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // 从header中取出 TOKEN
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::AuthError)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::AuthError)?;
        let claims = JwtConfig::verify_token(token).map_err(|_| AppError::AuthError)?;
        Ok(claims)
    }
}

pub struct JwtConfig;

impl JwtConfig {
    /// 用 用户id创建token
    pub fn create_token(
        user_id: i32,
        role_list: Vec<String>,
        permission_list: Vec<String>,
    ) -> anyhow::Result<String> {
        let now = Utc::now();
        let secret = std::env::var("TOKEN_SECRET")?;
        let expiration_time: i64 = std::env::var("TOKEN_EXPIRATION_TIME")?.parse()?;
        let claims = Claims {
            sub: user_id,
            role_list,
            permission_list,
            iat: now.timestamp() as usize,
            exp: (now + chrono::Duration::hours(expiration_time)).timestamp() as usize,
        };
        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?)
    }

    /// 验证token
    pub fn verify_token(token: &str) -> anyhow::Result<Claims> {
        let secret = std::env::var("TOKEN_SECRET")?;
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}
