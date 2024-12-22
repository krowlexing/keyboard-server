use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};

use crate::{
    db::Db,
    dto::difficulties::DifficultyData,
    extractors::{admin::Admin, jwt::Jwt},
    util::report_sql_error,
};

pub fn router(db: Db) -> Router {
    Router::new()
        .route("/:id", get(get_by_id))
        .route("/:id", post(update))
        .route("/", get(all))
        .with_state(db)
}

async fn get_by_id(State(db): State<Db>, Path(id): Path<i32>) -> impl IntoResponse {
    let result = db.difficulties.get(id).await;

    match result {
        Ok(exercise) => Json(exercise).into_response(),
        Err(e) => {
            report_sql_error(e, "error getting exercise");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn update(
    State(db): State<Db>,
    _: Admin,
    Path(id): Path<i32>,
    Json(diff): Json<DifficultyData>,
) -> impl IntoResponse {
    let result = db.difficulties.update(id, diff).await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            report_sql_error(e, "error updating exercise");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn all(State(db): State<Db>, _: Jwt) -> impl IntoResponse {
    let result = db.difficulties.all().await;
    match result {
        Ok(exercises) => Json(exercises).into_response(),
        Err(e) => {
            report_sql_error(e, "error getting exercises");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
