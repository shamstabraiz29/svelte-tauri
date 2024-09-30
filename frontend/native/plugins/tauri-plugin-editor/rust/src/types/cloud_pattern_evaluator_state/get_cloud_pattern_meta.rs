use common_dtos::editor_client::types::cloud_pattern::CloudPatternMeta;

use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn get_cloud_pattern_meta(
        &self,
        cloud_pattern_id: &str,
    ) -> Result<CloudPatternMeta, EditorError> {
        log::info!("get_cloud_pattern_meta");

        let cloud_patterns_meta_lock = self
            .cloud_patterns_meta
            .lock()
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorMetaLock)?;
        cloud_patterns_meta_lock
            .get(cloud_pattern_id)
            .cloned()
            .ok_or_else(|| {
                InternalApplicationError::CloudPatternEvaluatorMetaNotFound {
                    cloud_pattern_id: cloud_pattern_id.to_owned(),
                }
                .into()
            })
    }
}
