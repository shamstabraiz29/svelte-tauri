use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PwLoginResponse {
    pub req_id: String,
    pub state: PwLoginState,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum PwLoginState {
    Authenticated,
    MfaRequired { session: String },
    MfaSetupRequired { totp_uri: String, session: String },
    PasswordResetRequired,
    EmailConfirmationRequired,
    Forbidden,
    NotAuthorized,
    UserNotFound,
}
