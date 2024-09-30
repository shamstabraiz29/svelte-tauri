use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct CustomCursorRequest {
    pub req_id: String,
    pub icon_url: String,
}
