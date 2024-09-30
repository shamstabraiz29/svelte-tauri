use common_simple_types::ag_id::AgId;
use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct ViewportItemRemoveRequest {
    pub req_id: String,
    #[specta(type=String)]
    pub viewport_item_id: AgId,
}
