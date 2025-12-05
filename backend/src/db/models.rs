//! Database models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Device model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub device_type: String,
    pub public_key: Vec<u8>,
    pub paired_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
}

impl Device {
    pub fn new(
        id: String,
        name: String,
        device_type: String,
        public_key: Vec<u8>,
    ) -> Self {
        Self {
            id,
            name,
            device_type,
            public_key,
            paired_at: Utc::now(),
            last_seen: Some(Utc::now()),
        }
    }
}

/// Transfer model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transfer {
    pub id: String,
    pub device_id: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_hash: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Transfer {
    pub fn new(
        id: String,
        device_id: String,
        file_name: String,
        file_size: i64,
        file_hash: String,
    ) -> Self {
        Self {
            id,
            device_id,
            file_name,
            file_size,
            file_hash,
            status: "pending".to_string(),
            created_at: Utc::now(),
            completed_at: None,
        }
    }
}
