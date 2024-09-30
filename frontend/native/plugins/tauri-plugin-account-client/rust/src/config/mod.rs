use common_libs_account_api_client::AccountApiPaths;
use serde::Deserialize;

// Define the plugin config
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountClientConfig {
    pub url: String,
    pub api_paths: AccountApiPaths,
}
