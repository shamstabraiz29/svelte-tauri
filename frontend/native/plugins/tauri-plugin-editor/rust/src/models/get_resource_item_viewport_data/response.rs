use serde::Serialize;
use serde_json::Value as JsonValue;
use specta::Type;
use specta_util::Unknown;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceItemViewportDataResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
    pub viewport_data: ResourceItemViewportData,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ResourceItemViewportData {
    ViewportData {
        #[specta(type = Unknown)]
        value: JsonValue,
    },
    NoneFound,
}
