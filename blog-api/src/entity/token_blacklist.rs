use chrono::NaiveDateTime;
use sqlx::FromRow;

/// Token 黑名单，存储已登出的 JWT 的 jti
#[derive(Debug, Clone, FromRow)]
pub struct TokenBlacklist {
    pub jti: String,
    pub expires_at: NaiveDateTime,
}
