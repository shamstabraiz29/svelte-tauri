use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "type")]
pub struct AccountSetResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
}
