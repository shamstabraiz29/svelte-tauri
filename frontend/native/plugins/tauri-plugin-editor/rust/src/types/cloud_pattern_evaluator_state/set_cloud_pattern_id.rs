use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn set_cloud_pattern_id(&self, cloud_pattern_id: String) -> Result<(), EditorError> {
        self.cloud_pattern_id
            .lock()
            .map(|mut cid| {
                *cid = Some(cloud_pattern_id);
            })
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorIdLock.into())
    }
}
