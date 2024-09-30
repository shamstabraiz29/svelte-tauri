mod branch_detail;
pub mod commit_model_changes;
mod node_meta;
mod rel_meta;
pub mod removed_viewport_items;
pub mod removed_viewports;
pub mod upserted_viewport_items;
pub mod upserted_viewports;
mod viewport_item_meta;
mod viewport_meta;

use serde::Serialize;
use specta::Type;

use self::branch_detail::BranchDetail;

#[derive(Serialize, Clone, Debug, Type, tauri_specta::Event)]
#[serde(tag = "type")]

pub enum BranchDetailEvent {
    #[serde(rename = "branch")]
    Branch {
        #[serde(rename = "branchDetail")]
        branch_detail: BranchDetail,
    },
    Clear,
}
