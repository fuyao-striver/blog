use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: String,                         // varchar(20)
    pub role_name: String,                  // varchar(20), NOT NULL
    pub role_desc: Option<String>,          // varchar(50), 可为 NULL
    pub is_disable: bool,                   // tinyint(1) 映射为 bool (0=false, 1=true)
    pub create_time: NaiveDateTime,         // datetime, NOT NULL
    pub update_time: Option<NaiveDateTime>, // datetime, 可为 NULL
}
