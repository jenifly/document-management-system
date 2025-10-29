use opendal::{Operator, services::S3};
use std::sync::Arc;
use uuid::Uuid;

use crate::config::MinioConfig;
use crate::error::{AppError, Result};

#[derive(Clone)]
pub struct StorageService {
    operator: Arc<Operator>,
    config: MinioConfig, // 保存配置以便生成 OnlyOffice URL
}

impl StorageService {
    pub fn new(config: &MinioConfig) -> Result<Self> {
        tracing::info!("Initializing MinIO storage with endpoint: {}", config.endpoint);
        
        let builder = S3::default()
            .root("/")
            .bucket(&config.bucket)
            .endpoint(&config.endpoint)
            .region(&config.region)
            .access_key_id(&config.access_key)
            .secret_access_key(&config.secret_key);
            // 注意：不使用 enable_virtual_host_style()
            // MinIO 默认使用路径样式 (path-style): http://localhost:9000/bucket/key
            // 虚拟主机样式 (virtual-host-style) 需要 DNS: http://bucket.localhost:9000/key
            // 在本地开发时使用路径样式更可靠

        let operator = Operator::new(builder)
            .map_err(|e| AppError::InternalServerError(format!("Failed to create operator: {}", e)))?
            .finish();

        tracing::info!("MinIO storage initialized successfully");

        Ok(Self {
            operator: Arc::new(operator),
            config: config.clone(),
        })
    }

    pub async fn upload_file(&self, file_data: &[u8], file_name: &str, _content_type: &str) -> Result<String> {
        let object_key = format!("{}/{}", Uuid::new_v4(), file_name);
        
        self.operator
            .write(&object_key, file_data.to_vec())
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to upload file: {}", e)))?;

        Ok(object_key)
    }

    pub async fn download_file(&self, object_key: &str) -> Result<Vec<u8>> {
        let data = self.operator
            .read(object_key)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to download file: {}", e)))?
            .to_vec();

        Ok(data)
    }

    pub async fn delete_file(&self, object_key: &str) -> Result<()> {
        self.operator
            .delete(object_key)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to delete file: {}", e)))?;

        Ok(())
    }

    pub async fn get_file_url(&self, object_key: &str, expires_in: u32) -> Result<String> {
        let url = self.operator
            .presign_read(object_key, std::time::Duration::from_secs(expires_in as u64))
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to generate presigned URL: {}", e)))?
            .uri()
            .to_string();

        Ok(url)
    }

    /// 生成 OnlyOffice 可访问的文件 URL（使用内部 endpoint）
    pub async fn get_file_url_for_onlyoffice(&self, object_key: &str, expires_in: u32) -> Result<String> {
        let mut url = self.operator
            .presign_read(object_key, std::time::Duration::from_secs(expires_in as u64))
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to generate presigned URL: {}", e)))?
            .uri()
            .to_string();

        // 如果配置了内部 endpoint，则替换为内部地址（供 Docker 容器访问）
        if let Some(internal_endpoint) = &self.config.internal_endpoint {
            url = url.replace(&self.config.endpoint, internal_endpoint);
            tracing::debug!("Replaced endpoint for OnlyOffice: {} -> {}", &self.config.endpoint, internal_endpoint);
        }

        Ok(url)
    }

    pub async fn copy_file(&self, source_key: &str, dest_key: &str) -> Result<()> {
        self.operator
            .copy(source_key, dest_key)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to copy file: {}", e)))?;

        Ok(())
    }

    pub async fn file_exists(&self, object_key: &str) -> Result<bool> {
        match self.operator.stat(object_key).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
