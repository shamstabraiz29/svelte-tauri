use common_dtos::commit_client::{
    repo_get_commit_history::request::RepoGetCommitHistoryRequest, RepoCommitHistoryOperationStatus,
};
use common_libs_repo_api_client::RepoApiClient;
use frontend_tauri_plugins_common::{
    error::{Error, Result},
    types::bearer_tokens::BearerTokens,
};
use tauri::{State, Wry};

use crate::{
    config::RepoClientConfig,
    models::get_branch_commit_history::{
        request::RepoCommitHistoryRequest, response::RepoGetCommitHistoryResponse,
    },
};

#[tauri::command]
#[specta::specta]
pub async fn get_commit_history(
    config: State<'_, RepoClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    request: RepoCommitHistoryRequest,
) -> Result<RepoGetCommitHistoryResponse> {
    let Some(access_token) = bearer_tokens.access_token() else {
        return Err(Error {
            req_id: request.req_id,
            message: "No access token found".to_string(),
        });
    };

    let url = config.url.clone();
    let api_paths = config.api_paths.clone();

    let repo_api_client = RepoApiClient::new(&url, api_paths).unwrap();

    let param = RepoGetCommitHistoryRequest::new(request.repo_id, None, 100);

    let commit_history = repo_api_client
        .get_commit_history(&access_token, param)
        .await
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    let commit_history = match commit_history.status {
        RepoCommitHistoryOperationStatus::Success { commit_history } => commit_history,
        RepoCommitHistoryOperationStatus::Failure(e) => {
            return Err(Error {
                req_id: request.req_id,
                message: e,
            })
        }
    };

    log::debug!("commit_history: {:#?}", commit_history);

    Ok(RepoGetCommitHistoryResponse {
        history: commit_history.into(),
    })
}
