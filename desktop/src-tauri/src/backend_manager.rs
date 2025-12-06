use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug)]
pub struct BackendManager {
    process: Arc<Mutex<Option<Child>>>,
    port: u16,
}

impl BackendManager {
    pub fn new(port: u16) -> Self {
        Self {
            process: Arc::new(Mutex::new(None)),
            port,
        }
    }

    /// Start backend server automatically
    pub fn start(&self) -> Result<(), String> {
        let mut proc = self.process.lock().unwrap();
        
        if proc.is_some() {
            return Ok(()); // Already running
        }

        // Get backend executable path (bundled with app)
        let backend_path = self.get_backend_path()?;

        println!("[Backend] Starting server at {}", backend_path);

        // Start backend process
        let child = Command::new(&backend_path)
            .env("BRIDGEX_PORT", self.port.to_string())
            .env("BRIDGEX_AUTO_START", "1")
            .spawn()
            .map_err(|e| format!("Failed to start backend: {}", e))?;

        *proc = Some(child);
        println!("[Backend] Server started on port {}", self.port);

        Ok(())
    }

    /// Stop backend server gracefully
    pub fn stop(&self) -> Result<(), String> {
        let mut proc = self.process.lock().unwrap();
        
        if let Some(mut child) = proc.take() {
            println!("[Backend] Stopping server...");
            child.kill().map_err(|e| format!("Failed to kill backend: {}", e))?;
            println!("[Backend] Server stopped");
        }

        Ok(())
    }

    /// Check if backend is running and healthy
    pub async fn is_healthy(&self) -> bool {
        let url = format!("http://127.0.0.1:{}/api/v1/health", self.port);  // Fixed: added /api/v1
        
        match reqwest::get(&url).await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Wait for backend to be ready (with timeout)
    pub async fn wait_ready(&self, timeout_secs: u64) -> Result<(), String> {
        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(timeout_secs);

        loop {
            if self.is_healthy().await {
                println!("[Backend] Server is ready!");
                return Ok(());
            }

            if start.elapsed() > timeout {
                return Err("Backend startup timeout".to_string());
            }

            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }

    /// Get backend executable path (platform-specific)
    fn get_backend_path(&self) -> Result<String, String> {
        #[cfg(target_os = "windows")]
        {
            // Windows: backend bundled in same dir as .exe
            let exe_dir = std::env::current_exe()
                .map_err(|e| format!("Cannot get exe path: {}", e))?
                .parent()
                .ok_or("No parent directory")?;
            
            Ok(exe_dir.join("bridgex-server.exe").to_string_lossy().to_string())
        }

        #[cfg(target_os = "macos")]
        {
            // macOS: backend in Resources folder
            let exe_dir = std::env::current_exe()
                .map_err(|e| format!("Cannot get exe path: {}", e))?
                .parent()
                .ok_or("No parent directory")?;
            
            Ok(exe_dir.join("../Resources/bridgex-server").to_string_lossy().to_string())
        }

        #[cfg(target_os = "linux")]
        {
            // Linux: backend in same dir or /usr/bin
            let exe_dir = std::env::current_exe()
                .map_err(|e| format!("Cannot get exe path: {}", e))?
                .parent()
                .ok_or("No parent directory")?;
            
            let local_path = exe_dir.join("bridgex-server");
            if local_path.exists() {
                return Ok(local_path.to_string_lossy().to_string());
            }

            // Fallback to system path
            Ok("/usr/bin/bridgex-server".to_string())
        }
    }
}

impl Drop for BackendManager {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Tauri command to check backend status
#[tauri::command]
pub async fn check_backend_status(
    backend: tauri::State<'_, Arc<BackendManager>>  // Fixed: use State properly
) -> Result<bool, String> {
    Ok(backend.is_healthy().await)
}

/// Tauri command to restart backend
#[tauri::command]
pub async fn restart_backend(
    backend: tauri::State<'_, Arc<BackendManager>>  // Fixed: use State properly
) -> Result<(), String> {
    backend.stop()?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    backend.start()?;
    backend.wait_ready(10).await
}
