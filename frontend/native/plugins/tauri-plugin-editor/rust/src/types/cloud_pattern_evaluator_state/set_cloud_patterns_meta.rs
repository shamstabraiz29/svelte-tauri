use std::collections::HashMap;

use common_dtos::editor_client::types::cloud_pattern::CloudPatternMeta;

use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(crate) fn set_cloud_patterns_meta(
        &self,
        cloud_patterns_meta: HashMap<String, CloudPatternMeta>,
    ) -> Result<(), EditorError> {
        log::debug!("Setting CloudPatterns Meta: {:#?}", cloud_patterns_meta);
        {
            // Scoping for write lock
            let mut cloud_patterns_meta_lock = self
                .cloud_patterns_meta
                .lock()
                .map_err(|_| InternalApplicationError::CloudPatternEvaluatorMetaLock)?;
            *cloud_patterns_meta_lock = cloud_patterns_meta;
        }
        Ok(())
    }
}
