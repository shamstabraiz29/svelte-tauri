use serde::Serialize;

use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn push_to_context<T: Serialize>(
        &self,
        name: String,
        value: T,
    ) -> Result<(), EditorError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorStateLock)?;
        let value = serde_json::to_value(&value)?;
        state.push_to_context(name, value)
    }
}
