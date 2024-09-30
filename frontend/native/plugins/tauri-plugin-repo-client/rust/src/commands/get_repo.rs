use common_dtos::commit_client::{
    head_snapshot_request::HeadSnapshotRequest, types::repo::repo_dto::RepoAgDto,
};
use common_libs_repo_api_client::RepoApiClient;

use frontend_tauri_plugins_common::{
    dto_util::update_ag_dto_from_status::new_ag_dto_from_status,
    error::{Error, Result},
    types::bearer_tokens::BearerTokens,
};
use tauri::{State, Wry};

use crate::{
    config::RepoClientConfig,
    models::get_repo::{request::RepoRequest, response::RepoResponse},
};

#[tauri::command]
#[specta::specta]
pub async fn get_repo(
    config: State<'_, RepoClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    request: RepoRequest,
) -> Result<RepoResponse> {
    let url = config.url.clone();
    let api_paths = config.api_paths.clone();

    let Some(access_token) = bearer_tokens.access_token() else {
        return Err(Error {
            req_id: request.req_id,
            message: "No access token found".to_string(),
        });
    };

    let repo_api_client = RepoApiClient::new(&url, api_paths).unwrap();

    let payload = HeadSnapshotRequest::new(request.repo_id.to_owned());

    let repo_response = repo_api_client.get(&access_token, payload).await;

    match repo_response {
        Ok(response) => {
            let req_id = request.req_id.to_string();

            let (_, repo_ag_dto) =
                new_ag_dto_from_status::<RepoAgDto>(&request.repo_id, &req_id, response.status)
                    .unwrap();

            Ok(RepoResponse {
                repo_detail: repo_ag_dto.into(),
            })
        }
        Err(e) => Err(Error {
            req_id: request.req_id,
            message: e.to_string(),
        }),
    }
}
