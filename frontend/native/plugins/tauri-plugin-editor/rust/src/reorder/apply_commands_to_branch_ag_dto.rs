// frontend/tauri/plugins/tauri-plugin-editor/rust/src/reorder/apply_commands_to_branch_ag_dto.rs
use common_aggregates::{ag::AgBuilder, branch::branch_ag::BranchAgBuilder, BranchAg, CommitId};
use common_commands::model::ModelCommand;
use common_dtos::commit_client::types::branch::branch_dto::BranchAgDto;
use common_events_player::play_events;
use log::{debug, error, info};

use crate::EditorError;

use super::model_commands_to_events::model_commands_to_events;

pub fn apply_commands_to_branch_ag_dto(
    branch_ag_dto: BranchAgDto,
    next_commit_id: &CommitId,
    caller_id: &str,
    commands: &[ModelCommand],
) -> Result<BranchAg, EditorError> {
    info!("Starting to apply commands to BranchAgDto");
    debug!(
        "Next commit ID: {}, Caller ID: {}",
        next_commit_id, caller_id
    );
    debug!("Number of commands to apply: {}", commands.len());

    // Get events from the commands
    debug!("Converting model commands to events");
    let events = match model_commands_to_events(commands) {
        Ok(events) => {
            debug!(
                "Successfully converted {} commands to {} events",
                commands.len(),
                events.len()
            );
            events
        }
        Err(e) => {
            error!("Failed to convert model commands to events: {:?}", e);
            return Err(e);
        }
    };

    // Backup the current BranchAg's CommitId
    let previous_commit_id_bkp = branch_ag_dto.commit_id.to_owned();
    debug!("Backed up previous commit ID: {}", previous_commit_id_bkp);

    // Play events on an aggregate
    debug!("Creating BranchAgBuilder from BranchAgDto");
    let mut ag_builder = BranchAgBuilder::from(branch_ag_dto);

    debug!("Playing events on the aggregate");
    if let Err(e) = play_events::<BranchAg>(&mut ag_builder, next_commit_id, caller_id, &events) {
        error!("Failed to play events on the aggregate: {:?}", e);
        return Err(EditorError::from(e));
    }

    // Reset the CommitId to the previous one
    debug!("Resetting CommitId to the previous one");
    ag_builder.set_commit_id(Some(previous_commit_id_bkp));

    // Build the final BranchAg
    debug!("Building the final BranchAg");
    match ag_builder.build(caller_id) {
        Ok(branch_ag) => {
            info!("Successfully applied commands and built BranchAg");
            Ok(branch_ag)
        }
        Err(e) => {
            error!("Failed to build BranchAg: {:?}", e);
            Err(EditorError::from(e))
        }
    }
}
