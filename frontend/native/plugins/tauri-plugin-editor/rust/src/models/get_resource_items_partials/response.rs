use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value as JsonValue;
use specta::Type;
use specta_util::Unknown;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceItemsPartialsResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
    pub partials: ResourceItemsPartialsData,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ResourceItemsPartialsData {
    Partials(
        #[specta(type = HashMap<String, HashMap<String, Unknown>>)]
        HashMap<String, HashMap<String, JsonValue>>,
    ),
    NoneFound,
}
