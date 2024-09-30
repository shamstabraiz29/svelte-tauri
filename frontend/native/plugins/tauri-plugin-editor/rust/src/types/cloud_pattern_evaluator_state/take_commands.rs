use common_commands::model::ModelCommand;

use super::CloudPatternEvaluatorState;
use crate::{error::InternalApplicationError, EditorError};

impl CloudPatternEvaluatorState {
    pub(super) fn take_commands(&self) -> Result<Vec<ModelCommand>, EditorError> {
        let cloud_pattern_id = self.get_cloud_pattern_id()?;
        self.commands
            .lock()
            .map_err(|_| InternalApplicationError::CloudPatternEvaluatorCommandsLock)?
            .take()
            .ok_or(EditorError::RecoverableError(format!(
                "The '{}' CloudPattern did not generate any changes.",
                cloud_pattern_id
            )))
    }
}
