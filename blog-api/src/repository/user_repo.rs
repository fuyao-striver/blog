use crate::model::user::{NewUser, UpdateUser, User};
use sqlx::PgPool;

pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM t_user ORDER BY id")
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM t_user WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM t_user WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM t_user WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
}

pub async fn create(pool: &PgPool, user: &NewUser) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO t_user (
            nickname, username, password, avatar, web_site, intro, email,
            ip_address, ip_source, login_type, is_disable, login_time, create_time
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW())
        RETURNING *
        "#,
    )
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
    .await
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    user: &UpdateUser,
) -> Result<Option<User>, sqlx::Error> {
    let existing = find_by_id(pool, id).await?;
    if existing.is_none() {
        return Ok(None);
    }

    sqlx::query_as::<_, User>(
        r#"
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
        "#,
    )
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
    .await
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM t_user WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM t_user")
        .fetch_one(pool)
        .await?;
    Ok(count.0)
}
