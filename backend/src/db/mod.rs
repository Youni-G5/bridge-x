//! Database module for BridgeX
//!
//! Handles SQLite database operations for devices, transfers, and sessions

pub mod models;

use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};

/// Database connection pool
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    /// Create a new database connection
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    /// Initialize database schema
    pub async fn init_schema(&self) -> Result<()> {
        sqlx::query(include_str!("schema.sql"))
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Save a device pairing
    pub async fn save_device(&self, device: &models::Device) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO devices (id, name, type, public_key, paired_at, last_seen)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                last_seen = excluded.last_seen
            "#,
        )
        .bind(&device.id)
        .bind(&device.name)
        .bind(&device.device_type)
        .bind(&device.public_key)
        .bind(device.paired_at)
        .bind(device.last_seen)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Get all paired devices
    pub async fn get_devices(&self) -> Result<Vec<models::Device>> {
        let devices = sqlx::query_as::<_, models::Device>(
            "SELECT id, name, type, public_key, paired_at, last_seen FROM devices ORDER BY paired_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(devices)
    }

    /// Get device by ID
    pub async fn get_device(&self, id: &str) -> Result<Option<models::Device>> {
        let device = sqlx::query_as::<_, models::Device>(
            "SELECT id, name, type, public_key, paired_at, last_seen FROM devices WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(device)
    }

    /// Delete a device
    pub async fn delete_device(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM devices WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Save a transfer record
    pub async fn save_transfer(&self, transfer: &models::Transfer) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO transfers (id, device_id, file_name, file_size, file_hash, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&transfer.id)
        .bind(&transfer.device_id)
        .bind(&transfer.file_name)
        .bind(transfer.file_size)
        .bind(&transfer.file_hash)
        .bind(&transfer.status)
        .bind(transfer.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Update transfer status
    pub async fn update_transfer_status(
        &self,
        transfer_id: &str,
        status: &str,
        completed_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE transfers 
            SET status = ?, completed_at = ?
            WHERE id = ?
            "#,
        )
        .bind(status)
        .bind(completed_at)
        .bind(transfer_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Get transfers for a device
    pub async fn get_device_transfers(&self, device_id: &str) -> Result<Vec<models::Transfer>> {
        let transfers = sqlx::query_as::<_, models::Transfer>(
            r#"
            SELECT id, device_id, file_name, file_size, file_hash, status, created_at, completed_at
            FROM transfers 
            WHERE device_id = ?
            ORDER BY created_at DESC
            "#
        )
        .bind(device_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(transfers)
    }
}
