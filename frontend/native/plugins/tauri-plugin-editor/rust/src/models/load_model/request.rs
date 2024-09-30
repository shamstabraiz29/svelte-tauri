use common_simple_types::{ag_id::AgId, commit_addr::CommitAddr};
use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct LoadModelRequest {
    pub req_id: String,
    #[specta(type=String)]
    pub branch_id: AgId,
    #[specta(type=String)]
    pub address: CommitAddr,
}
