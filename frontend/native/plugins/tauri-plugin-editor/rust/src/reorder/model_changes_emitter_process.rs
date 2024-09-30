// frontend/tauri/plugins/tauri-plugin-editor/rust/src/reorder/model_changes_emitter_process.rs
use std::{
    collections::{BTreeMap, HashSet},
    pin::Pin,
};

use common_dtos::commit_client::types::branch::{
    branch_dto::BranchAgDto, viewport_item_dto::ViewportItemDto,
};
use common_simple_types::ag_id::AgId;
use futures::Future;
use log::{debug, error, info};
use serde::Serialize;
use serde_json::{json, Value as JsonValue};
use tauri::{AppHandle, Manager, Runtime};
use tauri_specta::Event;
use uuid::Uuid;

use crate::{
    error::InternalApplicationError,
    models::branch_detail_event::{
        removed_viewport_items::RemovedViewportItemsEvent,
        removed_viewports::RemovedViewportsEvent,
        upserted_viewport_items::UpsertedViewportItemsEvent,
        upserted_viewports::UpsertedViewportsEvent, BranchDetailEvent,
    },
    types::node_component_state::NodeComponentState,
};

use super::observer::Observer;
use crate::EditorError;

#[derive(Debug)]
pub struct ModelChangesEmitterProcess<R: Runtime> {
    id: Uuid,
    app: AppHandle<R>,
}

impl<R: Runtime> ModelChangesEmitterProcess<R> {
    pub fn new(app: AppHandle<R>) -> ModelChangesEmitterProcess<R> {
        info!("Creating new ModelChangesEmitterProcess");
        let id = Uuid::new_v4();
        debug!("Generated new UUID for ModelChangesEmitterProcess: {}", id);
        ModelChangesEmitterProcess { id, app }
    }

    fn emit_event<E>(&self, event: &E) -> Result<(), EditorError>
    where
        E: Event + Serialize + Clone,
    {
        debug!("Emitting event");
        event.clone().emit(&self.app).map_err(|e| {
            error!("Failed to emit event: {}", e);
            InternalApplicationError::EmitEvent {
                event: "Unknown", // We can't get the event name directly
                error: e.to_string(),
            }
            .into()
        })
    }

    /// Returns a list of all the node types that are associated with the branch's viewport items.
    fn get_branch_viewport_items_node_types_set(branch_ag_dto: &BranchAgDto) -> Vec<String> {
        let mut node_types = HashSet::new();

        for viewport_item in branch_ag_dto.viewport_items.values() {
            if let Some(node_dto) = branch_ag_dto.nodes.get(&viewport_item.model_item_id) {
                if node_dto.node_type != "canvas" {
                    node_types.insert(&node_dto.node_type);
                }
            }
        }

        node_types.into_iter().cloned().collect()
    }

    fn get_branch_viewport_items_meta(
        branch_ag_dto: &BranchAgDto,
        // BTreeMap<viewport_item_id, BTreeMap<prop_name, prop_value>>
    ) -> Result<BTreeMap<AgId, BTreeMap<String, JsonValue>>, crate::EditorError> {
        let mut node_types = BTreeMap::new();

        for viewport_item in branch_ag_dto.viewport_items.values() {
            if let Some(node_dto) = branch_ag_dto.nodes.get(&viewport_item.model_item_id) {
                if node_dto.node_type != "canvas" {
                    node_types.insert(viewport_item.id.to_owned(), node_dto.properties.clone());
                }
            }
        }

        Ok(node_types)
    }

