use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MfaSetupRequest {
    pub req_id: String,
    pub session: String,
    pub mfa_code: String,
}
