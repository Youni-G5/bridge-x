//! API endpoints for BridgeX backend

use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;

use super::{PairRequest, PairResponse, TransferRequest, TransferResponse};
use crate::crypto::keys::generate_keypair;

/// Health check endpoint
///
/// Returns server status and uptime information
pub async fn health() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "BridgeX Backend",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

/// Device pairing endpoint
///
/// Generates a keypair and QR code for device pairing
pub async fn pair(Json(payload): Json<PairRequest>) -> Result<Json<PairResponse>, AppError> {
    tracing::info!("Pairing request from device: {}", payload.device_name);

    // Generate keypair for this pairing session
    let keypair = generate_keypair();
    let device_id = Uuid::new_v4().to_string();

    // Create QR code data (base64 encoded public key + device ID)
    let qr_data = format!(
        "bridgex://pair?id={}&key={}",
        device_id,
        base64::encode(&keypair.public_key)
    );

    // Set expiration time (5 minutes from now)
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(5);

    tracing::info!("Generated pairing for device_id: {}", device_id);

    Ok(Json(PairResponse {
        device_id,
        public_key: base64::encode(&keypair.public_key),
        qr_data,
        expires_at,
    }))
}

/// Initialize file transfer
///
/// Creates a transfer session and returns upload URL
pub async fn transfer_init(
    Json(payload): Json<TransferRequest>,
) -> Result<Json<TransferResponse>, AppError> {
    tracing::info!(
        "Transfer request: {} ({} bytes) to device {}",
        payload.file_name,
        payload.file_size,
        payload.device_id
    );

    let transfer_id = Uuid::new_v4().to_string();
    let upload_url = format!("/api/v1/transfer/{}/upload", transfer_id);

    Ok(Json(TransferResponse {
        transfer_id,
        status: "pending".to_string(),
        upload_url,
    }))
}

/// Server status endpoint
///
/// Returns detailed server status
pub async fn status() -> impl IntoResponse {
    Json(json!({
        "uptime": "running",
        "active_connections": 0,
        "pending_transfers": 0,
        "paired_devices": 0,
    }))
}

/// Application error type
#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("API error: {:?}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": self.0.to_string(),
            })),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
