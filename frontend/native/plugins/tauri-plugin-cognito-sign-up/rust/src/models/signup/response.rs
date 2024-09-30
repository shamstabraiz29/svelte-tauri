use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "type")]

pub struct SignUpResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
    #[serde(rename = "status")]
    pub status: SignUpStatus,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "type")]
pub enum SignUpStatus {
    #[serde(rename = "confirmEmail")]
    ConfirmEmail,
    #[serde(rename = "userConfirmed")]
    UserConfirmed,
}
