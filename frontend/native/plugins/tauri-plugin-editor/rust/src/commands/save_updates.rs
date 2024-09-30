// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/node_upsert_properties.rs

use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::error::{Error, Result};
use serde_json::Value as JsonValue;
use tauri::{AppHandle, Manager, Runtime, State};

use crate::{
    models::save_updates::{request::SaveUpdatesRequest, response::SaveUpdatesResponse},
    reorder::{
        branch_ag_dto_subject::BranchAgDtoSubject, commit_commands,
        head_nmg_process::HeadNmgProcessSubject,
    },
    types::node_component_state::NodeComponentState,
};
use common_commands::model::{
    node::{
        RemovePropertiesData as NodeRemovePropertiesData,
        UpsertPropertiesData as NodeUpsertPropertiesData,
    },
    relationship::{
        RemovePropertiesData as RelRemovePropertiesData,
        UpsertPropertiesData as RelUpsertPropertiesData,
    },
    viewport_item::{
        RemovePropertiesData as VpItemRemovePropertiesData,
        UpsertPropertiesData as VpItemUpsertPropertiesData,
    },
};

struct CommandsData {
    branch_id: AgId,
    viewport_item_changes: HashMap<AgId, PropertiesChange>,
    node_changes: HashMap<AgId, (String, PropertiesChange)>,
    relationship_changes: HashMap<AgId, PropertiesChange>,
}

impl CommandsData {
    fn is_empty(&self) -> bool {
        self.viewport_item_changes.is_empty()
            && self.node_changes.is_empty()
            && self.relationship_changes.is_empty()
    }
}

struct PropertiesChange {
    upserted_properties: HashMap<String, JsonValue>,
    removed_properties: Vec<String>,
}

