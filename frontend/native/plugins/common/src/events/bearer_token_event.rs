use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value;
use specta::Type;
use specta_util::Unknown;

#[derive(Serialize, Clone, Debug, Type, tauri_specta::Event)]
#[serde(tag = "type")]
pub enum BearerTokenEvent {
    Tokens {
        #[serde(rename = "accessToken")]
        access_token: String,
        #[serde(rename = "accessTokenClaims")]
        #[specta(type = HashMap<String, Unknown>)]
        access_token_claims: HashMap<String, Value>,
        #[serde(rename = "idToken")]
        id_token: String,
        #[serde(rename = "idTokenClaims")]
        #[specta(type = HashMap<String, Unknown>)]
        id_token_claims: HashMap<String, Value>,
    },
    Clear,
}
