use common_simple_types::ag_id::AgId;
use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct ViewportRemovePropertiesRequest {
    pub req_id: String,
    #[specta(type=String)]
    pub viewport_id: AgId,
    pub properties: Vec<String>,
}
