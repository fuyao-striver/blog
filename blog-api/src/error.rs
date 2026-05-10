use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::modal::api_result::ApiResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("内部错误: {0}")]
    Internal(String),
}

// 实现 IntoResponse，让 AppError 可以作为响应返回
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Internal(e) => {
                tracing::error!("内部错误：{}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e)
            }
        };
        // 返回 JSON 格式的错误响应
        let body = ApiResponse::<()>::fail(status.as_u16(), message);
        (status, Json(body)).into_response()
    }
}