    pub(crate) async fn update_viewport_items_viewport_data(
        &self,
        branch_ag_dto: &mut BranchAgDto,
    ) -> Result<(), crate::EditorError> {
        log::debug!("update_viewport_items_viewport_data");
        let node_component_state = self.app.state::<NodeComponentState>();

        let node_types = Self::get_branch_viewport_items_node_types_set(branch_ag_dto);
        node_component_state
            .load_component_codes(self.app.app_handle(), node_types)
            .await?;

        let viewport_items_node_types = Self::get_branch_viewport_items_meta(branch_ag_dto)?;

        for (viewport_item_id, node_data) in viewport_items_node_types {
            let ViewportItemDto {
                viewport_id,
                model_item_id,
                model_item_type,
                properties: viewport_item_properties,
                ..
            } = branch_ag_dto
                .viewport_items
                .remove(&viewport_item_id)
                .unwrap(); // safe
            let viewport_item_node_properties = serde_json::to_value(node_data)?;
            let mut formatted_properties: BTreeMap<String, JsonValue> = BTreeMap::new();
            formatted_properties.insert("model_data".to_string(), viewport_item_node_properties);
            formatted_properties
                .insert("viewport_data".to_string(), json!(viewport_item_properties));

            let viewport_item_dto = ViewportItemDto {
                id: viewport_item_id.to_owned(),
                viewport_id,
                model_item_id,
                model_item_type,
                properties: formatted_properties,
            };
            branch_ag_dto
                .viewport_items
                .insert(viewport_item_id, viewport_item_dto);
        }

        log::debug!("{:#?}", branch_ag_dto);

        Ok(())
    }

    fn process_viewport_changes(
        &self,
        old: &BranchAgDto,
        new: &BranchAgDto,
    ) -> Result<(), EditorError> {
        info!("Processing viewport changes");
        let viewport_ids: HashSet<_> = old.viewports.keys().chain(new.viewports.keys()).collect();
        debug!("Total viewport IDs to process: {}", viewport_ids.len());

        let mut upserted_viewports = UpsertedViewportsEvent::default();
        let mut removed_viewports = RemovedViewportsEvent::default();

        for viewport_id in viewport_ids {
            match (
                old.viewports.get(viewport_id),
                new.viewports.get(viewport_id),
            ) {
                (Some(old_viewport), Some(new_viewport)) if old_viewport != new_viewport => {
                    debug!("Viewport {} has changed, marking for upsert", viewport_id);
                    upserted_viewports
                        .viewports
                        .insert(viewport_id.to_string(), new_viewport.clone().into());
                }
                (Some(_), None) => {
                    debug!("Viewport {} has been removed", viewport_id);
                    removed_viewports
                        .removed_viewports_ids
                        .push(viewport_id.to_string());
                }
                (None, Some(new_viewport)) => {
                    debug!("New viewport {} added", viewport_id);
                    upserted_viewports
                        .viewports
                        .insert(viewport_id.to_string(), new_viewport.clone().into());
                }
                _ => {}
            }
        }

        if !upserted_viewports.viewports.is_empty() {
            info!("Emitting upserted viewports event");
            debug!(
                "Number of upserted viewports: {}",
                upserted_viewports.viewports.len()
            );
            self.emit_event(&upserted_viewports)?;
        }

        if !removed_viewports.removed_viewports_ids.is_empty() {
            info!("Emitting removed viewports event");
            debug!(
                "Number of removed viewports: {}",
                removed_viewports.removed_viewports_ids.len()
            );
            self.emit_event(&removed_viewports)?;
        }

        Ok(())
    }

