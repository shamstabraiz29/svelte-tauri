use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn evaluator_stack_length(&self) -> Result<usize, EditorError> {
        self.evaluator
            .lock()
            .map(|cid| (*cid).len())
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorLock.into())
    }
}
