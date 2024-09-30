// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/node_remove_properties.rs

use std::collections::HashSet;

use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::error::{Error, Result};
use tauri::{AppHandle, Runtime, State};

use crate::{
    models::model_node_remove_properties::{
        request::ModelNodeRemovePropertiesRequest, response::ModelNodeRemovePropertiesResponse,
    },
    reorder::{branch_ag_dto_subject::BranchAgDtoSubject, commit_commands},
};
use common_commands::model::node::RemovePropertiesData as NodeRemovePropertiesData;

#[tauri::command]
#[specta::specta]
pub async fn model_node_remove_properties<R: Runtime>(
    app: AppHandle<R>,
    branch_ag_dto_subject: State<'_, BranchAgDtoSubject>,
    request: ModelNodeRemovePropertiesRequest,
) -> Result<ModelNodeRemovePropertiesResponse> {
    log::info!(
        "Starting model_node_remove_properties with request ID: {}",
        request.req_id
    );
    log::debug!("Received node remove properties request: {:?}", request);

    let mut branch_id: Result<AgId> = Err(Error {
        req_id: request.req_id.clone(),
        message: "No open branch was found".to_string(),
    });

    let mut props_to_remove: std::result::Result<HashSet<String>, Error> = Err(Error {
        req_id: request.req_id.clone(),
        message: "The target node was not found".to_string(),
    });

    branch_ag_dto_subject
        .eval_with_value(|branch_ag_dto| {
            if let Some(dto) = branch_ag_dto {
                branch_id = Ok(dto.id.to_owned());
                if let Some(target_node) = dto.nodes.get(&request.model_item_id) {
                    let mut props_in_node = HashSet::new();
                    for property in &request.properties {
                        if target_node.properties.contains_key(property) {
                            props_in_node.insert(property.to_owned());
                        }
                    }
                    props_to_remove = Ok(props_in_node);
                }
            }
        })
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    let props_to_remove = props_to_remove?;

    if props_to_remove.is_empty() {
        log::debug!("No properties to remove for request ID: {}", request.req_id);
        return Ok(ModelNodeRemovePropertiesResponse {
            req_id: request.req_id,
        });
    }

    let mut commands = Vec::new();

    let branch_id = branch_id?;

    let remove_data = NodeRemovePropertiesData {
        branch_id: branch_id.to_owned(),
        node_id: request.model_item_id,
        properties: props_to_remove.into_iter().collect(),
    };
    commands.push(remove_data.into());

    log::debug!("node remove properties commands: {:#?}", commands);

    log::info!("Committing node remove properties commands");
    match commit_commands(app, commands).await {
        Ok(_) => {
            log::info!(
                "Successfully removed node properties for request ID: {}",
                request.req_id
            );
            Ok(ModelNodeRemovePropertiesResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => {
            log::error!(
                "Error committing node remove properties commands for request ID: {}. Error: {}",
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
