use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    db::AppState,
    error::Result,
    middleware::AuthUser,
    services::{SearchService, search::DocumentSearchIndex},
};

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub offset: usize,
    pub owner_id: Option<String>,
    pub mime_type: Option<String>,
    pub is_folder: Option<bool>,
}

fn default_limit() -> usize {
    50
}

pub async fn search_documents(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<DocumentSearchIndex>>> {
    let search_service = SearchService::new(&state.config.meilisearch)?;

    // Build filters
    let mut filters = Vec::new();
    
    if let Some(owner_id) = params.owner_id {
        filters.push(format!("owner_id = \"{}\"", owner_id));
    }
    
    if let Some(mime_type) = params.mime_type {
        filters.push(format!("mime_type = \"{}\"", mime_type));
    }
    
    if let Some(is_folder) = params.is_folder {
        filters.push(format!("is_folder = {}", is_folder));
    }

    let filter_str = if filters.is_empty() {
        None
    } else {
        Some(filters.join(" AND "))
    };

    let results = search_service
        .search(&params.q, filter_str, Some(params.limit), Some(params.offset))
        .await?;

    Ok(Json(results.hits.into_iter().map(|hit| hit.result).collect()))
}

