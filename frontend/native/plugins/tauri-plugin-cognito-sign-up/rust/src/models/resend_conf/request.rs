use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct ResendConfRequest {
    pub req_id: String,
    pub email: String,
}
