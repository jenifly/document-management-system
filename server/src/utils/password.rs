use bcrypt::{hash, verify, DEFAULT_COST};
use crate::error::{AppError, Result};

pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST)
        .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    verify(password, hash)
        .map_err(|e| AppError::InternalServerError(format!("Failed to verify password: {}", e)))
}

