use axum::{routing::get, Router};

use crate::{db::Db, extractors::jwt::Jwt};

mod exercises;
mod users;

pub fn router(db: Db) -> Router {
    Router::new()
        .nest("/auth", users::router(db.clone()))
        .nest("/exercises", exercises::router(db))
        .route("/", get(root))
}

async fn root() -> &'static str {
    "Hello world!"
}
