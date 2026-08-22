use sha2::Digest;

use crate::{
    dao::user_dao::UserDao,
    modal::request::login::LoginRequest,
    utils::{error::AppError, jwt::JwtConfig},
};

#[derive(Clone)]
pub struct UserService {
    user_dao: UserDao,
}

impl UserService {
    pub fn new(user_dao: UserDao) -> Self {
        Self { user_dao }
    }

    /// 通过用户名和密码查询用户id,成功则返回token
    pub async fn login(&self, login_request: LoginRequest) -> Result<String, AppError> {
        let login_request = LoginRequest {
            password: hex::encode(sha2::Sha256::digest(login_request.password.as_bytes())),
            ..login_request
        };
        tracing::info!("login_request:{:?}", login_request);
        // 1. 查询用户ID，把数据库错误转换为 AppError::Database
        let id = self
            .user_dao
            .get_user_id(login_request)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?; // 现在是 Result<Option<Id>, AppError>

        // 2. 将 Option 转换为 Result，不存在则返回 NotFound
        let id = id.ok_or_else(|| AppError::NotFound("用户不存在或账号密码错误!".to_string()))?;

        // 3. 生成 Token，把 JWT 错误转换为 AppError::TokenError
        let token = JwtConfig::create_token(id).map_err(|_| AppError::TokenError)?;

        Ok(token)
    }
}
