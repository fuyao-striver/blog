use std::sync::Arc;

use anyhow::anyhow;
use itertools::Itertools;
use sqlx::PgPool;

use crate::{
    modal::{request::login::LoginRequest, response::user_back::UserBackInfoResp},
    repo::{menu::MenuRepo, role::RoleRepo, user::UserRepo},
    utils::{jwt::create_token, password::verify_password},
};

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<UserRepo>,
    role_repo: Arc<RoleRepo>,
    menu_repo: Arc<MenuRepo>,
}

impl UserService {
    pub fn new(db: PgPool) -> Self {
        Self {
            user_repo: Arc::new(UserRepo::new(db.clone())),
            menu_repo: Arc::new(MenuRepo::new(db.clone())),
            role_repo: Arc::new(RoleRepo::new(db)),
        }
    }

    /// 登录方法
    pub async fn login(&self, login_request: LoginRequest) -> anyhow::Result<String> {
        let user = self
            .user_repo
            .get_user(&login_request.username)
            .await?
            .ok_or_else(|| anyhow!("用户 {} 不存在", login_request.username))?;
        anyhow::ensure!(
            verify_password(&login_request.password, &user.password)?,
            "密码不正确，请重新输入密码！"
        );
        create_token(user.id)
    }

    pub async fn get_user_back_info(&self, id: i32) -> anyhow::Result<UserBackInfoResp> {
        let user = self
            .user_repo
            .get_user_by_id(id)
            .await?
            .ok_or_else(|| anyhow!("用户{}不存在", id))?;
        let role_list = self.role_repo.get_role_list_by_id(id).await?;
        let permission_list: Vec<String> = self
            .menu_repo
            .get_permission_by_role_id(id)
            .await?
            .iter()
            .filter(|value| !value.is_empty())
            .unique()
            .cloned()
            .collect();
        Ok(UserBackInfoResp {
            id,
            avatar: user.avatar,
            role_list,
            permission_list,
        })
    }
}
