use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct ConfirmEmailRequest {
    pub req_id: String,
    pub email: String,
    pub confirmation_code: String,
}
