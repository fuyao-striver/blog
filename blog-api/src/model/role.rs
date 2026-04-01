//! 角色模型模块
//!
//! 定义角色相关的数据结构

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 角色实体
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    /// 角色ID
    pub id: String,
    /// 角色名称
    pub role_name: String,
    /// 角色描述
    pub role_desc: Option<String>,
    /// 是否禁用（0: 正常, 1: 禁用）
    pub is_disable: i16,
    /// 创建时间
    pub create_time: NaiveDateTime,
    /// 更新时间
    pub update_time: Option<NaiveDateTime>,
}
