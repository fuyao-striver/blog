use sqlx::MySqlPool;

use crate::modal::request::login::LoginRequest;

#[derive(Clone)]
pub struct UserDao {
    db: MySqlPool,
}

impl UserDao {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }

    /// 通过用户名和密码查询用户id
    pub async fn get_user_id(&self, login_request: LoginRequest) -> anyhow::Result<Option<i32>> {
        let id: Option<i32> = sqlx::query_scalar!(
            r#"select id from t_user where username = ? and password = ?"#,
            login_request.username,
            login_request.password
        )
        .fetch_optional(&self.db)
        .await?;
        Ok(id)
    }
}
