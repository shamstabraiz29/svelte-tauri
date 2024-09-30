mod clear_commands;
mod clear_context;
mod create_node_request_into_commands;
mod create_relationship_request_into_commands;
mod current_context_contains;
mod evaluate;
mod evaluator_stack_length;
mod get_cloud_pattern_id;
mod get_cloud_pattern_meta;
mod get_current_context;
mod pop_context_namespace;
mod pop_evaluator;
mod push_context_namespace;
mod push_evaluator;
mod push_to_context;
mod reset;
mod set_cloud_pattern_id;
mod set_cloud_patterns_meta;
mod take_commands;
mod update_scope_with_response;

use std::{collections::HashMap, sync::Mutex};

use common_commands::model::ModelCommand;
use common_dtos::editor_client::types::cloud_pattern::CloudPatternMeta;
use common_wasm_evaluators::cloud_pattern::CloudPatternEvaluator;

use super::cloud_pattern_context::CloudPatternContext;

#[derive(Default)]
pub struct CloudPatternEvaluatorState {
    cloud_patterns_meta: Mutex<HashMap<String, CloudPatternMeta>>,
    evaluator: Mutex<Vec<CloudPatternEvaluator>>,
    state: Mutex<CloudPatternContext>,
    cloud_pattern_id: Mutex<Option<String>>,
    commands: Mutex<Option<Vec<ModelCommand>>>,
}
