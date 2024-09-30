use common_libs_repo_api_client::RepoApiPaths;
use serde::Deserialize;

// Define the plugin config
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RepoClientConfig {
    pub url: String,
    pub api_paths: RepoApiPaths,
}
