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

    // 通过用户id获取用户头像
    pub async fn get_avatar(&self, user_id: i32) -> anyhow::Result<String> {
        let avatar = sqlx::query_scalar!(r#"select avatar from t_user where id = ?"#, user_id)
            .fetch_one(&self.db)
            .await?;
        Ok(avatar)
    }
}
