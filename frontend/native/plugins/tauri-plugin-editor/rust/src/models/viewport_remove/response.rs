use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "type", rename_all = "camelCase")]

pub struct ViewportRemoveResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
}
