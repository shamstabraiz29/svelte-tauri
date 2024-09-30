use std::fmt::Debug;

// use common_dtos::commit_client::types::MHashable;
use serde::{de::DeserializeOwned, Serialize};

use super::notify_rw_lock::NotifyingRwLock;

pub trait NotifyingState: Send + Sync {
    type Dto: Clone + Debug + Serialize + DeserializeOwned; //+ MHashable;

    fn get_notifying_lock(&self) -> &NotifyingRwLock<Option<Self::Dto>>;

    fn set_state_value(&self, ag_dto: Option<Self::Dto>) {
        let dto_lock = self.get_notifying_lock();
        let mut lock = dto_lock.write(); // Acquire a write lock

        lock.clone_from(&ag_dto);

        log::trace!("Set ag detail - lock: {:?}", lock);
    }

    fn modify_state_value<F>(&self, f: F) -> bool
    where
        F: FnOnce(&mut Self::Dto),
    {
        let dto_lock = self.get_notifying_lock();
        let mut lock = dto_lock.write(); // Acquire a write lock

        if let Some(ref mut ag_dto) = *lock {
            f(ag_dto);
            true
        } else {
            false
        }
    }

    fn read_state_value<F>(&self, f: F) -> bool
    where
        F: FnOnce(&Self::Dto),
    {
        let dto_lock = self.get_notifying_lock();
        let lock = dto_lock.read(); // Acquire a read lock

        if let Some(ref ag_dto) = *lock {
            f(ag_dto);
            true
        } else {
            false
        }
    }
}
