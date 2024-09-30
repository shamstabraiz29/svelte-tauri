// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/viewport_item_remove_properties.rs
use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::error::{Error, Result};
use tauri::{AppHandle, Runtime, State};

use crate::{
    models::viewport_item_remove_properties::{
        request::ViewportItemRemovePropertiesRequest,
        response::ViewportItemRemovePropertiesResponse,
    },
    reorder::{branch_ag_dto_subject::BranchAgDtoSubject, commit_commands},
};
use common_commands::model::viewport_item::RemovePropertiesData as ViewportItemRemovePropertiesData;

#[tauri::command]
#[specta::specta]
pub async fn viewport_item_remove_properties<R: Runtime>(
    app: AppHandle<R>,
    branch_ag_dto_subject: State<'_, BranchAgDtoSubject>,
    request: ViewportItemRemovePropertiesRequest,
) -> Result<ViewportItemRemovePropertiesResponse> {
    log::info!(
        "Starting viewport_item_remove_properties with request ID: {}",
        request.req_id
    );
    log::debug!(
        "Received viewport item remove properties request: {:?}",
        request
    );

    let mut branch_id: Result<AgId> = Err(Error {
        req_id: request.req_id.clone(),
        message: "No open branch was found".to_string(),
    });

    let mut removed_properties: Vec<String> = Vec::new();

    branch_ag_dto_subject
        .eval_with_value(|branch_ag_dto| {
            //TODO: Modify this code so that it is not tied to the structure of the viewport item properties
            if let Some(dto) = branch_ag_dto {
                branch_id = Ok(dto.id.to_owned());
                let target_viewport_item_properties =
                    match dto.viewport_items.get(&request.viewport_item_id) {
                        Some(item) => &item.properties,
                        None => {
                            branch_id = Err(Error {
                                req_id: request.req_id.clone(),
                                message: "Target viewport item not found".to_string(),
                            });
                            return;
                        }
                    };
                for prop in request.properties {
                    if target_viewport_item_properties.contains_key(&prop) {
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
        return Ok(ViewportItemRemovePropertiesResponse {
            req_id: request.req_id,
        });
    }

    let mut commands = Vec::new();

    let branch_id = branch_id?;

    let remove_data = ViewportItemRemovePropertiesData::new(
        branch_id,
        request.viewport_item_id,
        removed_properties,
    );
    commands.push(remove_data.into());

    log::debug!("viewport item remove properties commands: {:#?}", commands);

    log::info!("Committing viewport item remove properties commands");
    match commit_commands(app, commands).await {
        Ok(_) => {
            log::info!(
                "Successfully removed viewport item properties for request ID: {}",
                request.req_id
            );
            Ok(ViewportItemRemovePropertiesResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => {
            log::error!("Error committing viewport item remove properties commands for request ID: {}. Error: {}", request.req_id, e);
            Err(Error {
                req_id: request.req_id.clone(),
                message: e.to_string(),
            })
        }
    }
}
