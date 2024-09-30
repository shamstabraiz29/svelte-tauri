use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn push_context_namespace(&self, namespace: String) -> Result<(), EditorError> {
        log::debug!("pushed_context_namespace: {}", namespace);
        let mut state = self
            .state
            .lock()
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorStateLock)?;
        state.push_context_namespace(namespace)
    }
}
