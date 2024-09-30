use common_dtos::editor_client::types::cloud_pattern::CloudPatternMeta;
use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]

pub struct UiCloudPatternMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
}

impl From<CloudPatternMeta> for UiCloudPatternMeta {
    fn from(meta: CloudPatternMeta) -> Self {
        Self {
            id: meta.id,
            name: meta.name,
            description: meta.description,
            category: meta.category,
        }
    }
}
