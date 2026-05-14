use std::sync::Arc;

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

    pub async fn get_user_menu(&self, id: i32) -> anyhow::Result<Vec<RouterResp>> {
        let menu_list = self.menu_repo.get_menu_by_user_id(id).await?;
        Ok(self.recur_routes(common_constant::PARENT_ID, &menu_list))
    }

    // 递归生成路由列表
    fn recur_routes(&self, parent_id: i32, menu_list: &[UserMenuResp]) -> Vec<RouterResp> {
        let mut list = Vec::new();

        for menu in menu_list.iter().filter(|m| m.parent_id == parent_id) {
            let mut route_vo = RouterResp {
                name: menu.menu_name.clone(),
                path: self.get_router_path(menu),
                component: self.get_component(menu),
                meta: Some(MetaResp {
                    title: menu.menu_name.clone(),
                    icon: menu.icon.clone(),
                    hidden: menu.is_hidden == true,
                }),
                children: None,
                always_show: None,
                redirect: None,
            };

            if menu.menu_type == common_constant::TYPE_DIR {
                let children = self.recur_routes(menu.id, menu_list);
                if !children.is_empty() {
                    route_vo.always_show = Some(true);
                    route_vo.redirect = Some("noRedirect".to_string());
                }
                route_vo.children = Some(children);
            } else if self.is_menu_frame(menu) {
                route_vo.meta = None;

                let mut children_list = Vec::new();
                let children = RouterResp {
                    name: menu.menu_name.clone(),
                    path: menu.path.clone().unwrap_or_default(),
                    component: menu.component.clone().unwrap_or_default(),
                    meta: Some(MetaResp {
                        title: menu.menu_name.clone(),
                        icon: menu.icon.clone(),
                        hidden: menu.is_hidden == true,
                    }),
                    children: None,
                    always_show: None,
                    redirect: None,
                };
                children_list.push(children);
                route_vo.children = Some(children_list);
            }

            list.push(route_vo);
        }

        list
    }

    // 获取路由地址
    fn get_router_path(&self, menu: &UserMenuResp) -> String {
        let router_path = menu.path.clone().unwrap_or_default();

        // 一级目录
        if menu.parent_id == common_constant::PARENT_ID
            && menu.menu_type == common_constant::TYPE_DIR
        {
            format!("/{}", router_path)
        }
        // 一级菜单
        else if self.is_menu_frame(menu) {
            "/".to_string()
        }
        // 其他情况
        else {
            router_path
        }
    }

    // 获取组件信息
    fn get_component(&self, menu: &UserMenuResp) -> String {
        let component = common_constant::LAYOUT.to_string();

        if menu.component.is_some()
            && !menu.component.as_ref().unwrap().is_empty()
            && !self.is_menu_frame(menu)
        {
            menu.component.clone().unwrap()
        } else if (menu.component.is_none() || menu.component.as_ref().unwrap().is_empty())
            && self.is_parent_view(menu)
        {
            common_constant::PARENT_VIEW.to_string()
        } else {
            component
        }
    }

    // 是否为菜单内部跳转
    fn is_menu_frame(&self, menu: &UserMenuResp) -> bool {
        menu.parent_id == common_constant::PARENT_ID && menu.menu_type == common_constant::TYPE_MENU
    }

    // 是否为parent_view组件
    fn is_parent_view(&self, menu: &UserMenuResp) -> bool {
        menu.parent_id != common_constant::PARENT_ID && menu.menu_type == common_constant::TYPE_DIR
    }
}
