use std::collections::HashMap;

use common_simple_types::ag_id::AgId;
use serde::Deserialize;
use serde_json::Value;
use specta::Type;
use specta_util::Unknown;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct SaveUpdatesRequest {
    pub req_id: String,
    pub tracked_viewport_items: Vec<TrackedViewportItem>,
}

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TrackedViewportItem {
    #[specta(type=String)]
    pub vp_id: AgId,
    #[specta(type=String)]
    pub m_id: AgId,
    pub vp_delta: ViewportChanges,
    pub m_delta: ModelChanges,
}

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ModelChanges {
    pub removed_properties: Vec<String>,
    #[specta(type=HashMap<String, Unknown>)]
    pub upserted_properties: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ViewportChanges {
    pub removed_properties: Vec<String>,
    #[specta(type=HashMap<String, Unknown>)]
    pub upserted_properties: HashMap<String, Value>,
}
