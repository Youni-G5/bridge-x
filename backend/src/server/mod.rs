//! Server module containing API and P2P logic

pub mod api;
pub mod p2p;
pub mod upload;

use serde::{Deserialize, Serialize};

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub public_key: String,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

/// Pairing request
#[derive(Debug, Deserialize)]
pub struct PairRequest {
    pub device_name: String,
}

/// Pairing response with QR code data
#[derive(Debug, Serialize)]
pub struct PairResponse {
    pub device_id: String,
    pub public_key: String,
    pub qr_data: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Transfer initialization request
#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    pub device_id: String,
    pub file_name: String,
    pub file_size: u64,
    pub file_hash: String,
}

/// Transfer response
#[derive(Debug, Serialize)]
pub struct TransferResponse {
    pub transfer_id: String,
    pub status: String,
    pub upload_url: String,
}
