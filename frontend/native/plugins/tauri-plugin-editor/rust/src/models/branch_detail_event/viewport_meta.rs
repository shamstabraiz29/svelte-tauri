use common_dtos::commit_client::types::branch::viewport_dto::ViewportDto;
use serde_json::Value as JsonValue;
use specta::Type;
use specta_util::Unknown;
use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct ViewportMeta {
    pub id: String,
    pub name: String,
    pub r#type: String,
    #[specta(type=BTreeMap<String, Unknown>)]
    pub properties: BTreeMap<String, JsonValue>,
}

impl From<ViewportDto> for ViewportMeta {
    fn from(viewport: ViewportDto) -> Self {
        ViewportMeta {
            id: viewport.id.to_string(),
            name: viewport.name.to_string(),
            r#type: viewport.r#type.to_string(),
            properties: viewport.properties,
        }
    }
}
