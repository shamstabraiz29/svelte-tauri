use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::{
    notifying_sync::notifying_state::NotifyingState, types::repo_detail_state::RepoDtoState,
};
use tauri::State;

pub fn assert_branches_exists(
    repo_ag_state: &State<'_, RepoDtoState>,
    branch_ids: &[&AgId],
) -> Result<(), String> {
    let mut payload: Result<(), String> = Ok(());
    repo_ag_state.read_state_value(|repo_ag_dto| {
        for branch_id in branch_ids {
            if !repo_ag_dto.branches.contains_key(branch_id) {
                payload = Err(format!("The branch with ID: ${branch_id} was not found"));
                break;
            }
        }
    });

    payload
}
