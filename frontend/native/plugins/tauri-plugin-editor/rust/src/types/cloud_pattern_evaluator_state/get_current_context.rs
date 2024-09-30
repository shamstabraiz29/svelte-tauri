use serde_json::{Map, Value as JsonValue};

use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn get_current_context(&self) -> Result<Map<String, JsonValue>, EditorError> {
        let state = self
            .state
            .lock()
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorStateLock)?;
        state.get_current_context().map(|cc| cc.to_owned())
    }
}
