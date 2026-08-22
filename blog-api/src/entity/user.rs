use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub avatar: String,
    pub web_site: Option<String>,
    pub intro: Option<String>,
    pub email: Option<String>,
    pub ip_address: Option<String>,
    pub ip_source: Option<String>,
    pub login_type: i8,
    pub is_disable: i8,
    pub login_time: Option<NaiveDateTime>,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
}
