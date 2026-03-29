//! JWT工具模块
//! 
//! 提供JWT Token的生成和验证功能

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT密钥
const JWT_SECRET: &[u8] = b"your-secret-key";
/// Token过期时间（小时）
const JWT_EXPIRE_HOURS: i64 = 24;

/// JWT Claims结构
/// 
/// 包含用户ID和Token的有效期信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// 用户ID
    pub sub: i32,
    /// 过期时间戳
    pub exp: i64,
    /// 签发时间戳
    pub iat: i64,
}

/// 生成JWT Token
/// 
/// # 参数
/// - `user_id`: 用户ID
/// 
/// # 返回
/// 成功返回Token字符串，失败返回错误
pub fn generate_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    // 获取当前时间
    let now = Utc::now();
    // 计算过期时间
    let exp = now + Duration::hours(JWT_EXPIRE_HOURS);
    
    // 构建Claims
    let claims = Claims {
        sub: user_id,
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    // 编码生成Token
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

/// 验证JWT Token
/// 
/// # 参数
/// - `token`: JWT Token字符串
/// 
/// # 返回
/// 成功返回Claims，失败返回错误
pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // 解码并验证Token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
