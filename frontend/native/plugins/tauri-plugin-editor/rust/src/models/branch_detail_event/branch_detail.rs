use common_dtos::commit_client::types::branch::branch_dto::BranchAgDto;

use std::collections::HashMap;

use serde::Serialize;
use specta::Type;

use super::{viewport_item_meta::ViewportItemMeta, viewport_meta::ViewportMeta};

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct BranchDetail {
    pub id: String,
    pub commit_id: String,
    pub acct_id: String,
    pub repo_id: String,
    pub viewports: HashMap<String, ViewportMeta>,
    pub viewport_items: HashMap<String, Vec<ViewportItemMeta>>,
}

impl From<BranchAgDto> for BranchDetail {
    fn from(branch: BranchAgDto) -> Self {
        let viewports = branch
            .viewports
            .into_iter()
            .map(|(viewport_id, viewport_dto)| {
                (viewport_id.to_string(), ViewportMeta::from(viewport_dto))
            })
            .collect();

        let mut viewport_items: HashMap<String, Vec<ViewportItemMeta>> = HashMap::new();

        for viewport_item in branch.viewport_items.into_values() {
            let viewport_id = viewport_item.viewport_id.to_string();
            match viewport_items.get_mut(&viewport_id) {
                Some(viewport_items) => {
                    viewport_items.push(viewport_item.into());
                }
                None => {
                    viewport_items.insert(viewport_id, vec![viewport_item.into()]);
                }
            }
        }

        BranchDetail {
            id: branch.id.to_string(),
            acct_id: branch.acct_id.to_string(),
            repo_id: branch.repo_id.to_string(),
            commit_id: branch.commit_id.to_string(),
            viewports,
            viewport_items,
        }
    }
}
