use std::fmt::Debug;

use axum::Json;
use serde::Serialize;

use crate::utils::error::AppError;

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
    pub fn ok(msg: &str, data: Option<T>) -> Self {
        let result = Self {
            flag: true,
            code: 200,
            msg: msg.to_string(),
            data,
        };
        tracing::info!("成功的相应结果为：{:?}", result);
        result
    }

    pub fn ok_msg(msg: &str) -> Self {
        let result = Self {
            flag: true,
            code: 200,
            msg: msg.to_string(),
            data: None,
        };
        tracing::info!("成功的相应结果为：{:?}", result);
        result
    }

    pub fn error_msg(code: u16, msg: &str) -> Self {
        let result = Self {
            flag: false,
            code,
            msg: msg.to_string(),
            data: None,
        };
        tracing::info!("失败的响应结果为:{:?}", result);
        result
    }
}

pub type AppResult<T> = Result<Json<AppResponse<T>>, AppError>;
