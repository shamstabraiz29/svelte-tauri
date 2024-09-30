use std::collections::HashMap;

use frontend_tauri_plugins_common::error::{Error, Result};
use tauri::{AppHandle, Manager, Runtime};

use crate::{
    models::get_resource_items_viewport_data::{
        request::GetResourceItemsViewportDataRequest,
        response::{GetResourceItemsViewportDataResponse, ResourceItemsViewportData},
    },
    types::resource_items_viewport_data_cache::ResourceItemsViewportDataCache,
};

#[tauri::command]
#[specta::specta]
pub async fn get_resource_items_viewport_data<R: Runtime>(
    app: AppHandle<R>,
    request: GetResourceItemsViewportDataRequest,
) -> Result<GetResourceItemsViewportDataResponse> {
    log::info!(
        "Starting get_resource_items_viewport_data function with request: {:?}",
        request
    );

    let resource_items_viewport_data_cache = app.state::<ResourceItemsViewportDataCache>();

    log::debug!("Retrieved ResourceItemsViewportDataCache state");

    let request_id = request.req_id.clone();
    let get_resources_viewport_data_request = HashMap::from([(
        request.viewport_type.to_owned(),
        request.resource_items_types.to_owned(),
    )]);

    log::debug!(
        "Constructed get_resources_viewport_data_request: {:?}",
        get_resources_viewport_data_request
    );

    let get_viewport_data_result = resource_items_viewport_data_cache
        .get_resource_items_viewport_data(&app, get_resources_viewport_data_request)
        .await;

    match get_viewport_data_result {
        Ok(viewport_data) => {
            log::debug!("Received viewport_data: {:?}", viewport_data);

            let has_viewport_data = !viewport_data.is_empty();

            let response_viewport_data = if has_viewport_data {
                log::debug!("Viewport data found, constructing response with data.");
                ResourceItemsViewportData::ViewportData(viewport_data)
            } else {
                log::debug!("No viewport data found, returning NoneFound.");
                ResourceItemsViewportData::NoneFound
            };

            log::info!(
                "Returning GetResourceItemsViewportDataResponse for req_id {}",
                request_id
            );

            Ok(GetResourceItemsViewportDataResponse {
                req_id: request_id,
                viewport_data: response_viewport_data,
            })
        }
        Err(e) => {
            log::error!("Error getting resource items viewport data: {:?}", e);
            Err(Error {
                req_id: request_id.clone(),
                message: format!("Error getting viewport data: {:?}", e),
            })
        }
    }
}
