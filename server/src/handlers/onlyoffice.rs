use axum::{
    extract::{Path, State},
    Json,
};
use diesel::prelude::*;
use jsonwebtoken::encode;
use uuid::Uuid;

use crate::{
    db::AppState,
    error::{AppError, Result},
    middleware::AuthUser,
    models::{document::Document, permission::PermissionType},
    schema::documents,
    services::{OnlyOfficeService, PermissionService, StorageService, onlyoffice::{OnlyOfficeCallbackData, CallbackResponse}},
};

pub async fn get_editor_config(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(document_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    let mut conn = state.get_connection()?;
    let user_id = auth_user.claims.user_id()?;

    // Get document
    let document = documents::table
        .find(document_id)
        .select(Document::as_select())
        .first::<Document>(&mut conn)?;

    if document.is_folder {
        return Err(AppError::BadRequest("Cannot edit a folder".to_string()));
    }

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

    // Check write permission
    let can_edit = PermissionService::check_permission(
        &mut conn,
        user_id,
        document_id,
        PermissionType::Write,
    )?;

    // Get presigned URL for the document (使用 OnlyOffice 可访问的 URL)
    let storage_service = StorageService::new(&state.config.minio)?;
    let file_url = storage_service.get_file_url_for_onlyoffice(&document.file_path, 3600).await?;
    tracing::info!("Generated OnlyOffice file URL: {}", file_url);

    // Get file extension
    let file_extension = std::path::Path::new(&document.name)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("docx");

    let file_type = OnlyOfficeService::get_file_type_from_extension(file_extension);
    let document_type = OnlyOfficeService::get_document_type_from_extension(file_extension);

    // Generate OnlyOffice config
    let onlyoffice_service = OnlyOfficeService::new(&state.config.onlyoffice);
    let editor_config = onlyoffice_service.generate_editor_config(
        document_id,
        &document.name,
        file_type,
        &file_url,
        user_id,
        &auth_user.claims.username,
        can_edit,
        &state.config.app.url,
    )?;

    // 构建完整的配置对象（传递给 DocEditor 的对象）
    let full_config = serde_json::json!({
        "document": editor_config.document,
        "documentType": document_type,
        "editorConfig": editor_config.editor_config,
    });

    // 使用完整配置生成 JWT token
    let token = encode(
        &jsonwebtoken::Header::default(),
        &full_config,
        &jsonwebtoken::EncodingKey::from_secret(state.config.onlyoffice.jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(format!("Failed to generate OnlyOffice JWT: {}", e)))?;

    Ok(Json(serde_json::json!({
        "config": {
            "document": editor_config.document,
            "documentType": document_type,
            "editorConfig": editor_config.editor_config,
            "token": token,
        },
        "onlyoffice_server": state.config.onlyoffice.server,
    })))
}

pub async fn onlyoffice_callback(
    State(state): State<AppState>,
    Path(document_id): Path<Uuid>,
    Json(callback_data): Json<OnlyOfficeCallbackData>,
) -> Result<Json<CallbackResponse>> {
    let mut conn = state.get_connection()?;

    // Verify document exists
    let document = documents::table
        .find(document_id)
        .select(Document::as_select())
        .first::<Document>(&mut conn)
        .map_err(|_| AppError::NotFound("Document not found".to_string()))?;

    let onlyoffice_service = OnlyOfficeService::new(&state.config.onlyoffice);
    let response = onlyoffice_service.handle_callback(callback_data.clone()).await?;

    // If document is ready for saving (status 2 or 6)
    if matches!(callback_data.status, 2 | 6) {
        if let Some(download_url) = &response.download_url {
            // Download the updated file from OnlyOffice
            let client = reqwest::Client::new();
            let file_data = client
                .get(download_url)
                .send()
                .await
                .map_err(|e| AppError::InternalServerError(format!("Failed to download updated file: {}", e)))?
                .bytes()
                .await
                .map_err(|e| AppError::InternalServerError(format!("Failed to read file data: {}", e)))?;

            // Upload new version to MinIO
            let storage_service = StorageService::new(&state.config.minio)?;
            let new_file_path = storage_service
                .upload_file(&file_data, &document.name, &document.mime_type)
                .await?;

            // Create new version record
            use crate::models::document::NewDocumentVersion;
            use crate::schema::document_versions;

            let new_version = NewDocumentVersion {
                document_id,
                version: document.version + 1,
                file_path: new_file_path.clone(),
                file_size: file_data.len() as i64,
                comment: Some("Updated via OnlyOffice".to_string()),
                created_by: document.owner_id, // TODO: Get actual editor user_id
            };

            diesel::insert_into(document_versions::table)
                .values(&new_version)
                .execute(&mut conn)?;

            // Update document with new version
            diesel::update(documents::table.find(document_id))
                .set((
                    documents::file_path.eq(&new_file_path),
                    documents::file_size.eq(file_data.len() as i64),
                    documents::version.eq(document.version + 1),
                    documents::updated_at.eq(diesel::dsl::now),
                ))
                .execute(&mut conn)?;

            // Delete old file from MinIO
            let _ = storage_service.delete_file(&document.file_path).await;
        }
    }

    Ok(Json(response))
}

