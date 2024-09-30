use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct RemoveFolderResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
}
