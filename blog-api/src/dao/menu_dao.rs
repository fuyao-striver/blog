use sqlx::{MySqlPool, QueryBuilder};

#[derive(Clone)]
pub struct MenuDao {
    db: MySqlPool,
}

impl MenuDao {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }

    /// 通过role_id获取权限
    pub async fn get_permissiosn_by_role_list(
        &self,
        role_list: &Vec<String>,
    ) -> anyhow::Result<Vec<String>> {
        let mut builder = QueryBuilder::new(
            "select distinct m.perms
        from t_menu m inner join t_role_menu rm on m.id = rm.menu_id 
        where rm.role_id in (",
        );
        let mut separated = builder.separated(",");

        for role_id in role_list {
            separated.push_bind(role_id);
        }
        builder.push(")");
        builder.push("and m.is_disable = 0");

        let permission_list = builder
            .build_query_as::<(Option<String>,)>()
            .fetch_all(&self.db)
            .await?
            .into_iter()
            .filter_map(|(param,)| param)
            .collect();
        Ok(permission_list)
    }
}
