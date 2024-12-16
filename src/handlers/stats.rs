use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::{
    db::Db,
    dto::stats::NewStat,
    extractors::{admin::Admin, jwt::Jwt},
    util::report_sql_error,
};

pub fn router(db: Db) -> Router {
    Router::new()
        .route("/diff/:diff", get(get_all))
        .route("/", post(create))
        .route("/level/:level", get(get_all_for_level))
        .with_state(db)
}

pub async fn get_all(State(db): State<Db>, _: Admin, Path(diff): Path<i32>) -> impl IntoResponse {
    let result = db.stats.get_all(diff).await;
    match result {
        Ok(stats) => Json(stats).into_response(),
        Err(e) => {
            report_sql_error(e, "error getting stats");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create(
    State(db): State<Db>,
    Jwt(user_id): Jwt,
    Json(exercise): Json<NewStat>,
) -> impl IntoResponse {
    let result = db.stats.create(user_id, exercise).await;

    match result {
        Ok(id) => (StatusCode::CREATED, Json(id)).into_response(),
        Err(e) => {
            report_sql_error(e, "error creating exercise");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_all_for_level(
    State(db): State<Db>,
    Jwt(user_id): Jwt,
    Path(level): Path<i32>,
) -> impl IntoResponse {
    let result = db.stats.for_user_and_level(user_id, level).await;
    match result {
        Ok(stats) => Json(stats).into_response(),
        Err(e) => {
            report_sql_error(e, "error getting stats");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
