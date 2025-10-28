use axum::{
    extract::{Multipart, Path, Query, State},
    Json,
};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::AppState,
    error::{AppError, Result},
    middleware::AuthUser,
    models::document::{
        CreateDocumentRequest, CreateFolderRequest, Document, MoveDocumentRequest, 
        NewDocument, NewDocumentVersion, UpdateDocumentRequest,
    },
    schema::{documents, document_versions},
    services::{PermissionService, StorageService, SearchService},
    models::permission::PermissionType,
};

#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

#[derive(Deserialize)]
pub struct ListDocumentsParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
    pub folder_id: Option<Uuid>,
}

fn default_limit() -> i64 {
    50
}

pub async fn create_folder(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateFolderRequest>,
) -> Result<Json<Document>> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    // If parent folder is specified, check permissions
    if let Some(parent_id) = payload.parent_folder_id {
        let can_write = PermissionService::check_permission(
            &mut conn,
            user_id,
            parent_id,
            PermissionType::Write,
        )?;

        if !can_write {
            return Err(AppError::Forbidden("No permission to create folder here".to_string()));
        }
    }

    let new_folder = NewDocument {
        name: payload.name,
        description: payload.description,
        file_path: String::new(),
        file_size: 0,
        mime_type: "inode/directory".to_string(),
        owner_id: user_id,
        parent_folder_id: payload.parent_folder_id,
        is_folder: true,
        tags: None,
        metadata: None,
    };

    let folder = diesel::insert_into(documents::table)
        .values(&new_folder)
        .returning(Document::as_returning())
        .get_result(&mut conn)?;

    // Index in search
    let search_service = SearchService::new(&state.config.meilisearch)?;
    search_service.index_document(folder.clone()).await?;

    Ok(Json(folder))
}

pub async fn upload_document(
    State(state): State<AppState>,
    auth_user: AuthUser,
    mut multipart: Multipart,
) -> Result<Json<Document>> {
    let user_id = auth_user.claims.user_id()?;
    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;
    let mut content_type: Option<String> = None;
    let mut parent_folder_id: Option<Uuid> = None;
    let mut description: Option<String> = None;
    let mut tags: Option<Vec<String>> = None;

    // Parse multipart form data
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::BadRequest(format!("Failed to parse multipart: {}", e))
    })? {
        let field_name = field.name().unwrap_or("").to_string();
        
        match field_name.as_str() {
            "file" => {
                file_name = field.file_name().map(|s| s.to_string());
                content_type = field.content_type().map(|s| s.to_string());
                file_data = Some(field.bytes().await.map_err(|e| {
                    AppError::BadRequest(format!("Failed to read file: {}", e))
                })?.to_vec());
            }
            "parent_folder_id" => {
                let value = field.text().await.map_err(|e| {
                    AppError::BadRequest(format!("Failed to read parent_folder_id: {}", e))
                })?;
                parent_folder_id = Uuid::parse_str(&value).ok();
            }
            "description" => {
                description = Some(field.text().await.map_err(|e| {
                    AppError::BadRequest(format!("Failed to read description: {}", e))
                })?);
            }
            "tags" => {
                let value = field.text().await.map_err(|e| {
                    AppError::BadRequest(format!("Failed to read tags: {}", e))
                })?;
                tags = Some(value.split(',').map(|s| s.trim().to_string()).collect());
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or_else(|| AppError::BadRequest("No file provided".to_string()))?;
    let file_name = file_name.ok_or_else(|| AppError::BadRequest("No filename provided".to_string()))?;
    let content_type = content_type.unwrap_or_else(|| "application/octet-stream".to_string());

    let mut conn = state.get_connection()?;

    // Check parent folder permissions
    if let Some(parent_id) = parent_folder_id {
        let can_write = PermissionService::check_permission(
            &mut conn,
            user_id,
            parent_id,
            PermissionType::Write,
        )?;

        if !can_write {
            return Err(AppError::Forbidden("No permission to upload here".to_string()));
        }
    }

    // Upload to MinIO
    let storage_service = StorageService::new(&state.config.minio)?;
    let file_path = storage_service.upload_file(&file_data, &file_name, &content_type).await?;

    // Create document record
    let new_document = NewDocument {
        name: file_name,
        description,
        file_path,
        file_size: file_data.len() as i64,
        mime_type: content_type,
        owner_id: user_id,
        parent_folder_id,
        is_folder: false,
        tags: tags.map(|t| t.into_iter().map(Some).collect()),
        metadata: None,
    };

    let document = diesel::insert_into(documents::table)
        .values(&new_document)
        .returning(Document::as_returning())
        .get_result(&mut conn)?;

    // Index in search
    let search_service = SearchService::new(&state.config.meilisearch)?;
    search_service.index_document(document.clone()).await?;

    Ok(Json(document))
}

pub async fn get_document(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(document_id): Path<Uuid>,
) -> Result<Json<Document>> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    let document = documents::table
        .find(document_id)
        .select(Document::as_select())
        .first(&mut conn)?;

    // Check read permission
    let can_read = PermissionService::check_permission(
        &mut conn,
        user_id,
        document_id,
        PermissionType::Read,
    )?;

    if !can_read {
        return Err(AppError::Forbidden("No permission to view this document".to_string()));
    }

    Ok(Json(document))
}

pub async fn list_documents(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<ListDocumentsParams>,
) -> Result<Json<Vec<Document>>> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    let mut query = documents::table
        .filter(documents::owner_id.eq(user_id))
        .filter(documents::deleted_at.is_null())
        .select(Document::as_select())
        .into_boxed();

    if let Some(folder_id) = params.folder_id {
        query = query.filter(documents::parent_folder_id.eq(folder_id));
    } else {
        query = query.filter(documents::parent_folder_id.is_null());
    }

    let documents = query
        .order(documents::created_at.desc())
        .limit(params.limit)
        .offset(params.offset)
        .load(&mut conn)?;

    Ok(Json(documents))
}

