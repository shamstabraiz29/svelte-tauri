use std::collections::HashMap;

use common_simple_types::ag_id::AgId;

use crate::models::wasm_evaluators_dtos::CreateRelationshipRequest;

use super::CloudPatternEvaluatorState;

use common_commands::model::{
    relationship::CreateData as RelationshipCreateCmdData,
    viewport_item::CreateData as ViewportItemCreateData, ModelCommand,
};

pub struct CreateRelationshipCmds {
    pub relationship_id: AgId,
    pub commands: Vec<ModelCommand>,
}

impl CloudPatternEvaluatorState {
    pub fn create_relationship_request_into_commands(
        branch_id: AgId,
        // labels: Vec<String>,
        // properties: HashMap<String, JsonValue>,
        request: CreateRelationshipRequest,
    ) -> CreateRelationshipCmds {
        // create relationship command
        let create_relationship_cmd = RelationshipCreateCmdData::new(
            branch_id.to_owned(),
            request.relationship_type.to_string(),
            request.source_node_id.to_owned(),
            request.target_node_id.to_owned(),
            request.labels,
            request.properties,
        );

        let relationship_id = create_relationship_cmd.get_rel_id().to_owned();

        // create relationship's viewport item command
        let create_viewport_cmd = ViewportItemCreateData::new(
            branch_id.to_owned(),
            relationship_id.to_owned(),
            request.relationship_type.to_owned(),
            request.viewport_id,
            HashMap::with_capacity(0),
        );

        let commands = vec![create_relationship_cmd.into(), create_viewport_cmd.into()];

        CreateRelationshipCmds {
            relationship_id,
            commands,
        }
    }
}
