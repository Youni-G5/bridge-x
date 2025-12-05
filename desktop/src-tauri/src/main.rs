// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::{Manager, State};

/// Backend process state
struct BackendProcess {
    child: Mutex<Option<Child>>,
}

/// Health check command
#[tauri::command]
async fn check_health() -> Result<String, String> {
    match reqwest::get("http://127.0.0.1:8080/api/v1/health").await {
        Ok(resp) if resp.status().is_success() => {
            let body = resp.text().await.map_err(|e| e.to_string())?;
            Ok(body)
        }
        Ok(resp) => Err(format!("Backend returned status: {}", resp.status())),
        Err(e) => Err(format!("Backend connection failed: {}", e)),
    }
}

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
            let body = resp.text().await.map_err(|e| e.to_string())?;
            Ok(body)
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
            let body = resp.text().await.map_err(|e| e.to_string())?;
            Ok(body)
        }
        Ok(resp) => Err(format!("Failed to get devices: {}", resp.status())),
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

/// Initialize file transfer
#[tauri::command]
async fn send_file(device_id: String, file_path: String) -> Result<String, String> {
    // TODO: Implement file transfer
    Ok(format!(
        "Transfer initiated: {} to {}",
        file_path, device_id
    ))
}

/// Start the backend server process
fn start_backend() -> Result<Child, std::io::Error> {
    #[cfg(target_os = "windows")]
    let backend_exe = "bridgex-server.exe";

    #[cfg(not(target_os = "windows"))]
    let backend_exe = "bridgex-server";

    // Try to find backend in multiple locations
    let possible_paths = vec![
        format!("./backend/target/release/{}", backend_exe),
        format!("./backend/target/debug/{}", backend_exe),
        format!("../backend/target/release/{}", backend_exe),
        format!("../../backend/target/release/{}", backend_exe),
        backend_exe.to_string(), // In PATH
    ];

    for path in possible_paths {
        if std::path::Path::new(&path).exists() || path == backend_exe {
            println!("Attempting to start backend: {}", path);
            match Command::new(&path)
                .env("BRIDGEX_HOST", "127.0.0.1")
                .env("BRIDGEX_PORT", "8080")
                .spawn()
            {
                Ok(child) => {
                    println!("✅ Backend started successfully (PID: {})", child.id());
                    return Ok(child);
                }
                Err(e) => {
                    println!("❌ Failed to start backend from {}: {}", path, e);
                    continue;
                }
            }
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("Backend executable '{}' not found", backend_exe),
    ))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(BackendProcess {
            child: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            check_health,
            pair_device,
            get_devices,
            send_file,
        ])
        .setup(|app| {
            let backend_state = app.state::<BackendProcess>();

            // Start backend server
            println!("🚀 Starting BridgeX backend...");
            match start_backend() {
                Ok(child) => {
                    *backend_state.child.lock().unwrap() = Some(child);
                    println!("✅ Backend process started");
                }
                Err(e) => {
                    eprintln!("⚠️  Failed to start backend: {}", e);
                    eprintln!("   The app will try to connect to an existing backend on port 8080");
                }
            }

            // Wait a moment for backend to start
            std::thread::sleep(std::time::Duration::from_secs(2));

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                // Cleanup: kill backend process
                if let Some(backend_state) = window.try_state::<BackendProcess>() {
                    if let Some(mut child) = backend_state.child.lock().unwrap().take() {
                        println!("🛑 Stopping backend process...");
                        let _ = child.kill();
                        println!("✅ Backend stopped");
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