pub async fn update_document(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(document_id): Path<Uuid>,
    Json(payload): Json<UpdateDocumentRequest>,
) -> Result<Json<Document>> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    // Check write permission
    let can_write = PermissionService::check_permission(
        &mut conn,
        user_id,
        document_id,
        PermissionType::Write,
    )?;

    if !can_write {
        return Err(AppError::Forbidden("No permission to update this document".to_string()));
    }

    // Build update changeset
    use crate::schema::documents::dsl::*;
    
    // Check if there are any fields to update
    if payload.name.is_none() && payload.description.is_none() && payload.tags.is_none() {
        return Err(AppError::BadRequest("No fields to update".to_string()));
    }

    let document = match (payload.name, payload.description, payload.tags) {
        (Some(new_name), Some(new_description), Some(new_tags)) => {
            let tags_opt: Vec<Option<String>> = new_tags.into_iter().map(Some).collect();
            diesel::update(documents.find(document_id))
                .set((
                    name.eq(new_name),
                    description.eq(Some(new_description)),
                    tags.eq(Some(tags_opt)),
                    updated_at.eq(diesel::dsl::now),
                ))
                .returning(Document::as_returning())
                .get_result::<Document>(&mut conn)?
        }
        (Some(new_name), Some(new_description), None) => {
            diesel::update(documents.find(document_id))
                .set((
                    name.eq(new_name),
                    description.eq(Some(new_description)),
                    updated_at.eq(diesel::dsl::now),
                ))
                .returning(Document::as_returning())
                .get_result::<Document>(&mut conn)?
        }
        (Some(new_name), None, Some(new_tags)) => {
            let tags_opt: Vec<Option<String>> = new_tags.into_iter().map(Some).collect();
            diesel::update(documents.find(document_id))
                .set((
                    name.eq(new_name),
                    tags.eq(Some(tags_opt)),
                    updated_at.eq(diesel::dsl::now),
                ))
                .returning(Document::as_returning())
                .get_result::<Document>(&mut conn)?
        }
        (None, Some(new_description), Some(new_tags)) => {
            let tags_opt: Vec<Option<String>> = new_tags.into_iter().map(Some).collect();
            diesel::update(documents.find(document_id))
                .set((
                    description.eq(Some(new_description)),
                    tags.eq(Some(tags_opt)),
                    updated_at.eq(diesel::dsl::now),
                ))
                .returning(Document::as_returning())
                .get_result::<Document>(&mut conn)?
        }
        (Some(new_name), None, None) => {
            diesel::update(documents.find(document_id))
                .set((
                    name.eq(new_name),
                    updated_at.eq(diesel::dsl::now),
                ))
                .returning(Document::as_returning())
                .get_result::<Document>(&mut conn)?
        }
        (None, Some(new_description), None) => {
            diesel::update(documents.find(document_id))
                .set((
                    description.eq(Some(new_description)),
                    updated_at.eq(diesel::dsl::now),
                ))
                .returning(Document::as_returning())
                .get_result::<Document>(&mut conn)?
        }
        (None, None, Some(new_tags)) => {
            let tags_opt: Vec<Option<String>> = new_tags.into_iter().map(Some).collect();
            diesel::update(documents.find(document_id))
                .set((
                    tags.eq(Some(tags_opt)),
                    updated_at.eq(diesel::dsl::now),
                ))
                .returning(Document::as_returning())
                .get_result::<Document>(&mut conn)?
        }
        (None, None, None) => {
            // Already checked above
            unreachable!()
        }
    };

    // Update search index
    let search_service = SearchService::new(&state.config.meilisearch)?;
    search_service.update_document(document.clone()).await?;

    Ok(Json(document))
}

