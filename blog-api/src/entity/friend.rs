use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Friend {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub avatar: String,
    pub url: String,
    pub introduction: String,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
}
