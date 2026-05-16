use serde::{Deserialize, Serialize};

pub mod friend_req;
pub mod login;
pub mod user_req;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageReq {
    /**
     * 当前页
     */
    pub current: i32,
    /**
     * 每页大小
     */
    pub size: i32,

    /**
     *  关键字
     */
    pub keyword: Option<String>,
}
