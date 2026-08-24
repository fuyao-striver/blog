use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBackInfo {
    // 用户id
    pub id: i32,
    // 用户头像
    pub avatar: String,
    // 用户角色
    pub role_list: Vec<String>,
    // 用户权限
    pub permission_list: Vec<String>,
}
