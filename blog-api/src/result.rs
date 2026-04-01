use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// 统一响应结果结构
#[derive(Serialize, Debug)]
pub struct Result<T> {
    /// 是否成功
    pub flag: bool,
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 数据
    pub data: Option<T>,
}

impl<T: Serialize> Result<T> {
    pub fn success(data: T) -> Self {
        Self {
            flag: true,
            code: 200,
            msg: "操作成功".to_string(),
            data: Some(data),
        }
    }

    pub fn success_with_msg(msg: impl Into<String>, data: T) -> Self {
        Self {
            flag: true,
            code: 200,
            msg: msg.into(),
            data: Some(data),
        }
    }

    pub fn fail(code: i32, msg: impl Into<String>) -> Self {
        Self {
            flag: false,
            code,
            msg: msg.into(),
            data: None,
        }
    }

    pub fn fail_with_data(code: i32, msg: impl Into<String>, data: T) -> Self {
        Self {
            flag: false,
            code,
            msg: msg.into(),
            data: Some(data),
        }
    }
}

impl<T: Serialize> IntoResponse for Result<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub type AppResult<T> = Result<T>;
