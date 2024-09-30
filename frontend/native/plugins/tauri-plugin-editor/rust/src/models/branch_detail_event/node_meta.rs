use common_dtos::commit_client::types::branch::node_dto::NodeDto;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct NodeMeta {
    pub id: String,
    pub node_type: String,
    pub labels: Vec<String>,

    pub properties: BTreeMap<String, JsonValue>,
}

impl From<NodeDto> for NodeMeta {
    fn from(node: NodeDto) -> Self {
        NodeMeta {
            id: node.id.to_string(),
            node_type: node.node_type.to_string(),
            labels: node.labels,
            properties: node.properties,
        }
    }
}
