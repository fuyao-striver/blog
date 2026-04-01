//! 菜单模型模块
//!
//! 定义菜单/权限相关的数据结构

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 菜单实体
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Menu {
    /// 菜单ID
    pub id: i32,
    /// 父菜单ID
    pub parent_id: i32,
    /// 权限类型（M: 目录, C: 菜单, B: 按钮）
    pub menu_type: String,
    /// 菜单名称
    pub menu_name: String,
    /// 路由地址
    pub path: Option<String>,
    /// 菜单图标
    pub icon: Option<String>,
    /// 菜单组件
    pub component: Option<String>,
    /// 权限标识
    pub perms: Option<String>,
    /// 是否隐藏（0: 否, 1: 是）
    pub is_hidden: i16,
    /// 是否禁用（0: 否, 1: 是）
    pub is_disable: i16,
    /// 排序
    pub order_num: i32,
    /// 创建时间
    pub create_time: NaiveDateTime,
    /// 更新时间
    pub update_time: Option<NaiveDateTime>,
}