pub async fn delete_document(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(document_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    // Check delete permission
    let can_delete = PermissionService::check_permission(
        &mut conn,
        user_id,
        document_id,
        PermissionType::Delete,
    )?;

    if !can_delete {
        return Err(AppError::Forbidden("No permission to delete this document".to_string()));
    }

    let document = documents::table
        .find(document_id)
        .select(Document::as_select())
        .first(&mut conn)?;

    // Soft delete
    diesel::update(documents::table.find(document_id))
        .set(documents::deleted_at.eq(diesel::dsl::now))
        .execute(&mut conn)?;

    // Remove from search index
    let search_service = SearchService::new(&state.config.meilisearch)?;
    search_service.delete_document(document_id).await?;

    // Delete from MinIO if not a folder
    if !document.is_folder {
        let storage_service = StorageService::new(&state.config.minio)?;
        storage_service.delete_file(&document.file_path).await?;
    }

    Ok(Json(serde_json::json!({
        "message": "Document deleted successfully"
    })))
}

pub async fn download_document(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(document_id): Path<Uuid>,
) -> Result<String> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    let document = documents::table
        .find(document_id)
        .select(Document::as_select())
        .first(&mut conn)?;

    // Check read permission
    let can_read = PermissionService::check_permission(
        &mut conn,
        user_id,
        document_id,
        PermissionType::Read,
    )?;

    if !can_read {
        return Err(AppError::Forbidden("No permission to download this document".to_string()));
    }

    if document.is_folder {
        return Err(AppError::BadRequest("Cannot download a folder".to_string()));
    }

    // Generate presigned URL
    let storage_service = StorageService::new(&state.config.minio)?;
    let url = storage_service.get_file_url(&document.file_path, 3600).await?;

    Ok(url)
}

pub async fn move_document(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(document_id): Path<Uuid>,
    Json(payload): Json<MoveDocumentRequest>,
) -> Result<Json<Document>> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    // Check write permission on the document
    let can_write = PermissionService::check_permission(
        &mut conn,
        user_id,
        document_id,
        PermissionType::Write,
    )?;

    if !can_write {
        return Err(AppError::Forbidden("No permission to move this document".to_string()));
    }

    // Check write permission on target folder
    if let Some(target_id) = payload.target_folder_id {
        let can_write_target = PermissionService::check_permission(
            &mut conn,
            user_id,
            target_id,
            PermissionType::Write,
        )?;

        if !can_write_target {
            return Err(AppError::Forbidden("No permission to move to target folder".to_string()));
        }
    }

    let document = diesel::update(documents::table.find(document_id))
        .set(documents::parent_folder_id.eq(payload.target_folder_id))
        .returning(Document::as_returning())
        .get_result::<Document>(&mut conn)?;

    Ok(Json(document))
}
