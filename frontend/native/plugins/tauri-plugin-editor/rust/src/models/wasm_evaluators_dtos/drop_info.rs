use common_simple_types::ag_id::AgId;
use common_wasm_evaluators::cloud_pattern::exports::cloudcad::cloud_pattern::evaluator::DropInfo as WasmDropInfo;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DropInfo {
    pub x: i32,
    pub y: i32,
    pub viewport_id: AgId,
    pub viewport_type: String,
    pub dropped_on_node: AgId,
}

impl From<WasmDropInfo> for DropInfo {
    fn from(drop_info: WasmDropInfo) -> Self {
        DropInfo {
            x: drop_info.x,
            y: drop_info.y,
            viewport_id: AgId::from(drop_info.viewport_id),
            viewport_type: drop_info.viewport_type,
            dropped_on_node: AgId::from(drop_info.dropped_on_node),
        }
    }
}
