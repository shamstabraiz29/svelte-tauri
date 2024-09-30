use std::collections::HashMap;

use serde::Serialize;
use specta::Type;

use super::viewport_item_meta::ViewportItemMeta;

#[derive(Serialize, Clone, Debug, Type, tauri_specta::Event, Default)]
#[serde(tag = "type")]
pub struct UpsertedViewportItemsEvent {
    pub viewport_hash_items: HashMap<String, Vec<ViewportItemMeta>>,
}
