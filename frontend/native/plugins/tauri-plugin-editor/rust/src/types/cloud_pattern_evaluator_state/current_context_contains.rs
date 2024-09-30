use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn current_context_contains(&self, key: &str) -> Result<bool, EditorError> {
        let state = self
            .state
            .lock()
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorStateLock)?;
        state.get_current_context().map(|cc| cc.contains_key(key))
    }
}
