use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct RemoveFolderPropertiesResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
}
