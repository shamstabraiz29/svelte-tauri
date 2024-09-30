use serde::Serialize;
use std::fmt::Debug;

use specta::Type;

#[derive(Serialize, Clone, Debug, Type, tauri_specta::Event)]
#[serde(tag = "type")]
pub enum CommitInfoEvent {
    #[serde(rename = "commitInfo")]
    CommitInfo {
        #[serde(rename = "commitId")]
        commit_id: String,
        #[serde(rename = "nextCommitId")]
        next_commit_id: String,
    },
    #[serde(rename = "clear")]
    Clear,
}
