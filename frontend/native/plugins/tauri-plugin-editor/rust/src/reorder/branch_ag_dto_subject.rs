// frontend/tauri/plugins/tauri-plugin-editor/rust/src/reorder/branch_ag_dto_subject.rs
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, RwLock},
};

use common_dtos::commit_client::types::branch::branch_dto::BranchAgDto;
use common_simple_types::ag_id::AgId;
use futures::future::{self, BoxFuture};
use log::{debug, error, info};

use super::{observer::Observer, subject::Subject};
use crate::{error::InternalApplicationError, EditorError};

#[derive(Default)]
pub struct BranchAgValues {
    old_value: Option<BranchAgDto>,
    new_value: Option<BranchAgDto>,
}

pub struct BranchAgDtoSubject {
    values: RwLock<BranchAgValues>,
    observers: Vec<Arc<dyn Observer<Option<BranchAgDto>, EditorError>>>,
}

impl BranchAgDtoSubject {
    pub(crate) fn new() -> Self {
        debug!("Creating new BranchAgDtoSubject");
        BranchAgDtoSubject {
            values: RwLock::new(BranchAgValues::default()),
            observers: Vec::new(),
        }
    }

    pub(crate) fn eval_with_value<Fxn>(&self, fxn: Fxn) -> Result<(), EditorError>
    where
        Fxn: FnOnce(&Option<BranchAgDto>),
    {
        let values = self.values.read().map_err(|e| {
            error!("Failed to acquire write lock on BranchAgValues: {:?}", e);
            InternalApplicationError::BranchAgValuesReadLock
        })?;
        fxn(&values.new_value);
        Ok(())
    }

    pub(crate) async fn set_value(&self, value: Option<BranchAgDto>) -> Result<(), EditorError> {
        info!("Setting new value for BranchAgDtoSubject");
        match &value {
            Some(value) => debug!("New value: {:#?}", value),
            None => debug!("New value is None"),
        }

        {
            // Scoping for write lock
            let mut values = self.values.write().map_err(|e| {
                error!("Failed to acquire write lock on BranchAgValues: {:?}", e);
                InternalApplicationError::BranchAgValuesWriteLock
            })?;

            match value {
                Some(value) => {
                    values.old_value = values.new_value.replace(value);
                    debug!("Updated BranchAgValues with new value");
                }
                None => {
                    values.old_value = values.new_value.take();
                    debug!("Cleared BranchAgValues");
                }
            }
        }

        debug!("Notifying observers about the value change");
        self.notify_observers().await?;
        info!("Successfully set new value for BranchAgDtoSubject");
        Ok(())
    }

    pub(crate) fn get_model_id(&self) -> Result<Option<AgId>, EditorError> {
        debug!("Getting model ID from BranchAgDtoSubject");
        let values = self.values.read().map_err(|e| {
            error!("Failed to acquire read lock on BranchAgValues: {:?}", e);
            InternalApplicationError::BranchAgValuesReadLock
        })?;

        let model_id = values
            .new_value
            .as_ref()
            .map(|branch_ag_dto| branch_ag_dto.id.clone());
        match &model_id {
            Some(id) => debug!("Retrieved model ID: {:?}", id),
            None => debug!("No model ID found"),
        }
        Ok(model_id)
    }

    pub(crate) fn get_model_root_node_id(&self) -> Result<Option<AgId>, EditorError> {
        debug!("Getting model root node ID from BranchAgDtoSubject");
        self.values
            .read()
            .map_err(|e| {
                error!("Failed to acquire read lock on BranchAgValues: {:?}", e);
                InternalApplicationError::BranchAgValuesReadLock.into()
            })
            .map(|values| {
                values
                    .new_value
                    .as_ref()
                    .map(|branch_ag_dto| branch_ag_dto.root_node_id.clone())
            })
            .map(|root_node_id| {
                match &root_node_id {
                    Some(id) => debug!("Retrieved root node ID: {:?}", id),
                    None => debug!("No root node ID found"),
                }
                root_node_id
            })
    }

    pub(crate) fn get_branch_ag_dto(&self) -> Result<Option<BranchAgDto>, EditorError> {
        debug!("Getting BranchAgDto");
        let branch_ag_dto = {
            let values = self.values.read().map_err(|e| {
                error!("Failed to acquire read lock on BranchAgValues: {:?}", e);
                InternalApplicationError::BranchAgValuesReadLock
            })?;
            values.new_value.as_ref().cloned()
        };
        Ok(branch_ag_dto)
    }
}

impl Subject<Option<BranchAgDto>, EditorError> for BranchAgDtoSubject {
    fn register_observer(
        &mut self,
        observer: Arc<dyn Observer<Option<BranchAgDto>, EditorError>>,
    ) -> Result<(), EditorError> {
        debug!("Registering new observer for BranchAgDtoSubject");
        self.observers.push(observer);
        Ok(())
    }

    fn notify_observers(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<(), EditorError>> + Send + '_>> {
        Box::pin(async move {
            info!("Notifying observers about BranchAgDto change");
            let (old_value, new_value) = {
                let values = self.values.read().map_err(|e| {
                    error!("Failed to acquire read lock on BranchAgValues: {:?}", e);
                    InternalApplicationError::BranchAgValuesReadLock
                })?;
                (values.old_value.clone(), values.new_value.clone())
            };

            let futures: Vec<_> = self
                .observers
                .iter()
                .enumerate()
                .map(|(index, observer)| {
                    let old_value = old_value.clone();
                    let new_value = new_value.clone();
                    Box::pin(async move {
                        debug!("Updating observer {}", index);
                        observer.update(old_value, new_value).await
                    }) as BoxFuture<'_, Result<(), EditorError>>
                })
                .collect();

            future::try_join_all(futures).await?;

            info!("Finished notifying all observers");
            Ok(())
        })
    }
}
