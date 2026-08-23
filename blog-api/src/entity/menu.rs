use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 菜单类型：M-目录，C-菜单，B-按钮
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")] // 序列化为 "M","C","B"
pub enum MenuType {
    M, // 目录
    C, // 菜单
    B, // 按钮
}

/// 对应 t_menu 表
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Menu {
    pub id: i32,                            // int, AUTO_INCREMENT
    pub parent_id: i32,                     // int, NOT NULL, default 0
    pub menu_type: MenuType,                // char(1) → 枚举
    pub menu_name: String,                  // varchar(50), NOT NULL
    pub path: Option<String>,               // varchar(255), 可为 NULL
    pub icon: Option<String>,               // varchar(50), 可为 NULL
    pub component: Option<String>,          // varchar(50), 可为 NULL
    pub perms: Option<String>,              // varchar(100), DEFAULT ''，但允许 NULL (使用 Option)
    pub is_hidden: bool,                    // tinyint(1) → bool
    pub is_disable: bool,                   // tinyint(1) → bool
    pub order_num: i32,                     // int, default 1
    pub create_time: NaiveDateTime,         // datetime, NOT NULL
    pub update_time: Option<NaiveDateTime>, // datetime, 可为 NULL
}
