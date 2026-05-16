use chrono::Utc;
use sqlx::PgPool;

pub struct TokenBlacklistRepo {
    db: PgPool,
}

impl TokenBlacklistRepo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    /// 将 token 的 jti 加入黑名单，有效期对齐 token 本身的过期时间
    pub async fn add(&self, jti: &str, expires_at: i64) -> anyhow::Result<()> {
        // 先清理已过期的黑名单记录（机会性清理）
        let now = Utc::now().naive_utc();
        sqlx::query("DELETE FROM t_token_blacklist WHERE expires_at < $1")
            .bind(now)
            .execute(&self.db)
            .await?;

        // 插入黑名单
        let expires_at_dt =
            chrono::DateTime::from_timestamp(expires_at, 0).map(|dt| dt.naive_utc());
        if let Some(exp) = expires_at_dt {
            sqlx::query("INSERT INTO t_token_blacklist (jti, expires_at) VALUES ($1, $2) ON CONFLICT (jti) DO NOTHING")
                .bind(jti)
                .bind(exp)
                .execute(&self.db)
                .await?;
        }

        Ok(())
    }

    /// 检查 jti 是否在黑名单中
    pub async fn is_blacklisted(&self, jti: &str) -> anyhow::Result<bool> {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM t_token_blacklist WHERE jti = $1)")
                .bind(jti)
                .fetch_one(&self.db)
                .await?;
        Ok(exists)
    }
}
