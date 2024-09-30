use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "type", rename_all = "camelCase")]

pub struct CustomCursorResponse {
    pub req_id: String,
    pub svg_uri: String,
}
