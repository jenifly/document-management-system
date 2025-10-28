use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct OnlyOfficeEditorResponse {
    pub document: OnlyOfficeDocumentConfig,
    pub editor_config: OnlyOfficeEditorConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnlyOfficeDocumentConfig {
    #[serde(rename = "fileType")]
    pub file_type: String,
    pub key: String,
    pub title: String,
    pub url: String,
    pub permissions: OnlyOfficePermissions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnlyOfficeEditorConfig {
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    pub mode: String,
    pub user: OnlyOfficeUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnlyOfficePermissions {
    pub download: bool,
    pub edit: bool,
    pub print: bool,
    pub review: bool,
    pub comment: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnlyOfficeUser {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlyOfficeCallbackData {
    pub key: String,
    pub status: i32,
    pub url: Option<String>,
    pub users: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct OnlyOfficeService {
    config: Arc<crate::config::OnlyOfficeConfig>,
}

impl OnlyOfficeService {
    pub fn new(config: &crate::config::OnlyOfficeConfig) -> Self {
        Self {
            config: Arc::new(config.clone()),
        }
    }

    pub fn generate_editor_config(
        &self,
        document_id: Uuid,
        document_name: &str,
        file_type: &str,
        file_url: &str,
        user_id: Uuid,
        user_name: &str,
        can_edit: bool,
        app_url: &str,
    ) -> Result<OnlyOfficeEditorResponse> {
        let permissions = OnlyOfficePermissions {
            download: true,
            edit: can_edit,
            print: true,
            review: can_edit,
            comment: can_edit,
        };

        let document = OnlyOfficeDocumentConfig {
            file_type: file_type.to_string(),
            key: document_id.to_string(),
            title: document_name.to_string(),
            url: file_url.to_string(),
            permissions,
        };

        let editor_config = OnlyOfficeEditorConfig {
            callback_url: format!("{}/api/onlyoffice/callback/{}", app_url, document_id),
            mode: if can_edit { "edit".to_string() } else { "view".to_string() },
            user: OnlyOfficeUser {
                id: user_id.to_string(),
                name: user_name.to_string(),
            },
        };

        Ok(OnlyOfficeEditorResponse {
            document,
            editor_config,
        })
    }

    pub fn generate_jwt_token(&self, config: &OnlyOfficeEditorResponse) -> Result<String> {
        let token = encode(
            &Header::default(),
            config,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("Failed to generate OnlyOffice JWT: {}", e)))?;

        Ok(token)
    }

    pub fn get_file_type_from_extension(extension: &str) -> &str {
        match extension.to_lowercase().as_str() {
            "docx" | "doc" => "docx",
            "xlsx" | "xls" => "xlsx",
            "pptx" | "ppt" => "pptx",
            "txt" => "txt",
            "pdf" => "pdf",
            _ => "docx",
        }
    }
    
    pub fn get_document_type_from_extension(extension: &str) -> &str {
        match extension.to_lowercase().as_str() {
            "docx" | "doc" | "txt" => "word",
            "xlsx" | "xls" => "cell",
            "pptx" | "ppt" => "slide",
            "pdf" => "word", // PDF 当作只读文档处理
            _ => "word",
        }
    }

    pub fn can_edit_file(mime_type: &str) -> bool {
        matches!(
            mime_type,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                | "text/plain"
        )
    }

    pub async fn handle_callback(&self, callback_data: OnlyOfficeCallbackData) -> Result<CallbackResponse> {
        // Status codes:
        // 0 - Document is being edited
        // 1 - Document is being edited, but the current user has closed the document
        // 2 - Document is ready for saving
        // 3 - Document saving error
        // 4 - Document closed with no changes
        // 6 - Document is being edited, but the saving timeout has expired
        // 7 - Error has occurred while force saving

        match callback_data.status {
            2 | 6 => {
                // Document is ready for saving
                if let Some(url) = callback_data.url {
                    Ok(CallbackResponse {
                        error: 0,
                        message: "Document saved".to_string(),
                        download_url: Some(url),
                    })
                } else {
                    Err(AppError::BadRequest("Missing document URL in callback".to_string()))
                }
            }
            1 | 4 => {
                // Document closed, no action needed
                Ok(CallbackResponse {
                    error: 0,
                    message: "Document closed".to_string(),
                    download_url: None,
                })
            }
            _ => {
                // Other statuses
                Ok(CallbackResponse {
                    error: 0,
                    message: "Status received".to_string(),
                    download_url: None,
                })
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallbackResponse {
    pub error: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

