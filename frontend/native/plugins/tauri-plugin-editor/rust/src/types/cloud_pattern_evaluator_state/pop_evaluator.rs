use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn pop_evaluator(&self) -> Result<(), EditorError> {
        let evaluator = {
            self.evaluator
                .lock()
                .map(|mut cid| (*cid).pop())
                .map_err(|_| InternalApplicationError::CloudPatternEvaluatorLock)?
        };
        let cloud_pattern_id = evaluator
            .map(|e| e.cloud_pattern_id().to_owned())
            .unwrap_or_default();
        log::debug!("Popping evaluator for CloudPattern: {}", cloud_pattern_id);
        Ok(())
    }
}
