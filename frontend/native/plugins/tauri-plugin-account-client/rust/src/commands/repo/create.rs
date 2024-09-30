use common_dtos::commit_client::acct_repo_create::builder::AcctRepoCreateRequestBuilder;
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
    models::create_repo::{request::CreateRepoRequest, response::CreateRepoResponse},
};

#[tauri::command]
#[specta::specta]
pub async fn create_repo(
    config: State<'_, AccountClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    acct_ag_state: State<'_, AccountDtoState>,
    acct_commit_state: State<'_, AgCommitState<AcctCommitInfo>>,
    request: CreateRepoRequest,
) -> Result<CreateRepoResponse> {
    let url = config.url.clone();
    let api_paths = config.api_paths.clone();

    let account_api_client = AccountApiClient::new(&url, api_paths).unwrap();

    let Some(access_token) = bearer_tokens.access_token() else {
        return Err(Error {
            req_id: request.req_id,
            message: "No access token found".to_string(),
        });
    };

    log::debug!("create_repo_request: {:?}", request);

    let commit_info = match acct_commit_state.get_ag_commit_info() {
        Some(c) => c,
        None => {
            return Err(Error {
                req_id: request.req_id,
                message: "Commit info not found".to_string(),
            })
        }
    };

    let mut payload = None;
    let mut acct_id = None;

    acct_ag_state.read_state_value(|acct_ag_dto| {
        let parent_folder_id = request.parent_folder_id;
        acct_id = Some(acct_ag_dto.id.clone());

        let builder = AcctRepoCreateRequestBuilder::new()
            .set_parent_folder_id(parent_folder_id)
            .expect("Invalid parent_folder_id");

        payload = Some(
            builder
                .build(commit_info.next_commit_id, acct_ag_dto)
                .expect("Failed to build AcctRepoCreateRequest"),
        );

        log::debug!(
            "create repo - modify_acct_ag_dto - acct_ag_dto: {:#?}",
            acct_ag_dto
        );
    });

    log::debug!("create_repo_payload: {:?}", payload);

    let payload = match payload {
        Some(p) => p,
        None => {
            return Err(Error {
                req_id: request.req_id,
                message: "Failed to build payload".to_string(),
            })
        }
    };

    let response = account_api_client
        .create_repo(&access_token, payload)
        .await
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    log::debug!("create_repo_response: {:#?}", response);

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
        Ok(req_id) => Ok(CreateRepoResponse { req_id }),
        Err((req_id, message)) => Err(Error { req_id, message }),
    }
}
