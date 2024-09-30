#[tauri::command]
#[specta::specta]
pub fn log_message(level: String, message: String) {
    match level.to_ascii_lowercase().as_str() {
        "trace" => log::trace!("UI: {}", message),
        "debug" => log::debug!("UI: {}", message),
        "info" => log::info!("UI: {}", message),
        "warn" => log::warn!("UI: {}", message),
        "error" => log::error!("UI: {}", message),
        _ => log::info!("UI: Unknown log level: {}", message),
    }
}
