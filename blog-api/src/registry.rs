use std::sync::Arc;

use sqlx::PgPool;

use crate::repo::{menu::MenuRepo, role::RoleRepo, user::UserRepo};

/// 应用注册中心 —— 所有 repo 的唯一创建源，确保全应用单例
///
/// 今后所有 repo 都必须在这里注册，service 只能通过 AppRegistry 获取 repo，
/// 严禁在 service 中自行 `new` repo。
#[derive(Clone)]
pub struct AppRegistry {
    pub user_repo: Arc<UserRepo>,
    pub role_repo: Arc<RoleRepo>,
    pub menu_repo: Arc<MenuRepo>,
}

impl AppRegistry {
    pub fn new(db: &PgPool) -> Self {
        Self {
            user_repo: Arc::new(UserRepo::new(db.clone())),
            role_repo: Arc::new(RoleRepo::new(db.clone())),
            menu_repo: Arc::new(MenuRepo::new(db.clone())),
        }
    }
}
