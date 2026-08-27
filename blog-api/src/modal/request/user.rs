use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePassword {
    pub old_password: String,
    pub new_password: String,
}
