use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;
use specta::Type;
use specta_util::Unknown;

use crate::models::get_repo::response::ParentBranchPointDetail;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct CreateBranchRequest {
    pub req_id: String,
    pub name: String,
    pub parent_branch: Option<ParentBranchPointDetail>,
    #[specta(type=HashMap<String, Unknown>)]
    pub properties: Option<HashMap<String, Value>>,
}
