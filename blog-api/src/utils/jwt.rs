use chrono::{Duration, Local};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,   // 用户ID
    pub exp: usize, // 过期时间
    pub iat: usize, // 签发时间
}

const SECRET: &[u8] = b"651908384@qq.com";

pub fn create_token(user_id: i32) -> anyhow::Result<String> {
    let iat = Local::now();
    let exp = iat + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        exp: exp.timestamp() as usize,
        iat: iat.timestamp() as usize,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )?)
}
