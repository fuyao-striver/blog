use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,   // 用户id
    pub exp: usize, // 过期时间
    pub iat: usize, // 签发时间
}

pub struct JwtConfig;

impl JwtConfig {
    /// 用 用户id创建token
    pub fn create_token(user_id: i32) -> anyhow::Result<String> {
        let now = Utc::now();
        let secret = std::env::var("TOKEN_SECRET")?;
        let expiration_time: i64 = std::env::var("TOKEN_EXPIRATION_TIME")?.parse()?;
        let claims = Claims {
            sub: user_id,
            iat: now.timestamp() as usize,
            exp: (now + chrono::Duration::hours(expiration_time)).timestamp() as usize,
        };
        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?)
    }
}
