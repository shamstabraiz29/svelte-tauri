pub mod repo_client;

use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use self::repo_client::RepoClient;

pub(crate) fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<RepoClient<R>> {
    Ok(RepoClient(app.clone()))
}
