use std::collections::HashMap;

use serde::Serialize;
use specta::Type;

#[derive(Serialize, Clone, Debug, Type, tauri_specta::Event, Default)]
#[serde(tag = "type")]
pub struct RemovedViewportItemsEvent {
    pub removed_viewport_items: HashMap<String, Vec<String>>,
}
