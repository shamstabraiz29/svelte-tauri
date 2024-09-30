use std::collections::HashMap;

use serde::Serialize;
use specta::Type;

use super::viewport_meta::ViewportMeta;

#[derive(Serialize, Clone, Debug, Type, tauri_specta::Event, Default)]
#[serde(tag = "type")]
pub struct UpsertedViewportsEvent {
    pub viewports: HashMap<String, ViewportMeta>,
}
