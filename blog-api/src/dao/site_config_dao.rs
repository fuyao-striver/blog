use sqlx::MySqlPool;

use crate::entity::site_config::SiteConfig;

#[derive(Clone)]
pub struct SiteConfigDao {
    db: MySqlPool,
}

impl SiteConfigDao {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }

    pub async fn get_site_config(&self) -> anyhow::Result<SiteConfig> {
        let site_config =
            sqlx::query_as!(SiteConfig, r#"select * from t_site_config where id = ?"#, 1)
                .fetch_one(&self.db)
                .await?;
        Ok(site_config)
    }
}
