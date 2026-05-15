use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use futures::future::join_all;
use itertools::Itertools;

use crate::{
    constant::common_constant,
    modal::{
        request::login::LoginRequest,
        response::{
            router_resp::{MetaResp, RouterResp},
            user_back::{UserBackInfoResp, UserMenuResp},
        },
    },
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
        let t0 = std::time::Instant::now();

        // 1. 数据库查询用户
        let user = self
            .user_repo
            .get_user(&login_request.username)
            .await?
            .ok_or_else(|| anyhow!("用户 {} 不存在", login_request.username))?;
        tracing::debug!(
            username = %login_request.username,
            db_us = t0.elapsed().as_micros(),
            "DB 查询用户完成"
        );

        // 2. 密码验证（CPU 密集型，放入 blocking 线程池避免阻塞 async runtime）
        let t1 = std::time::Instant::now();
        let password_input = login_request.password.clone();
        let password_hash = user.password.clone();
        let valid =
            tokio::task::spawn_blocking(move || verify_password(&password_input, &password_hash))
                .await
                .map_err(|e| anyhow!("密码验证任务异常: {}", e))??;
        tracing::debug!(argon2_us = t1.elapsed().as_micros(), "Argon2 密码验证完成");
        anyhow::ensure!(valid, "密码不正确，请重新输入密码！");

        // 3. 生成 JWT Token
        let token = create_token(user.id)?;
        tracing::debug!(total_us = t0.elapsed().as_micros(), "登录完成");
        Ok(token)
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
            .filter_map(|r| r.ok())
            .flatten()
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

    pub async fn get_user_menu(&self, id: i32) -> anyhow::Result<Vec<RouterResp>> {
        let menu_list = self.menu_repo.get_menu_by_user_id(id).await?;

        // 按 parent_id 分组，O(n) 构建索引，后续递归 O(1) 查找子节点
        let menu_map: HashMap<i32, Vec<&UserMenuResp>> =
            menu_list.iter().fold(HashMap::new(), |mut acc, menu| {
                acc.entry(menu.parent_id).or_default().push(menu);
                acc
            });

        Ok(self.build_routes(common_constant::PARENT_ID, &menu_map))
    }

    // ─── 路由构建（非递归，基于 HashMap 索引） ───

    fn build_routes(
        &self,
        parent_id: i32,
        menu_map: &HashMap<i32, Vec<&UserMenuResp>>,
    ) -> Vec<RouterResp> {
        let Some(children) = menu_map.get(&parent_id) else {
            return Vec::new();
        };

        let mut list = Vec::with_capacity(children.len());

        for menu in children {
            let mut route_vo = RouterResp {
                name: menu.menu_name.clone(),
                path: self.get_router_path(menu),
                component: self.get_component(menu),
                meta: Some(MetaResp {
                    title: menu.menu_name.clone(),
                    icon: menu.icon.clone(),
                    hidden: menu.is_hidden,
                }),
                children: None,
                always_show: None,
                redirect: None,
            };

            if menu.menu_type == common_constant::MENU_TYPE_DIR {
                let sub_routes = self.build_routes(menu.id, menu_map);
                if !sub_routes.is_empty() {
                    route_vo.always_show = Some(true);
                    route_vo.redirect = Some("noRedirect".to_string());
                }
                route_vo.children = Some(sub_routes);
            } else if self.is_menu_frame(menu) {
                route_vo.meta = None;

                let child = RouterResp {
                    name: menu.menu_name.clone(),
                    path: menu.path.clone().unwrap_or_default(),
                    component: menu.component.clone().unwrap_or_default(),
                    meta: Some(MetaResp {
                        title: menu.menu_name.clone(),
                        icon: menu.icon.clone(),
                        hidden: menu.is_hidden,
                    }),
                    children: None,
                    always_show: None,
                    redirect: None,
                };

                route_vo.children = Some(vec![child]);
            }

            list.push(route_vo);
        }

        list
    }

    // ─── 辅助方法 ───

    /// 获取路由地址
    fn get_router_path(&self, menu: &UserMenuResp) -> String {
        let router_path = menu.path.clone().unwrap_or_default();

        // 一级目录：加 / 前缀
        if menu.parent_id == common_constant::PARENT_ID
            && menu.menu_type == common_constant::MENU_TYPE_DIR
        {
            format!("/{}", router_path)
        }
        // 一级菜单框架
        else if self.is_menu_frame(menu) {
            "/".to_string()
        }
        // 其他
        else {
            router_path
        }
    }

    /// 获取组件信息
    fn get_component(&self, menu: &UserMenuResp) -> String {
        if self.is_menu_frame(menu) {
            return common_constant::LAYOUT.to_string();
        }

        match &menu.component {
            Some(comp) if !comp.is_empty() => comp.clone(),
            _ if self.is_parent_view(menu) => common_constant::PARENT_VIEW.to_string(),
            _ => common_constant::LAYOUT.to_string(),
        }
    }

    /// 是否为菜单框架（一级菜单，内部跳转）
    fn is_menu_frame(&self, menu: &UserMenuResp) -> bool {
        menu.parent_id == common_constant::PARENT_ID
            && menu.menu_type == common_constant::MENU_TYPE_PAGE
    }

    /// 是否为 ParentView 组件
    fn is_parent_view(&self, menu: &UserMenuResp) -> bool {
        menu.parent_id != common_constant::PARENT_ID
            && menu.menu_type == common_constant::MENU_TYPE_DIR
    }
}
