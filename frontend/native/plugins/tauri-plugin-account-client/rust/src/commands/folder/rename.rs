use std::{future::Future, pin::Pin};

use common_dtos::commit_client::{
    acct_modify::{response::AcctModifyResponse, FolderModifyRequestBuilder, FolderRenameRequest},
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

use crate::{
    commands::{
        assert_folders_exists::assert_folders_exits, execute_modify_action::execute_modify_action,
    },
    config::AccountClientConfig,
    lifecycle::init::AcctCommitInfo,
    models::rename_folder::{request::RenameFolderRequest, response::RenameFolderResponse},
};

#[tauri::command]
#[specta::specta]
pub async fn rename_folder(
    config: State<'_, AccountClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    acct_ag_state: State<'_, AccountDtoState>,
    acct_commit_state: State<'_, AgCommitState<AcctCommitInfo>>,
    request: RenameFolderRequest,
) -> Result<RenameFolderResponse> {
    let req_id = request.req_id.clone();

    assert_folders_exits(&acct_ag_state, &[&request.folder_id]).map_err(|msg| Error {
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
                let payload = FolderModifyRequestBuilder::<FolderRenameRequest>::default()
                    .set_acct_id(acct_id)
                    .set_folder_id(request.folder_id)
                    .set_name(request.name)
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

                let AcctModifyResponse::FolderRename(response) = response else {
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

    Ok(RenameFolderResponse { req_id })
}
