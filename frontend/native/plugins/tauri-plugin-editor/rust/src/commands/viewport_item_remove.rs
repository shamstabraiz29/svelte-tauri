// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/viewport_item_remove.rs

use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::error::{Error, Result};
use tauri::{AppHandle, Runtime, State};

use crate::{
    models::viewport_item_remove::{
        request::ViewportItemRemoveRequest, response::ViewportItemRemoveResponse,
    },
    reorder::{branch_ag_dto_subject::BranchAgDtoSubject, commit_commands},
};
use common_commands::model::viewport_item::RemoveData as ViewportItemRemoveData;

#[tauri::command]
#[specta::specta]
pub async fn viewport_item_remove<R: Runtime>(
    app: AppHandle<R>,
    branch_ag_dto_subject: State<'_, BranchAgDtoSubject>,
    request: ViewportItemRemoveRequest,
) -> Result<ViewportItemRemoveResponse> {
    log::info!(
        "Starting viewport_item_remove with request ID: {}",
        request.req_id
    );
    log::debug!("Received viewport item remove request: {:?}", request);

    let mut branch_id: Result<AgId> = Err(Error {
        req_id: request.req_id.clone(),
        message: "No open branch was found".to_string(),
    });

    branch_ag_dto_subject
        .eval_with_value(|branch_ag_dto| {
            if let Some(dto) = branch_ag_dto {
                branch_id = Ok(dto.id.to_owned());
                if !dto.viewport_items.contains_key(&request.viewport_item_id) {
                    branch_id = Err(Error {
                        req_id: request.req_id.clone(),
                        message: "Target viewport item not found".to_string(),
                    });
                };
            }
        })
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    let mut commands = Vec::new();

    let branch_id = branch_id?;

    let remove_data = ViewportItemRemoveData::new(branch_id, request.viewport_item_id);
    commands.push(remove_data.into());

    log::debug!("viewport item remove command: {:#?}", commands);

    log::info!("Committing viewport item remove command");
    match commit_commands(app, commands).await {
        Ok(_) => {
            log::info!(
                "Successfully removed viewport item for request ID: {}",
                request.req_id
            );
            Ok(ViewportItemRemoveResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => {
            log::error!(
                "Error committing viewport item remove command for request ID: {}. Error: {}",
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
