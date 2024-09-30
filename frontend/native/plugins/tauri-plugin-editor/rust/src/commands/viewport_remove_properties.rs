// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/viewport_remove_properties.rs
use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::error::{Error, Result};
use tauri::{AppHandle, Runtime, State};

use crate::{
    models::viewport_remove_properties::{
        request::ViewportRemovePropertiesRequest, response::ViewportRemovePropertiesResponse,
    },
    reorder::{branch_ag_dto_subject::BranchAgDtoSubject, commit_commands},
};
use common_commands::model::viewport::RemovePropertiesData as ViewportRemovePropertiesData;

#[tauri::command]
#[specta::specta]
pub async fn viewport_remove_properties<R: Runtime>(
    app: AppHandle<R>,
    branch_ag_dto_subject: State<'_, BranchAgDtoSubject>,
    request: ViewportRemovePropertiesRequest,
) -> Result<ViewportRemovePropertiesResponse> {
    log::info!(
        "Starting viewport_remove_properties with request ID: {}",
        request.req_id
    );
    log::debug!("Received viewport remove properties request: {:?}", request);

    let mut branch_id: Result<AgId> = Err(Error {
        req_id: request.req_id.clone(),
        message: "No open branch was found".to_string(),
    });

    let mut removed_properties: Vec<String> = Vec::new();

    branch_ag_dto_subject
        .eval_with_value(|branch_ag_dto| {
            if let Some(dto) = branch_ag_dto {
                branch_id = Ok(dto.id.to_owned());
                let viewport = match dto.viewports.get(&request.viewport_id) {
                    Some(viewport) => viewport,
                    None => {
                        branch_id = Err(Error {
                            req_id: request.req_id.clone(),
                            message: "Target viewport not found".to_string(),
                        });
                        return;
                    }
                };
                for prop in request.properties {
                    if viewport.properties.contains_key(&prop) {
                        removed_properties.push(prop);
                    }
                }
            }
        })
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    if removed_properties.is_empty() {
        log::debug!("No properties to remove for request ID: {}", request.req_id);
        return Ok(ViewportRemovePropertiesResponse {
            req_id: request.req_id,
        });
    }

    let mut commands = Vec::new();

    let branch_id = branch_id?;

    let remove_data =
        ViewportRemovePropertiesData::new(branch_id, request.viewport_id, removed_properties);
    commands.push(remove_data.into());

    log::debug!("viewport remove properties commands: {:#?}", commands);

    log::info!("Committing viewport remove properties commands");
    match commit_commands(app, commands).await {
        Ok(_) => {
            log::info!(
                "Successfully removed viewport properties for request ID: {}",
                request.req_id
            );
            Ok(ViewportRemovePropertiesResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => {
            log::error!("Error committing viewport remove properties commands for request ID: {}. Error: {}", request.req_id, e);
            Err(Error {
                req_id: request.req_id.clone(),
                message: e.to_string(),
            })
        }
    }
}
