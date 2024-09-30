use std::collections::{HashMap, HashSet};

use frontend_tauri_plugins_common::error::{Error, Result};
use tauri::{AppHandle, Manager, Runtime};

use crate::{
    models::get_resource_item_viewport_data::{
        request::GetResourceItemViewportDataRequest,
        response::{GetResourceItemViewportDataResponse, ResourceItemViewportData},
    },
    types::resource_items_viewport_data_cache::ResourceItemsViewportDataCache,
};

#[tauri::command]
#[specta::specta]
pub async fn get_resource_item_viewport_data<R: Runtime>(
    app: AppHandle<R>,
    request: GetResourceItemViewportDataRequest,
) -> Result<GetResourceItemViewportDataResponse> {
    log::info!(
        "Starting get_resource_item_viewport_data function with request: {:?}",
        request
    );

    let viewport_items_viewport_data_cache = app.state::<ResourceItemsViewportDataCache>();

    log::debug!("Retrieved ResourceItemsViewportDataCache state");

    let request_id = request.req_id.clone();
    let get_resources_viewport_data_request = HashMap::from([(
        request.viewport_type.to_owned(),
        HashSet::from([request.resource_item_type.to_owned()]),
    )]);

    log::debug!(
        "Constructed get_resources_viewport_data_request: {:?}",
        get_resources_viewport_data_request
    );

    let get_viewport_data_result = viewport_items_viewport_data_cache
        .get_resource_items_viewport_data(&app, get_resources_viewport_data_request)
        .await;

    match get_viewport_data_result {
        Ok(mut viewport_data_map) => {
            log::debug!("Received viewport_data_map: {:?}", viewport_data_map);

            let viewport_data =
                viewport_data_map
                    .remove(&request.viewport_type)
                    .and_then(|mut viewport_items| {
                        log::debug!(
                            "Retrieved viewport_items for {}: {:?}",
                            request.viewport_type,
                            viewport_items
                        );
                        viewport_items.remove(&request.resource_item_type)
                    });

            let response_viewport_data = match viewport_data {
                Some(viewport_data) => {
                    log::debug!(
                        "Found viewport_data for resource_item_type {}: {:?}",
                        request.resource_item_type,
                        viewport_data
                    );
                    ResourceItemViewportData::ViewportData {
                        value: viewport_data,
                    }
                }
                None => {
                    log::debug!(
                        "No viewport_data found for resource_item_type {}",
                        request.resource_item_type
                    );
                    ResourceItemViewportData::NoneFound
                }
            };

            log::info!(
                "Returning GetResourceItemViewportDataResponse for req_id {}",
                request_id
            );

            Ok(GetResourceItemViewportDataResponse {
                req_id: request_id,
                viewport_data: response_viewport_data,
            })
        }
        Err(e) => {
            log::error!("Error getting resource item viewport data: {:?}", e);
            Err(Error {
                req_id: request_id.clone(),
                message: format!("Error getting viewport data: {:?}", e),
            })
        }
    }
}
