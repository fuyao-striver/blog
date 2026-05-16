use chrono::{Duration, Local, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,    // 用户ID
    pub jti: String, // JWT ID，用于黑名单精准失效
    pub exp: usize,  // 过期时间
    pub iat: usize,  // 签发时间
}

impl Claims {
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp() as usize;
        self.exp < now
    }
}

const SECRET: &[u8] = b"651908384@qq.com";

pub fn create_token(user_id: i32) -> anyhow::Result<String> {
    let iat = Local::now();
    let exp = iat + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        jti: Uuid::new_v4().to_string(),
        exp: exp.timestamp() as usize,
        iat: iat.timestamp() as usize,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )?)
}
