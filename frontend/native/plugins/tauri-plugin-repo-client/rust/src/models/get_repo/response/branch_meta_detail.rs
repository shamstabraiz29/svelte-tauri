use std::collections::BTreeMap;

use common_dtos::commit_client::types::repo::branch_meta_dto::BranchMetaDto;
use common_simple_types::ag_id::AgId;
use serde::Serialize;
use specta::Type;
use specta_util::Unknown;

use super::ParentBranchPointDetail;

use serde_json::Value;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct BranchMetaDetail {
    #[specta(type=String)]
    pub id: AgId,
    pub name: String,
    pub parent_branch_point: Option<ParentBranchPointDetail>,
    #[specta(type = BTreeMap<String, Unknown>)]
    pub properties: BTreeMap<String, Value>,
}

impl From<BranchMetaDto> for BranchMetaDetail {
    fn from(branch: BranchMetaDto) -> Self {
        BranchMetaDetail {
            id: branch.id,
            name: branch.name,
            parent_branch_point: branch
                .parent_branch_point
                .map(|parent_branch_point| parent_branch_point.into()),
            properties: branch.properties,
        }
    }
}
