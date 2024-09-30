use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn clear_context(&self) -> Result<(), EditorError> {
        self.state
            .lock()
            .map(|mut state| {
                state.clear();
            })
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorStateLock.into())
    }
}
