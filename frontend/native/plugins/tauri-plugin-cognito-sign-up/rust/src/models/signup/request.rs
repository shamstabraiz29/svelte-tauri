use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct SignUpRequest {
    pub req_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub password: String,
}
