//! 菜单仓库模块
//!
//! 提供菜单相关的数据库操作

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
    let perms = sqlx::query_as::<_, (Option<String>,)>(
        r#"
        SELECT DISTINCT m.perms 
        FROM t_menu m
        INNER JOIN t_role_menu rm ON m.id = rm.menu_id
        INNER JOIN t_user_role ur ON rm.role_id = ur.role_id
        WHERE ur.user_id = $1 AND m.perms IS NOT NULL AND m.perms != ''
        ORDER BY m.perms
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(perms.into_iter().filter_map(|(p,)| p).collect())
}
