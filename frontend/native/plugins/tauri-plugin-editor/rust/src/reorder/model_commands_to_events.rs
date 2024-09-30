// frontend/tauri/plugins/tauri-plugin-editor/rust/src/reorder/model_commands_to_events.rs
use common_commands::model::ModelCommand;
use common_events::Event;
use common_traits::AgCommand;
use log::{debug, error, info};

use crate::error::InternalApplicationError;

pub(crate) fn model_commands_to_events(
    model_cmds: &[ModelCommand],
) -> Result<Vec<Event>, crate::EditorError> {
    info!("Starting model_commands_to_events conversion");
    debug!("Number of model commands to process: {}", model_cmds.len());

    let mut events = Vec::new();
    let mut ag_id = None;

    for (index, command) in model_cmds.iter().enumerate() {
        debug!("Processing command {} of {}", index + 1, model_cmds.len());

        let cmd_interpreter = command.get_command_interpreter();
        let pags = cmd_interpreter.get_partial_ag_items();

        debug!(
            "Number of partial AG items for this command: {}",
            pags.len()
        );

        for (pag_index, pag) in pags.iter().enumerate() {
            debug!(
                "Processing partial AG item {} of {}",
                pag_index + 1,
                pags.len()
            );

            match &ag_id {
                Some(existing_ag_id) => {
                    if *existing_ag_id != pag.ag_id {
                        error!("Multiple AG IDs detected in transaction. Existing: {:?}, Current: {:?}", existing_ag_id, pag.ag_id);
                        return Err(InternalApplicationError::MultiAgCommandsInTransaction.into());
                    }
                }
                None => {
                    ag_id = Some(pag.ag_id.clone()); // Clone the ag_id
                    debug!("Set AG ID to {:?}", ag_id);
                }
            }

            debug!("Adding {} events from partial AG item", pag.events.len());
            events.extend(pag.events.iter().cloned());
        }
    }

    info!("Finished model_commands_to_events conversion");
    debug!("Total number of events generated: {}", events.len());

    Ok(events)
}
