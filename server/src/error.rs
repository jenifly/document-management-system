use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    NotFound(String),
    Unauthorized(String),
    Forbidden(String),
    BadRequest(String),
    InternalServerError(String),
    ValidationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

impl From<diesel::r2d2::PoolError> for AppError {
    fn from(err: diesel::r2d2::PoolError) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

