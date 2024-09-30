// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/viewport_create.rs
use common_libs_editor_api_client::{EditorApiClient, ViewportCreateCmds};
use frontend_tauri_plugins_common::error::{Error, Result};
use log::{debug, error, info, warn};
use tauri::{AppHandle, Runtime, State};

use crate::{
    models::viewport_create::{
        request::UiViewportCreateRequest, response::UiViewportCreateResponse,
    },
    reorder::{branch_ag_dto_subject::BranchAgDtoSubject, commit_commands},
};

#[tauri::command]
#[specta::specta]
pub async fn viewport_create<R: Runtime>(
    app: AppHandle<R>,
    editor_api_client: State<'_, EditorApiClient>,
    branch_ag_dto_subject: State<'_, BranchAgDtoSubject>,
    request: UiViewportCreateRequest,
) -> Result<UiViewportCreateResponse> {
    info!(
        "Starting viewport_create with request ID: {}",
        request.req_id
    );
    debug!("Received viewport create request: {:?}", request);

    let branch_id = match branch_ag_dto_subject.get_model_id() {
        Ok(Some(id)) => {
            debug!("Retrieved branch ID: {}", id);
            id
        }
        Ok(None) => {
            warn!(
                "No open branch was found for request ID: {}",
                request.req_id
            );
            return Err(Error {
                req_id: request.req_id.clone(),
                message: "No open branch was found".to_string(),
            });
        }
        Err(e) => {
            error!(
                "Error getting model ID for request ID: {}. Error: {}",
                request.req_id, e
            );
            return Err(Error {
                req_id: request.req_id.clone(),
                message: e.to_string(),
            });
        }
    };

    let model_root_node_id = match branch_ag_dto_subject.get_model_root_node_id() {
        Ok(Some(id)) => {
            debug!("Retrieved model root node ID: {}", id);
            id
        }
        Ok(None) => {
            warn!(
                "No model root node ID found for request ID: {}",
                request.req_id
            );
            return Err(Error {
                req_id: request.req_id.clone(),
                message: "No open branch was found".to_string(),
            });
        }
        Err(e) => {
            error!(
                "Error getting model root node ID for request ID: {}. Error: {}",
                request.req_id, e
            );
            return Err(Error {
                req_id: request.req_id.clone(),
                message: e.to_string(),
            });
        }
    };

    info!("Creating ViewportCreateCmds");
    let viewport_create_cmds = ViewportCreateCmds {
        branch_id,
        model_root_node_id,
        name: request.name.clone(),
        viewport_type: request.viewport_type.clone(),
        properties: request.config.clone(),
    };

    debug!("Generating viewport create commands");
    let commands = editor_api_client.viewport_create_cmds(viewport_create_cmds);

    info!("Committing viewport create commands");
    match commit_commands(app, commands).await {
        Ok(_) => {
            info!(
                "Successfully created viewport for request ID: {}",
                request.req_id
            );
            Ok(UiViewportCreateResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => {
            error!(
                "Error committing viewport create commands for request ID: {}. Error: {}",
                request.req_id, e
            );
            Err(Error {
                req_id: request.req_id.clone(),
                message: e.to_string(),
            })
        }
    }
}
