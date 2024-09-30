use tauri::{ipc::IpcResponse, AppHandle, Runtime};
use tauri_specta::Event;

use crate::{
    events::commit_info_event::CommitInfoEvent, notifying_sync::notify_rw_lock::NotifyingRwLock,
    types::commit_info::CommitInfo,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct AgCommitState<U: Send + Sync + Debug + Clone> {
    ag_commit_info: NotifyingRwLock<Option<CommitInfo<U>>>,
}

impl<U: Send + Sync + Debug + Clone> AgCommitState<U> {
    pub fn new<R: Runtime>(app: AppHandle<R>) -> Self {
        let callback = Box::new(move |commit_info: &Option<CommitInfo<U>>| {
            let ag_commit_info_event: CommitInfoEvent = from(commit_info.clone());

            log::debug!(
                "Emitting ag commit info event: payload: {:?}",
                ag_commit_info_event.clone().body()
            );

            // FIXME: This may be an issue if the compiler creates a single type.
            ag_commit_info_event
                .emit(&app)
                .expect("Failed to emit ag commit info event");
        });

        Self {
            ag_commit_info: NotifyingRwLock::new(None, callback, None),
        }
    }
    pub fn set_ag_commit_info(&self, ag_commit_info: Option<CommitInfo<U>>) {
        log::debug!("set_ag_commit_info: {:?}", ag_commit_info);

        let mut write_lock = self.ag_commit_info.write(); // Acquire a write lock

        write_lock.clone_from(&ag_commit_info);

        // log::trace!("Set ag commit info - lock: {:?}", write_lock);
    }

    pub fn get_ag_commit_info(&self) -> Option<CommitInfo<U>> {
        log::debug!("get_ag_commit_info");

        self.ag_commit_info.read().clone() // Acquire a read lock

        // log::debug!("Get ag commit info - lock: {:?}", read_lock);

        // read_lock.clone()
    }
}

fn from<U: Send + Sync + Debug + Clone>(ag_commit_into: Option<CommitInfo<U>>) -> CommitInfoEvent {
    log::trace!("from - : {:?}", ag_commit_into);

    match ag_commit_into {
        Some(ag_commit_info) => {
            let commit_id = ag_commit_info.commit_id.to_string();
            let next_commit_id = ag_commit_info.next_commit_id.to_string();

            CommitInfoEvent::CommitInfo {
                commit_id,
                next_commit_id,
            }
        }
        None => CommitInfoEvent::Clear,
    }
}
