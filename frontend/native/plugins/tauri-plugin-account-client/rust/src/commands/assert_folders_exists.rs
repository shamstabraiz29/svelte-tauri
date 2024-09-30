use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::{
    notifying_sync::notifying_state::NotifyingState, types::account_dto_state::AccountDtoState,
};
use tauri::State;

pub fn assert_folders_exits(
    acct_ag_state: &State<'_, AccountDtoState>,
    folder_ids: &[&AgId],
) -> Result<(), String> {
    // Ensure that the parent repo ID exists in the account DTO
    let mut payload: Result<(), String> = Ok(());
    acct_ag_state.read_state_value(|acct_ag_dto| {
        for folder_id in folder_ids {
            if acct_ag_dto.find_folder_by_id(folder_id).is_none() {
                payload = Err(format!("The folder with ID: ${folder_id} was not found"));
                break;
            }
        }
    });

    payload
}
