// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/custom_cursor.rs
use common_custom_cursor::generate_custom_cursor;
use common_custom_cursor_client::CustomCursorClient;
use frontend_tauri_plugins_common::error::{Error, Result};
use log::{debug, error, info, trace};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::models::custom_cursor::{request::CustomCursorRequest, response::CustomCursorResponse};

static ICON_CACHE: LazyLock<Mutex<HashMap<String, Vec<u8>>>> = LazyLock::new(|| {
    info!("Initializing ICON_CACHE");
    Mutex::new(HashMap::new())
});

#[tauri::command]
#[specta::specta]
pub async fn custom_cursor(request: CustomCursorRequest) -> Result<CustomCursorResponse> {
    info!(
        "Starting custom_cursor function with request ID: {}",
        request.req_id
    );
    debug!("Received custom cursor request: {:?}", request);

    let icon_cursor = fetch_or_generate_cursor(&request.icon_url, &request.req_id).await?;

    info!(
        "Custom cursor generation completed successfully. Request ID: {}",
        request.req_id
    );

    Ok(CustomCursorResponse {
        req_id: request.req_id,
        svg_uri: String::from_utf8_lossy(&icon_cursor).into_owned(),
    })
}

async fn fetch_or_generate_cursor(icon_url: &str, req_id: &str) -> Result<Vec<u8>> {
    trace!(
        "Entering fetch_or_generate_cursor. URL: {}, Request ID: {}",
        icon_url,
        req_id
    );

    if let Some(cached_cursor) = ICON_CACHE.lock().unwrap().get(icon_url).cloned() {
        debug!(
            "Using cached cursor for URL: {}. Request ID: {}",
            icon_url, req_id
        );
        return Ok(cached_cursor);
    }

    info!(
        "Cache miss. Fetching icon from URL: {}. Request ID: {}",
        icon_url, req_id
    );
    let icon = fetch_icon(icon_url, req_id).await?;

    debug!(
        "Successfully fetched icon. Size: {} bytes. Request ID: {}",
        icon.len(),
        req_id
    );

    info!("Generating custom cursor. Request ID: {}", req_id);
    let icon_cursor = generate_custom_cursor(&String::from_utf8_lossy(&icon));
    let icon_cursor_bytes = icon_cursor.into_bytes();
    debug!(
        "Successfully generated custom cursor. Length: {} bytes. Request ID: {}",
        icon_cursor_bytes.len(),
        req_id
    );

    // Cache the generated cursor
    ICON_CACHE
        .lock()
        .unwrap()
        .insert(icon_url.to_string(), icon_cursor_bytes.clone());
    debug!(
        "Cached cursor for URL: {}. Cache size: {}. Request ID: {}",
        icon_url,
        ICON_CACHE.lock().unwrap().len(),
        req_id
    );

    trace!("Exiting fetch_or_generate_cursor. Request ID: {}", req_id);
    Ok(icon_cursor_bytes)
}

async fn fetch_icon(icon_url: &str, req_id: &str) -> Result<Vec<u8>> {
    trace!(
        "Entering fetch_icon. URL: {}, Request ID: {}",
        icon_url,
        req_id
    );

    match CustomCursorClient::fetch_icon(icon_url).await {
        Ok(icon) => {
            info!(
                "Successfully fetched icon. URL: {}, Size: {} bytes, Request ID: {}",
                icon_url,
                icon.len(),
                req_id
            );
            Ok(icon.into())
        }
        Err(e) => {
            error!(
                "Failed to fetch icon. URL: {}, Error: {}, Request ID: {}",
                icon_url, e, req_id
            );
            Err(Error {
                req_id: req_id.to_owned(),
                message: e.to_string(),
            })
        }
    }
}
