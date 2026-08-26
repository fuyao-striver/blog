use sqlx::{MySqlPool, QueryBuilder};

use crate::modal::response::user_reponse::UserMenu;

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

    /// 通过用户id获取用户菜单
    pub async fn get_menu_by_user_id(&self, user_id: i32) -> anyhow::Result<Vec<UserMenu>> {
        let user_menu = sqlx::query_as!(UserMenu,r#"select distinct
        m.id,m.parent_id,m.menu_name,m.menu_type,m.path,m.icon,m.component,m.is_hidden
        from t_menu m inner join t_role_menu rm on m.id = rm.menu_id
        inner join t_user_role ur on rm.role_id = ur.role_id 
        inner join t_role r on ur.role_id = r.id
        where m.menu_type in ('M','C')
        and m.is_disable = 0 and r.is_disable = 0 and ur.user_id = ? order by m.parent_id,m.order_num"#,user_id).fetch_all(&self.db).await?;
        Ok(user_menu)
    }
}
