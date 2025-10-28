use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub minio: MinioConfig,
    pub meilisearch: MeilisearchConfig,
    pub onlyoffice: OnlyOfficeConfig,
    pub app: AppConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MinioConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub region: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeilisearchConfig {
    pub host: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OnlyOfficeConfig {
    pub server: String,
    pub jwt_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenv::dotenv().ok();

        let database = DatabaseConfig {
            url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
        };

        let server = ServerConfig {
            host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid u16"),
        };

        let jwt = JwtConfig {
            secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .expect("JWT_EXPIRATION must be a valid i64"),
        };

        let minio = MinioConfig {
            endpoint: env::var("MINIO_ENDPOINT")
                .expect("MINIO_ENDPOINT must be set"),
            access_key: env::var("MINIO_ACCESS_KEY")
                .expect("MINIO_ACCESS_KEY must be set"),
            secret_key: env::var("MINIO_SECRET_KEY")
                .expect("MINIO_SECRET_KEY must be set"),
            bucket: env::var("MINIO_BUCKET")
                .expect("MINIO_BUCKET must be set"),
            region: env::var("MINIO_REGION")
                .unwrap_or_else(|_| "us-east-1".to_string()),
        };

        let meilisearch = MeilisearchConfig {
            host: env::var("MEILISEARCH_HOST")
                .expect("MEILISEARCH_HOST must be set"),
            api_key: env::var("MEILISEARCH_API_KEY")
                .expect("MEILISEARCH_API_KEY must be set"),
        };

        let onlyoffice = OnlyOfficeConfig {
            server: env::var("ONLYOFFICE_SERVER")
                .expect("ONLYOFFICE_SERVER must be set"),
            jwt_secret: env::var("ONLYOFFICE_JWT_SECRET")
                .expect("ONLYOFFICE_JWT_SECRET must be set"),
        };

        let app = AppConfig {
            url: env::var("APP_URL")
                .expect("APP_URL must be set"),
        };

        Ok(Config {
            database,
            server,
            jwt,
            minio,
            meilisearch,
            onlyoffice,
            app,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

