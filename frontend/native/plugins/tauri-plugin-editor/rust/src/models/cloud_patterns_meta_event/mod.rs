mod ui_cloud_pattern_meta;

use std::collections::HashMap;

use serde::Serialize;
use specta::Type;

use self::ui_cloud_pattern_meta::UiCloudPatternMeta;

#[derive(Serialize, Clone, Debug, Type, tauri_specta::Event)]
#[serde(tag = "type")]
pub enum CloudPatternsMetaEvent {
    #[serde(rename = "upsert")]
    Upsert {
        #[serde(rename = "cloudPatternsMeta")]
        cloud_patterns_meta: HashMap<String, Vec<UiCloudPatternMeta>>,
    },
    Clear,
}
