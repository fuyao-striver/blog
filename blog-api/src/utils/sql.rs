use std::time::Duration;

use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

/// 连接MySQL数据库
pub fn connect_mysql() -> anyhow::Result<Pool<MySql>> {
    let database_url = std::env::var("DATABASE_URL")?;
    Ok(MySqlPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_mins(30))
        .connect_lazy(&database_url)?)
}
