use common_dtos::commit_client::{
    head_snapshot_request::HeadSnapshotRequest, types::repo::repo_dto::RepoAgDto,
};
use common_libs_repo_api_client::RepoApiClient;
use common_simple_types::ag_id::AgId;

use frontend_tauri_plugins_common::dto_util::update_ag_dto_from_status::new_ag_dto_from_status;
use tauri::{AppHandle, Manager, Runtime, State};

/// Access to the repo-client APIs.
pub struct RepoClient<R: Runtime>(pub AppHandle<R>);

impl<R: Runtime> RepoClient<R> {
    // functions to expose Rust APIs to main app (Rust side)
    pub async fn load(
        &self,
        access_token: &str,
        repo_id: AgId,
    ) -> Result<RepoAgDto, crate::RepoError> {
        let repo_api_client: State<'_, RepoApiClient> = self.0.state();
        let payload = HeadSnapshotRequest::new(repo_id.to_owned());
        let response = repo_api_client.get(access_token, payload).await?;
        let (_, repo_ag_dto) =
            new_ag_dto_from_status::<RepoAgDto>(&repo_id, "repo-client-load", response.status)
                .unwrap();
        Ok(repo_ag_dto)
    }
}
