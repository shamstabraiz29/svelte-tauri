use common_dtos::commit_client::acct_folder_create::{
    builder::AcctFolderCreateRequestBuilder, request::AcctFolderCreateRequest,
};
use common_libs_account_api_client::AccountApiClient;

use frontend_tauri_plugins_common::{
    dto_util::update_ag_dto_from_status::update_ag_dto_from_status,
    error::{Error, Result},
    notifying_sync::notifying_state::NotifyingState,
    states::ag_commit_state::AgCommitState,
    types::{account_dto_state::AccountDtoState, bearer_tokens::BearerTokens},
};

use tauri::{State, Wry};

use crate::{
    config::AccountClientConfig,
    lifecycle::init::AcctCommitInfo,
    models::create_folder::{request::CreateFolderRequest, response::CreateFolderResponse},
};

#[tauri::command]
#[specta::specta]
pub async fn create_folder(
    config: State<'_, AccountClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    acct_ag_state: State<'_, AccountDtoState>,
    acct_commit_state: State<'_, AgCommitState<AcctCommitInfo>>,
    request: CreateFolderRequest,
) -> Result<CreateFolderResponse> {
    let url = config.url.clone();
    let api_paths = config.api_paths.clone();

    let account_api_client = AccountApiClient::new(&url, api_paths).unwrap();

    let Some(access_token) = bearer_tokens.access_token() else {
        return Err(Error {
            req_id: request.req_id,
            message: "No access token found".to_string(),
        });
    };

    log::debug!("create_folder_request: {:?}", request);

    let commit_info = match acct_commit_state.get_ag_commit_info() {
        Some(c) => c,
        None => {
            return Err(Error {
                req_id: request.req_id,
                message: "Commit info not found".to_string(),
            })
        }
    };

    let mut payload: std::result::Result<AcctFolderCreateRequest, String> =
        Err("Failed to build payload".to_string());
    let mut acct_id = None;

    acct_ag_state.read_state_value(|acct_ag_dto| {
        let parent_folder_id = request.parent_folder_id;
        acct_id = Some(acct_ag_dto.id.clone());

        let builder =
            match AcctFolderCreateRequestBuilder::new().set_parent_folder_id(parent_folder_id) {
                Ok(b) => b,
                Err(e) => {
                    payload = Err(e);
                    return;
                }
            };

        payload = builder.build(commit_info.next_commit_id, acct_ag_dto);

        log::debug!("acct_ag_dto: {:#?}", acct_ag_dto);
    });

    log::debug!("create_folder_payload: {:?}", payload);

    let payload = match payload {
        Ok(p) => p,
        Err(err_msg) => {
            return Err(Error {
                req_id: request.req_id,
                message: err_msg,
            })
        }
    };

    let response = account_api_client
        .create_folder(&access_token, payload)
        .await
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    log::debug!("create_folder_response: {:#?}", response);

    let status = response.status;

    let acct_id = match acct_id {
        Some(id) => id,
        None => {
            return Err(Error {
                req_id: request.req_id,
                message: "Failed to get account id".to_string(),
            })
        }
    };

    match update_ag_dto_from_status(
        &acct_id,
        &request.req_id,
        status,
        &acct_commit_state,
        &acct_ag_state,
    ) {
        Ok(req_id) => Ok(CreateFolderResponse { req_id }),
        Err((req_id, message)) => Err(Error { req_id, message }),
    }
}
