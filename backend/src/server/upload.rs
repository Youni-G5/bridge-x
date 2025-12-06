//! File upload handling

use axum::{
    body::Bytes,
    extract::{Multipart, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct FinalizeRequest {
    pub transfer_id: String,
}

/// Upload file chunk (multipart form)
pub async fn upload_chunk(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
    let mut transfer_id: Option<String> = None;
    let mut offset: Option<usize> = None;
    let mut chunk_data: Option<Vec<u8>> = None;

    // Parse multipart fields
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart error: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "transfer_id" => {
                transfer_id = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "offset" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                offset = text.parse().ok();
            }
            "chunk" => {
                chunk_data = Some(field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec());
            }
            _ => {}
        }
    }

    let transfer_id = transfer_id.ok_or(StatusCode::BAD_REQUEST)?;
    let offset = offset.ok_or(StatusCode::BAD_REQUEST)?;
    let chunk_data = chunk_data.ok_or(StatusCode::BAD_REQUEST)?;

    tracing::info!(
        "Upload chunk for transfer: {} (offset: {}, size: {})",
        transfer_id,
        offset,
        chunk_data.len()
    );

    // Create upload directory
    let upload_dir = PathBuf::from("./data/uploads");
    fs::create_dir_all(&upload_dir).map_err(|e| {
        tracing::error!("Failed to create upload directory: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Create transfer directory
    let transfer_dir = upload_dir.join(&transfer_id);
    fs::create_dir_all(&transfer_dir).map_err(|e| {
        tracing::error!("Failed to create transfer directory: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Write chunk to file with offset as name
    let chunk_path = transfer_dir.join(format!("chunk_{:010}", offset));
    let mut file = fs::File::create(&chunk_path).map_err(|e| {
        tracing::error!("Failed to create chunk file: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    file.write_all(&chunk_data).map_err(|e| {
        tracing::error!("Failed to write chunk: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::debug!("Chunk at offset {} saved successfully", offset);

    Ok(Json(json!({
        "status": "ok",
        "offset": offset,
        "bytes_received": chunk_data.len(),
    })))
}

/// Finalize transfer - assemble all chunks into final file
pub async fn finalize_transfer(
    State(state): State<AppState>,
    Json(payload): Json<FinalizeRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let transfer_id = &payload.transfer_id;
    tracing::info!("Finalizing transfer: {}", transfer_id);

    let upload_dir = PathBuf::from("./data/uploads");
    let transfer_dir = upload_dir.join(transfer_id);

    if !transfer_dir.exists() {
        tracing::error!("Transfer directory not found: {:?}", transfer_dir);
        return Err(StatusCode::NOT_FOUND);
    }

    // Get all chunk files sorted by offset
    let mut chunk_files: Vec<_> = fs::read_dir(&transfer_dir)
        .map_err(|e| {
            tracing::error!("Failed to read transfer directory: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("chunk_")
        })
        .collect();

    chunk_files.sort_by_key(|entry| entry.file_name());

    tracing::info!("Found {} chunks to assemble", chunk_files.len());

    // Assemble chunks into final file
    let final_path = transfer_dir.join("file");
    let mut final_file = fs::File::create(&final_path).map_err(|e| {
        tracing::error!("Failed to create final file: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut total_bytes = 0;
    for chunk_file in &chunk_files {
        let chunk_data = fs::read(chunk_file.path()).map_err(|e| {
            tracing::error!("Failed to read chunk: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        
        final_file.write_all(&chunk_data).map_err(|e| {
            tracing::error!("Failed to write to final file: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        total_bytes += chunk_data.len();

        // Delete chunk after adding to final file
        fs::remove_file(chunk_file.path()).ok();
    }

    tracing::info!(
        "File assembled successfully at: {:?} ({} bytes)",
        final_path,
        total_bytes
    );

    // Update transfer status in database
    if let Err(e) = state
        .db
        .update_transfer_status(transfer_id, "completed", Some(chrono::Utc::now()))
        .await
    {
        tracing::error!("Failed to update transfer status: {}", e);
    }

    Ok(Json(json!({
        "status": "completed",
        "transfer_id": transfer_id,
        "total_bytes": total_bytes,
        "file_path": final_path.to_string_lossy(),
    })))
}

/// Get upload status
pub async fn get_upload_status(
    State(_state): State<AppState>,
    Path(transfer_id): Path<String>,
) -> impl IntoResponse {
    let upload_dir = PathBuf::from("./data/uploads");
    let transfer_dir = upload_dir.join(&transfer_id);

    if !transfer_dir.exists() {
        return Json(json!({
            "transfer_id": transfer_id,
            "status": "not_found",
        }));
    }

    // Count chunks
    let chunk_count = fs::read_dir(&transfer_dir)
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_name()
                        .to_string_lossy()
                        .starts_with("chunk_")
                })
                .count()
        })
        .unwrap_or(0);

    Json(json!({
        "transfer_id": transfer_id,
        "status": "uploading",
        "chunks_received": chunk_count,
    }))
}
