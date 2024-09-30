// frontend/tauri/plugins/tauri-plugin-editor/rust/src/reorder/send_model_commands_to_cloud.rs
use common_commands::model::ModelCommand;
use common_dtos::{
    commit_client::{
        types::{branch::branch_dto::BranchAgDto, MHashable},
        AgOperationStatus,
    },
    editor_client::branch_model_modify::request::BranchModelModifyRequest,
};
use common_libs_editor_api_client::EditorApiClient;

use frontend_tauri_plugins_common::{
    dto_util::update_ag_dto_from_status::patch_dto, states::ag_commit_state::AgCommitState,
    types::commit_info::CommitInfo,
};
use log::{debug, error, info};
use tauri::{AppHandle, Manager, Runtime};

use crate::{error::InternalApplicationError, EditorError};

use super::branch_ag_dto_subject::BranchAgDtoSubject;

pub async fn send_model_commands_to_cloud<R: Runtime>(
    app: AppHandle<R>,
    access_token: &str,
    commands: Vec<ModelCommand>,
) -> Result<BranchAgDto, EditorError> {
    info!("Starting send_model_commands_to_cloud");
    debug!("Number of commands to send: {}", commands.len());

    let branch_ag_dto_subject = app.state::<BranchAgDtoSubject>();

    debug!("Retrieving branch AG DTO");
    let branch_ag_dto = match branch_ag_dto_subject.get_branch_ag_dto()? {
        Some(dto) => {
            debug!("Successfully retrieved branch AG DTO");
            dto
        }
        None => {
            error!("No open branch was found! Missing snapshot.");
            return Err(EditorError::RecoverableError(
                "No open branch was found! Missing snapshot.".to_owned(),
            ));
        }
    };

    debug!("Extracting AgCommitState<BranchAgDto>");
    let branch_commit_state = app.state::<AgCommitState<BranchAgDto>>();

    debug!("Retrieving AG commit info");
    let commit_info = match branch_commit_state.get_ag_commit_info() {
        Some(c) => {
            debug!("Successfully retrieved commit info");
            c
        }
        None => {
            error!("Commit info not found");
            return Err(EditorError::RecoverableError(
                "Commit info not found".to_string(),
            ));
        }
    };

    let branch_model_modify_request = BranchModelModifyRequest {
        repo_id: branch_ag_dto.repo_id.to_owned(),
        branch_id: branch_ag_dto.id.to_owned(),
        next_commit_id: commit_info.next_commit_id,
        commands,
    };

    let branch_id = branch_ag_dto.id.to_owned();
    debug!(
        "Prepared branch model modify request for branch ID: {}",
        branch_id
    );

    info!("Sending branch_model_modify request to API");
    let editor_api_client = app.state::<EditorApiClient>();
    let response = match editor_api_client
        .branch_model_modify(access_token, branch_model_modify_request)
        .await
    {
        Ok(response) => {
            debug!("Successfully received response from branch_model_modify");
            response
        }
        Err(e) => {
            error!("Failed to send branch_model_modify request: {:?}", e);
            return Err(e.into()); // Convert the error type
        }
    };

    debug!("Processing branch_model_modify response");
    let ag_delta = match response.status {
        AgOperationStatus::Success { ag_deltas } => match ag_deltas.into_values().next() {
            Some(ag_delta) => {
                debug!("Successfully extracted AG delta from response");
                ag_delta
            }
            None => {
                error!("Received empty AG delta");
                return Err(InternalApplicationError::EmptyAgDelta.into());
            }
        },
        AgOperationStatus::Failure(message) => {
            error!(
                "Branch model modify operation failed. Branch ID: {}, Message: {}",
                branch_id, message
            );
            return Err(
                InternalApplicationError::BranchModelModifyError { branch_id, message }.into(),
            );
        }
    };

    info!("Patching AG DTO with received delta");
    let patched_ag_dto: BranchAgDto = patch_dto(&branch_ag_dto, &ag_delta.patch);

    debug!("Verifying AG DTO hash");
    if ag_delta.m_hash.clone() != patched_ag_dto.m_hash() {
        error!("Branch AG hash mismatch detected");
        return Err(InternalApplicationError::BranchAgHashMismatch.into());
    }

    let commit_info = CommitInfo {
        commit_id: ag_delta.commit_id.clone(),
        next_commit_id: ag_delta.next_commit_id.clone(),
        _phantom: std::marker::PhantomData,
    };
    debug!("Setting new AG commit info: {:?}", commit_info);
    branch_commit_state.set_ag_commit_info(Some(commit_info));

    info!("Successfully completed send_model_commands_to_cloud");
    Ok(patched_ag_dto)
}
