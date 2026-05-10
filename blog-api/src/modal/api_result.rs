use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

/// 统一响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 返回状态
    pub flag: bool,
    /// 状态码
    pub code: u16,
    /// 返回信息
    pub msg: String,
    /// 返回数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            flag: true,
            code: 200,
            msg: "success".into(),
            data: Some(data),
        }
    }

    pub fn ok() -> Self {
        Self {
            flag: true,
            code: 200,
            msg: "success".into(),
            data: None,
        }
    }

    pub fn ok_message(message: &str) -> Self {
        Self {
            flag: true,
            code: 200,
            msg: message.into(),
            data: None,
        }
    }

    pub fn fail(code: u16, msg: impl Into<String>) -> Self {
        Self {
            flag: false,
            code,
            msg: msg.into(),
            data: None,
        }
    }
}

// 添加这个实现：为 ApiResponse 实现 IntoResponse
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        // 根据 code 确定 HTTP 状态码
        let status = if self.flag {
            StatusCode::OK
        } else {
            match self.code {
                400 => StatusCode::BAD_REQUEST,
                401 => StatusCode::UNAUTHORIZED,
                403 => StatusCode::FORBIDDEN,
                404 => StatusCode::NOT_FOUND,
                409 => StatusCode::CONFLICT,
                422 => StatusCode::UNPROCESSABLE_ENTITY,
                500 => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::OK, // 默认为 200，但 flag=false
            }
        };

        (status, Json(self)).into_response()
    }
}
