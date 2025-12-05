//! API endpoints for BridgeX backend

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use super::{PairRequest, PairResponse, TransferRequest, TransferResponse};
use crate::crypto::keys::generate_keypair;
use crate::db::models::{Device, Transfer};
use crate::qr::generate_pairing_qr;
use crate::AppState;

/// Health check endpoint
pub async fn health() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "BridgeX Backend",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

/// Device pairing endpoint
pub async fn pair(
    State(state): State<AppState>,
    Json(payload): Json<PairRequest>,
) -> Result<Json<PairResponse>, AppError> {
    tracing::info!("Pairing request from device: {}", payload.device_name);

    let keypair = generate_keypair();
    let device_id = Uuid::new_v4().to_string();

    // Generate QR code with pairing information
    let (qr_data_url, pairing_uri) = generate_pairing_qr(&device_id, &keypair.public_key)
        .map_err(|e| anyhow::anyhow!("QR generation failed: {}", e))?;

    // Save device to database
    let device = Device::new(
        device_id.clone(),
        payload.device_name,
        "desktop".to_string(),
        keypair.public_key.to_vec(),
    );
    
    state.db.save_device(&device).await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(5);

    tracing::info!("Generated pairing for device_id: {}", device_id);
    tracing::debug!("Pairing URI: {}", pairing_uri);

    Ok(Json(PairResponse {
        device_id,
        public_key: base64::encode(&keypair.public_key),
        qr_data: qr_data_url,
        expires_at,
    }))
}

/// Initialize file transfer
pub async fn transfer_init(
    State(state): State<AppState>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<TransferResponse>, AppError> {
    tracing::info!(
        "Transfer request: {} ({} bytes) to device {}",
        payload.file_name,
        payload.file_size,
        payload.device_id
    );

    // Verify device exists
    let device = state.db.get_device(&payload.device_id).await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;
    
    if device.is_none() {
        return Err(AppError(anyhow::anyhow!("Device not found")));
    }

    let transfer_id = Uuid::new_v4().to_string();
    let upload_url = format!("/api/v1/transfer/{}/upload", transfer_id);

    // Save transfer to database
    let transfer = Transfer::new(
        transfer_id.clone(),
        payload.device_id,
        payload.file_name,
        payload.file_size as i64,
        payload.file_hash,
    );
    
    state.db.save_transfer(&transfer).await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

    Ok(Json(TransferResponse {
        transfer_id,
        status: "pending".to_string(),
        upload_url,
    }))
}

/// Server status endpoint
pub async fn status(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let devices = state.db.get_devices().await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;
    
    Ok(Json(json!({
        "uptime": "running",
        "active_connections": 0,
        "pending_transfers": 0,
        "paired_devices": devices.len(),
    })))
}

/// List all paired devices
pub async fn list_devices(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let devices = state.db.get_devices().await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;
    
    Ok(Json(devices))
}

/// Delete a device
pub async fn delete_device(
    State(state): State<AppState>,
    Path(device_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Deleting device: {}", device_id);
    
    state.db.delete_device(&device_id).await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;
    
    Ok(StatusCode::NO_CONTENT)
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
