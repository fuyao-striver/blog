use sqlx::MySqlPool;

#[derive(Clone)]
pub struct RoleDao {
    db: MySqlPool,
}

impl RoleDao {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }

    /// 通过用户id获取用户权限
    pub async fn get_role_list_by_user_id(&self, user_id: i32) -> anyhow::Result<String> {
        let role_list: String = sqlx::query_scalar!(
            r#"select r.id 
        from t_role r inner join t_user_role ur on r.id = ur.role_id 
        where ur.user_id = ? and r.is_disable = 0"#,
            user_id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(role_list)
    }
}
