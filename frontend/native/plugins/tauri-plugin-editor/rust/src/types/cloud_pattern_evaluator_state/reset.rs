use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn reset(&self) -> Result<(), EditorError> {
        self.clear_context()?;
        self.clear_commands()?;
        let _ = self
            .cloud_pattern_id
            .lock()
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorIdLock)?
            .take();
        self.evaluator
            .lock()
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorLock)?
            .clear();
        Ok(())
    }
}
