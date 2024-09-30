use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct CloudPatternsGetRequest {
    pub req_id: String,
}
