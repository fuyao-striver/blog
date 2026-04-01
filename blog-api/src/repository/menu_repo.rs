//! 菜单仓库模块
//!
//! 提供菜单相关的数据库操作

use crate::utils::sql::format_sql;
use sqlx::PgPool;

/// 根据用户ID查询权限标识列表
///
/// 通过用户->角色->菜单的关联关系查询用户拥有的所有权限标识
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `user_id`: 用户ID
///
/// # 返回
/// 成功返回权限标识列表，失败返回错误
pub async fn find_permissions_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<String>, sqlx::Error> {
    let sql = r#"
        SELECT DISTINCT m.perms 
        FROM t_menu m
        INNER JOIN t_role_menu rm ON m.id = rm.menu_id
        INNER JOIN t_user_role ur ON rm.role_id = ur.role_id
        WHERE ur.user_id = $1 AND m.perms IS NOT NULL AND m.perms != ''
        ORDER BY m.perms
        "#;
    
    // 记录SQL日志
    tracing::debug!(
        sql = %format_sql(sql),
        user_id = user_id,
        "执行查询用户权限列表"
    );
    
    let start = std::time::Instant::now();
    let result = sqlx::query_as::<_, (Option<String>,)>(sql)
        .bind(user_id)
        .fetch_all(pool)
        .await;
    
    // 记录SQL执行结果
    match &result {
        Ok(perms) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                permission_count = perms.len(),
                "SQL查询完成"
            );
        }
        Err(e) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                error = %e,
                "SQL查询失败"
            );
        }
    }
    
    result.map(|perms| perms.into_iter().filter_map(|(p,)| p).collect())
}
