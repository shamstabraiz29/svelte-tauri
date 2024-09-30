use crate::error::InternalApplicationError;

use common_simple_types::ag_id::AgId;
use common_wasm_evaluators::cloud_pattern::exports::cloudcad::cloud_pattern::evaluator::NodeRelationshipData as WasmNodeRelationshipData;

pub struct NodeRelationshipData {
    pub node_id: AgId,
    pub relationship_type: String,
    pub properties: String,
}

impl TryFrom<WasmNodeRelationshipData> for NodeRelationshipData {
    type Error = InternalApplicationError;

    fn try_from(node_relationship_data: WasmNodeRelationshipData) -> Result<Self, Self::Error> {
        Ok(NodeRelationshipData {
            node_id: AgId::from(node_relationship_data.node_id),
            relationship_type: node_relationship_data.relationship_type,
            properties: serde_json::to_string(&node_relationship_data.properties)?,
        })
    }
}
