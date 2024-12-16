use axum::Router;

use crate::db::Db;

mod difficulties;
mod exercises;
mod stats;
mod users;

pub fn router(db: Db) -> Router {
    Router::new()
        .nest("/auth", users::router(db.clone()))
        .nest("/exercises", exercises::router(db.clone()))
        .nest("/stats", stats::router(db.clone()))
        .nest("/diff", difficulties::router(db.clone()))
}
