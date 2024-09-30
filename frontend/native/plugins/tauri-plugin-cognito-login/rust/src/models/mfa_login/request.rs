use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MfaLoginRequest {
    pub req_id: String,
    pub session: String,
    pub username: String,
    pub mfa_code: String,
}
