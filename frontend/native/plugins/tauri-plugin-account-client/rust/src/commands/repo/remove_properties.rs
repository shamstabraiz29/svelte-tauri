use std::{future::Future, pin::Pin};

use crate::{
    commands::{
        assert_repos_exists::assert_repos_exits, execute_modify_action::execute_modify_action,
    },
    config::AccountClientConfig,
    lifecycle::init::AcctCommitInfo,
    models::remove_repo_properties::{
        request::RemoveRepoPropertiesRequest, response::RemoveRepoPropertiesResponse,
    },
};
use common_dtos::commit_client::{
    acct_modify::{
        response::AcctModifyResponse, RepoModifyRequestBuilder, RepoRemovePropertiesRequest,
    },
    AgOperationStatus,
};
use common_libs_account_api_client::AccountApiClient;
use common_simple_types::{ag_id::AgId, commit_id::CommitId};
use frontend_tauri_plugins_common::{
    error::{Error, Result},
    states::ag_commit_state::AgCommitState,
    types::{account_dto_state::AccountDtoState, bearer_tokens::BearerTokens},
};
use tauri::{State, Wry};

#[tauri::command]
#[specta::specta]
pub async fn remove_repo_properties(
    config: State<'_, AccountClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    acct_ag_state: State<'_, AccountDtoState>,
    acct_commit_state: State<'_, AgCommitState<AcctCommitInfo>>,
    request: RemoveRepoPropertiesRequest,
) -> Result<RemoveRepoPropertiesResponse> {
    let req_id = request.req_id.clone();

    assert_repos_exits(&acct_ag_state, &[&request.repo_id]).map_err(|msg| Error {
        req_id: req_id.clone(),
        message: msg,
    })?;

    let commit_fn = Box::new(
        move |acct_api_client: AccountApiClient,
              acct_id: AgId,
              commit_id: CommitId,
              access_token: String|
              -> Pin<Box<dyn Future<Output = Result<AgOperationStatus>> + Send>> {
            Box::pin(async move {
                let payload = RepoModifyRequestBuilder::<RepoRemovePropertiesRequest>::default()
                    .set_acct_id(acct_id)
                    .set_repo_id(request.repo_id)
                    .set_property_names(request.properties)
                    .build(commit_id)
                    .map_err(|msg| Error {
                        req_id: request.req_id.clone(),
                        message: msg,
                    })?;

                let response = acct_api_client
                    .modify_acct(&access_token, payload)
                    .await
                    .map_err(|e| Error {
                        req_id: request.req_id.clone(),
                        message: e.to_string(),
                    })?;

                let AcctModifyResponse::RepoRemoveProperties(response) = response else {
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
        &acct_ag_state,
        &acct_commit_state,
        commit_fn,
    )
    .await?;

    Ok(RemoveRepoPropertiesResponse { req_id })
}
