use std::fmt::Debug;

use common_dtos::commit_client::{types::MHashable, AgOperationStatus};
use common_simple_types::{ag_id::AgId, commit_id::CommitId};
use serde::{de::DeserializeOwned, Serialize};
use tauri::State;

use crate::{
    notifying_sync::notifying_state::NotifyingState, states::ag_commit_state::AgCommitState,
    types::commit_info::CommitInfo,
};

pub fn update_ag_dto_from_status<T: NotifyingState, U: Send + Sync + Debug + Clone>(
    ag_id: &AgId,
    req_id: &str,
    status: AgOperationStatus,
    ag_commit_state: &State<'_, AgCommitState<U>>,
    ag_state: &State<'_, T>,
) -> Result<String, (String, String)>
where
    T::Dto: MHashable,
{
    match status {
        AgOperationStatus::Success { ag_deltas } => {
            let ag_delta = match ag_deltas.get(ag_id) {
                Some(ag_delta) => ag_delta,
                None => todo!("Handle error: ag_delta not found"),
            };

            let cloud_m_hash = ag_delta.m_hash.clone();

            let mut ag_commit_id = None;
            let mut success = false;

            ag_state.modify_state_value(|ag_dto| {
                let patched_ag_dto: T::Dto = patch_dto(ag_dto, &ag_delta.patch);

                ag_commit_id = Some(ag_delta.commit_id.clone());

                let local_m_hash = patched_ag_dto.m_hash();

                if cloud_m_hash != local_m_hash {
                    log::error!(
                        "Cloud m_hash does not match local m_hash - out of sync with cloud"
                    );
                } else {
                    success = true;
                    *ag_dto = patched_ag_dto;
                }
            });

            if !success {
                ag_state.set_state_value(None);
                ag_commit_state.set_ag_commit_info(None);
                return Err((
                    req_id.to_string(),
                    "Cloud m_hash does not match local m_hash - out of sync with cloud".to_string(),
                ));
            }

            let commit_info = CommitInfo {
                commit_id: ag_commit_id.unwrap(),
                next_commit_id: ag_delta.next_commit_id.clone(),
                _phantom: std::marker::PhantomData,
            };
            ag_commit_state.set_ag_commit_info(Some(commit_info));
            Ok(req_id.to_string())
        }
        AgOperationStatus::Failure(message) => {
            log::error!("Error creating folder: {:?}", message);
            ag_commit_state.set_ag_commit_info(None);
            Err((req_id.to_string(), message))
        }
    }
}

pub fn new_ag_dto_from_status<T: Serialize + DeserializeOwned>(
    ag_id: &AgId,
    req_id: &str,
    status: AgOperationStatus,
) -> Result<(CommitId, T), (String, String)> {
    match status {
        AgOperationStatus::Success { ag_deltas } => {
            let ag_delta = match ag_deltas.get(ag_id) {
                Some(ag_delta) => ag_delta,
                None => todo!("Handle error: ag_delta not found"),
            };

            let next_commit_id = ag_delta.next_commit_id.clone();

            Ok((next_commit_id, dto_from_patch(&ag_delta.patch)))
        }
        AgOperationStatus::Failure(message) => {
            log::error!("Error creating folder: {:?}", message);
            Err((req_id.to_string(), message))
        }
    }
}

pub fn patch_dto<T: Serialize + DeserializeOwned>(dto: &T, diff: &json_patch::Patch) -> T {
    log::debug!("patch_dto");
    let mut dto_json = serde_json::to_value(dto).unwrap();

    log::trace!("dto_json: {:#?}", dto_json);
    log::trace!("diff: {:#?}", diff);

    json_patch::patch(&mut dto_json, diff).unwrap();

    serde_json::from_value(dto_json).unwrap()
}

pub(crate) fn dto_from_patch<T: Serialize + DeserializeOwned>(diff: &json_patch::Patch) -> T {
    let mut dto_json = serde_json::Value::Null;

    log::trace!("diff: {:#?}", diff);

    json_patch::patch(&mut dto_json, diff).unwrap();
    serde_json::from_value(dto_json).unwrap()
}
