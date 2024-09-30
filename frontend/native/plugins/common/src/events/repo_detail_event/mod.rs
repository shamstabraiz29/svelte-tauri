use common_dtos::commit_client::types::repo::{
    branch_meta_dto::BranchMetaDto, repo_dto::RepoAgDto,
};
use serde_json::Value as JsonValue;
use specta::Type;
use specta_util::Unknown;
use std::collections::{BTreeMap, HashMap};
use tauri_specta::Event;

use serde::Serialize;

#[derive(Serialize, Clone, Debug, Type, Event)]
#[serde(tag = "type")]

pub enum RepoDetailEvent {
    #[serde(rename = "repo")]
    Repo {
        #[serde(rename = "repoDetail")]
        repo_detail: RepoDetail,
    },
    Clear,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct RepoDetail {
    pub id: String,
    pub acct_id: String,
    pub commit_id: String,
    pub branches: HashMap<String, BranchMeta>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct BranchMeta {
    pub id: String,
    pub name: String,
    pub parent_branch_point: Option<ParentBranchPoint>,

    #[specta(type = BTreeMap<String, Unknown>)]
    pub properties: BTreeMap<String, JsonValue>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct ParentBranchPoint {
    pub branch_id: String,
    pub commit_id: String,
}

impl From<RepoAgDto> for RepoDetail {
    fn from(repo: RepoAgDto) -> Self {
        let branches = repo
            .branches
            .into_iter()
            .map(|(branch_id, branch_meta)| (branch_id.to_string(), BranchMeta::from(branch_meta)))
            .collect();

        RepoDetail {
            id: repo.id.to_string(),
            acct_id: repo.acct_id.to_string(),
            commit_id: repo.commit_id.to_string(),
            branches,
        }
    }
}

impl From<BranchMetaDto> for BranchMeta {
    fn from(branch: BranchMetaDto) -> Self {
        let parent_branch_point =
            branch
                .parent_branch_point
                .map(|parent_branch_point| ParentBranchPoint {
                    branch_id: parent_branch_point.branch_id.to_string(),
                    commit_id: parent_branch_point.commit_id.to_string(),
                });

        BranchMeta {
            id: branch.id.to_string(),
            name: branch.name,
            parent_branch_point,
            properties: branch.properties,
        }
    }
}
