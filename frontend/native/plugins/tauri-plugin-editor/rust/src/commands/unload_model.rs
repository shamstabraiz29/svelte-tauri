// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/unload_model.rs
use frontend_tauri_plugins_common::error::{Error, Result};
use log::{debug, error, info};
use tauri::State;

use crate::{
    models::unload_model::{request::UnloadModelRequest, response::UnloadModelResponse},
    reorder::branch_ag_dto_subject::BranchAgDtoSubject,
};

#[tauri::command]
#[specta::specta]
pub async fn unload_model(
    branch_ag_dto_subject: State<'_, BranchAgDtoSubject>,
    request: UnloadModelRequest,
) -> Result<UnloadModelResponse> {
    info!("Starting unload_model with request ID: {}", request.req_id);
    debug!("Received unload model request: {:?}", request);

    match branch_ag_dto_subject.set_value(None).await {
        Ok(_) => {
            info!(
                "Successfully unloaded model for request ID: {}",
                request.req_id
            );
            debug!("Branch AG DTO subject value set to None");
            Ok(UnloadModelResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => {
            error!(
                "Error setting branch_ag_dto_subject to None. Request ID: {}, Error: {:?}",
                request.req_id, e
            );
            Err(Error {
                req_id: request.req_id,
                message: "Error closing the opened branch".to_string(),
            })
        }
    }
}
