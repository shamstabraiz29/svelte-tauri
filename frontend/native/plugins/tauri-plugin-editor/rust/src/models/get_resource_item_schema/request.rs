use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceItemSchemaRequest {
    pub req_id: String,
    pub resource_item_type: String,
}
