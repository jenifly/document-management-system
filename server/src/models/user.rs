use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable, Validate)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    #[validate(length(min = 3, max = 100))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 100))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub full_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 100))]
    pub username: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            full_name: user.full_name,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Guest => "guest",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Some(UserRole::Admin),
            "user" => Some(UserRole::User),
            "guest" => Some(UserRole::Guest),
            _ => None,
        }
    }
}

