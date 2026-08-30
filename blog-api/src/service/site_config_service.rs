use crate::{
    dao::site_config_dao::SiteConfigDao, entity::site_config::SiteConfig, utils::error::AppError,
};

#[derive(Clone)]
pub struct SiteConfigService {
    site_config_dao: SiteConfigDao,
}

impl SiteConfigService {
    pub fn new(site_config_dao: SiteConfigDao) -> Self {
        Self { site_config_dao }
    }

    pub async fn get_site_config(&self) -> Result<SiteConfig, AppError> {
        let site_config = self
            .site_config_dao
            .get_site_config()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(site_config)
    }
}
