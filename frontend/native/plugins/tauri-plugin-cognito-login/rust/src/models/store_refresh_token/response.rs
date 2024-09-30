use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct StoreRefreshTokenResponse {
    pub req_id: String,
    pub state: StoreRefreshTokenState,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum StoreRefreshTokenState {
    Success,
    AccessMismatchToken,
    NotAuthenticated,
}
