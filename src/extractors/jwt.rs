use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};

use crate::dto::users::UserClaim;

pub struct Jwt(pub i32);

#[async_trait]
impl<S> FromRequestParts<S> for Jwt {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .headers
            .get("Authorization")
            .and_then(|auth| UserClaim::verify(auth.to_str().unwrap()).ok())
            .map(|claim| Jwt(claim.user_id))
            .ok_or(StatusCode::UNAUTHORIZED.into_response())
    }
}
