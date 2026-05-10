use sqlx::PgPool;

use crate::entity::user::User;

pub struct UserRepo {
    db: PgPool,
}

impl UserRepo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
    /// 根据用户名查找用户
    pub async fn get_user(&self, username: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(r"select * from t_user where username = $1")
            .bind(username)
            .fetch_optional(&self.db)
            .await?;
        Ok(user)
    }
}
