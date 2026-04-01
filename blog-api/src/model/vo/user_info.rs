//! 用户信息VO模块
//!
//! 定义获取用户信息接口返回的数据结构

use serde::{Deserialize, Serialize};

/// 用户信息VO
///
/// 符合前端期望的返回格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoVO {
    /// 用户ID
    pub id: i32,
    /// 头像
    pub avatar: String,
    /// 角色集合（角色名称列表）
    #[serde(rename = "roleList")]
    pub role_list: Vec<String>,
    /// 权限集合（权限标识列表）
    #[serde(rename = "permissionList")]
    pub permission_list: Vec<String>,
}
