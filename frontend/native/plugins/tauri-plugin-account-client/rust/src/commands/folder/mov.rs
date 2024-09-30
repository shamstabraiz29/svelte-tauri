use std::{future::Future, pin::Pin};

use common_dtos::commit_client::{
    acct_modify::{response::AcctModifyResponse, FolderModifyRequestBuilder, FolderMoveRequest},
    AgOperationStatus,
};
use common_libs_account_api_client::AccountApiClient;
use common_simple_types::{ag_id::AgId, commit_id::CommitId};
use frontend_tauri_plugins_common::{
    error::{Error, Result},
    notifying_sync::notifying_state::NotifyingState,
    states::ag_commit_state::AgCommitState,
    types::{account_dto_state::AccountDtoState, bearer_tokens::BearerTokens},
};
use tauri::{State, Wry};

use crate::{
    commands::{
        assert_folders_exists::assert_folders_exits, execute_modify_action::execute_modify_action,
    },
    config::AccountClientConfig,
    lifecycle::init::AcctCommitInfo,
    models::move_folder::{request::MoveFolderRequest, response::MoveFolderResponse},
};

#[tauri::command]
#[specta::specta]
pub async fn move_folder(
    config: State<'_, AccountClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    acct_ag_state: State<'_, AccountDtoState>,
    acct_commit_state: State<'_, AgCommitState<AcctCommitInfo>>,
    request: MoveFolderRequest,
) -> Result<MoveFolderResponse> {
    let req_id = request.req_id.clone();

    assert_folders_exits(
        &acct_ag_state,
        &[&request.folder_id, &request.parent_folder_id],
    )
    .map_err(|msg| Error {
        req_id: req_id.clone(),
        message: msg,
    })?;

    // Ensure that the parent folder ID exists in the account DTO
    let mut payload: std::result::Result<(), String> =
        Err("Target parent folder not found".to_string());
    acct_ag_state.read_state_value(|acct_ag_dto| {
        if acct_ag_dto
            .find_folder_by_id(&request.parent_folder_id)
            .is_some()
        {
            payload = Ok(());
        }
    });
    if let Err(err_msg) = payload {
        return Err(Error {
            req_id: req_id.clone(),
            message: err_msg,
        });
    }

    let commit_fn = Box::new(
        move |acct_api_client: AccountApiClient,
              acct_id: AgId,
              commit_id: CommitId,
              access_token: String|
              -> Pin<Box<dyn Future<Output = Result<AgOperationStatus>> + Send>> {
            Box::pin(async move {
                let payload = FolderModifyRequestBuilder::<FolderMoveRequest>::default()
                    .set_acct_id(acct_id.to_owned())
                    .set_folder_id(request.folder_id)
                    .set_new_parent_folder_id(request.parent_folder_id)
                    .build(commit_id)
                    .map_err(|msg| Error {
                        req_id: req_id.clone(),
                        message: msg,
                    })?;

                let response = acct_api_client
                    .modify_acct(&access_token, payload)
                    .await
                    .map_err(|e| Error {
                        req_id: req_id.clone(),
                        message: e.to_string(),
                    })?;

                let AcctModifyResponse::FolderMove(response) = response else {
                    return Err(Error {
                        req_id,
                        message: "Unexpected response".to_string(),
                    });
                };

                Ok(response.status)
            })
        },
    );

    let req_id = execute_modify_action(
        &request.req_id,
        &config,
        &bearer_tokens,
        &acct_ag_state,
        &acct_commit_state,
        commit_fn,
    )
    .await?;

    Ok(MoveFolderResponse { req_id })
}
