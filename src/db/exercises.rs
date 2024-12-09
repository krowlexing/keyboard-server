use std::sync::Arc;

use sqlx::{postgres::PgQueryResult, Executor, PgPool};

use crate::dto::exercises::{Exercise, NewExercise};

#[derive(Clone)]
pub struct Exercises {
    pool: PgPool,
}

impl Exercises {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_table(&self) -> Result<PgQueryResult, sqlx::Error> {
        self.pool
            .execute(
                "CREATE TABLE IF NOT EXISTS exercises(
                id SERIAL PRIMARY KEY,
                text TEXT,
                level INTEGER
            );",
            )
            .await
    }

    pub async fn create(&self, exercise: NewExercise) -> Result<i32, sqlx::Error> {
        create(&self.pool, exercise).await
    }

    pub async fn all(&self) -> Result<Vec<Exercise>, sqlx::Error> {
        all(&self.pool).await
    }

    pub async fn all_for_level(&self, level: i32) -> Result<Vec<Exercise>, sqlx::Error> {
        all_for_level(&self.pool, level).await
    }

    pub async fn update(&self, id: i32, exercise: NewExercise) {
        update(&self.pool, id, exercise).await;
    }

    pub async fn by_id(&self, id: i32) -> Result<Option<Exercise>, sqlx::Error> {
        by_id(&self.pool, id).await
    }
}

async fn create(pool: &PgPool, exercise: NewExercise) -> Result<i32, sqlx::Error> {
    sqlx::query_as("INSERT INTO exercises(text, level) VALUES ($1, $2) RETURNING id;")
        .bind(exercise.text)
        .bind(exercise.level)
        .fetch_one(pool)
        .await
        .map(|x: (i32,)| x.0)
}

async fn all(pool: &PgPool) -> Result<Vec<Exercise>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM exercises;")
        .fetch_all(pool)
        .await
}

async fn all_for_level(pool: &PgPool, level: i32) -> Result<Vec<Exercise>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM exercises WHERE level = $1;")
        .bind(level)
        .fetch_all(pool)
        .await
}

async fn update(
    pool: &PgPool,
    id: i32,
    exercise: NewExercise,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query("UPDATE exercises SET text = $1, level = $2 WHERE id = $3;")
        .bind(exercise.text)
        .bind(exercise.level)
        .bind(id)
        .execute(pool)
        .await
}

async fn by_id(pool: &PgPool, id: i32) -> Result<Option<Exercise>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM exercises WHERE id = $1;")
        .bind(id)
        .fetch_optional(pool)
        .await
}
