use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};

use crate::{
    db::Db,
    dto::exercises::NewExercise,
    extractors::{admin::Admin, jwt::Jwt},
    util::report_sql_error,
};

pub fn router(db: Db) -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/", post(create))
        .route("/level/:level", get(get_all_for_level))
        .route("/:id", get(get_by_id))
        .route("/:id", put(update))
        .with_state(db)
}

async fn create(
    State(db): State<Db>,
    _: Admin,
    Json(exercise): Json<NewExercise>,
) -> impl IntoResponse {
    let result = db.exercises.create(exercise).await;

    match result {
        Ok(id) => (StatusCode::CREATED, Json(id)).into_response(),
        Err(e) => {
            report_sql_error(e, "error creating exercise");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn get_all(State(db): State<Db>, _: Admin) -> impl IntoResponse {
    let result = db.exercises.all().await;
    match result {
        Ok(exercises) => Json(exercises).into_response(),
        Err(e) => {
            report_sql_error(e, "error getting exercises");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn get_all_for_level(
    State(db): State<Db>,
    _: Jwt,
    Path(level): Path<i32>,
) -> impl IntoResponse {
    let result = db.exercises.all_for_level(level).await;
    match result {
        Ok(exercises) => Json(exercises).into_response(),
        Err(e) => {
            report_sql_error(e, "error getting exercises");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn get_by_id(
    State(db): State<Db>,
    Jwt(user_id): Jwt,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = db.exercises.by_id(id).await;
    match result {
        Ok(Some(exercise)) => Json(exercise).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
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
    Json(exercise): Json<NewExercise>,
) {
    db.exercises.update(id, exercise).await;
}
