use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};

use crate::dto::users::UserClaim;

pub struct Admin();

#[async_trait]
impl<S> FromRequestParts<S> for Admin {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .headers
            .get("Authorization")
            .and_then(|auth| UserClaim::verify(auth.to_str().unwrap()).ok())
            .filter(|claim| claim.user_id == 1)
            .map(|_| Admin())
            .ok_or(StatusCode::UNAUTHORIZED.into_response())
    }
}
