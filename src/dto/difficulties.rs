use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DifficultyData {
    pub min_chars: i32,
    pub max_chars: i32,
    pub errors: i32,
    pub time_limit: i32,
    pub zones: String,
}
