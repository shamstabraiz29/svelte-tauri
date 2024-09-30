// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/viewport_remove.rs

use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::error::{Error, Result};
use tauri::{AppHandle, Runtime, State};

use crate::{
    models::viewport_remove::{request::ViewportRemoveRequest, response::ViewportRemoveResponse},
    reorder::{branch_ag_dto_subject::BranchAgDtoSubject, commit_commands},
};
use common_commands::model::{
    viewport::RemoveData as ViewportRemoveData, viewport_item::RemoveData as ViewportItemRemoveData,
};

#[tauri::command]
#[specta::specta]
pub async fn viewport_remove<R: Runtime>(
    app: AppHandle<R>,
    branch_ag_dto_subject: State<'_, BranchAgDtoSubject>,
    request: ViewportRemoveRequest,
) -> Result<ViewportRemoveResponse> {
    log::info!(
        "Starting viewport_remove with request ID: {}",
        request.req_id
    );
    log::debug!("Received viewport remove request: {:?}", request);

    let mut branch_id: Result<AgId> = Err(Error {
        req_id: request.req_id.clone(),
        message: "No open branch was found".to_string(),
    });

    let mut removed_viewport_items: Vec<AgId> = Vec::new();

    branch_ag_dto_subject
        .eval_with_value(|branch_ag_dto| {
            if let Some(dto) = branch_ag_dto {
                branch_id = Ok(dto.id.to_owned());
                if !dto.viewports.contains_key(&request.viewport_id) {
                    branch_id = Err(Error {
                        req_id: request.req_id.clone(),
                        message: "Target viewport not found".to_string(),
                    });
                    return;
                };
                for viewport_item in dto.viewport_items.values() {
                    if viewport_item.viewport_id == request.viewport_id {
                        removed_viewport_items.push(viewport_item.id.to_owned());
                    }
                }
            }
        })
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    let mut commands = Vec::new();

    let branch_id = branch_id?;

    if !removed_viewport_items.is_empty() {
        for viewport_item_id in removed_viewport_items {
            let remove_data = ViewportItemRemoveData::new(branch_id.to_owned(), viewport_item_id);
            commands.push(remove_data.into());
        }
    }

    let remove_data = ViewportRemoveData::new(branch_id, request.viewport_id);
    commands.push(remove_data.into());

    log::debug!("viewport remove command: {:#?}", commands);

    log::info!("Committing viewport remove command");
    match commit_commands(app, commands).await {
        Ok(_) => {
            log::info!(
                "Successfully removed viewport for request ID: {}",
                request.req_id
            );
            Ok(ViewportRemoveResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => {
            log::error!(
                "Error committing viewport remove command for request ID: {}. Error: {}",
                request.req_id,
                e
            );
            Err(Error {
                req_id: request.req_id.clone(),
                message: e.to_string(),
            })
        }
    }
}
