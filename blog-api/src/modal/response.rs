use std::fmt::Debug;

use serde::Serialize;

pub mod user_reponse;

#[derive(Debug, Serialize)]
pub struct AppResponse<T> {
    /// 返回状态
    pub flag: bool,
    /// 状态码
    pub code: u16,
    /// 返回信息
    pub msg: String,
    /// 返回数据
    pub data: Option<T>,
}

impl<T> AppResponse<T>
where
    T: Debug + Serialize,
{
    pub fn ok(msg: String, data: Option<T>) -> Self {
        let result = Self {
            flag: true,
            code: 200,
            msg,
            data,
        };
        tracing::info!("成功的相应结果为：{:?}", result);
        result
    }

    pub fn error_msg(code: u16, msg: String) -> Self {
        let result = Self {
            flag: false,
            code,
            msg,
            data: None,
        };
        tracing::info!("失败的响应结果为:{:?}", result);
        result
    }
}
