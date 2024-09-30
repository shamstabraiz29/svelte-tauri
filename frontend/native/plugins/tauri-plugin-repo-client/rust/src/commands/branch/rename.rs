use std::{future::Future, pin::Pin};

use common_dtos::commit_client::{
    repo_modify::{response::RepoModifyResponse, BranchModifyRequestBuilder, BranchRenameRequest},
    AgOperationStatus,
};
use common_libs_repo_api_client::RepoApiClient;
use common_simple_types::{ag_id::AgId, commit_id::CommitId};
use frontend_tauri_plugins_common::{
    error::{Error, Result},
    states::ag_commit_state::AgCommitState,
    types::{bearer_tokens::BearerTokens, repo_detail_state::RepoDtoState},
};
use tauri::{State, Wry};

use crate::{
    commands::{
        assert_branches_exists::assert_branches_exists,
        execute_modify_action::execute_modify_action,
    },
    config::RepoClientConfig,
    lifecycle::init::RepoCommitInfo,
    models::rename_branch::{request::RenameBranchRequest, response::RenameBranchResponse},
};

#[tauri::command]
#[specta::specta]
pub async fn rename_branch(
    config: State<'_, RepoClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    repo_ag_state: State<'_, RepoDtoState>,
    repo_commit_state: State<'_, AgCommitState<RepoCommitInfo>>,
    request: RenameBranchRequest,
) -> Result<RenameBranchResponse> {
    let req_id = request.req_id.clone();

    assert_branches_exists(&repo_ag_state, &[&request.branch_id]).map_err(|msg| Error {
        req_id: req_id.clone(),
        message: msg,
    })?;

    let commit_fn = Box::new(
        move |acct_api_client: RepoApiClient,
              repo_id: AgId,
              commit_id: CommitId,
              access_token: String|
              -> Pin<Box<dyn Future<Output = Result<AgOperationStatus>> + Send>> {
            Box::pin(async move {
                let payload = BranchModifyRequestBuilder::<BranchRenameRequest>::default()
                    .set_repo_id(repo_id)
                    .set_branch_id(request.branch_id)
                    .set_name(request.name)
                    .build(commit_id)
                    .map_err(|msg| Error {
                        req_id: request.req_id.clone(),
                        message: msg,
                    })?;

                let response = acct_api_client
                    .modify(&access_token, payload)
                    .await
                    .map_err(|e| Error {
                        req_id: request.req_id.clone(),
                        message: e.to_string(),
                    })?;

                let RepoModifyResponse::BranchRename(response) = response else {
                    return Err(Error {
                        req_id: request.req_id,
                        message: "Unexpected response".to_string(),
                    });
                };

                Ok(response.status)
            })
        },
    );

    let req_id = execute_modify_action(
        &req_id,
        &config,
        &bearer_tokens,
        &repo_ag_state,
        &repo_commit_state,
        commit_fn,
    )
    .await?;

    Ok(RenameBranchResponse { req_id })
}
