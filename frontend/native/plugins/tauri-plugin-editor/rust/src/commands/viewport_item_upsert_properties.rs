// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/viewport_item_upsert_properties.rs
use std::collections::HashMap;

use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::error::{Error, Result};
use serde_json::Value as JsonValue;
use tauri::{AppHandle, Runtime, State};

use crate::{
    models::viewport_item_upsert_properties::{
        request::ViewportItemUpsertPropertiesRequest,
        response::ViewportItemUpsertPropertiesResponse,
    },
    reorder::{branch_ag_dto_subject::BranchAgDtoSubject, commit_commands},
};
use common_commands::model::viewport_item::UpsertPropertiesData as ViewportItemUpsertPropertiesData;

#[tauri::command]
#[specta::specta]
pub async fn viewport_item_upsert_properties<R: Runtime>(
    app: AppHandle<R>,
    branch_ag_dto_subject: State<'_, BranchAgDtoSubject>,
    request: ViewportItemUpsertPropertiesRequest,
) -> Result<ViewportItemUpsertPropertiesResponse> {
    log::info!(
        "Starting viewport_item_upsert_properties with request ID: {}",
        request.req_id
    );
    log::debug!(
        "Received viewport item upsert properties request: {:?}",
        request
    );

    let mut branch_id: Result<AgId> = Err(Error {
        req_id: request.req_id.clone(),
        message: "No open branch was found".to_string(),
    });

    let mut upserted_properties: HashMap<String, JsonValue> = HashMap::new();

    branch_ag_dto_subject
        .eval_with_value(|branch_ag_dto| {
            if let Some(dto) = branch_ag_dto {
                branch_id = Ok(dto.id.to_owned());
                let target_viewport_item_properties =
                    match dto.viewport_items.get(&request.viewport_item_id) {
                        Some(item) => &item.properties,
                        None => {
                            branch_id = Err(Error {
                                req_id: request.req_id.clone(),
                                message: "Target viewport item not found".to_string(),
                            });
                            return;
                        }
                    };
                for (prop_key, incoming_value) in request.properties {
                    match target_viewport_item_properties.get(&prop_key) {
                        Some(existing_value) => {
                            if incoming_value != *existing_value {
                                upserted_properties
                                    .insert(prop_key.to_owned(), incoming_value.to_owned());
                            }
                        }
                        None => {
                            upserted_properties
                                .insert(prop_key.to_owned(), incoming_value.to_owned());
                        }
                    }
                }
            }
        })
        .map_err(|e| Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        })?;

    if upserted_properties.is_empty() {
        log::debug!("No properties to upsert for request ID: {}", request.req_id);
        return Ok(ViewportItemUpsertPropertiesResponse {
            req_id: request.req_id,
        });
    }

    let mut commands = Vec::new();

    let branch_id = branch_id?;

    let upsert_data = ViewportItemUpsertPropertiesData::new(
        branch_id.to_owned(),
        request.viewport_item_id.to_owned(),
        upserted_properties,
    );
    commands.push(upsert_data.into());

    log::debug!("viewport item upsert properties commands: {:#?}", commands);

    log::info!("Committing viewport item upsert properties commands");
    match commit_commands(app, commands).await {
        Ok(_) => {
            log::info!(
                "Successfully upsertd viewport item properties for request ID: {}",
                request.req_id
            );
            Ok(ViewportItemUpsertPropertiesResponse {
                req_id: request.req_id,
            })
        }
        Err(e) => {
            log::error!("Error committing viewport item upsert properties commands for request ID: {}. Error: {}", request.req_id, e);
            Err(Error {
                req_id: request.req_id.clone(),
                message: e.to_string(),
            })
        }
    }
}
