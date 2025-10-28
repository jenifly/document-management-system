use axum::{extract::State, Json};
use diesel::prelude::*;
use validator::Validate;

use crate::{
    db::AppState,
    error::{AppError, Result},
    models::user::{CreateUserRequest, LoginRequest, LoginResponse, NewUser, User, UserResponse, UserRole},
    schema::users,
    utils::{hash_password, verify_password, Claims, encode_jwt},
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let mut conn = state.get_connection()?;

    // Check if username or email already exists
    let existing_user = users::table
        .filter(
            users::username.eq(&payload.username)
                .or(users::email.eq(&payload.email))
        )
        .first::<User>(&mut conn)
        .optional()?;

    if existing_user.is_some() {
        return Err(AppError::BadRequest("Username or email already exists".to_string()));
    }

    // Hash password
    let password_hash = hash_password(&payload.password)?;

    // Create new user
    let new_user = NewUser {
        username: payload.username,
        email: payload.email,
        password_hash,
        full_name: payload.full_name,
        role: UserRole::User.as_str().to_string(),
    };

    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn)?;

    Ok(Json(user.into()))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    // 验证输入
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    tracing::debug!("Login attempt for username: {}", payload.username);
    
    let mut conn = state.get_connection()?;

    // Find user by username
    let user = users::table
        .filter(users::username.eq(&payload.username))
        .first::<User>(&mut conn)
        .map_err(|_| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Check if user is active
    if !user.is_active {
        return Err(AppError::Unauthorized("Account is not active".to_string()));
    }

    // Verify password
    let is_valid = verify_password(&payload.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Generate JWT token
    let claims = Claims::new(
        user.id,
        user.username.clone(),
        user.role.clone(),
        state.config.jwt.expiration,
    );

    let token = encode_jwt(&claims, &state.config.jwt.secret)?;

    Ok(Json(LoginResponse {
        token,
        user: user.into(),
    }))
}

pub async fn get_current_user(
    State(state): State<AppState>,
    user: crate::middleware::AuthUser,
) -> Result<Json<UserResponse>> {
    let mut conn = state.get_connection()?;
    let user_id = user.claims.user_id()?;

    let user = users::table
        .find(user_id)
        .first::<User>(&mut conn)?;

    Ok(Json(user.into()))
}

