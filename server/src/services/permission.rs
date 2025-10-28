use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbConnection;
use crate::error::{AppError, Result};
use crate::models::permission::{DocumentPermission, PermissionType};
use crate::schema::{document_permissions, documents, group_members, group_permissions};

pub struct PermissionService;

impl PermissionService {
    /// Check if a user has a specific permission on a document
    pub fn check_permission(
        conn: &mut DbConnection,
        user_id: Uuid,
        document_id: Uuid,
        required_permission: PermissionType,
    ) -> Result<bool> {
        // First check if user is the owner
        let is_owner: bool = diesel::select(diesel::dsl::exists(
            documents::table
                .filter(documents::id.eq(document_id))
                .filter(documents::owner_id.eq(user_id))
        ))
        .get_result::<bool>(conn)?;

        if is_owner {
            return Ok(true);
        }

        // Check direct permissions
        let permission_str = required_permission.as_str();
        let has_direct_permission = diesel::select(diesel::dsl::exists(
            document_permissions::table
                .filter(document_permissions::document_id.eq(document_id))
                .filter(document_permissions::user_id.eq(user_id))
                .filter(document_permissions::permission.eq(permission_str))
                .filter(
                    document_permissions::expires_at
                        .is_null()
                        .or(document_permissions::expires_at.gt(diesel::dsl::now)),
                )
        ))
        .get_result::<bool>(conn)?;

        if has_direct_permission {
            return Ok(true);
        }

        // Check group permissions
        let has_group_permission = diesel::select(diesel::dsl::exists(
            group_permissions::table
                .filter(group_permissions::document_id.eq(document_id))
                .filter(group_permissions::permission.eq(permission_str))
                .filter(
                    group_permissions::expires_at
                        .is_null()
                        .or(group_permissions::expires_at.gt(diesel::dsl::now)),
                )
                .inner_join(group_members::table.on(group_permissions::group_id.eq(group_members::group_id)))
                .filter(group_members::user_id.eq(user_id))
        ))
        .get_result::<bool>(conn)?;

        Ok(has_group_permission)
    }

    /// Check if user has admin permission (owner or admin permission)
    pub fn check_admin_permission(
        conn: &mut DbConnection,
        user_id: Uuid,
        document_id: Uuid,
    ) -> Result<bool> {
        Self::check_permission(conn, user_id, document_id, PermissionType::Admin)
    }

    /// Get all permissions for a user on a document
    pub fn get_user_permissions(
        conn: &mut DbConnection,
        usr_id: Uuid,
        doc_id: Uuid,
    ) -> Result<Vec<PermissionType>> {
        use crate::schema::document_permissions::dsl::*;

        let perms: Vec<DocumentPermission> = document_permissions
            .filter(document_id.eq(doc_id))
            .filter(user_id.eq(usr_id))
            .filter(
                expires_at
                    .is_null()
                    .or(expires_at.gt(diesel::dsl::now)),
            )
            .select(DocumentPermission::as_select())
            .load::<DocumentPermission>(conn)?;

        Ok(perms
            .into_iter()
            .filter_map(|p| p.permission_type())
            .collect())
    }

    /// Grant permission to a user
    pub fn grant_permission(
        conn: &mut DbConnection,
        doc_id: Uuid,
        usr_id: Uuid,
        perm: PermissionType,
        granter_id: Uuid,
        exp_at: Option<chrono::NaiveDateTime>,
    ) -> Result<DocumentPermission> {
        use crate::schema::document_permissions::dsl::*;

        diesel::insert_into(document_permissions)
            .values((
                document_id.eq(doc_id),
                user_id.eq(usr_id),
                permission.eq(perm.as_str()),
                granted_by.eq(granter_id),
                expires_at.eq(exp_at),
            ))
            .returning(DocumentPermission::as_returning())
            .get_result::<DocumentPermission>(conn)
            .map_err(Into::into)
    }

    /// Revoke permission from a user
    pub fn revoke_permission(
        conn: &mut DbConnection,
        doc_id: Uuid,
        usr_id: Uuid,
        perm: PermissionType,
    ) -> Result<()> {
        use crate::schema::document_permissions::dsl::*;

        let perm_str = perm.as_str();
        diesel::delete(
            document_permissions
                .filter(document_id.eq(doc_id))
                .filter(user_id.eq(usr_id))
                .filter(permission.eq(perm_str)),
        )
        .execute(conn)?;

        Ok(())
    }

    /// Check if user can perform an action based on permission hierarchy
    pub fn can_perform_action(
        conn: &mut DbConnection,
        user_id: Uuid,
        document_id: Uuid,
        action: &str,
    ) -> Result<bool> {
        let required_permission = match action {
            "read" | "view" | "download" => PermissionType::Read,
            "edit" | "update" | "upload" => PermissionType::Write,
            "delete" => PermissionType::Delete,
            "share" => PermissionType::Share,
            "admin" | "permissions" => PermissionType::Admin,
            _ => return Err(AppError::BadRequest(format!("Unknown action: {}", action))),
        };

        Self::check_permission(conn, user_id, document_id, required_permission)
    }
}
