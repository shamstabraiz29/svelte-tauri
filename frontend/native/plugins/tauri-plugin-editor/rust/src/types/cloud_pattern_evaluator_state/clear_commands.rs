use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn clear_commands(&self) -> Result<(), EditorError> {
        self.commands
            .lock()
            .map(|mut cmds| {
                cmds.take();
            })
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorCommandsLock.into())
    }
}
