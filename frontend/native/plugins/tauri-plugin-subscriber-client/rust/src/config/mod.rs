use serde::Deserialize;

// Define the plugin config
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriberClientConfig {
    pub url: String,
    pub api_path: String,
}
