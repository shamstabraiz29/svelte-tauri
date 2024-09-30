use common_dtos::commit_client::types::account::account_dto::AccountAgDto;

use tauri::{AppHandle, Runtime};
use tauri_specta::Event;

use crate::{
    events::acct_detail_event::AcctDetailEvent,
    notifying_sync::{notify_rw_lock::NotifyingRwLock, notifying_state::NotifyingState},
};

#[derive(Debug)]
pub struct AccountDtoState {
    acct_detail: NotifyingRwLock<Option<AccountAgDto>>,
}

impl AccountDtoState {
    pub fn new<R: Runtime>(app: AppHandle<R>) -> Self {
        let callback = Box::new(move |acct_dto: &Option<AccountAgDto>| {
            // This callback now captures app_clone and can use it to emit events
            let acct_event = from(acct_dto.clone());
            log::trace!("Emitted account event: {:?}", acct_event);

            acct_event.emit(&app).expect("Failed to emit account event");
        });

        Self {
            acct_detail: NotifyingRwLock::new(None, callback, None), // Assuming no read callback needed
        }
    }
}

impl NotifyingState for AccountDtoState {
    type Dto = AccountAgDto;

    fn get_notifying_lock(&self) -> &NotifyingRwLock<Option<Self::Dto>> {
        &self.acct_detail
    }
}

fn from(acct_ag_dto: Option<AccountAgDto>) -> AcctDetailEvent {
    log::trace!("from - : {:?}", acct_ag_dto);

    match acct_ag_dto {
        Some(acct_dto) => AcctDetailEvent::Account {
            account_detail: acct_dto.into(),
        },
        None => AcctDetailEvent::Clear,
    }
}
