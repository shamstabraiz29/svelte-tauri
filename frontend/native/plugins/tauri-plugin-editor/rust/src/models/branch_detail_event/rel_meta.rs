use common_dtos::commit_client::types::branch::rel_dto::RelDto;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct RelMeta {
    pub id: String,
    pub rel_type: String,
    pub from: String,
    pub to: String,
    pub labels: Vec<String>,

    pub properties: BTreeMap<String, JsonValue>,
}

impl From<RelDto> for RelMeta {
    fn from(rel: RelDto) -> Self {
        RelMeta {
            id: rel.id.to_string(),
            rel_type: rel.rel_type.to_string(),
            from: rel.from.to_string(),
            to: rel.to.to_string(),
            labels: rel.labels,
            properties: rel.properties,
        }
    }
}
