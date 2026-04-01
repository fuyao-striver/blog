//! 用户处理模块
//!
//! 处理获取用户信息等用户相关请求

use std::sync::Arc;

use axum::extract::State;

use crate::{
    AppState,
    model::vo::UserInfoVO,
    repository::{menu_repo, role_repo, user_repo},
    result::Result,
    utils::auth::AuthUser,
};

/// 获取当前登录用户信息
///
/// 通过多表联查获取用户基本信息、角色列表和权限列表
///
/// # 参数
/// - `auth`: 认证用户信息（从Token中解析出的用户ID）
/// - `state`: 应用状态（包含数据库连接池）
///
/// # 返回
/// 成功返回用户信息（id, avatar, roleList, permissionList），失败返回错误信息
pub async fn get_user_info(
    auth: AuthUser,
    State(state): State<Arc<AppState>>,
) -> Result<UserInfoVO> {
    // 查询用户基本信息
    let user = match user_repo::find_by_id(&state.db, auth.user_id).await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("查询用户信息失败: {}", e);
            return Result::fail(500, "服务器内部错误");
        }
    };

    // 检查用户是否存在
    let user = match user {
        Some(u) => u,
        None => return Result::fail(404, "用户不存在"),
    };

    // 查询用户的角色列表（通过 t_user_role 关联表）
    let role_list = match role_repo::find_role_names_by_user_id(&state.db, auth.user_id).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("查询用户角色失败: {}", e);
            return Result::fail(500, "服务器内部错误");
        }
    };

    // 查询用户的权限列表（通过 t_user_role -> t_role_menu -> t_menu 多表关联）
    let permission_list = match menu_repo::find_permissions_by_user_id(&state.db, auth.user_id).await {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("查询用户权限失败: {}", e);
            return Result::fail(500, "服务器内部错误");
        }
    };

    // 构建并返回用户信息VO
    Result::success(UserInfoVO {
        id: user.id,
        avatar: user.avatar,
        role_list,
        permission_list,
    })
}
