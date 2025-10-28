use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// PermissionType 枚举仅用于应用层类型安全，不直接映射到数据库
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PermissionType {
    Read,
    Write,
    Delete,
    Share,
    Admin,
}

impl PermissionType {
    pub fn as_str(&self) -> &str {
        match self {
            PermissionType::Read => "read",
            PermissionType::Write => "write",
            PermissionType::Delete => "delete",
            PermissionType::Share => "share",
            PermissionType::Admin => "admin",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "read" => Some(PermissionType::Read),
            "write" => Some(PermissionType::Write),
            "delete" => Some(PermissionType::Delete),
            "share" => Some(PermissionType::Share),
            "admin" => Some(PermissionType::Admin),
            _ => None,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable, Clone)]
#[diesel(table_name = crate::schema::document_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DocumentPermission {
    pub id: Uuid,
    pub document_id: Uuid,
    pub user_id: Uuid,
    #[serde(with = "permission_string")]
    pub permission: String,  // 在数据库层使用 String
    pub granted_by: Uuid,
    pub granted_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
}

// 辅助方法用于类型转换
impl DocumentPermission {
    pub fn permission_type(&self) -> Option<PermissionType> {
        PermissionType::from_str(&self.permission)
    }
}

// Serde 自定义序列化，对外仍然使用 PermissionType
mod permission_string {
    use super::PermissionType;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(perm_type) = PermissionType::from_str(value) {
            perm_type.serialize(serializer)
        } else {
            serializer.serialize_str(value)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let perm_type = PermissionType::deserialize(deserializer)?;
        Ok(perm_type.as_str().to_string())
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::document_permissions)]
pub struct NewDocumentPermission {
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub permission: String,  // 使用 String
    pub granted_by: Uuid,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = crate::schema::share_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShareLink {
    pub id: Uuid,
    pub document_id: Uuid,
    pub token: String,
    pub created_by: Uuid,
    #[serde(with = "permission_string")]
    pub permission: String,  // 使用 String
    pub password_hash: Option<String>,
    pub max_access_count: Option<i32>,
    pub access_count: i32,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl ShareLink {
    pub fn permission_type(&self) -> Option<PermissionType> {
        PermissionType::from_str(&self.permission)
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::share_links)]
pub struct NewShareLink {
    pub document_id: Uuid,
    pub token: String,
    pub created_by: Uuid,
    pub permission: String,  // 使用 String
    pub password_hash: Option<String>,
    pub max_access_count: Option<i32>,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrantPermissionRequest {
    pub user_id: Uuid,
    pub permission: PermissionType,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShareLinkRequest {
    pub permission: PermissionType,
    pub password: Option<String>,
    pub max_access_count: Option<i32>,
    pub expires_at: Option<NaiveDateTime>,
}

