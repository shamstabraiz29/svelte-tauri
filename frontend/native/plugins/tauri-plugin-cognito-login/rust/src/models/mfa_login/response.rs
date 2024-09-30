// frontend/tauri/plugins/tauri-plugin-cognito-login/rust/src/models/mfa_login/response.rs

use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MfaLoginResponse {
    pub req_id: String,
    pub state: MfaLoginState,
}

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum MfaLoginState {
    Authenticated,
    EmailConfirmationRequired,
    Forbidden,
    MfaLoginFailed,
    MfaNotSetup,
    NotAuthorized,
    UserNotFound,
    PasswordResetRequired,
}