    async fn process_viewport_item_changes(
        &self,
        old: BranchAgDto,
        new: BranchAgDto,
    ) -> Result<(), EditorError> {
        info!("Processing viewport item changes");

        let viewport_item_ids: HashSet<_> = old
            .viewport_items
            .keys()
            .chain(new.viewport_items.keys())
            .collect();
        debug!(
            "Total viewport item IDs to process: {}",
            viewport_item_ids.len()
        );

        let mut upserted_viewport_items = UpsertedViewportItemsEvent::default();
        let mut removed_viewport_items = RemovedViewportItemsEvent::default();

        for viewport_item_id in viewport_item_ids {
            match (
                old.viewport_items.get(viewport_item_id),
                new.viewport_items.get(viewport_item_id),
            ) {
                (Some(old_item), Some(new_item)) => {
                    if old_item.viewport_id != new_item.viewport_id {
                        error!(
                            "Viewport item {} has mismatched viewport IDs",
                            viewport_item_id
                        );
                        return Err(InternalApplicationError::ViewportItemViewportIdMismatch {
                            old_viewport_id: old_item.viewport_id.to_owned(),
                            new_viewport_id: new_item.viewport_id.to_owned(),
                            viewport_item_id: viewport_item_id.to_owned(),
                        }
                        .into());
                    }
                    if old_item != new_item {
                        debug!(
                            "Viewport item {} has changed, marking for upsert",
                            viewport_item_id
                        );
                        upserted_viewport_items
                            .viewport_hash_items
                            .entry(new_item.viewport_id.to_string())
                            .or_insert_with(Vec::new)
                            .push(new_item.clone().into());
                    }
                }
                (Some(old_item), None) => {
                    debug!("Viewport item {} has been removed", viewport_item_id);
                    removed_viewport_items
                        .removed_viewport_items
                        .entry(old_item.viewport_id.to_string())
                        .or_insert_with(Vec::new)
                        .push(viewport_item_id.to_string());
                }
                (None, Some(new_item)) => {
                    debug!("New viewport item {} added", viewport_item_id);
                    upserted_viewport_items
                        .viewport_hash_items
                        .entry(new_item.viewport_id.to_string())
                        .or_insert_with(Vec::new)
                        .push(new_item.clone().into());
                }
                (None, None) => {}
            }
        }

        if !upserted_viewport_items.viewport_hash_items.is_empty() {
            info!("Emitting upserted viewport items event");
            debug!(
                "Number of viewports with upserted items: {}",
                upserted_viewport_items.viewport_hash_items.len()
            );
            self.emit_event(&upserted_viewport_items)?;
        }

        if !removed_viewport_items.removed_viewport_items.is_empty() {
            info!("Emitting removed viewport items event");
            debug!(
                "Number of viewports with removed items: {}",
                removed_viewport_items.removed_viewport_items.len()
            );
            self.emit_event(&removed_viewport_items)?;
        }

        Ok(())
    }
}

impl<R: Runtime> PartialEq for ModelChangesEmitterProcess<R> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<R: Runtime> Observer<Option<BranchAgDto>, EditorError> for ModelChangesEmitterProcess<R> {
    fn update(
        &self,
        old_value: Option<BranchAgDto>,
        new_value: Option<BranchAgDto>,
    ) -> Pin<Box<dyn Future<Output = Result<(), EditorError>> + Send + '_>> {
        Box::pin(async move {
            info!("Updating ModelChangesEmitterProcess");
            debug!(
                "Old value is None: {}, New value is None: {}",
                old_value.is_none(),
                new_value.is_none()
            );

            match (old_value, new_value) {
                (None, None) => {
                    debug!("Both old and new values are None, no action needed");
                }
                (Some(mut old), Some(mut new)) => {
                    debug!("Both old and new values are Some, processing changes");

                    self.update_viewport_items_viewport_data(&mut old).await?;
                    self.update_viewport_items_viewport_data(&mut new).await?;

                    if old.id != new.id {
                        info!("DTOs belong to different branches, emitting branch detail event");
                        let payload = BranchDetailEvent::Branch {
                            branch_detail: new.clone().into(),
                        };
                        self.emit_event(&payload)?;
                        return Ok(());
                    }

                    if old.viewport_items == new.viewport_items && old.commit_id == new.commit_id {
                        info!("DTOs have the same CommitId and viewport items, emitting branch detail event");
                        let payload = BranchDetailEvent::Branch {
                            branch_detail: new.clone().into(),
                        };
                        self.emit_event(&payload)?;
                        return Ok(());
                    }

                    self.process_viewport_changes(&old, &new)?;
                    self.process_viewport_item_changes(old, new).await?;
                }
                (Some(_), None) => {
                    info!("New value is None, emitting branch detail clear event");
                    self.emit_event(&BranchDetailEvent::Clear)?;
                }
                (None, Some(mut new)) => {
                    info!("Old value is None, emitting branch detail branch event");
                    self.update_viewport_items_viewport_data(&mut new).await?;
                    debug!("New BranchAg value: {:#?}", new);
                    let payload = BranchDetailEvent::Branch {
                        branch_detail: new.into(),
                    };
                    self.emit_event(&payload)?;
                }
            }

            Ok(())
        })
    }
}
