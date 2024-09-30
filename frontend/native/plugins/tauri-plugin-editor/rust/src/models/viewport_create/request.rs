use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;
use specta::Type;
use specta_util::Unknown;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct UiViewportCreateRequest {
    pub req_id: String,
    pub name: String,
    pub viewport_type: String,
    #[specta(type=HashMap<String, Unknown>)]
    pub config: Option<HashMap<String, Value>>,
}
