use axum::{
    extract::State,
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod crypto;
mod db;
mod qr;
mod server;
mod util;

use db::Database;
use server::{api, upload};

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bridgex_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let db_path = std::env::var("BRIDGEX_DB_PATH")
        .unwrap_or_else(|_| "./data/bridge.db".to_string());
    
    // Create data directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let database_url = format!("sqlite:{}", db_path);
    tracing::info!("Connecting to database: {}", database_url);
    
    let db = Database::new(&database_url).await?;
    db.init_schema().await?;
    tracing::info!("Database initialized");

    let state = AppState {
        db: Arc::new(db),
    };

    // Build application routes
    let app = Router::new()
        .route("/api/v1/health", get(api::health))
        .route("/api/v1/pair", post(api::pair))
        .route("/api/v1/transfer", post(api::transfer_init))
        .route("/api/v1/transfer/:id/upload", post(upload::upload_chunk))
        .route("/api/v1/transfer/:id/status", get(upload::get_upload_status))
        .route("/api/v1/status", get(api::status))
        .route("/api/v1/devices", get(api::list_devices))
        .route("/api/v1/devices/:id", delete(api::delete_device))
        .with_state(state)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http());

    // Get host and port from environment or use defaults
    let host = std::env::var("BRIDGEX_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("BRIDGEX_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    let addr = SocketAddr::from((
        host.parse::<std::net::IpAddr>()?,
        port,
    ));

    tracing::info!("BridgeX backend starting on http://{}", addr);
    tracing::info!("API endpoints:");
    tracing::info!("  GET    /api/v1/health            - Health check");
    tracing::info!("  POST   /api/v1/pair              - Device pairing");
    tracing::info!("  POST   /api/v1/transfer          - Initialize transfer");
    tracing::info!("  POST   /api/v1/transfer/:id/upload - Upload file chunk");
    tracing::info!("  GET    /api/v1/transfer/:id/status - Get upload status");
    tracing::info!("  GET    /api/v1/status            - Server status");
    tracing::info!("  GET    /api/v1/devices           - List devices");
    tracing::info!("  DELETE /api/v1/devices/:id       - Delete device");

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
