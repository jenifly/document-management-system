use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable, Clone)]
#[diesel(table_name = crate::schema::documents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Document {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub version: i32,
    pub status: String,
    pub owner_id: Uuid,
    pub parent_folder_id: Option<Uuid>,
    pub is_folder: bool,
    pub tags: Option<Vec<Option<String>>>,
    pub metadata: Option<JsonValue>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::documents)]
pub struct NewDocument {
    pub name: String,
    pub description: Option<String>,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub owner_id: Uuid,
    pub parent_folder_id: Option<Uuid>,
    pub is_folder: bool,
    pub tags: Option<Vec<Option<String>>>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateDocumentRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    pub parent_folder_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateDocumentRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateFolderRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    pub parent_folder_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = crate::schema::document_versions)]
pub struct DocumentVersion {
    pub id: Uuid,
    pub document_id: Uuid,
    pub version: i32,
    pub file_path: String,
    pub file_size: i64,
    pub comment: Option<String>,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::document_versions)]
pub struct NewDocumentVersion {
    pub document_id: Uuid,
    pub version: i32,
    pub file_path: String,
    pub file_size: i64,
    pub comment: Option<String>,
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentSearchResult {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: String,
    pub owner_id: Uuid,
    pub is_folder: bool,
    pub tags: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub filters: Option<SearchFilters>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchFilters {
    pub mime_type: Option<String>,
    pub owner_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub is_folder: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveDocumentRequest {
    pub target_folder_id: Option<Uuid>,
}

