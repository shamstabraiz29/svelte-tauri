use std::collections::HashMap;

use frontend_tauri_plugins_common::error::{Error, Result};
use tauri::{AppHandle, Manager, Runtime};

use crate::{
    models::get_resource_items_partials::{
        request::GetResourceItemsPartialsRequest,
        response::{GetResourceItemsPartialsResponse, ResourceItemsPartialsData},
    },
    types::resource_items_partials_cache::ResourceItemsPartialsCache,
};

#[tauri::command]
#[specta::specta]
pub async fn get_resource_items_partials<R: Runtime>(
    app: AppHandle<R>,
    request: GetResourceItemsPartialsRequest,
) -> Result<GetResourceItemsPartialsResponse> {
    log::info!(
        "Starting get_resource_items_partials function with request: {:?}",
        request
    );

    let resource_items_partials_cache = app.state::<ResourceItemsPartialsCache>();

    log::debug!("Retrieved ResourceItemsPartialsCache state");

    let request_id = request.req_id.clone();
    let get_resources_static_data_request = HashMap::from([(
        request.viewport_type.to_owned(),
        request.resource_items_types.to_owned(),
    )]);

    log::debug!(
        "Constructed get_resources_static_data_request: {:?}",
        get_resources_static_data_request
    );

    let get_partials_result = resource_items_partials_cache
        .get_resource_items_partials(&app, get_resources_static_data_request)
        .await;

    match get_partials_result {
        Ok(partials) => {
            log::debug!("Received partials: {:?}", partials);

            let has_partials = !partials.is_empty();

            let response_partials = if has_partials {
                log::debug!("Partials found, constructing response with partials.");
                ResourceItemsPartialsData::Partials(partials)
            } else {
                log::debug!("No partials found, returning NoneFound.");
                ResourceItemsPartialsData::NoneFound
            };

            log::info!(
                "Returning GetResourceItemsPartialsResponse for req_id {}",
                request_id
            );

            Ok(GetResourceItemsPartialsResponse {
                req_id: request_id,
                partials: response_partials,
            })
        }
        Err(e) => {
            log::error!("Error getting resource items partials: {:?}", e);
            Err(Error {
                req_id: request_id.clone(),
                message: format!("Error getting partials: {:?}", e),
            })
        }
    }
}
