use common_dtos::commit_client::types::branch::viewport_item_dto::ViewportItemDto;
use common_simple_types::ag_id::AgId;
use serde::Serialize;
use serde_json::Value as JsonValue;
use specta::Type;
use specta_util::Unknown;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ViewportItemMeta {
    pub id: String,
    #[specta(type = String)]
    pub resource_item_id: AgId,
    pub resource_item_type: String,
    #[specta(type = BTreeMap<String, Unknown>)]
    pub properties: BTreeMap<String, JsonValue>,
    #[specta(type = BTreeMap<String, Unknown>)]
    pub model_data: BTreeMap<String, JsonValue>,
}

impl From<ViewportItemDto> for ViewportItemMeta {
    fn from(viewport_item: ViewportItemDto) -> Self {
        fn extract_map(
            properties: &BTreeMap<String, JsonValue>,
            key: &str,
        ) -> BTreeMap<String, JsonValue> {
            properties
                .get(key)
                .and_then(JsonValue::as_object)
                .map_or_else(BTreeMap::new, |map| map.clone().into_iter().collect())
        }

        let properties = &viewport_item.properties;

        ViewportItemMeta {
            id: viewport_item.id.to_string(),
            resource_item_id: viewport_item.model_item_id,
            resource_item_type: viewport_item.model_item_type.clone(),
            properties: extract_map(properties, "viewport_data"),
            model_data: extract_map(properties, "model_data"),
        }
    }
}
