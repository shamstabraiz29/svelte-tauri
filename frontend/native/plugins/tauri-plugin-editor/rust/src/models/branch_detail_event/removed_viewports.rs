use serde::Serialize;
use specta::Type;

#[derive(Serialize, Clone, Debug, Type, tauri_specta::Event, Default)]
#[serde(tag = "type")]
pub struct RemovedViewportsEvent {
    pub removed_viewports_ids: Vec<String>,
}
