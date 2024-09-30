use std::collections::HashMap;

use common_simple_types::ag_id::AgId;
use serde::Deserialize;
use serde_json::Value;
use specta::Type;
use specta_util::Unknown;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct UpsertFolderPropertiesRequest {
    pub req_id: String,
    #[specta(type=String)]
    pub folder_id: AgId,
    #[specta(type=Unknown)]
    pub properties: HashMap<String, Value>,
}
