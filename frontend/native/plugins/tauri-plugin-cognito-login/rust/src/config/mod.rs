use serde::Deserialize;

// Define the plugin config
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginConfig {
    pub user_pool_id: String,
    pub client_id: String,
    pub region: String,
}
