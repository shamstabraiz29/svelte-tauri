use serde::Deserialize;

// Define the plugin config
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupConfig {
    pub client_id: String,
    pub region: String,
}
