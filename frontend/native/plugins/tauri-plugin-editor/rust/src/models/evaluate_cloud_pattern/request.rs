use common_simple_types::ag_id::AgId;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use specta::Type;
use specta_util::Unknown;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct EvaluateCloudPatternRequest {
    pub req_id: String,
    pub responding_with: CloudPatternRequestResponse,
}

#[derive(Debug, Deserialize, Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CloudPatternRequestResponse {
    Init {
        #[serde(rename = "cloudPatternId")]
        cloud_pattern_id: String,
    },
    DropLocationInfo(DropLocationRequestResponse),
    PropertiesValues(PropertyValueRequestResponses),
}

#[derive(Debug, Deserialize, Type, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DropLocationRequestResponse {
    pub in_context_name: String,
    pub x: i32,
    pub y: i32,
    #[specta(type=String)]
    pub viewport_id: AgId,
    #[specta(type=String)]
    pub dropped_on_node: AgId,
}

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PropertyValueRequestResponses {
    pub responses: Vec<PropertyValueRequestResponse>,
}

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct PropertyValueRequestResponse {
    pub in_context_name: String,
    #[specta(type=Unknown)]
    pub value: JsonValue,
}
