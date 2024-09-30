use std::collections::HashMap;

use common_simple_types::ag_id::AgId;
use serde_json::json;

use crate::models::wasm_evaluators_dtos::CreateNodeRequest;

use super::CloudPatternEvaluatorState;

use common_commands::model::{
    node::CreateData as NodeCreateCmdData, relationship::CreateData as RelCreateData,
    viewport_item::CreateData as ViewportItemCreateData, ModelCommand,
};

pub struct CreateNodeCmds {
    pub node_id: String,
    pub commands: Vec<ModelCommand>,
}

impl CloudPatternEvaluatorState {
    pub(super) fn create_node_request_into_commands(
        branch_id: AgId,
        labels: Vec<String>,
        request: CreateNodeRequest,
    ) -> CreateNodeCmds {
        // create node command
        let create_node_cmd = NodeCreateCmdData::new(
            branch_id.to_owned(),
            request.node_type.to_owned(),
            labels,
            request.properties,
        );
        let node_id = create_node_cmd.get_node_id();

        let mut commands = vec![create_node_cmd.into()];

        if let Some(drop_info) = request.drop_info {
            // create node's viewport item command
            let mut viewport_item_properties = HashMap::new();
            viewport_item_properties.insert(
                "position".to_owned(),
                json!({
                    "x": drop_info.x,
                    "y": drop_info.y,
                }),
            );
            let create_viewport_cmd = ViewportItemCreateData::new(
                branch_id.to_owned(),
                node_id.to_owned(),
                request.node_type.to_owned(),
                drop_info.viewport_id,
                viewport_item_properties,
            );
            commands.push(create_viewport_cmd.into());
        }

        // create relationship command
        let create_rel_cmd = RelCreateData::new(
            branch_id.to_owned(),
            request.insert_relationship.relationship_type.to_string(),
            node_id.to_owned(),
            request.insert_relationship.node_id,
            vec![],
            HashMap::new(),
        );
        commands.push(create_rel_cmd.into());

        CreateNodeCmds {
            node_id: node_id.to_string(),
            commands,
        }
    }
}
