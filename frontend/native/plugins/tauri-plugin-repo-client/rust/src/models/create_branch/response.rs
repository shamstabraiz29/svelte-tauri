use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]

pub struct CreateBranchResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
}
