use std::collections::HashMap;

use common_dtos::commit_client::types::repo::repo_dto::RepoAgDto;
use common_simple_types::ag_id::AgId;
use serde::Serialize;
use specta::Type;

use super::BranchMetaDetail;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct RepoBranches {
    #[specta(type=String)]
    pub id: AgId,
    //
    pub branches: HashMap<String, BranchMetaDetail>,
}

impl From<RepoAgDto> for RepoBranches {
    fn from(repo: RepoAgDto) -> Self {
        RepoBranches {
            id: repo.id,
            branches: repo
                .branches
                .into_iter()
                .map(|(id, branch)| (id.to_string(), branch.into()))
                .collect(),
        }
    }
}
