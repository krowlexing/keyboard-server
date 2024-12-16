use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewStat {
    pub exercise_id: i32,
    pub time: i32,
    pub chars: i32,
    pub errors: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub id: i32,
    pub user_id: i32,
    pub exercise_id: i32,
    pub chars: i32,
    pub time: i32,
    pub errors: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct StatWithUser {
    pub id: i32,
    pub username: String,
    pub exercise_id: i32,
    pub chars: i32,
    pub time: i32,
    pub errors: i32,
    pub created: chrono::DateTime<chrono::Utc>,
}
