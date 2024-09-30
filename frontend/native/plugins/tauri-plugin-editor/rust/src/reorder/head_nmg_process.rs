// frontend/tauri/plugins/tauri-plugin-editor/rust/src/reorder/head_nmg_process.rs
use std::{pin::Pin, sync::Arc};

use common_dtos::commit_client::types::branch::branch_dto::BranchAgDto;
use common_nmg_core::{
    db::NmgArc,
    executor::Executor,
    ql::{Mutation, NmgQlAst},
};
use futures::{future, Future};
use log::{debug, error, info};
use nmg_from_branch_ag_dto::populate_nmg_from_branch_ag_dto;
use rustc_hash::FxHashMap;
use uuid::Uuid;

use super::{nmg_from_branch_ag_dto, observer::Observer, subject::Subject};
use crate::{
    error::{HeadNmgProcessError, InternalApplicationError},
    EditorError,
};

pub struct HeadNmgProcessSubject {
    id: Uuid,
    nmg: NmgArc,
    observers: Vec<Arc<dyn Observer<NmgArc, EditorError>>>,
}

impl PartialEq for HeadNmgProcessSubject {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl HeadNmgProcessSubject {
    pub(crate) fn new() -> Self {
        info!("Creating new HeadNmgProcessSubject");
        let id = Uuid::new_v4();
        debug!("Generated new UUID for HeadNmgProcessSubject: {}", id);
        Self {
            id,
            nmg: NmgArc::default(),
            observers: Vec::new(),
        }
    }

    pub(crate) fn get_model(&self) -> NmgArc {
        debug!("Retrieving NmgArc model from HeadNmgProcessSubject");
        self.nmg.clone()
    }
}

impl Observer<Option<BranchAgDto>, EditorError> for HeadNmgProcessSubject {
    fn update(
        &self,
        _old_value: Option<BranchAgDto>,
        new_value: Option<BranchAgDto>,
    ) -> Pin<Box<dyn Future<Output = Result<(), EditorError>> + Send + '_>> {
        Box::pin(async move {
            info!("Updating HeadNmgProcessSubject");
            debug!("New value is null: {}", new_value.is_none());

            // TODO: Delete the nodes in the Nmg only if the new_value is None
            info!("Clearing existing nodes in NmgArc");
            let ast =
                NmgQlAst::<Mutation>::mutation_from_string("MATCH (n) DETACH DELETE n RETURN n")
                    .unwrap();
            let ctx = FxHashMap::default();
            let executor = Executor::new();

            match executor.execute(self.nmg.clone(), &ast, &ctx) {
                Ok(_) => debug!("Successfully cleared existing nodes in NmgArc"),
                Err(e) => {
                    error!("Failed to clear existing nodes in NmgArc: {:?}", e);
                    return Err(InternalApplicationError::HeadNmgClear(e).into());
                }
            }

            let branch_ag_dto = match new_value {
                Some(branch_ag_dto) => branch_ag_dto,
                None => {
                    info!("No new BranchAgDto provided, update complete");
                    return Ok(());
                }
            };

            info!("Populating NmgArc from new BranchAgDto");
            match populate_nmg_from_branch_ag_dto(self.nmg.clone(), &branch_ag_dto) {
                Ok(_) => debug!("Successfully populated NmgArc from BranchAgDto"),
                Err(e) => {
                    error!("Failed to populate NmgArc from BranchAgDto: {:?}", e);
                    return Err(HeadNmgProcessError::NmgFromBranchAgDto(e).into());
                }
            }

            info!("Notifying observers of update");
            if let Err(e) = self.notify_observers().await {
                error!("Failed to notify observers: {:?}", e);
                return Err(e);
            }

            info!("HeadNmgProcessSubject update completed successfully");
            Ok(())
        })
    }
}

impl Subject<NmgArc, EditorError> for HeadNmgProcessSubject {
    fn register_observer(
        &mut self,
        observer: Arc<dyn Observer<NmgArc, EditorError>>,
    ) -> Result<(), EditorError> {
        self.observers.push(observer);
        Ok(())
    }

    fn notify_observers(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<(), EditorError>> + Send + '_>> {
        Box::pin(async move {
            let futures: Vec<_> = self
                .observers
                .iter()
                .map(|observer| observer.update(self.nmg.clone(), self.nmg.clone()))
                .collect();

            future::try_join_all(futures).await?;
            Ok(())
        })
    }
}
