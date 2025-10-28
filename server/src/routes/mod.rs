use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{db::AppState, handlers};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // Auth routes
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/me", get(handlers::get_current_user))
        // Document routes
        .route("/api/documents", get(handlers::list_documents))
        .route("/api/documents/upload", post(handlers::upload_document))
        .route("/api/documents/:id", get(handlers::get_document))
        .route("/api/documents/:id", put(handlers::update_document))
        .route("/api/documents/:id", delete(handlers::delete_document))
        .route("/api/documents/:id/download", get(handlers::download_document))
        .route("/api/documents/:id/move", post(handlers::move_document))
        // Folder routes
        .route("/api/folders", post(handlers::create_folder))
        // Permission routes
        .route("/api/documents/:id/permissions", get(handlers::list_permissions))
        .route("/api/documents/:id/permissions", post(handlers::grant_permission))
        .route(
            "/api/documents/:document_id/permissions/:user_id/:permission",
            delete(handlers::revoke_permission),
        )
        // Share link routes
        .route("/api/documents/:id/shares", post(handlers::create_share_link))
        .route("/api/documents/:id/shares", get(handlers::list_share_links))
        .route("/api/documents/:document_id/shares/:share_id", delete(handlers::delete_share_link))
        .route("/api/shares/access/:token", get(handlers::get_share_link))
        // Search routes
        .route("/api/search", get(handlers::search_documents))
        // OnlyOffice routes
        .route("/api/onlyoffice/:id/config", get(handlers::get_editor_config))
        .route("/api/onlyoffice/callback/:id", post(handlers::onlyoffice_callback))
}

async fn health_check() -> &'static str {
    "OK"
}

