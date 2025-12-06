// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend_manager;
mod file_picker;

use std::sync::Arc;
use backend_manager::{BackendManager, check_backend_status, restart_backend};
use file_picker::{pick_file, pick_files, pick_folder, get_file_info, read_file_base64};
use tauri::Manager;

/// Request device pairing
#[tauri::command]
async fn pair_device(device_name: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "device_name": device_name
    });

    match client
        .post("http://127.0.0.1:8080/api/v1/pair")
        .json(&payload)
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            resp.text().await.map_err(|e| e.to_string())
        }
        Ok(resp) => Err(format!("Pairing failed: {}", resp.status())),
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

/// Get paired devices list
#[tauri::command]
async fn get_devices() -> Result<String, String> {
    match reqwest::get("http://127.0.0.1:8080/api/v1/devices").await {
        Ok(resp) if resp.status().is_success() => {
            resp.text().await.map_err(|e| e.to_string())
        }
        Ok(resp) => Err(format!("Failed to get devices: {}", resp.status())),
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

/// Initialize file transfer
#[tauri::command]
async fn send_file(device_id: String, file_path: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    // Read file metadata
    let metadata = tokio::fs::metadata(&file_path)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let file_size = metadata.len();
    let file_name = std::path::Path::new(&file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid file name")?;
    
    // Initialize transfer
    let init_payload = serde_json::json!({
        "device_id": device_id,
        "file_name": file_name,
        "file_size": file_size,
    });
    
    let init_resp = client
        .post("http://127.0.0.1:8080/api/v1/transfer/init")
        .json(&init_payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if !init_resp.status().is_success() {
        return Err(format!("Transfer init failed: {}", init_resp.status()));
    }
    
    let init_data: serde_json::Value = init_resp.json()
        .await
        .map_err(|e| e.to_string())?;
    
    let transfer_id = init_data["transfer_id"]
        .as_str()
        .ok_or("Missing transfer_id")?;
    
    // Read and upload file in chunks
    let file_data = tokio::fs::read(&file_path)
        .await
        .map_err(|e| format!("Failed to read file data: {}", e))?;
    
    let chunk_size = 1024 * 1024; // 1MB chunks
    let mut offset = 0;
    
    while offset < file_data.len() {
        let end = std::cmp::min(offset + chunk_size, file_data.len());
        let chunk = &file_data[offset..end];
        
        let form = reqwest::multipart::Form::new()
            .text("transfer_id", transfer_id.to_string())
            .text("offset", offset.to_string())
            .part("chunk", reqwest::multipart::Part::bytes(chunk.to_vec()));
        
        let upload_resp = client
            .post("http://127.0.0.1:8080/api/v1/transfer/upload")
            .multipart(form)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        if !upload_resp.status().is_success() {
            return Err(format!("Chunk upload failed at offset {}: {}", offset, upload_resp.status()));
        }
        
        offset = end;
    }
    
    // Finalize transfer
    let finalize_payload = serde_json::json!({
        "transfer_id": transfer_id,
    });
    
    let finalize_resp = client
        .post("http://127.0.0.1:8080/api/v1/transfer/finalize")
        .json(&finalize_payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if !finalize_resp.status().is_success() {
        return Err(format!("Transfer finalize failed: {}", finalize_resp.status()));
    }
    
    Ok(format!("File '{}' transferred successfully to device {}", file_name, device_id))
}

#[tokio::main]
async fn main() {
    // Initialize backend manager
    let backend = BackendManager::new(8080);
    let backend_arc = Arc::new(backend);
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(backend_arc.clone())
        .invoke_handler(tauri::generate_handler![
            check_backend_status,
            restart_backend,
            pick_file,
            pick_files,
            pick_folder,
            get_file_info,
            read_file_base64,
            pair_device,
            get_devices,
            send_file,
        ])
        .setup(move |app| {
            let backend_clone = backend_arc.clone();
            
            // Start backend in async task
            tauri::async_runtime::spawn(async move {
                println!("🚀 Starting BridgeX backend...");
                
                if let Err(e) = backend_clone.start() {
                    eprintln!("❌ Failed to start backend: {}", e);
                    eprintln!("   Please ensure the backend binary is available");
                    return;
                }
                
                println!("⏳ Waiting for backend to be ready...");
                match backend_clone.wait_ready(15).await {
                    Ok(_) => println!("✅ Backend is ready and healthy!"),
                    Err(e) => {
                        eprintln!("⚠️  Backend health check failed: {}", e);
                        eprintln!("   The app will continue but may not function correctly");
                    }
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
