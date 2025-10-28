mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

// Embed migrations
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    config::Config,
    db::{create_pool, AppState},
    routes::create_routes,
    services::SearchService,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,document_management_system=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Document Management System...");

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded");

    // Create database connection pool
    let pool = create_pool(&config.database.url)?;
    tracing::info!("Database connection pool created");

    // Run migrations automatically
    tracing::info!("Running database migrations...");
    {
        let mut conn = pool.get()?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;
        tracing::info!("Database migrations completed successfully");
    }

    // Initialize MeiliSearch index
    tracing::info!("Initializing search index...");
    let search_service = SearchService::new(&config.meilisearch)?;
    search_service.initialize_index().await?;
    tracing::info!("Search index initialized");

    // Create application state
    let state = AppState::new(pool, config.clone());

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create router
    let app = create_routes()
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr: SocketAddr = config.server_address().parse()?;
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

