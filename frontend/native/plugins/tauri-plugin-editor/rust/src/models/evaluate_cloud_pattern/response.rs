use common_simple_types::ag_id::AgId;
use common_wasm_evaluators::cloud_pattern::exports::cloudcad::cloud_pattern::evaluator::ValueType;
use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateCloudPatternResponse {
    pub req_id: String,
    pub eval_result: CloudPatternEvaluationStep,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CloudPatternEvaluationStep {
    Complete,
    DropInfoRequest(DropInfoRequest),
    SelectItemsRequest(SelectItemsRequest),
    PropertiesValuesRequest(PropertiesValuesRequest),
}

#[derive(Serialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct SelectItemsRequest {
    pub in_context_name: String,
    #[specta(type = Vec<String>)]
    pub viewport_items: Vec<AgId>,
}

#[derive(Serialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct DropInfoRequest {
    pub in_context_name: String,
    pub cursor_icon_url: Option<String>,
    pub valid_drop_locations: Vec<ViewPortItem>,
}

#[derive(Serialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct PropertiesValuesRequest {
    pub requests: Vec<PropertyValueRequest>,
}

#[derive(Serialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct PropertyValueRequest {
    pub in_context_name: String,
    pub value_type: InputType,
}

#[derive(Serialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct ViewPortItem {
    #[specta(type = String)]
    pub model_item_id: AgId,
    pub item_type: ItemType,
    #[specta(type = String)]
    pub viewport_item_id: AgId,
}

#[derive(Serialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub enum ItemType {
    Node,
    Relationship,
}

#[derive(Serialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub enum InputType {
    Text(Option<FieldValidation>),
    Password(Option<FieldValidation>),
    Email(Option<FieldValidation>),
    Number(Option<FieldValidation>),
    Radio(Options, Option<FieldValidation>),
    Checkbox(Options, Option<FieldValidation>),
    Select(Options, Option<FieldValidation>),
    Textarea(Option<FieldValidation>),
    // File,
    // Submit,
    // Reset,
    // Hidden,
    Date(Option<FieldValidation>),
    Time(Option<FieldValidation>),
    DatetimeLocal(Option<FieldValidation>),
}

#[derive(Serialize, Clone, Debug, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub options: Vec<String>,
    pub selected: Option<String>,
}

#[derive(Serialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct FieldValidation {
    // TODO: the needs to be iterated on
    pub validation: String,
}

impl From<ValueType> for InputType {
    fn from(value_type: ValueType) -> Self {
        match value_type {
            ValueType::Text => InputType::Text(None),
            ValueType::Number => InputType::Number(None),
            ValueType::Boolean => InputType::Radio(
                Options {
                    options: vec!["true".to_string(), "false".to_string()],
                    selected: None,
                },
                None,
            ),
            ValueType::Select(options) => InputType::Select(
                Options {
                    options,
                    selected: None,
                },
                None,
            ),
        }
    }
}
