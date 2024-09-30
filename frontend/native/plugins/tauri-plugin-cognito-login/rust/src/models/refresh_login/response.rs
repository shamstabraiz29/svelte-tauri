use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RefreshLoginResponse {
    pub req_id: String,
    pub state: RefreshLoginState,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum RefreshLoginState {
    Authenticated,
    NoRefreshTokenAvailable,
    Forbidden,
    NotAuthorized,
}
