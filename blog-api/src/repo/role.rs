use sqlx::PgPool;

pub struct RoleRepo {
    db: PgPool,
}

impl RoleRepo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    // 通过用户id获取用户角色
    pub async fn get_role_list_by_id(&self, id: i32) -> anyhow::Result<Vec<String>> {
        let role = sqlx::query_scalar(
            r#"SELECT r.id
        FROM t_role r
                 INNER JOIN t_user_role ur ON r.id = ur.role_id
        WHERE ur.user_id = $1
          AND r.is_disable = false"#,
        )
        .bind(id)
        .fetch_all(&self.db)
        .await?;
        Ok(role)
    }
}
