use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn pop_context_namespace(&self) -> Result<(), EditorError> {
        log::debug!("popped_context_namespace");
        let mut state = self
            .state
            .lock()
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorStateLock)?;
        state.pop_context_namespace()
    }
}
