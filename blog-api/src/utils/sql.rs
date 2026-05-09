use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

/// 创建 PostgreSQL 连接池
///
/// # 参数
/// * `database_url` - 数据库连接字符串，格式：`postgres://user:password@host:port/dbname`
///
/// # 返回
/// 返回配置好的数据库连接池实例
///
/// # 示例
/// ```
/// let pool = connect_postgres("postgres://postgres@localhost:5432/myapp").await;
/// ```
pub async fn connect_postgres(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10) // 最大连接数，推荐 CPU 核数 * 2
        .min_connections(2) // 最小连接数，保持热备
        .acquire_timeout(Duration::from_secs(5)) // 获取连接超时
        .idle_timeout(Duration::from_mins(10)) // 空闲连接超时（10分钟）
        .max_lifetime(Duration::from_mins(30)) // 连接最大存活时间（30分钟）
        .test_before_acquire(true) // 获取连接前先测试有效性
        .connect(database_url)
        .await
        .expect("连接postgres数据库失败！")
}
