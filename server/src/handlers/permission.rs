use axum::{
    extract::{Path, State},
    Json,
};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    db::AppState,
    error::{AppError, Result},
    middleware::AuthUser,
    models::permission::{
        CreateShareLinkRequest, DocumentPermission, GrantPermissionRequest, 
        NewShareLink, PermissionType, ShareLink,
    },
    schema::{document_permissions, share_links},
    services::PermissionService,
    utils::hash_password,
};

pub async fn grant_permission(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(document_id): Path<Uuid>,
    Json(payload): Json<GrantPermissionRequest>,
) -> Result<Json<DocumentPermission>> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    // Check if requester has admin permission
    let has_admin = PermissionService::check_permission(
        &mut conn,
        user_id,
        document_id,
        PermissionType::Admin,
    )?;

    if !has_admin {
        return Err(AppError::Forbidden("No permission to grant permissions".to_string()));
    }

    let permission = PermissionService::grant_permission(
        &mut conn,
        document_id,
        payload.user_id,
        payload.permission,
        user_id,
        payload.expires_at,
    )?;

    Ok(Json(permission))
}

pub async fn revoke_permission(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((document_id, target_user_id, permission)): Path<(Uuid, Uuid, String)>,
) -> Result<Json<serde_json::Value>> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    // Check if requester has admin permission
    let has_admin = PermissionService::check_permission(
        &mut conn,
        user_id,
        document_id,
        PermissionType::Admin,
    )?;

    if !has_admin {
        return Err(AppError::Forbidden("No permission to revoke permissions".to_string()));
    }

    let perm_type = PermissionType::from_str(&permission)
        .ok_or_else(|| AppError::BadRequest("Invalid permission type".to_string()))?;

    PermissionService::revoke_permission(&mut conn, document_id, target_user_id, perm_type)?;

    Ok(Json(serde_json::json!({
        "message": "Permission revoked successfully"
    })))
}

pub async fn list_permissions(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(document_id): Path<Uuid>,
) -> Result<Json<Vec<DocumentPermission>>> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    // Check if requester has admin permission
    let has_admin = PermissionService::check_permission(
        &mut conn,
        user_id,
        document_id,
        PermissionType::Admin,
    )?;

    if !has_admin {
        return Err(AppError::Forbidden("No permission to list permissions".to_string()));
    }

    let permissions = document_permissions::table
        .filter(document_permissions::document_id.eq(document_id))
        .load::<DocumentPermission>(&mut conn)?;

    Ok(Json(permissions))
}

pub async fn create_share_link(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(document_id): Path<Uuid>,
    Json(payload): Json<CreateShareLinkRequest>,
) -> Result<Json<ShareLink>> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    // Check if requester has share permission
    let can_share = PermissionService::check_permission(
        &mut conn,
        user_id,
        document_id,
        PermissionType::Share,
    )?;

    if !can_share {
        return Err(AppError::Forbidden("No permission to share this document".to_string()));
    }

    let token = Uuid::new_v4().to_string();
    let password_hash = if let Some(password) = payload.password {
        Some(hash_password(&password)?)
    } else {
        None
    };

    let new_link = NewShareLink {
        document_id,
        token,
        created_by: user_id,
        permission: payload.permission.as_str().to_string(),
        password_hash,
        max_access_count: payload.max_access_count,
        expires_at: payload.expires_at,
    };

    let link = diesel::insert_into(share_links::table)
        .values(&new_link)
        .get_result::<ShareLink>(&mut conn)?;

    Ok(Json(link))
}

pub async fn get_share_link(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<Json<ShareLink>> {
    let mut conn = state.get_connection()?;

    let link = share_links::table
        .filter(share_links::token.eq(&token))
        .first::<ShareLink>(&mut conn)?;

    // Check if link is expired
    if let Some(expires_at) = link.expires_at {
        if expires_at < chrono::Local::now().naive_local() {
            return Err(AppError::BadRequest("Share link has expired".to_string()));
        }
    }

    // Check max access count
    if let Some(max_count) = link.max_access_count {
        if link.access_count >= max_count {
            return Err(AppError::BadRequest("Share link access limit reached".to_string()));
        }
    }

    // Increment access count
    diesel::update(share_links::table.filter(share_links::token.eq(&token)))
        .set(share_links::access_count.eq(link.access_count + 1))
        .execute(&mut conn)?;

    Ok(Json(link))
}

pub async fn list_share_links(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(document_id): Path<Uuid>,
) -> Result<Json<Vec<ShareLink>>> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    // Check if requester has share permission
    let can_share = PermissionService::check_permission(
        &mut conn,
        user_id,
        document_id,
        PermissionType::Share,
    )?;

    if !can_share {
        return Err(AppError::Forbidden("No permission to view share links".to_string()));
    }

    let links = share_links::table
        .filter(share_links::document_id.eq(document_id))
        .load::<ShareLink>(&mut conn)?;

    Ok(Json(links))
}

pub async fn delete_share_link(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((document_id, share_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    let link = share_links::table
        .find(share_id)
        .first::<ShareLink>(&mut conn)?;

    // Verify the share link belongs to the specified document
    if link.document_id != document_id {
        return Err(AppError::NotFound("Share link not found for this document".to_string()));
    }

    // Check if requester created the link or has admin permission
    if link.created_by != user_id {
        let has_admin = PermissionService::check_permission(
            &mut conn,
            user_id,
            link.document_id,
            PermissionType::Admin,
        )?;

        if !has_admin {
            return Err(AppError::Forbidden("No permission to delete this share link".to_string()));
        }
    }

    diesel::delete(share_links::table.find(share_id))
        .execute(&mut conn)?;

    Ok(Json(serde_json::json!({
        "message": "Share link deleted successfully"
    })))
}

