use serde::Serialize;

/// 用于插入的数据结构
#[derive(Debug, Serialize)]
pub struct NewFriend {
    pub name: String,
    pub color: String,
    pub avatar: String,
    pub url: String,
    pub introduction: String,
}
