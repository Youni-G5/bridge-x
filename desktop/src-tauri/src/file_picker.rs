use std::path::PathBuf;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{AppHandle, Manager, Window};

/// Tauri command to open file picker (single file)
#[tauri::command]
pub async fn pick_file(window: Window) -> Result<Option<String>, String> {
    let (tx, rx) = std::sync::mpsc::channel();

    FileDialogBuilder::new()
        .set_title("Select a file to send")
        .pick_file(move |path| {
            tx.send(path).ok();
        });

    match rx.recv() {
        Ok(Some(path)) => Ok(Some(path.to_string_lossy().to_string())),
        Ok(None) => Ok(None),
        Err(e) => Err(format!("File picker error: {}", e)),
    }
}

/// Tauri command to open file picker (multiple files)
#[tauri::command]
pub async fn pick_files(window: Window) -> Result<Vec<String>, String> {
    let (tx, rx) = std::sync::mpsc::channel();

    FileDialogBuilder::new()
        .set_title("Select files to send")
        .pick_files(move |paths| {
            tx.send(paths).ok();
        });

    match rx.recv() {
        Ok(Some(paths)) => {
            Ok(paths.iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect())
        }
        Ok(None) => Ok(vec![]),
        Err(e) => Err(format!("File picker error: {}", e)),
    }
}

/// Tauri command to open folder picker
#[tauri::command]
pub async fn pick_folder(window: Window) -> Result<Option<String>, String> {
    let (tx, rx) = std::sync::mpsc::channel();

    FileDialogBuilder::new()
        .set_title("Select a folder to send")
        .pick_folder(move |path| {
            tx.send(path).ok();
        });

    match rx.recv() {
        Ok(Some(path)) => Ok(Some(path.to_string_lossy().to_string())),
        Ok(None) => Ok(None),
        Err(e) => Err(format!("Folder picker error: {}", e)),
    }
}

/// Tauri command to get file metadata
#[tauri::command]
pub async fn get_file_info(path: String) -> Result<FileInfo, String> {
    let path = PathBuf::from(path);
    
    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    let metadata = std::fs::metadata(&path)
        .map_err(|e| format!("Cannot read file metadata: {}", e))?;

    let name = path.file_name()
        .ok_or("Invalid file name")?.
        to_string_lossy()
        .to_string();

    let extension = path.extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();

    Ok(FileInfo {
        name,
        path: path.to_string_lossy().to_string(),
        size: metadata.len(),
        extension,
        is_directory: metadata.is_dir(),
        modified: metadata.modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs()),
    })
}

/// Tauri command to read file as base64 (for preview)
#[tauri::command]
pub async fn read_file_base64(path: String, max_size: Option<u64>) -> Result<String, String> {
    use std::io::Read;
    
    let path = PathBuf::from(path);
    let metadata = std::fs::metadata(&path)
        .map_err(|e| format!("Cannot read file: {}", e))?;

    let max = max_size.unwrap_or(5 * 1024 * 1024); // 5MB default
    if metadata.len() > max {
        return Err(format!("File too large for preview (max {}MB)", max / 1024 / 1024));
    }

    let mut file = std::fs::File::open(&path)
        .map_err(|e| format!("Cannot open file: {}", e))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Cannot read file: {}", e))?;

    Ok(base64::encode(&buffer))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub extension: String,
    pub is_directory: bool,
    pub modified: Option<u64>,
}

/// Initialize file drop handler for drag-and-drop
pub fn init_file_drop_handler(app: &tauri::App) {
    use tauri::Manager;
    
    let main_window = app.get_window("main").unwrap();
    
    main_window.on_file_drop(move |event| {
        match event {
            tauri::window::FileDropEvent::Dropped(paths) => {
                let paths: Vec<String> = paths.iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect();
                
                println!("[FileDrop] Files dropped: {:?}", paths);
                
                // Emit event to frontend
                main_window.emit("files-dropped", paths).ok();
            }
            tauri::window::FileDropEvent::Hovered(paths) => {
                println!("[FileDrop] Files hovering: {:?}", paths);
                main_window.emit("files-hovering", ()).ok();
            }
            tauri::window::FileDropEvent::Cancelled => {
                println!("[FileDrop] Drop cancelled");
                main_window.emit("files-cancelled", ()).ok();
            }
            _ => {}
        }
    });
}
