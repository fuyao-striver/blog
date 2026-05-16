use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordReq {
    /**
     * 旧密码
     */
    pub old_password: String,
    /**
     * 新密码
     */
    pub new_password: String,
    /**
     * 旧密码
     */
    pub check_password: String,
}
