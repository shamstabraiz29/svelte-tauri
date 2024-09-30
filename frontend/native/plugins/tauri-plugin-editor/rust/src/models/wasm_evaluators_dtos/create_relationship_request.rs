use std::collections::HashMap;

use common_simple_types::ag_id::AgId;
use common_wasm_evaluators::cloud_pattern::exports::cloudcad::cloud_pattern::evaluator::CreateRelationshipRequest as WasmCreateRelationshipRequest;
use serde_json::Value as JsonValue;

use crate::error::InternalApplicationError;

pub struct CreateRelationshipRequest {
    pub in_context_name: String,
    pub viewport_id: AgId,
    pub source_node_id: AgId,
    pub relationship_type: String,
    pub target_node_id: AgId,
    pub labels: Vec<String>,
    pub properties: HashMap<String, JsonValue>,
}

impl TryFrom<WasmCreateRelationshipRequest> for CreateRelationshipRequest {
    type Error = InternalApplicationError;

    fn try_from(
        create_relationship_request: WasmCreateRelationshipRequest,
    ) -> Result<Self, Self::Error> {
        Ok(CreateRelationshipRequest {
            in_context_name: create_relationship_request.in_context_name,
            viewport_id: AgId::from(create_relationship_request.viewport_id),
            source_node_id: AgId::from(create_relationship_request.source_node_id),
            relationship_type: create_relationship_request.relationship_type,
            target_node_id: AgId::from(create_relationship_request.target_node_id),
            labels: create_relationship_request.labels,
            properties: serde_json::from_str(&create_relationship_request.properties)?,
        })
    }
}
