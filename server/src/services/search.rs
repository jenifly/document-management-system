use meilisearch_sdk::{client::Client, indexes::Index, search::SearchResults};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::config::MeilisearchConfig;
use crate::error::{AppError, Result};
use crate::models::document::Document;

const DOCUMENTS_INDEX: &str = "documents";

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentSearchIndex {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: String,
    pub owner_id: String,
    pub is_folder: bool,
    pub tags: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Document> for DocumentSearchIndex {
    fn from(doc: Document) -> Self {
        DocumentSearchIndex {
            id: doc.id.to_string(),
            name: doc.name,
            description: doc.description,
            mime_type: doc.mime_type,
            owner_id: doc.owner_id.to_string(),
            is_folder: doc.is_folder,
            tags: doc.tags
                .unwrap_or_default()
                .into_iter()
                .filter_map(|t| t)
                .collect(),
            created_at: doc.created_at.and_utc().timestamp(),
            updated_at: doc.updated_at.and_utc().timestamp(),
        }
    }
}

#[derive(Clone)]
pub struct SearchService {
    client: Arc<Client>,
}

impl SearchService {
    pub fn new(config: &MeilisearchConfig) -> Result<Self> {
        let client = Client::new(&config.host, Some(&config.api_key))
            .map_err(|e| AppError::InternalServerError(format!("Failed to create MeiliSearch client: {}", e)))?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub async fn initialize_index(&self) -> Result<()> {
        // Create index if it doesn't exist
        let _ = self.client
            .create_index(DOCUMENTS_INDEX, Some("id"))
            .await;

        // Configure searchable attributes
        let index = self.client.index(DOCUMENTS_INDEX);
        
        index
            .set_searchable_attributes(&["name", "description", "tags"])
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to set searchable attributes: {}", e)))?;

        // Configure filterable attributes
        index
            .set_filterable_attributes(&["owner_id", "mime_type", "is_folder", "tags"])
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to set filterable attributes: {}", e)))?;

        // Configure sortable attributes
        index
            .set_sortable_attributes(&["created_at", "updated_at", "name"])
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to set sortable attributes: {}", e)))?;

        Ok(())
    }

    pub async fn index_document(&self, document: Document) -> Result<()> {
        let index = self.client.index(DOCUMENTS_INDEX);
        let doc_index = DocumentSearchIndex::from(document);
        
        index
            .add_documents(&[doc_index], Some("id"))
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to index document: {}", e)))?;

        Ok(())
    }

    pub async fn update_document(&self, document: Document) -> Result<()> {
        self.index_document(document).await
    }

    pub async fn delete_document(&self, document_id: Uuid) -> Result<()> {
        let index = self.client.index(DOCUMENTS_INDEX);
        
        index
            .delete_document(&document_id.to_string())
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to delete document from index: {}", e)))?;

        Ok(())
    }

    pub async fn search(
        &self,
        query: &str,
        filters: Option<String>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<SearchResults<DocumentSearchIndex>> {
        let index = self.client.index(DOCUMENTS_INDEX);
        
        let mut search_query = index.search();
        search_query.with_query(query);
        
        if let Some(ref filter) = filters {
            search_query.with_filter(filter);
        }
        
        if let Some(limit) = limit {
            search_query.with_limit(limit);
        }
        
        if let Some(offset) = offset {
            search_query.with_offset(offset);
        }

        let results = search_query
            .execute::<DocumentSearchIndex>()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Search failed: {}", e)))?;

        Ok(results)
    }

    pub async fn batch_index_documents(&self, documents: Vec<Document>) -> Result<()> {
        let index = self.client.index(DOCUMENTS_INDEX);
        let docs: Vec<DocumentSearchIndex> = documents.into_iter().map(Into::into).collect();
        
        index
            .add_documents(&docs, Some("id"))
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to batch index documents: {}", e)))?;

        Ok(())
    }
}

