use crate::AppState;
use crate::entity::friend::Friend;
use crate::error::AppError;
use crate::modal::api_result::{ApiResponse, PageResult};
use crate::modal::request::PageReq;
use axum::extract::{Query, State};

#[axum::debug_handler]
pub async fn get_friends(
    State(state): State<AppState>,
    Query(page_req): Query<PageReq>,
) -> Result<ApiResponse<PageResult<Friend>>, AppError> {
    let result = state.friend_service.get_friend(page_req).await;
    match result {
        Ok(value) => Ok(ApiResponse::success(value)),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}
