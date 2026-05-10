use std::sync::Arc;

use anyhow::bail;
use sqlx::PgPool;

use crate::{
    repo::user::UserRepo,
    request::login::LoginRequest,
    utils::{jwt::create_token, password::verify_password},
};

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<UserRepo>,
}

impl UserService {
    pub fn new(db: PgPool) -> Self {
        Self {
            user_repo: Arc::new(UserRepo::new(db)),
        }
    }

    /// 登录方法
    pub async fn login(&self, login_request: LoginRequest) -> anyhow::Result<String> {
        // 1. 根据用户名查找用户
        let user = self.user_repo.get_user(&login_request.username).await?;
        match user {
            Some(value) => {
                // 2. 验证密码
                let verfiy = verify_password(&login_request.password, &value.password)?;
                if verfiy {
                    // 创建token
                    let token = create_token(value.id)?;
                    Ok(token)
                } else {
                    bail!("密码不正确，请重新输入密码！")
                }
            }
            None => {
                bail!("用户 {} 不存在", &login_request.username)
            }
        }
    }
}
