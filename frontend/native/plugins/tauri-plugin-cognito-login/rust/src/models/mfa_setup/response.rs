use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MfaSetupResponse {
    pub req_id: String,
    pub state: MfaSetupState,
}

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum MfaSetupState {
    MfaSetupSucceeded,
    EmailConfirmationRequired,
    Forbidden,
    MfaSetupFailed,
    NotAuthorized,
    PasswordResetRequired,
    UserNotFound,
}
