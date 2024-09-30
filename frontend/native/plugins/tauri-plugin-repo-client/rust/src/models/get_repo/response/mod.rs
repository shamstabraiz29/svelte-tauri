mod branch_meta_detail;
mod parent_branch_point_detail;
mod repo_detail;

pub use branch_meta_detail::BranchMetaDetail;
pub use parent_branch_point_detail::ParentBranchPointDetail;
pub use repo_detail::RepoBranches;
use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "type")]

pub struct RepoResponse {
    #[serde(rename = "repoDetail")]
    pub repo_detail: RepoBranches,
}
