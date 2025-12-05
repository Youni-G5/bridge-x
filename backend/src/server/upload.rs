//! File upload handling

use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::AppState;

/// Upload file chunk
pub async fn upload_chunk(
    State(state): State<AppState>,
    Path(transfer_id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("Upload chunk for transfer: {}", transfer_id);

    // Get chunk metadata from headers
    let chunk_index = headers
        .get("X-Chunk-Index")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);

    let total_chunks = headers
        .get("X-Total-Chunks")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1);

    tracing::debug!(
        "Receiving chunk {}/{} ({} bytes)",
        chunk_index + 1,
        total_chunks,
        body.len()
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

    // Write chunk to file
    let chunk_path = transfer_dir.join(format!("chunk_{:06}", chunk_index));
    let mut file = fs::File::create(&chunk_path).map_err(|e| {
        tracing::error!("Failed to create chunk file: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    file.write_all(&body).map_err(|e| {
        tracing::error!("Failed to write chunk: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::debug!("Chunk {} saved successfully", chunk_index);

    // Check if this is the last chunk
    if chunk_index + 1 == total_chunks {
        tracing::info!("All chunks received, assembling file...");

        // Assemble chunks into final file
        match assemble_chunks(&transfer_dir, total_chunks) {
            Ok(_) => {
                tracing::info!("File assembled successfully");

                // Update transfer status in database
                if let Err(e) = state
                    .db
                    .update_transfer_status(&transfer_id, "completed", Some(chrono::Utc::now()))
                    .await
                {
                    tracing::error!("Failed to update transfer status: {}", e);
                }
            }
            Err(e) => {
                tracing::error!("Failed to assemble file: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    Ok(Json(json!({
        "status": "ok",
        "chunk": chunk_index,
        "total": total_chunks,
    })))
}

/// Assemble chunks into final file
fn assemble_chunks(transfer_dir: &PathBuf, total_chunks: usize) -> std::io::Result<()> {
    let final_path = transfer_dir.join("file");
    let mut final_file = fs::File::create(&final_path)?;

    for i in 0..total_chunks {
        let chunk_path = transfer_dir.join(format!("chunk_{:06}", i));
        let chunk_data = fs::read(&chunk_path)?;
        final_file.write_all(&chunk_data)?;

        // Delete chunk after adding to final file
        fs::remove_file(&chunk_path)?;
    }

    tracing::info!("File assembled at: {:?}", final_path);
    Ok(())
}

/// Get upload status
pub async fn get_upload_status(
    State(_state): State<AppState>,
    Path(transfer_id): Path<String>,
) -> impl IntoResponse {
    // TODO: Implement status checking
    Json(json!({
        "transfer_id": transfer_id,
        "status": "uploading",
        "progress": 0.5,
    }))
}
