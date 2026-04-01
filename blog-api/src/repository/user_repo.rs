//! 用户仓库模块
//!
//! 提供用户相关的数据库操作

use crate::model::user::{NewUser, UpdateUser, User};
use crate::utils::sql::format_sql;
use sqlx::PgPool;

/// 查询所有用户
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// 成功返回用户列表，失败返回错误
pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let sql = "SELECT * FROM t_user ORDER BY id";
    
    // 记录SQL日志
    tracing::debug!(
        sql = %format_sql(sql),
        "执行查询所有用户"
    );
    
    let start = std::time::Instant::now();
    let result = sqlx::query_as::<_, User>(sql)
        .fetch_all(pool)
        .await;
    
    // 记录SQL执行结果
    match &result {
        Ok(users) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                rows = users.len(),
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
    
    result
}

/// 根据ID查询用户
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 用户ID
///
/// # 返回
/// 成功返回用户信息，失败返回错误
pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, sqlx::Error> {
    let sql = "SELECT * FROM t_user WHERE id = $1";
    
    // 记录SQL日志
    tracing::debug!(
        sql = %format_sql(sql),
        param = id,
        "执行根据ID查询用户"
    );
    
    let start = std::time::Instant::now();
    let result = sqlx::query_as::<_, User>(sql)
        .bind(id)
        .fetch_optional(pool)
        .await;
    
    // 记录SQL执行结果
    match &result {
        Ok(user) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                found = user.is_some(),
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
    
    result
}

/// 根据用户名查询用户
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `username`: 用户名
///
/// # 返回
/// 成功返回用户信息，失败返回错误
pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    let sql = "SELECT * FROM t_user WHERE username = $1";
    
    // 记录SQL日志
    tracing::debug!(
        sql = %format_sql(sql),
        param = %username,
        "执行根据用户名查询用户"
    );
    
    let start = std::time::Instant::now();
    let result = sqlx::query_as::<_, User>(sql)
        .bind(username)
        .fetch_optional(pool)
        .await;
    
    // 记录SQL执行结果
    match &result {
        Ok(user) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                found = user.is_some(),
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
    
    result
}

/// 根据邮箱查询用户
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `email`: 邮箱地址
///
/// # 返回
/// 成功返回用户信息，失败返回错误
pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let sql = "SELECT * FROM t_user WHERE email = $1";
    
    // 记录SQL日志
    tracing::debug!(
        sql = %format_sql(sql),
        param = %email,
        "执行根据邮箱查询用户"
    );
    
    let start = std::time::Instant::now();
    let result = sqlx::query_as::<_, User>(sql)
        .bind(email)
        .fetch_optional(pool)
        .await;
    
    // 记录SQL执行结果
    match &result {
        Ok(user) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                found = user.is_some(),
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
    
    result
}

/// 创建用户
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `user`: 新用户信息
///
/// # 返回
/// 成功返回创建的用户，失败返回错误
pub async fn create(pool: &PgPool, user: &NewUser) -> Result<User, sqlx::Error> {
    let sql = r#"
        INSERT INTO t_user (
            nickname, username, password, avatar, web_site, intro, email,
            ip_address, ip_source, login_type, is_disable, login_time, create_time
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW())
        RETURNING *
        "#;
    
    // 记录SQL日志
    tracing::debug!(
        sql = %format_sql(sql),
        username = %user.username,
        email = ?user.email,
        "执行创建用户"
    );
    
    let start = std::time::Instant::now();
    let result = sqlx::query_as::<_, User>(sql)
        .bind(&user.nickname)
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.avatar)
        .bind(&user.web_site)
        .bind(&user.intro)
        .bind(&user.email)
        .bind(&user.ip_address)
        .bind(&user.ip_source)
        .bind(user.login_type)
        .bind(user.is_disable.unwrap_or(0))
        .bind(user.login_time)
        .fetch_one(pool)
        .await;
    
    // 记录SQL执行结果
    match &result {
        Ok(user) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                user_id = user.id,
                "SQL插入完成"
            );
        }
        Err(e) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                error = %e,
                "SQL插入失败"
            );
        }
    }
    
    result
}

/// 更新用户信息
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 用户ID
/// - `user`: 更新的用户信息
///
/// # 返回
/// 成功返回更新后的用户，失败返回错误
pub async fn update(
    pool: &PgPool,
    id: i32,
    user: &UpdateUser,
) -> Result<Option<User>, sqlx::Error> {
    let existing = find_by_id(pool, id).await?;
    if existing.is_none() {
        return Ok(None);
    }

    let sql = r#"
        UPDATE t_user SET
            nickname = COALESCE($1, nickname),
            username = COALESCE($2, username),
            password = COALESCE($3, password),
            avatar = COALESCE($4, avatar),
            web_site = COALESCE($5, web_site),
            intro = COALESCE($6, intro),
            email = COALESCE($7, email),
            ip_address = COALESCE($8, ip_address),
            ip_source = COALESCE($9, ip_source),
            login_type = COALESCE($10, login_type),
            is_disable = COALESCE($11, is_disable),
            login_time = COALESCE($12, login_time),
            update_time = NOW()
        WHERE id = $13
        RETURNING *
        "#;
    
    // 记录SQL日志
    tracing::debug!(
        sql = %format_sql(sql),
        user_id = id,
        "执行更新用户"
    );
    
    let start = std::time::Instant::now();
    let result = sqlx::query_as::<_, User>(sql)
        .bind(&user.nickname)
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.avatar)
        .bind(&user.web_site)
        .bind(&user.intro)
        .bind(&user.email)
        .bind(&user.ip_address)
        .bind(&user.ip_source)
        .bind(user.login_type)
        .bind(user.is_disable)
        .bind(user.login_time)
        .bind(id)
        .fetch_optional(pool)
        .await;
    
    // 记录SQL执行结果
    match &result {
        Ok(user) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                updated = user.is_some(),
                "SQL更新完成"
            );
        }
        Err(e) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                error = %e,
                "SQL更新失败"
            );
        }
    }
    
    result
}

/// 删除用户
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 用户ID
///
/// # 返回
/// 成功返回是否删除成功，失败返回错误
pub async fn delete(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let sql = "DELETE FROM t_user WHERE id = $1";
    
    // 记录SQL日志
    tracing::debug!(
        sql = %format_sql(sql),
        param = id,
        "执行删除用户"
    );
    
    let start = std::time::Instant::now();
    let result = sqlx::query(sql)
        .bind(id)
        .execute(pool)
        .await;
    
    // 记录SQL执行结果
    match &result {
        Ok(res) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                rows_affected = res.rows_affected(),
                "SQL删除完成"
            );
        }
        Err(e) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                error = %e,
                "SQL删除失败"
            );
        }
    }
    
    result.map(|r| r.rows_affected() > 0)
}

/// 统计用户数量
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// 成功返回用户数量，失败返回错误
pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let sql = "SELECT COUNT(*) FROM t_user";
    
    // 记录SQL日志
    tracing::debug!(
        sql = %format_sql(sql),
        "执行统计用户数量"
    );
    
    let start = std::time::Instant::now();
    let result = sqlx::query_as::<_, (i64,)>(sql)
        .fetch_one(pool)
        .await;
    
    // 记录SQL执行结果
    match &result {
        Ok((count,)) => {
            tracing::debug!(
                sql = %format_sql(sql),
                elapsed_ms = start.elapsed().as_millis(),
                count = count,
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
    
    result.map(|(count,)| count)
}
