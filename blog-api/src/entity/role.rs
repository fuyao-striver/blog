use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: String,
    pub role_name: String,
    pub role_desc: String,
    pub is_disable: bool,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
}
