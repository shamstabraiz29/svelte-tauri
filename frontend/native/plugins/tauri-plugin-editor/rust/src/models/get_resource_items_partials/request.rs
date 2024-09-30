use std::collections::HashSet;

use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceItemsPartialsRequest {
    pub req_id: String,
    pub viewport_type: String,
    #[specta(type=HashSet<String>)]
    pub resource_items_types: HashSet<String>,
}
