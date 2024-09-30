use std::collections::HashSet;

use frontend_tauri_plugins_common::error::{Error, Result};
use tauri::{AppHandle, Manager, Runtime};

use crate::{
    models::get_resource_item_schema::{
        request::GetResourceItemSchemaRequest,
        response::{GetResourceItemSchemaResponse, ResourceItemSchemaResponseData},
    },
    types::resource_items_schema_cache::ResourceItemsSchemasCache,
};

#[tauri::command]
#[specta::specta]
pub async fn get_resource_item_schema<R: Runtime>(
    app: AppHandle<R>,
    request: GetResourceItemSchemaRequest,
) -> Result<GetResourceItemSchemaResponse> {
    log::info!(
        "Starting get_resource_item_schema function with request: {:?}",
        request
    );

    let viewport_items_schema_cache = app.state::<ResourceItemsSchemasCache>();

    log::debug!("Retrieved ResourceItemsSchemasCache state");

    let request_id = request.req_id.clone();
    let get_resources_schema_request = HashSet::from([request.resource_item_type.to_owned()]);

    log::debug!(
        "Constructed get_resources_schema_request: {:?}",
        get_resources_schema_request
    );

    let get_schema_result = viewport_items_schema_cache
        .get_resource_items_schemas(&app, get_resources_schema_request)
        .await;

    match get_schema_result {
        Ok(mut schemas) => {
            log::debug!("Received schemas: {:?}", schemas);

            let schema = schemas.remove(&request.resource_item_type);

            let response_schema = match schema {
                Some(schema) => {
                    log::debug!(
                        "Found schema for resource_item_type {}: {:?}",
                        request.resource_item_type,
                        schema
                    );
                    ResourceItemSchemaResponseData::Schema(schema.into())
                }
                None => {
                    log::debug!(
                        "No schema found for resource_item_type {}",
                        request.resource_item_type
                    );
                    ResourceItemSchemaResponseData::NoneFound
                }
            };

            log::info!(
                "Returning GetResourceItemSchemaResponse for req_id {}",
                request_id
            );

            Ok(GetResourceItemSchemaResponse {
                req_id: request_id,
                schema: response_schema,
            })
        }
        Err(e) => {
            log::error!("Error getting resource item schemas: {:?}", e);
            Err(Error {
                req_id: request_id.clone(),
                message: format!("Error getting static data: {:?}", e),
            })
        }
    }
}