fn compute_properties_change(
    requested_properties_to_upsert: HashMap<String, JsonValue>,
    requested_properties_to_remove: Vec<String>,
    saved_properties: &BTreeMap<String, JsonValue>,
    properties_change: &mut PropertiesChange,
) {
    // Add properties to the upserted properties map if they are different from the saved properties
    // or if they are new properties.
    for (prop_name, prop_value) in requested_properties_to_upsert {
        match saved_properties.get(&prop_name) {
            Some(saved_prop_value) => {
                if *saved_prop_value != prop_value {
                    properties_change
                        .upserted_properties
                        .insert(prop_name.to_owned(), prop_value.to_owned());
                }
            }
            None => {
                properties_change
                    .upserted_properties
                    .insert(prop_name.to_owned(), prop_value.to_owned());
            }
        }
    }

    // Remove properties from the upserted properties map if they are in the removed properties list.
    // Add properties to the removed properties list if they are in the saved properties.
    for prop_name in requested_properties_to_remove {
        match saved_properties.get(&prop_name) {
            Some(_) => {
                properties_change
                    .removed_properties
                    .push(prop_name.to_owned());
            }
            None => {
                if properties_change
                    .upserted_properties
                    .contains_key(&prop_name)
                {
                    properties_change.upserted_properties.remove(&prop_name);
                }
            }
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn save_updates<R: Runtime>(
    app: AppHandle<R>,
    branch_ag_dto_subject: State<'_, BranchAgDtoSubject>,
    head_nmg_process: State<'_, Arc<HeadNmgProcessSubject>>,
    node_template_state: State<'_, NodeComponentState>,
    request: SaveUpdatesRequest,
) -> Result<SaveUpdatesResponse> {
    log::info!("Starting save_updates with request ID: {}", request.req_id);
    log::debug!("Received save updates request: {:?}", request);

    let mut commands_data: Result<CommandsData> = Err(Error {
        req_id: request.req_id.clone(),
        message: "No open branch was found".to_string(),
    });

    branch_ag_dto_subject
        .eval_with_value(|branch_ag_dto| {
            if let Some(dto) = branch_ag_dto {
                let mut commands_data_inner = CommandsData {
                    branch_id: dto.id.to_owned(),
                    viewport_item_changes: HashMap::new(),
                    node_changes: HashMap::new(),
                    relationship_changes: HashMap::new(),
                };

                for tracked_viewport_item in request.tracked_viewport_items {
                    let Some(saved_viewport_item_properties) = dto
                        .viewport_items
                        .get(&tracked_viewport_item.vp_id)
                        .map(|vp| &vp.properties)
                    else {
                        commands_data = Err(Error {
                            req_id: request.req_id.clone(),
                            message: format!(
                                "Viewport item not found: {}",
                                tracked_viewport_item.vp_id
                            ),
                        });
                        return;
                    };

                    // Get the viewport item change for the tracked viewport item or create a new one.
                    let mut viewport_item_change = match commands_data_inner
                        .viewport_item_changes
                        .remove(&tracked_viewport_item.vp_id)
                    {
                        Some(viewport_item_change) => viewport_item_change,
                        None => PropertiesChange {
                            upserted_properties: HashMap::new(),
                            removed_properties: Vec::new(),
                        },
                    };

                    compute_properties_change(
                        tracked_viewport_item.vp_delta.upserted_properties,
                        tracked_viewport_item.vp_delta.removed_properties,
                        saved_viewport_item_properties,
                        &mut viewport_item_change,
                    );

                    commands_data_inner
                        .viewport_item_changes
                        .insert(tracked_viewport_item.vp_id, viewport_item_change);

                    if let Some(saved_node) = dto.nodes.get(&tracked_viewport_item.m_id) {
                        // Get the model item change for the tracked viewport item or create a new one.
                        let (node_type, mut properties_change) = match commands_data_inner
                            .node_changes
                            .remove(&tracked_viewport_item.m_id)
                        {
                            Some((node_type, properties_change)) => (node_type, properties_change),
                            None => (
                                saved_node.node_type.to_owned(),
                                PropertiesChange {
                                    upserted_properties: HashMap::new(),
                                    removed_properties: Vec::new(),
                                },
                            ),
                        };

                        compute_properties_change(
                            tracked_viewport_item.m_delta.upserted_properties,
                            tracked_viewport_item.m_delta.removed_properties,
                            &saved_node.properties,
                            &mut properties_change,
                        );

                        commands_data_inner
                            .node_changes
                            .insert(tracked_viewport_item.m_id, (node_type, properties_change));
                    } else if let Some(saved_relationship) =
                        dto.relationships.get(&tracked_viewport_item.m_id)
                    {
                        // Get the model item change for the tracked viewport item or create a new one.
                        let mut properties_change = match commands_data_inner
                            .relationship_changes
                            .remove(&tracked_viewport_item.m_id)
                        {
                            Some(properties_change) => properties_change,
                            None => PropertiesChange {
                                upserted_properties: HashMap::new(),
                                removed_properties: Vec::new(),
                            },
                        };

                        compute_properties_change(
                            tracked_viewport_item.m_delta.upserted_properties,
                            tracked_viewport_item.m_delta.removed_properties,
                            &saved_relationship.properties,
                            &mut properties_change,
                        );

                        commands_data_inner
                            .relationship_changes
                            .insert(tracked_viewport_item.m_id, properties_change);
                    } else {
                        commands_data = Err(Error {
                            req_id: request.req_id.clone(),
                            message: format!(
                                "Model item not found: {}",
                                tracked_viewport_item.m_id
                            ),
                        });
                        return;
                    }
                }

                commands_data = Ok(commands_data_inner);
            }
        })
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    let commands_data = commands_data?;

    // validate the node properties
    for (node_type, properties_change) in commands_data.node_changes.values() {
        let nmg = head_nmg_process.get_model();

        let properties =
            serde_json::to_string(&properties_change.upserted_properties).map_err(|e| Error {
                req_id: request.req_id.clone(),
                message: e.to_string(),
            })?;

        node_template_state
            .validate_properties(app.app_handle(), node_type, &properties, nmg)
            .await
            .map_err(|e| Error {
                req_id: request.req_id.clone(),
                message: e.to_string(),
            })?;
    }

    if commands_data.is_empty() {
        log::debug!("Request ID: {} contains no viable updates.", request.req_id);
        return Ok(SaveUpdatesResponse {
            req_id: request.req_id,
        });
    }

    let mut commands = Vec::new();

    let branch_id = commands_data.branch_id.to_owned();

    for (node_id, (_node_type, properties_change)) in commands_data.node_changes {
        if !properties_change.upserted_properties.is_empty() {
            let upsert_data = NodeUpsertPropertiesData {
                branch_id: branch_id.to_owned(),
                node_id: node_id.to_owned(),
                properties: properties_change.upserted_properties,
            };
            commands.push(upsert_data.into());
        }

        if !properties_change.removed_properties.is_empty() {
            let remove_data = NodeRemovePropertiesData {
                branch_id: branch_id.to_owned(),
                node_id,
                properties: properties_change.removed_properties,
            };
            commands.push(remove_data.into());
        }
    }

    for (rel_id, properties_change) in commands_data.relationship_changes {
        if !properties_change.upserted_properties.is_empty() {
            let upsert_data = RelUpsertPropertiesData {
                branch_id: branch_id.to_owned(),
                rel_id: rel_id.to_owned(),
                properties: properties_change.upserted_properties,
            };
            commands.push(upsert_data.into());
        }

        if !properties_change.removed_properties.is_empty() {
            let remove_data = RelRemovePropertiesData {
                branch_id: branch_id.to_owned(),
                rel_id,
                properties: properties_change.removed_properties,
            };
            commands.push(remove_data.into());
        }
    }

    for (viewport_item_id, properties_change) in commands_data.viewport_item_changes {
        if !properties_change.upserted_properties.is_empty() {
            let upsert_data = VpItemUpsertPropertiesData {
                branch_id: branch_id.to_owned(),
                viewport_item_id: viewport_item_id.to_owned(),
                properties: properties_change.upserted_properties,
            };
            commands.push(upsert_data.into());
        }

        if !properties_change.removed_properties.is_empty() {
            let remove_data = VpItemRemovePropertiesData {
                branch_id: branch_id.to_owned(),
                viewport_item_id,
                properties: properties_change.removed_properties,
            };
            commands.push(remove_data.into());
        }
    }

    log::debug!("save updates commands: {:#?}", commands);

    log::info!("Committing save updates commands");
    match commit_commands(app, commands).await {
        Ok(_) => {
            log::info!(
                "Successfully upsertd node properties for request ID: {}",
                request.req_id
            );
            Ok(SaveUpdatesResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => {
            log::error!(
                "Error committing save updates commands for request ID: {}. Error: {}",
                request.req_id,
                e
            );
            Err(Error {
                req_id: request.req_id.clone(),
                message: e.to_string(),
            })
        }
    }
}
