use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserBackInfoResp {
    pub id: i32,
    pub avatar: String,
    pub role_list: Vec<String>,
    pub permission_list: Vec<String>,
}
