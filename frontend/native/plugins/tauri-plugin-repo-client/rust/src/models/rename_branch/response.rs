use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]

pub struct RenameBranchResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
}
