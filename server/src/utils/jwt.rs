use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user_id
    pub username: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, username: String, role: String, expiration_seconds: i64) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::seconds(expiration_seconds)).timestamp();
        
        Claims {
            sub: user_id.to_string(),
            username,
            role,
            exp,
            iat: now.timestamp(),
        }
    }

    pub fn user_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.sub)
            .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))
    }
}

pub fn encode_jwt(claims: &Claims, secret: &str) -> Result<String> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(format!("Failed to encode JWT: {}", e)))
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    Ok(token_data.claims)
}

