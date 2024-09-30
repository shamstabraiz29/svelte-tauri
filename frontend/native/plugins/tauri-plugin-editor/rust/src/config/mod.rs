use common_libs_editor_api_client::EditorApiPaths;
use serde::Deserialize;

// Define the plugin config
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditorClientConfig {
    pub url: String,
    pub api_paths: EditorApiPaths,
}
