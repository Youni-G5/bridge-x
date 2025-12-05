// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

/// Health check command
#[tauri::command]
async fn check_health() -> Result<String, String> {
    // TODO: Call backend health endpoint
    Ok("Backend is healthy".to_string())
}

/// Request device pairing
#[tauri::command]
async fn pair_device(device_name: String) -> Result<String, String> {
    // TODO: Call backend pairing endpoint
    Ok(format!("Pairing requested for: {}", device_name))
}

/// Get paired devices list
#[tauri::command]
async fn get_devices() -> Result<Vec<String>, String> {
    // TODO: Get devices from backend
    Ok(vec!["Desktop PC".to_string(), "My Phone".to_string()])
}

/// Initialize file transfer
#[tauri::command]
async fn send_file(device_id: String, file_path: String) -> Result<String, String> {
    // TODO: Call backend transfer endpoint
    Ok(format!(
        "Transfer initiated: {} to {}",
        file_path, device_id
    ))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            check_health,
            pair_device,
            get_devices,
            send_file,
        ])
        .setup(|app| {
            // Start backend server in background
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                println!("Starting BridgeX backend...");
                // TODO: Spawn backend process
                // let backend = Command::new("bridgex-server")
                //     .spawn()
                //     .expect("Failed to start backend");
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
