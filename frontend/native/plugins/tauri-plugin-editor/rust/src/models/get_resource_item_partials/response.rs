use serde::Serialize;
use serde_json::Value as JsonValue;
use specta::Type;
use specta_util::Unknown;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceItemPartialsResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
    pub partials: ResourceItemPartials,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ResourceItemPartials {
    Partials {
        #[specta(type = Unknown)]
        values: JsonValue,
    },
    NoneFound,
}
