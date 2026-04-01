//! 角色仓库模块
//!
//! 提供角色相关的数据库操作

use crate::model::role::Role;
use sqlx::PgPool;

/// 根据用户ID查询角色列表
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `user_id`: 用户ID
///
/// # 返回
/// 成功返回角色列表，失败返回错误
pub async fn find_roles_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<Role>, sqlx::Error> {
    sqlx::query_as::<_, Role>(
        r#"
        SELECT r.* 
        FROM t_role r
        INNER JOIN t_user_role ur ON r.id = ur.role_id
        WHERE ur.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// 根据用户ID查询角色标识列表
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `user_id`: 用户ID
///
/// # 返回
/// 成功返回角色名称列表，失败返回错误
pub async fn find_role_names_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<String>, sqlx::Error> {
    let roles = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT r.role_name 
        FROM t_role r
        INNER JOIN t_user_role ur ON r.id = ur.role_id
        WHERE ur.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(roles.into_iter().map(|(name,)| name).collect())
}
