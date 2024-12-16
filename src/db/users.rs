use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, prelude::FromRow, Executor, PgPool};

use crate::dto::users::UserCreds;

#[derive(Clone)]
pub struct Users {
    pool: PgPool,
}

impl Users {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_table(&self) -> Result<PgQueryResult, sqlx::Error> {
        self.pool
            .execute(
                "CREATE TABLE IF NOT EXISTS users(
            id SERIAL PRIMARY KEY,
            username TEXT UNIQUE,
            password TEXT
        );",
            )
            .await
    }

    /**
     * Creates a new user using the provided credentials
     */
    pub async fn create(&self, user: UserCreds) -> Result<i32, CreateErrors> {
        create(&self.pool, user).await
    }

    /**
     * Finds a user by name
     */
    pub async fn find_by_name(&self, username: String) -> Result<Option<User>, sqlx::Error> {
        find_by_name(&self.pool, username).await
    }

    /**
     * Finds a user by name and password
     */
    pub async fn find_user(&self, username: &str, password: &str) -> Result<User, LoginError> {
        find_user(&self.pool, username, password).await
    }

    pub async fn exists(&self, user_id: i32) -> Result<bool, sqlx::Error> {
        exists(&self.pool, user_id).await
    }
}

pub type SqlResult = Result<PgQueryResult, sqlx::Error>;

impl UserCreds {
    pub fn validate(username: String, password: String) -> Option<Self> {
        // TODO add validation
        Some(Self { username, password })
    }
}

#[derive(Debug)]
pub enum CreateErrors {
    AlreadyExists,
    Sql(sqlx::Error),
}

async fn create(pool: &PgPool, user: UserCreds) -> Result<i32, CreateErrors> {
    let result: Result<(i32,), _> =
        sqlx::query_as("INSERT INTO users(username, password) VALUES ($1, $2) RETURNING id;")
            .bind(user.username)
            .bind(bcrypt::hash(user.password, 10).unwrap())
            .fetch_one(pool)
            .await;
    result.map(|x| x.0).map_err(
        |e| match e.as_database_error().map(|e| e.is_unique_violation()) {
            Some(true) => CreateErrors::AlreadyExists,
            _ => CreateErrors::Sql(e),
        },
    )
}

async fn find_by_name(pool: &PgPool, user: String) -> Result<Option<User>, sqlx::Error> {
    let result: Option<User> = sqlx::query_as("SELECT * FROM users WHERE username = ?;")
        .bind(user)
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub enum LoginError {
    WrongPassword,
    SqlError(sqlx::Error),
}

// CREATE TABLE stats {
//     id SERIAL PRIMARY KEY,
//     user_id INTEGER,
//     exercise_id INTEGER,
//     time INTEGER,
//     errors INTEGER
// }
impl From<sqlx::Error> for LoginError {
    fn from(value: sqlx::Error) -> Self {
        LoginError::SqlError(value)
    }
}

async fn find_user(pool: &PgPool, username: &str, password: &str) -> Result<User, LoginError> {
    let user = sqlx::query_as("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
        .map(|user: Option<User>| {
            user.and_then(|user| {
                bcrypt::verify(password, &user.password)
                    .unwrap()
                    .then_some(user)
            })
        });

    match user {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(LoginError::WrongPassword),
        Err(e) => Err(LoginError::SqlError(e)),
    }
}

async fn exists(pool: &PgPool, user_id: i32) -> Result<bool, sqlx::Error> {
    sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map(|user: Option<User>| user.is_some())
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
