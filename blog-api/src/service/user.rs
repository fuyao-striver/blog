use std::sync::Arc;

use anyhow::anyhow;
use futures::future::join_all;
use itertools::Itertools;

use crate::{
    modal::{request::login::LoginRequest, response::user_back::UserBackInfoResp},
    registry::AppRegistry,
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
    pub fn new(registry: &AppRegistry) -> Self {
        Self {
            user_repo: Arc::clone(&registry.user_repo),
            role_repo: Arc::clone(&registry.role_repo),
            menu_repo: Arc::clone(&registry.menu_repo),
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

        // 并发查询所有角色的权限
        let futures: Vec<_> = role_list
            .iter()
            .map(|role_id| self.menu_repo.get_permission_by_role_id(role_id))
            .collect();

        let results = join_all(futures).await;

        // 合并所有权限，去重，过滤空字符串
        let permission_list: Vec<String> = results
            .into_iter()
            .filter_map(|r| r.ok()) // 处理 Result，忽略错误的
            .flatten() // 展开 Vec<Vec<String>> -> Vec<String>
            .filter(|s| !s.is_empty())
            .unique()
            .collect();

        Ok(UserBackInfoResp {
            id,
            avatar: user.avatar,
            role_list,
            permission_list,
        })
    }
}
