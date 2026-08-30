use sqlx::MySqlPool;

use crate::{
    dao::{
        menu_dao::MenuDao, role_dao::RoleDao, site_config_dao::SiteConfigDao, user_dao::UserDao,
    },
    service::{site_config_service::SiteConfigService, user_service::UserService},
};

pub mod constants;
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
    pub site_config_service: SiteConfigService,
}

impl AppState {
    pub fn new(db: MySqlPool) -> Self {
        let site_config_dao = SiteConfigDao::new(db.clone());
        let user_dao = UserDao::new(db.clone());
        let menu_dao = MenuDao::new(db.clone());
        let role_dao = RoleDao::new(db);
        let user_service = UserService::new(user_dao, role_dao, menu_dao);
        let site_config_service = SiteConfigService::new(site_config_dao);
        Self {
            user_service,
            site_config_service,
        }
    }
}
