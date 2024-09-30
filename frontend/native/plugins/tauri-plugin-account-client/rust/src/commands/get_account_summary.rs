use common_dtos::commit_client::{
    head_snapshot_request::HeadSnapshotRequest, types::account::account_dto::AccountAgDto,
};
use common_libs_account_api_client::AccountApiClient;

use frontend_tauri_plugins_common::{
    dto_util::update_ag_dto_from_status::new_ag_dto_from_status,
    error::{Error, Result},
    types::bearer_tokens::BearerTokens,
};
use tauri::{State, Wry};

use crate::{
    config::AccountClientConfig,
    models::get_account::{request::AccountSummaryRequest, response::AccountSummaryResponse},
};

#[tauri::command]
#[specta::specta]
pub async fn get_account_summary(
    config: State<'_, AccountClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    request: AccountSummaryRequest,
) -> Result<AccountSummaryResponse> {
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

    log::trace!("***********************************************************************");
    log::trace!("***********************************************************************");
    log::trace!("************** GET ACCOUNT SUMMARY: get_account response **************");
    log::trace!("{:#?}", acct_response);
    log::trace!("***********************************************************************");
    log::trace!("***********************************************************************");

    match acct_response {
        Ok(response) => {
            let req_id = request.req_id.to_string();

            let (_, acct_ag_dto) =
                new_ag_dto_from_status::<AccountAgDto>(&request.acct_id, &req_id, response.status)
                    .unwrap();

            Ok(AccountSummaryResponse {
                req_id,
                acct_summary: acct_ag_dto.into(),
            })
        }
        Err(e) => Err(Error {
            req_id: request.req_id,
            message: e.to_string(),
        }),
    }
}
