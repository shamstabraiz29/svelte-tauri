use common_dtos::commit_client::types::repo::repo_dto::RepoAgDto;

use crate::{
    events::repo_detail_event::RepoDetailEvent,
    notifying_sync::{notify_rw_lock::NotifyingRwLock, notifying_state::NotifyingState},
};
use tauri::{AppHandle, Runtime};

use tauri_specta::Event;

#[derive(Debug)]
pub struct RepoDtoState {
    repo_detail: NotifyingRwLock<Option<RepoAgDto>>,
}

impl RepoDtoState {
    pub fn new<R: Runtime>(app: AppHandle<R>) -> Self {
        let callback = Box::new(move |repo_dto: &Option<RepoAgDto>| {
            // This callback now captures app_clone and can use it to emit events
            let repo_event = from(repo_dto.clone());
            log::trace!("Emitted account event: {:?}", repo_event);
            repo_event.emit(&app).unwrap_or_else(|e| {
                log::error!("Failed to emit account event: {:?}", e);
            })
        });

        Self {
            repo_detail: NotifyingRwLock::new(None, callback, None), // Assuming no read callback needed
        }
    }
}

impl NotifyingState for RepoDtoState {
    type Dto = RepoAgDto;

    fn get_notifying_lock(&self) -> &NotifyingRwLock<Option<Self::Dto>> {
        &self.repo_detail
    }
}

fn from(repo_ag_dto: Option<RepoAgDto>) -> RepoDetailEvent {
    log::trace!("from - : {:?}", repo_ag_dto);

    match repo_ag_dto {
        Some(repo_dto) => RepoDetailEvent::Repo {
            repo_detail: repo_dto.into(),
        },
        None => RepoDetailEvent::Clear,
    }
}
