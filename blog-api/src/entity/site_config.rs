use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 站点配置表
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SiteConfig {
    pub id: i32,
    pub user_avatar: String,
    pub tourist_avatar: String,
    pub site_name: String,
    pub site_address: String,
    pub site_intro: String,
    pub site_notice: String,
    pub create_site_time: String, // 建站日期，存为字符串（如 "2022-08-25"）
    pub record_number: String,
    pub author_avatar: String,
    pub site_author: String,
    pub article_cover: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about_me: Option<String>, // text 可为 NULL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gitee: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bilibili: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qq: Option<String>,
    pub comment_check: i8,
    pub message_check: i8,
    pub is_reward: i8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wei_xin_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ali_code: Option<String>,
    pub email_notice: i8,
    pub social_list: String, // 社交列表，如 "gitee,bilibili,github,qq"
    pub login_list: String,  // 登录方式，如 ",gitee,github"
    pub is_music: i8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub music_id: Option<String>,
    pub is_chat: i8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub websocket_url: Option<String>,
    pub create_time: NaiveDateTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<NaiveDateTime>,
}

// 如果需要默认值（例如新建时），可以附加实现 Default，但此处省略。
