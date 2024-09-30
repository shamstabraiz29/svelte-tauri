use std::collections::{HashMap, HashSet};

use frontend_tauri_plugins_common::error::{Error, Result};
use tauri::{AppHandle, Manager, Runtime};

use crate::{
    models::get_resource_item_partials::{
        request::GetResourceItemPartialsRequest,
        response::{GetResourceItemPartialsResponse, ResourceItemPartials},
    },
    types::resource_items_partials_cache::ResourceItemsPartialsCache,
};

#[tauri::command]
#[specta::specta]
pub async fn get_resource_item_partials<R: Runtime>(
    app: AppHandle<R>,
    request: GetResourceItemPartialsRequest,
) -> Result<GetResourceItemPartialsResponse> {
    log::info!(
        "Starting get_resource_item_partials function with request: {:?}",
        request
    );

    let viewport_items_static_data_cache = app.state::<ResourceItemsPartialsCache>();

    log::debug!("Retrieved ResourceItemsPartialsCache state");

    let request_id = request.req_id.clone();
    let get_resources_static_data_request = HashMap::from([(
        request.viewport_type.to_owned(),
        HashSet::from([request.resource_item_type.to_owned()]),
    )]);

    log::debug!(
        "Constructed get_resources_static_data_request: {:?}",
        get_resources_static_data_request
    );

    let get_partials_result = viewport_items_static_data_cache
        .get_resource_items_partials(&app, get_resources_static_data_request)
        .await;

    match get_partials_result {
        Ok(mut partials_map) => {
            log::debug!("Received partials_map: {:?}", partials_map);

            let partials =
                partials_map
                    .remove(&request.viewport_type)
                    .and_then(|mut viewport_partials| {
                        log::debug!(
                            "Retrieved viewport_partials for {}: {:?}",
                            request.viewport_type,
                            viewport_partials
                        );
                        viewport_partials.remove(&request.resource_item_type)
                    });

            let response_partials = match partials {
                Some(partials) => {
                    log::debug!(
                        "Found partials for resource_item_type {}: {:?}",
                        request.resource_item_type,
                        partials
                    );
                    ResourceItemPartials::Partials { values: partials }
                }
                None => {
                    log::debug!(
                        "No partials found for resource_item_type {}",
                        request.resource_item_type
                    );
                    ResourceItemPartials::NoneFound
                }
            };

            log::info!(
                "Returning GetResourceItemPartialsResponse for req_id {}",
                request_id
            );

            Ok(GetResourceItemPartialsResponse {
                req_id: request_id,
                partials: response_partials,
            })
        }
        Err(e) => {
            log::error!("Error getting resource item partials: {:?}", e);
            Err(Error {
                req_id: request_id.clone(),
                message: format!("Error getting partials: {:?}", e),
            })
        }
    }
}
