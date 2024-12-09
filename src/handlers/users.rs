use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{
    db,
    dto::users::{UserClaim, UserCreds},
    Db,
};

pub fn router(db: Db) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(db)
}

async fn register(State(db): State<Db>, Json(creds): Json<UserCreds>) -> impl IntoResponse {
    use db::users::CreateErrors::*;

    let result = db.users.create(creds).await;

    println!("{result:?}");
    match result {
        Ok(user_id) => (StatusCode::CREATED, UserClaim { user_id }.sign()).into_response(),
        Err(AlreadyExists) => (StatusCode::CONFLICT, Json("already exists")).into_response(),
        Err(Sql(e)) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn login(State(db): State<Db>, Json(creds): Json<UserCreds>) -> impl IntoResponse {
    use db::users::LoginError::*;

    let result = db.users.find_user(&creds.username, &creds.password).await;

    match result {
        Ok(user) => (StatusCode::OK, UserClaim { user_id: user.id }.sign()).into_response(),
        Err(WrongPassword) => (StatusCode::UNAUTHORIZED, Json("wrong password")).into_response(),
        Err(SqlError(e)) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
