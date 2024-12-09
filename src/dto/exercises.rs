use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize)]
pub struct NewExercise {
    pub text: String,
    pub level: i32,
}

impl NewExercise {
    pub fn new(text: String, level: i32) -> Self {
        Self { text, level }
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Exercise {
    pub id: i32,
    pub text: String,
    pub level: i32,
}
