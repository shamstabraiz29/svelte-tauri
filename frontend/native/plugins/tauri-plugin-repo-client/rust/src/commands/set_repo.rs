use common_dtos::commit_client::{
    head_snapshot_request::HeadSnapshotRequest, types::repo::repo_dto::RepoAgDto,
};
use common_libs_repo_api_client::RepoApiClient;

use frontend_tauri_plugins_common::{
    dto_util::update_ag_dto_from_status::new_ag_dto_from_status,
    error::{Error, Result},
    notifying_sync::notifying_state::NotifyingState,
    states::ag_commit_state::AgCommitState,
    types::{
        bearer_tokens::BearerTokens, commit_info::CommitInfo, repo_detail_state::RepoDtoState,
    },
};
use tauri::{State, Wry};

use crate::{
    config::RepoClientConfig,
    lifecycle::init::RepoCommitInfo,
    models::set_repo::{request::RepoSetRequest, response::RepoSetResponse},
};

#[tauri::command]
#[specta::specta]
pub async fn set_repo(
    config: State<'_, RepoClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    repo_ag_state: State<'_, RepoDtoState>,
    repo_commit_state: State<'_, AgCommitState<RepoCommitInfo>>,
    request: RepoSetRequest,
) -> Result<RepoSetResponse> {
    let url = config.url.clone();
    let api_paths = config.api_paths.clone();

    let Some(access_token) = bearer_tokens.access_token() else {
        return Err(Error {
            req_id: request.req_id,
            message: "No access token found".to_string(),
        });
    };

    let account_api_client = RepoApiClient::new(&url, api_paths).unwrap();

    let payload = HeadSnapshotRequest::new(request.repo_id.clone());

    let repo_response = account_api_client.get(&access_token, payload).await;

    log::trace!("************************************************************");
    log::trace!("************************************************************");
    log::trace!("************************ get repo response **************");
    log::trace!("{:#?}", repo_response);
    log::trace!("************************************************************");
    log::trace!("************************************************************");

    match repo_response {
        Ok(response) => {
            let req_id = request.req_id.to_string();

            let (next_commit_id, repo_ag_dto) =
                new_ag_dto_from_status::<RepoAgDto>(&request.repo_id, &req_id, response.status)
                    .unwrap();

            let commit_info: CommitInfo<RepoCommitInfo> = CommitInfo {
                commit_id: repo_ag_dto.commit_id.clone(),
                next_commit_id,
                _phantom: std::marker::PhantomData,
            };

            repo_ag_state.set_state_value(Some(repo_ag_dto));

            repo_commit_state.set_ag_commit_info(Some(commit_info));

            Ok(RepoSetResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => Err(Error {
            req_id: request.req_id,
            message: e.to_string(),
        }),
    }
}
