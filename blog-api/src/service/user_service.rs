use sha2::Digest;

use crate::{
    constants::common_constant,
    dao::{menu_dao::MenuDao, role_dao::RoleDao, user_dao::UserDao},
    modal::{
        request::login::LoginRequest,
        response::user_reponse::{MetaResp, RouterResp, UserBackInfo, UserMenu},
    },
    utils::{
        error::AppError,
        jwt::{Claims, JwtConfig},
    },
};

#[derive(Clone)]
pub struct UserService {
    user_dao: UserDao,
    role_dao: RoleDao,
    menu_dao: MenuDao,
}

impl UserService {
    pub fn new(user_dao: UserDao, role_dao: RoleDao, menu_dao: MenuDao) -> Self {
        Self {
            user_dao,
            role_dao,
            menu_dao,
        }
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

        let role_list = self
            .role_dao
            .get_role_list_by_user_id(id)
            .await
            .map_err(|_| AppError::NotFound("用户角色代码不存在！请联系管理员！".to_string()))?;

        tracing::info!("role_list:{:?}", role_list);
        let permission_list = self
            .menu_dao
            .get_permissiosn_by_role_list(&role_list)
            .await
            .map_err(|_| AppError::NotFound("用户权限代码不存在！请联系管理员！".to_string()))?;

        tracing::info!("permission_list:{:?}", permission_list);

        // 3. 生成 Token，把 JWT 错误转换为 AppError::TokenError
        let token = JwtConfig::create_token(id, role_list, permission_list)
            .map_err(|_| AppError::TokenError)?;

        Ok(token)
    }

    /// 获取用户信息
    pub async fn get_user_back_info(&self, claims: Claims) -> Result<UserBackInfo, AppError> {
        let avatar = self
            .user_dao
            .get_avatar(claims.sub)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(UserBackInfo {
            id: claims.sub,
            avatar,
            role_list: claims.role_list,
            permission_list: claims.permission_list,
        })
    }

    /// 获取用户菜单
    pub async fn get_user_menu(&self, user_id: i32) -> Result<Vec<RouterResp>, AppError> {
        // 查询用户菜单
        let user_menu = self
            .menu_dao
            .get_menu_by_user_id(user_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        // 递归生成路由，parent_id为0
        Ok(Self::recur_routes(common_constant::PARENT_ID, &user_menu))
    }

    /**
     * 递归生成路由列表
     */
    fn recur_routes(parent_id: i32, menu_list: &[UserMenu]) -> Vec<RouterResp> {
        let mut list = Vec::new();

        for menu in menu_list.iter().filter(|m| m.parent_id == parent_id) {
            let mut route_vo = RouterResp {
                name: menu.menu_name.clone(),
                path: Some(Self::get_router_path(menu)),
                component: Some(Self::get_component(menu)),
                meta: Some(MetaResp {
                    title: menu.menu_name.clone(),
                    icon: menu.icon.clone(),
                    hidden: menu.is_hidden == 1,
                }),
                always_show: None,
                redirect: None,
                children: None,
            };

            if menu.menu_type == common_constant::TYPE_DIR {
                // 目录类型：递归获取子路由
                let children = Self::recur_routes(menu.id, menu_list);
                if !children.is_empty() {
                    route_vo.always_show = Some(true);
                    route_vo.redirect = Some("noRedirect".to_string());
                }
                route_vo.children = Some(children);
            } else if Self::is_menu_frame(menu) {
                // 一级菜单（内部跳转）
                route_vo.meta = None;
                let mut children_list = Vec::new();
                let child = RouterResp {
                    name: menu.menu_name.clone(),
                    path: menu.path.clone(),
                    component: menu.component.clone(),
                    meta: Some(MetaResp {
                        title: menu.menu_name.clone(),
                        icon: menu.icon.clone(),
                        hidden: menu.is_hidden == 1,
                    }),
                    always_show: None,
                    redirect: None,
                    children: None,
                };
                children_list.push(child);
                route_vo.children = Some(children_list);
            }

            list.push(route_vo);
        }

        list
    }

    /**
     * 获取路由地址
     */
    fn get_router_path(menu: &UserMenu) -> String {
        let router_path = menu.path.clone().unwrap_or_default();

        if menu.parent_id == common_constant::PARENT_ID
            && menu.menu_type == common_constant::TYPE_DIR
        {
            // 一级目录：添加前缀斜杠
            format!("/{}", router_path)
        } else if Self::is_menu_frame(menu) {
            // 一级菜单：路径设为根
            "/".to_string()
        } else {
            router_path
        }
    }

    /**
     * 获取组件信息
     */
    fn get_component(menu: &UserMenu) -> String {
        // 如果 component 存在且非空，并且不是一级菜单，则直接使用该组件
        if let Some(comp) = &menu.component
            && !comp.is_empty()
            && !Self::is_menu_frame(menu)
        {
            return comp.clone();
        }

        // 如果 component 为空（None 或空字符串），且是 parent_view 类型（非一级目录），则使用 ParentView
        let is_empty_or_none = match &menu.component {
            Some(comp) => comp.is_empty(),
            None => true,
        };
        if is_empty_or_none && Self::is_parent_view(menu) {
            return common_constant::PARENT_VIEW.to_string();
        }

        // 默认返回 Layout
        common_constant::LAYOUT.to_string()
    }

    /**
     * 是否为菜单内部跳转（一级菜单）
     */
    fn is_menu_frame(menu: &UserMenu) -> bool {
        menu.parent_id == common_constant::PARENT_ID && menu.menu_type == common_constant::TYPE_MENU
    }

    /**
     * 是否为 parent_view 组件（非一级目录）
     */
    fn is_parent_view(menu: &UserMenu) -> bool {
        menu.parent_id != common_constant::PARENT_ID && menu.menu_type == common_constant::TYPE_DIR
    }
}
