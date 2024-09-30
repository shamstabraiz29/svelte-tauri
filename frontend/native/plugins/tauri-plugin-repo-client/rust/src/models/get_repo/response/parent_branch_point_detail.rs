use common_dtos::commit_client::types::repo::branch_meta_dto::ParentBranchPointDto;
use common_simple_types::{ag_id::AgId, commit_id::CommitId};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct ParentBranchPointDetail {
    #[specta(type = String)]
    pub branch_id: AgId,
    #[specta(type = String)]
    pub commit_id: CommitId,
}

impl From<ParentBranchPointDto> for ParentBranchPointDetail {
    fn from(parent_branch_point: ParentBranchPointDto) -> Self {
        ParentBranchPointDetail {
            branch_id: parent_branch_point.branch_id,
            commit_id: parent_branch_point.commit_id,
        }
    }
}

impl From<ParentBranchPointDetail> for ParentBranchPointDto {
    fn from(parent_branch_point: ParentBranchPointDetail) -> Self {
        ParentBranchPointDto {
            branch_id: parent_branch_point.branch_id,
            commit_id: parent_branch_point.commit_id,
        }
    }
}
