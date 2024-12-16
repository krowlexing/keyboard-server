use sqlx::{postgres::PgQueryResult, Executor, PgPool};

use crate::dto::{difficulties::DifficultyData, exercises::Exercise};

#[derive(Clone)]
pub struct Difficulties {
    pool: PgPool,
}

impl Difficulties {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_table(&self) -> Result<i32, sqlx::Error> {
        self.pool
            .execute(
                "CREATE TABLE IF NOT EXISTS difficulties(
                id SERIAL PRIMARY KEY,
                min_chars INTEGER,
                max_chars INTEGER,
                errors INTEGER,
                time_limit INTEGER,
                zones TEXT
            );",
            )
            .await?;

        let count = self.count().await?;
        if count == 0 {
            self.create_default().await?;
            self.create_default().await?;
            self.create_default().await?;
            self.create_default().await?;
            self.create_default().await?;
        }

        Ok(0)
    }

    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        sqlx::query_as("SELECT COUNT(*) FROM difficulties;")
            .fetch_one(&self.pool)
            .await
            .map(|x: (i64,)| x.0)
    }

    pub async fn create_default(&self) -> Result<i32, sqlx::Error> {
        create(&self.pool).await
    }

    pub async fn get(&self, level: i32) -> Result<DifficultyData, sqlx::Error> {
        sqlx::query_as("SELECT * FROM difficulties WHERE id = $1;")
            .bind(level)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update(
        &self,
        id: i32,
        diff: DifficultyData,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE difficulties SET min_chars = $1, max_chars = $2, errors = $3, time_limit = $4, zones = $5 WHERE id = $6;")
            .bind(diff.min_chars)
            .bind(diff.max_chars)
            .bind(diff.errors)
            .bind(diff.time_limit)
            .bind(diff.zones)
            .bind(id)
            .execute(&self.pool)
            .await
    }
}

async fn create(pool: &PgPool) -> Result<i32, sqlx::Error> {
    sqlx::query_as("INSERT INTO difficulties(min_chars, max_chars, errors, time_limit, zones) VALUES ($1, $2, $3, $4, $5) RETURNING id;")
        .bind(25)
        .bind(50)
        .bind(5)
        .bind(1)
        .bind("12345")
        .fetch_one(pool)
        .await
        .map(|x: (i32,)| x.0)
}