use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBackInfoResp {
    pub id: i32,
    pub avatar: String,
    pub role_list: Vec<String>,
    pub permission_list: Vec<String>,
}

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserMenuResp {
    pub id: i32,
    pub parent_id: i32,
    pub menu_name: String,
    pub menu_type: String,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub order_num: Option<i32>,
    pub component: Option<String>,
    pub is_hidden: bool,
}
