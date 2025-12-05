use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod crypto;
mod server;
mod util;

use server::api;

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

    // Build application routes
    let app = Router::new()
        .route("/api/v1/health", get(api::health))
        .route("/api/v1/pair", post(api::pair))
        .route("/api/v1/transfer", post(api::transfer_init))
        .route("/api/v1/status", get(api::status))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
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
    tracing::info!("  GET  /api/v1/health  - Health check");
    tracing::info!("  POST /api/v1/pair    - Device pairing");
    tracing::info!("  POST /api/v1/transfer - Initialize transfer");
    tracing::info!("  GET  /api/v1/status  - Server status");

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
