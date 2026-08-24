use sqlx::MySqlPool;

use crate::{
    dao::{menu_dao::MenuDao, role_dao::RoleDao, user_dao::UserDao},
    service::user_service::UserService,
};

pub mod dao;
pub mod entity;
pub mod handler;
pub mod modal;
pub mod router;
pub mod service;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
}

impl AppState {
    pub fn new(db: MySqlPool) -> Self {
        let user_dao = UserDao::new(db.clone());
        let menu_dao = MenuDao::new(db.clone());
        let role_dao = RoleDao::new(db);
        let user_service = UserService::new(user_dao, role_dao, menu_dao);
        Self { user_service }
    }
}
