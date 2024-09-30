use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PwLoginRequest {
    pub req_id: String,
    pub email: String,
    pub password: String,
}
