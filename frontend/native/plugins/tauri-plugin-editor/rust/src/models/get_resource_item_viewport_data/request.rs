use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceItemViewportDataRequest {
    pub req_id: String,
    pub viewport_type: String,
    pub resource_item_type: String,
}
