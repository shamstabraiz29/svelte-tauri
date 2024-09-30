use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn get_cloud_pattern_id(&self) -> Result<String, EditorError> {
        self.cloud_pattern_id
            .lock()
            .map(|id| id.to_owned())
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorIdReadLock)?
            .ok_or_else(|| {
                EditorError::RecoverableError("CloudPattern evaluator not initialized.".to_owned())
            })
    }
}
