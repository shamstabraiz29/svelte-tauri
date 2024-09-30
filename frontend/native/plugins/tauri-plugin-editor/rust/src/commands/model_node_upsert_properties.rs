// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/node_upsert_properties.rs

use std::sync::Arc;

use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::error::{Error, Result};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::{
    models::model_node_upsert_properties::{
        request::ModelNodeUpsertPropertiesRequest, response::ModelNodeUpsertPropertiesResponse,
    },
    reorder::{
        branch_ag_dto_subject::BranchAgDtoSubject, commit_commands,
        head_nmg_process::HeadNmgProcessSubject,
    },
    types::node_component_state::NodeComponentState,
};
use common_commands::model::node::UpsertPropertiesData as NodeUpsertPropertiesData;

#[tauri::command]
#[specta::specta]
pub async fn model_node_upsert_properties<R: Runtime>(
    app: AppHandle<R>,
    branch_ag_dto_subject: State<'_, BranchAgDtoSubject>,
    node_template_state: State<'_, NodeComponentState>,
    head_nmg_process: State<'_, Arc<HeadNmgProcessSubject>>,
    request: ModelNodeUpsertPropertiesRequest,
) -> Result<ModelNodeUpsertPropertiesResponse> {
    log::info!(
        "Starting model_node_upsert_properties with request ID: {}",
        request.req_id
    );
    log::debug!("Received node upsert properties request: {:?}", request);

    let mut branch_id: Result<AgId> = Err(Error {
        req_id: request.req_id.clone(),
        message: "No open branch was found".to_string(),
    });

    let mut node_type = Err(Error {
        req_id: request.req_id.clone(),
        message: "The target node was not found".to_string(),
    });

    branch_ag_dto_subject
        .eval_with_value(|branch_ag_dto| {
            if let Some(dto) = branch_ag_dto {
                branch_id = Ok(dto.id.to_owned());
                if let Some(node) = dto.nodes.get(&request.model_item_id) {
                    node_type = Ok(node.node_type.clone());
                }
            }
        })
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    let node_type = node_type?;

    let nmg = head_nmg_process.get_model();

    let properties = serde_json::to_string(&request.properties).map_err(|e| Error {
        req_id: request.req_id.clone(),
        message: e.to_string(),
    })?;

    node_template_state
        .validate_properties(app.app_handle(), &node_type, &properties, nmg)
        .await
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    if request.properties.is_empty() {
        log::debug!("No properties to upsert for request ID: {}", request.req_id);
        return Ok(ModelNodeUpsertPropertiesResponse {
            req_id: request.req_id,
        });
    }

    let mut commands = Vec::new();

    let branch_id = branch_id?;

    let upsert_data = NodeUpsertPropertiesData {
        branch_id: branch_id.to_owned(),
        node_id: request.model_item_id,
        properties: request.properties,
    };
    commands.push(upsert_data.into());

    log::debug!("node upsert properties commands: {:#?}", commands);

    log::info!("Committing node upsert properties commands");
    match commit_commands(app, commands).await {
        Ok(_) => {
            log::info!(
                "Successfully upsertd node properties for request ID: {}",
                request.req_id
            );
            Ok(ModelNodeUpsertPropertiesResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => {
            log::error!(
                "Error committing node upsert properties commands for request ID: {}. Error: {}",
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
