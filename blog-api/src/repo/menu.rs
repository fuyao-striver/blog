use sqlx::PgPool;

pub struct MenuRepo {
    db: PgPool,
}

impl MenuRepo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_permission_by_role_id(&self, id: &str) -> anyhow::Result<Vec<String>> {
        let permission_list = sqlx::query_scalar(
            r#"
            SELECT DISTINCT m.perms
            FROM t_menu m
                     INNER JOIN t_role_menu rm ON m.id = rm.menu_id
            WHERE rm.role_id = $1
              AND m.is_disable = 0"#,
        )
        .bind(id)
        .fetch_all(&self.db)
        .await?;
        Ok(permission_list)
    }
}
