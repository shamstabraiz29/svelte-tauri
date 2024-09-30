use std::{future::Future, pin::Pin};

use common_dtos::commit_client::AgOperationStatus;
use common_libs_account_api_client::AccountApiClient;
use common_simple_types::{ag_id::AgId, commit_id::CommitId};
use frontend_tauri_plugins_common::{
    dto_util::update_ag_dto_from_status::update_ag_dto_from_status,
    error::{Error, Result},
    notifying_sync::notifying_state::NotifyingState,
    states::ag_commit_state::AgCommitState,
    types::{account_dto_state::AccountDtoState, bearer_tokens::BearerTokens},
};
use tauri::{State, Wry};

use crate::{config::AccountClientConfig, lifecycle::init::AcctCommitInfo};

pub(super) async fn execute_modify_action<F>(
    req_id: &str,
    config: &State<'_, AccountClientConfig>,
    bearer_tokens: &State<'_, BearerTokens<Wry>>,
    acct_ag_state: &State<'_, AccountDtoState>,
    acct_commit_state: &State<'_, AgCommitState<AcctCommitInfo>>,
    commit_fn: Box<F>,
) -> Result<String>
where
    F: FnOnce(
        AccountApiClient,
        AgId,
        CommitId,
        String,
    ) -> Pin<Box<dyn Future<Output = Result<AgOperationStatus>> + Send>>,
{
    let Some(access_token) = bearer_tokens.access_token() else {
        return Err(Error {
            req_id: req_id.to_string(),
            message: "No access token found".to_string(),
        });
    };

    let url = config.url.clone();
    let api_paths = config.api_paths.clone();

    let account_api_client = AccountApiClient::new(&url, api_paths).unwrap();

    let commit_info = match acct_commit_state.get_ag_commit_info() {
        Some(c) => c,
        None => {
            return Err(Error {
                req_id: req_id.to_string(),
                message: "Commit info not found".to_string(),
            })
        }
    };

    let mut acct_id = None;

    acct_ag_state.read_state_value(|acct_ag_dto| {
        acct_id = Some(acct_ag_dto.id.clone());
    });

    let Some(acct_id) = acct_id else {
        return Err(Error {
            req_id: req_id.to_string(),
            message: "Failed to get account id".to_string(),
        });
    };

    let status = commit_fn(
        account_api_client,
        acct_id.to_owned(),
        commit_info.next_commit_id,
        access_token,
    )
    .await?;

    update_ag_dto_from_status(&acct_id, req_id, status, acct_commit_state, acct_ag_state).map_err(
        |e| Error {
            req_id: e.0,
            message: e.1,
        },
    )
}
