// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/load_model.rs
use common_dtos::{
    commit_client::types::branch::branch_dto::BranchAgDto,
    editor_client::branch_get::request::BranchGetRequest,
};
use common_libs_editor_api_client::EditorApiClient;

use frontend_tauri_plugins_common::{
    dto_util::update_ag_dto_from_status::new_ag_dto_from_status,
    error::{Error, Result},
    notifying_sync::notifying_state::NotifyingState,
    states::ag_commit_state::AgCommitState,
    types::{
        bearer_tokens::BearerTokens, commit_info::CommitInfo, repo_detail_state::RepoDtoState,
    },
};
use tauri::{AppHandle, Manager, Runtime};

use crate::{
    models::load_model::{request::LoadModelRequest, response::LoadModelResponse},
    reorder::branch_ag_dto_subject::BranchAgDtoSubject,
};

#[tauri::command]
#[specta::specta]
pub async fn get_nodes_model_data<R: Runtime>(
    app: AppHandle<R>,
    request: LoadModelRequest,
) -> Result<LoadModelResponse> {
    log::info!("Starting load_model function with request: {:?}", request);

    let bearer_tokens = app.state::<BearerTokens<R>>();
    let editor_api_client = app.state::<EditorApiClient>();
    let repo_ag_state = app.state::<RepoDtoState>();
    let branch_ag_dto_subject = app.state::<BranchAgDtoSubject>();
    let branch_commit_state = app.state::<AgCommitState<BranchAgDto>>();

    log::debug!("Retrieved all necessary app states");

    let access_token = match bearer_tokens.access_token() {
        Some(token) => token,
        None => {
            log::error!("No access token found");
            return Err(Error {
                req_id: request.req_id.clone(),
                message: "No access token found".to_string(),
            });
        }
    };

    log::trace!("Access token retrieved successfully");

    let mut repo_id = None;
    let mut acct_id = None;

    repo_ag_state.read_state_value(|repo_ag_dto| {
        repo_id = Some(repo_ag_dto.id.clone());
        acct_id = Some(repo_ag_dto.acct_id.clone());
    });

    log::debug!(
        "Retrieved account ID: {:?} and repository ID: {:?}",
        acct_id,
        repo_id
    );

    let acct_id = acct_id.ok_or_else(|| {
        log::error!("Failed to get account ID");
        Error {
            req_id: request.req_id.clone(),
            message: "Failed to get account ID".to_string(),
        }
    })?;

    let repo_id = repo_id.ok_or_else(|| {
        log::error!("Failed to get repository ID");
        Error {
            req_id: request.req_id.clone(),
            message: "Failed to get repository ID".to_string(),
        }
    })?;

    let payload = BranchGetRequest {
        acct_id,
        repo_id,
        branch_id: request.branch_id.clone(),
        address: request.address.clone(),
    };

    log::debug!("Prepared BranchGetRequest: {:?}", payload);

    log::info!("Sending request to get branch information");
    let branch_response = editor_api_client.get_branch(&access_token, payload).await;

    match branch_response {
        Ok(response) => {
            log::info!("Received successful branch response");
            log::debug!("Branch response details: {:?}", response);

            let req_id = request.req_id.clone();

            log::debug!("Creating new AG DTO from status");
            let (next_commit_id, branch_ag_dto) = match new_ag_dto_from_status::<BranchAgDto>(
                &request.branch_id,
                &req_id,
                response.status,
            ) {
                Ok((next_commit_id, branch_ag_dto)) => (next_commit_id, branch_ag_dto),
                Err(e) => {
                    log::error!("Failed to create new AG DTO: {:?}", e);
                    return Err(Error {
                        req_id: e.0,
                        message: e.1,
                    });
                }
            };

            let commit_info: CommitInfo<BranchAgDto> = CommitInfo {
                commit_id: branch_ag_dto.commit_id.clone(),
                next_commit_id,
                _phantom: Default::default(),
            };

            log::debug!("Setting AG commit info: {:?}", commit_info);
            branch_commit_state.set_ag_commit_info(Some(commit_info));

            log::info!("Setting branch AG DTO subject");
            if let Err(e) = branch_ag_dto_subject
                .set_value(Some(branch_ag_dto.clone()))
                .await
            {
                log::error!("Error setting branch_ag_dto_subject: {:?}", e);
                return Err(Error {
                    req_id: request.req_id.clone(),
                    message: "Error setting the opened branch".to_string(),
                });
            }

            log::info!("load_model function completed successfully");
            Ok(LoadModelResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => {
            log::error!("Failed to get branch information: {:?}", e);
            Err(Error {
                req_id: request.req_id,
                message: e.to_string(),
            })
        }
    }
}
