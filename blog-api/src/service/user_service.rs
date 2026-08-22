use crate::{dao::user_dao::UserDao, modal::request::login::LoginRequest, utils::error::AppError};

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
        let id = self.user_dao.get_user_id(login_request).await;
        match id {
            Ok(result) => match result {
                Some(id) => Ok("hello world".to_string()),
                None => Err(AppError::NotFound("用户不存在或账号密码错误!".to_string())),
            },
            Err(e) => Err(AppError::Database(e.to_string())),
        }
    }
}
