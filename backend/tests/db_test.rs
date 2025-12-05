//! Database module tests

use bridgex_backend::db::{models::Device, Database};
use uuid::Uuid;

#[tokio::test]
async fn test_database_init() {
    let db = Database::new("sqlite::memory:").await.unwrap();
    db.init_schema().await.unwrap();
}

#[tokio::test]
async fn test_save_and_get_device() {
    let db = Database::new("sqlite::memory:").await.unwrap();
    db.init_schema().await.unwrap();

    let device = Device::new(
        Uuid::new_v4().to_string(),
        "Test Device".to_string(),
        "mobile".to_string(),
        vec![1, 2, 3, 4],
    );

    db.save_device(&device).await.unwrap();

    let retrieved = db.get_device(&device.id).await.unwrap();
    assert!(retrieved.is_some());
    
    let retrieved_device = retrieved.unwrap();
    assert_eq!(retrieved_device.id, device.id);
    assert_eq!(retrieved_device.name, device.name);
}

#[tokio::test]
async fn test_list_devices() {
    let db = Database::new("sqlite::memory:").await.unwrap();
    db.init_schema().await.unwrap();

    let device1 = Device::new(
        Uuid::new_v4().to_string(),
        "Device 1".to_string(),
        "mobile".to_string(),
        vec![1, 2, 3],
    );

    let device2 = Device::new(
        Uuid::new_v4().to_string(),
        "Device 2".to_string(),
        "desktop".to_string(),
        vec![4, 5, 6],
    );

    db.save_device(&device1).await.unwrap();
    db.save_device(&device2).await.unwrap();

    let devices = db.get_devices().await.unwrap();
    assert_eq!(devices.len(), 2);
}

#[tokio::test]
async fn test_delete_device() {
    let db = Database::new("sqlite::memory:").await.unwrap();
    db.init_schema().await.unwrap();

    let device = Device::new(
        Uuid::new_v4().to_string(),
        "Test Device".to_string(),
        "mobile".to_string(),
        vec![1, 2, 3],
    );

    db.save_device(&device).await.unwrap();
    db.delete_device(&device.id).await.unwrap();

    let retrieved = db.get_device(&device.id).await.unwrap();
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_update_device_last_seen() {
    let db = Database::new("sqlite::memory:").await.unwrap();
    db.init_schema().await.unwrap();

    let mut device = Device::new(
        Uuid::new_v4().to_string(),
        "Test Device".to_string(),
        "mobile".to_string(),
        vec![1, 2, 3],
    );

    db.save_device(&device).await.unwrap();

    // Wait a bit and update
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    device.last_seen = Some(chrono::Utc::now());
    db.save_device(&device).await.unwrap();

    let retrieved = db.get_device(&device.id).await.unwrap().unwrap();
    assert!(retrieved.last_seen.is_some());
}
