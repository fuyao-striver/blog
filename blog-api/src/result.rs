use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Result<T> {
    pub flag: bool,
    pub code: i32,
    pub msg: String,
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
