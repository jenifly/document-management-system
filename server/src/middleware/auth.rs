use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{header, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::{
    db::AppState,
    error::AppError,
    utils::jwt::{decode_jwt, Claims},
};

pub struct AuthUser {
    pub claims: Claims,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the Authorization header
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Missing authorization header"})),
                )
                    .into_response()
            })?;

        // Check if it's a Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Invalid authorization header format"})),
                )
                    .into_response()
            })?;

        // Decode the JWT token
        let claims = decode_jwt(token, &state.config.jwt.secret).map_err(|e| {
            let error_msg = format!("{}", e);
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": error_msg})),
            )
                .into_response()
        })?;

        Ok(AuthUser { claims })
    }
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    auth_header: Option<String>,
) -> Result<Claims, AppError> {
    let auth_value = auth_header
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;
    
    let token = auth_value
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization header format".to_string()))?;

    decode_jwt(token, &state.config.jwt.secret)
}

