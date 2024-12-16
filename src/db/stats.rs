use sqlx::{postgres::PgQueryResult, Executor, PgPool};

use crate::dto::stats::{NewStat, Stat, StatWithUser};

#[derive(Clone)]
pub struct Stats {
    pool: PgPool,
}

impl Stats {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_table(&self) -> Result<PgQueryResult, sqlx::Error> {
        self.pool
            .execute(
                "CREATE TABLE IF NOT EXISTS stats(
                id SERIAL PRIMARY KEY,
                user_id INTEGER,
                exercise_id INTEGER,
                chars INTEGER,
                time INTEGER,
                errors INTEGER,
                created TIMESTAMPTZ DEFAULT NOW()
            );",
            )
            .await
    }

    pub async fn create(&self, user_id: i32, new_stat: NewStat) -> Result<i32, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO stats(user_id, exercise_id, chars, time, errors) 
            VALUES ($1, $2, $3, $4, $5) RETURNING id;",
        )
        .bind(user_id)
        .bind(new_stat.exercise_id)
        .bind(new_stat.chars)
        .bind(new_stat.time)
        .bind(new_stat.errors)
        .fetch_one(&self.pool)
        .await
        .map(|x: (i32,)| x.0)
    }

    pub async fn for_user_and_level(
        &self,
        user_id: i32,
        level: i32,
    ) -> Result<Vec<Stat>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM stats 
            WHERE user_id = $1 AND exercise_id IN (SELECT id FROM exercises WHERE level = $2);",
        )
        .bind(user_id)
        .bind(level)
        .fetch_all(&self.pool)
        .await
        .map_err(
            |e| match e.as_database_error().map(|e| e.is_unique_violation()) {
                Some(true) => sqlx::Error::RowNotFound,
                _ => e,
            },
        )
    }

    pub async fn get_all(&self, diff: i32) -> Result<Vec<StatWithUser>, sqlx::Error> {
        sqlx::query_as(
            "SELECT stats.*, users.username FROM stats
             JOIN exercises ON stats.exercise_id = exercises.id 
             JOIN users ON stats.user_id = users.id 
             WHERE exercises.level = $1;",
        )
        .bind(diff)
        .fetch_all(&self.pool)
        .await
    }
}
