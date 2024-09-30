use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::{
    notifying_sync::notifying_state::NotifyingState, types::account_dto_state::AccountDtoState,
};
use tauri::State;

pub fn assert_repos_exits(
    acct_ag_state: &State<'_, AccountDtoState>,
    repo_ids: &[&AgId],
) -> Result<(), String> {
    // Ensure that the parent repo ID exists in the account DTO
    let mut payload: Result<(), String> = Ok(());
    acct_ag_state.read_state_value(|acct_ag_dto| {
        for repo_id in repo_ids {
            if acct_ag_dto.find_repo_by_id(repo_id).is_none() {
                payload = Err(format!("The repo with ID: ${repo_id} was not found"));
                break;
            }
        }
    });

    payload
}
