use std::collections::HashMap;

use crate::error::InternalApplicationError;
use common_wasm_evaluators::cloud_pattern::exports::cloudcad::cloud_pattern::evaluator::CreateNodeRequest as WasmCreateNodeRequest;
use serde_json::Value as JsonValue;

use super::{DropInfo, NodeRelationshipData};

pub struct CreateNodeRequest {
    pub in_context_name: String,
    pub node_type: String,
    pub drop_info: Option<DropInfo>,
    pub properties: HashMap<String, JsonValue>,
    pub insert_relationship: NodeRelationshipData,
}

impl TryFrom<WasmCreateNodeRequest> for CreateNodeRequest {
    type Error = InternalApplicationError;

    fn try_from(create_node_request: WasmCreateNodeRequest) -> Result<Self, Self::Error> {
        Ok(CreateNodeRequest {
            in_context_name: create_node_request.in_context_name,
            node_type: create_node_request.node_type,
            drop_info: create_node_request.drop_info.map(Into::into),
            properties: serde_json::from_str(&create_node_request.properties)?,
            insert_relationship: create_node_request.insert_relationship.try_into()?,
        })
    }
}
