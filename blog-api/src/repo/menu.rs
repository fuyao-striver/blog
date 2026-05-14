use sqlx::PgPool;

use crate::modal::response::user_back::UserMenuResp;

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
              AND m.is_disable = false"#,
        )
        .bind(id)
        .fetch_all(&self.db)
        .await?;
        Ok(permission_list)
    }

    pub async fn get_menu_by_user_id(&self, id: i32) -> anyhow::Result<Vec<UserMenuResp>> {
        let menu_list = sqlx::query_as::<_, UserMenuResp>(
            r#"SELECT DISTINCT m.id,
                        m.parent_id,
                        m.menu_name,
                        m.menu_type,
                        m.path,
                        m.icon,
                        m.order_num,
                        m.component,
                        m.is_hidden
        FROM t_menu m
                 INNER JOIN t_role_menu rm ON m.id = rm.menu_id
                 INNER JOIN t_user_role ur ON rm.role_id = ur.role_id
                 INNER JOIN t_role r ON ur.role_id = r.id
        WHERE m.menu_type in ('M', 'C')
          AND m.is_disable = false
          AND r.is_disable = false
          AND ur.user_id = $1
        ORDER BY m.parent_id, m.order_num"#,
        )
        .bind(id)
        .fetch_all(&self.db)
        .await?;
        Ok(menu_list)
    }
}
