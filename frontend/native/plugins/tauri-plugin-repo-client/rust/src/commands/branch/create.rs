use common_dtos::commit_client::repo_branch_create::builder::RepoBranchCreateRequestBuilder;
use common_libs_repo_api_client::RepoApiClient;

use frontend_tauri_plugins_common::{
    dto_util::update_ag_dto_from_status::update_ag_dto_from_status,
    error::{Error, Result},
    notifying_sync::notifying_state::NotifyingState,
    states::ag_commit_state::AgCommitState,
    types::{bearer_tokens::BearerTokens, repo_detail_state::RepoDtoState},
};
use tauri::{State, Wry};

use crate::{
    config::RepoClientConfig,
    lifecycle::init::RepoCommitInfo,
    models::create_branch::{request::CreateBranchRequest, response::CreateBranchResponse},
};

#[tauri::command]
#[specta::specta]
pub async fn create_branch(
    config: State<'_, RepoClientConfig>,
    repo_ag_state: State<'_, RepoDtoState>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    repo_commit_state: State<'_, AgCommitState<RepoCommitInfo>>,
    request: CreateBranchRequest,
) -> Result<CreateBranchResponse> {
    let url = config.url.clone();
    let api_paths = config.api_paths.clone();

    let Some(access_token) = bearer_tokens.access_token() else {
        return Err(Error {
            req_id: request.req_id,
            message: "No access token found".to_string(),
        });
    };

    let repo_api_client = RepoApiClient::new(&url, api_paths).unwrap();

    log::debug!("create_branch_request: {:?}", request);

    let commit_info = match repo_commit_state.get_ag_commit_info() {
        Some(c) => c,
        None => {
            return Err(Error {
                req_id: request.req_id,
                message: "Commit info not found".to_string(),
            })
        }
    };

    let mut payload = None;
    let mut repo_id = None;

    repo_ag_state.read_state_value(|repo_ag_dto| {
        repo_id = Some(repo_ag_dto.id.clone());

        let mut builder = RepoBranchCreateRequestBuilder::new().set_name(request.name);

        if let Some(properties) = request.properties {
            builder = builder.set_properties(properties);
        }

        if let Some(parent_branch) = request.parent_branch {
            builder = builder.set_parent_branch(parent_branch.branch_id, parent_branch.commit_id);
        }

        payload = Some(
            builder
                .build(commit_info.next_commit_id, repo_ag_dto)
                .unwrap(),
        );

        log::debug!("repo_ag_dto: {:#?}", repo_ag_dto);
    });

    log::debug!("create_repo_branch_payload: {:?}", payload);

    let payload = match payload {
        Some(p) => p,
        None => {
            return Err(Error {
                req_id: request.req_id,
                message: "Failed to build payload".to_string(),
            })
        }
    };

    let response = repo_api_client
        .create_branch(&access_token, payload)
        .await
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    log::debug!("create_repo_branch_response: {:#?}", response);

    let status = response.status;

    let repo_id = match repo_id {
        Some(id) => id,
        None => {
            return Err(Error {
                req_id: request.req_id,
                message: "Failed to get repository id".to_string(),
            })
        }
    };

    match update_ag_dto_from_status(
        &repo_id,
        &request.req_id,
        status,
        &repo_commit_state,
        &repo_ag_state,
    ) {
        Ok(req_id) => Ok(CreateBranchResponse { req_id }),
        Err((req_id, message)) => Err(Error { req_id, message }),
    }
}
