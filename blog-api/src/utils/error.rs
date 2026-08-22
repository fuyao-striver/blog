use axum::{Json, http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::modal::response::AppResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(String),
    #[error("404错误：{0}")]
    NotFound(String),

    #[error("token创建失败!")]
    TokenError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::Database(s) => (StatusCode::INTERNAL_SERVER_ERROR, s),
            AppError::NotFound(s) => (StatusCode::NOT_FOUND, s),
            AppError::TokenError => (StatusCode::BAD_REQUEST, "token创建失败".to_string()),
        };
        (
            status,
            Json(AppResponse::<()>::error_msg(status.as_u16(), error_message)),
        )
            .into_response()
    }
}
