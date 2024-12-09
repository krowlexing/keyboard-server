use axum::{response::IntoResponse, routing::get, Router};

use crate::{db::Db, extractors::jwt::Jwt};

pub fn router(db: Db) -> Router {
    Router::new()
        .route("/", get(test))
        .route("/test", get(|| async { "Hello world" }))
}

async fn test(Jwt(user_id): Jwt) -> impl IntoResponse {
    format!("hello, there #{user_id}")
}
