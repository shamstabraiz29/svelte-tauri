use common_dtos::commit_client::{
    head_snapshot_request::HeadSnapshotRequest, types::account::account_dto::AccountAgDto,
};
use common_libs_account_api_client::AccountApiClient;

use frontend_tauri_plugins_common::{
    dto_util::update_ag_dto_from_status::new_ag_dto_from_status,
    error::{Error, Result},
    notifying_sync::notifying_state::NotifyingState,
    states::ag_commit_state::AgCommitState,
    types::{
        account_dto_state::AccountDtoState, bearer_tokens::BearerTokens, commit_info::CommitInfo,
    },
};
use tauri::{State, Wry};

use crate::{
    config::AccountClientConfig,
    lifecycle::init::AcctCommitInfo,
    models::set_account::{request::AccountSetRequest, response::AccountSetResponse},
};

#[tauri::command]
#[specta::specta]
pub async fn set_account(
    config: State<'_, AccountClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    acct_ag_state: State<'_, AccountDtoState>,
    acct_commit_state: State<'_, AgCommitState<AcctCommitInfo>>,
    request: AccountSetRequest,
) -> Result<AccountSetResponse> {
    let url = config.url.clone();
    let api_paths = config.api_paths.clone();

    let Some(access_token) = bearer_tokens.access_token() else {
        return Err(Error {
            req_id: request.req_id,
            message: "No access token found".to_string(),
        });
    };

    let account_api_client = AccountApiClient::new(&url, api_paths).unwrap();

    let payload = HeadSnapshotRequest::new(request.acct_id.clone());

    let acct_response = account_api_client.get(&access_token, payload).await;

    log::trace!("************************************************************");
    log::trace!("************************************************************");
    log::trace!("************************ get account response **************");
    log::trace!("{:#?}", acct_response);
    log::trace!("************************************************************");
    log::trace!("************************************************************");

    match acct_response {
        Ok(response) => {
            let req_id = request.req_id.to_string();

            let (next_commit_id, acct_ag_dto) =
                new_ag_dto_from_status::<AccountAgDto>(&request.acct_id, &req_id, response.status)
                    .unwrap();

            let commit_info: CommitInfo<AcctCommitInfo> = CommitInfo {
                commit_id: acct_ag_dto.commit_id.clone(),
                next_commit_id,
                _phantom: std::marker::PhantomData,
            };

            acct_ag_state.set_state_value(Some(acct_ag_dto));

            acct_commit_state.set_ag_commit_info(Some(commit_info));

            Ok(AccountSetResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => Err(Error {
            req_id: request.req_id,
            message: e.to_string(),
        }),
    }
}
