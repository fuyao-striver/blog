//! 用户模型模块
//!
//! 定义用户相关的数据结构

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 用户登录表单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginForm {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

/// 用户实体
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// 用户ID
    pub id: i32,
    /// 昵称
    pub nickname: String,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 头像
    pub avatar: String,
    /// 个人网站
    pub web_site: Option<String>,
    /// 个人简介
    pub intro: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// IP来源
    pub ip_source: Option<String>,
    /// 登录类型
    pub login_type: i16,
    /// 是否禁用（0: 正常, 1: 禁用）
    pub is_disable: i16,
    /// 最后登录时间
    pub login_time: Option<NaiveDateTime>,
    /// 创建时间
    pub create_time: NaiveDateTime,
    /// 更新时间
    pub update_time: Option<NaiveDateTime>,
}

/// 新用户（用于创建）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    /// 昵称
    pub nickname: String,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 头像
    pub avatar: String,
    /// 个人网站
    pub web_site: Option<String>,
    /// 个人简介
    pub intro: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// IP来源
    pub ip_source: Option<String>,
    /// 登录类型
    pub login_type: i16,
    /// 是否禁用
    pub is_disable: Option<i16>,
    /// 最后登录时间
    pub login_time: Option<NaiveDateTime>,
}

/// 用户更新（用于修改）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    /// 昵称
    pub nickname: Option<String>,
    /// 用户名
    pub username: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 头像
    pub avatar: Option<String>,
    /// 个人网站
    pub web_site: Option<String>,
    /// 个人简介
    pub intro: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// IP来源
    pub ip_source: Option<String>,
    /// 登录类型
    pub login_type: Option<i16>,
    /// 是否禁用
    pub is_disable: Option<i16>,
    /// 最后登录时间
    pub login_time: Option<NaiveDateTime>,
}
