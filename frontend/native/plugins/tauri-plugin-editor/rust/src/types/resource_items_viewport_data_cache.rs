use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use common_dtos::editor_client::resource_items_viewport_data_get::{
    request::ResourceItemsViewportDataGetRequest, response::ResourceItemsViewportDataGetStatus,
};
use common_libs_editor_api_client::EditorApiClient;
use frontend_tauri_plugins_common::types::bearer_tokens::BearerTokens;
use serde_json::Value as JsonValue;
use tauri::{AppHandle, Manager, Runtime};

use crate::error::InternalApplicationError;

#[derive(Debug, Default)]
pub struct ResourceItemsViewportDataCache {
    // HashMap<viewport_type, HashMap<resource_item_type, JsonValue>>
    loaded_data: Mutex<HashMap<String, HashMap<String, JsonValue>>>,
}

impl ResourceItemsViewportDataCache {
    fn get_access_token<R: Runtime>(app: &AppHandle<R>) -> Result<String, crate::EditorError> {
        let bearer_tokens = app.state::<BearerTokens<R>>();

        bearer_tokens
            .access_token()
            .ok_or_else(|| InternalApplicationError::BearerTokensAccessTokenNotPresent.into())
    }

    pub(crate) async fn get_resource_items_viewport_data<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        // HashMap<viewport_type, Vec<resource_item_type>>
        request: HashMap<String, HashSet<String>>,
    ) -> Result<HashMap<String, HashMap<String, JsonValue>>, crate::EditorError> {
        // HashMap<viewport_type, HashMap<resource_item_type, JsonValue>>
        let mut requested_static_data: HashMap<String, HashMap<String, JsonValue>> = HashMap::new();

        // HashMap<viewport_type, Vec<resource_item_type>>
        let mut non_loaded_static_data: HashMap<String, HashSet<String>> = HashMap::new();

        {
            let loaded_data = self
                .loaded_data
                .lock()
                .map_err(|_| InternalApplicationError::ResourceItemsViewportDataLock)?;

            for (viewport_type, resource_items_types) in request.iter() {
                match loaded_data.get(viewport_type) {
                    Some(resource_item_static_data_map) => {
                        for resource_item_type in resource_items_types.iter() {
                            match resource_item_static_data_map.get(resource_item_type) {
                                Some(static_data) => {
                                    requested_static_data
                                        .entry(viewport_type.to_owned())
                                        .or_default()
                                        .insert(
                                            resource_item_type.to_owned(),
                                            static_data.to_owned(),
                                        );
                                }
                                None => {
                                    non_loaded_static_data
                                        .entry(viewport_type.to_owned())
                                        .or_default()
                                        .insert(resource_item_type.to_owned());
                                }
                            }
                        }
                    }
                    None => {
                        non_loaded_static_data
                            .insert(viewport_type.to_owned(), resource_items_types.to_owned());
                    }
                }
            }
        }

        if non_loaded_static_data.is_empty() {
            return Ok(requested_static_data);
        }

        let editor_api_client = app.state::<EditorApiClient>();
        let access_token = Self::get_access_token(app)?;
        let payload = ResourceItemsViewportDataGetRequest {
            per_viewport_resource_items_types: non_loaded_static_data.to_owned(),
        };
        let node_info_get_response = editor_api_client
            .get_resource_items_viewport_data(&access_token, payload)
            .await?;

        match node_info_get_response.status {
            ResourceItemsViewportDataGetStatus::Success {
                resource_items_viewport_data: viewport_resource_item_type_static_data,
            } => {
                {
                    let mut loaded_data = self
                        .loaded_data
                        .lock()
                        .map_err(|_| InternalApplicationError::ResourceItemsViewportDataLock)?;

                    for (viewport_type, resource_item_type_static_data) in
                        viewport_resource_item_type_static_data
                    {
                        let loaded_viewport_data =
                            loaded_data.entry(viewport_type.to_owned()).or_default();

                        let requested_viewport_data = requested_static_data
                            .entry(viewport_type.to_owned())
                            .or_default();

                        for (resource_item_type, static_data) in
                            resource_item_type_static_data.iter()
                        {
                            loaded_viewport_data
                                .insert(resource_item_type.to_owned(), static_data.to_owned());

                            requested_viewport_data
                                .insert(resource_item_type.to_owned(), static_data.to_owned());
                        }
                    }
                }
                Ok(requested_static_data)
            }
            ResourceItemsViewportDataGetStatus::Failure(msg) => {
                Err(crate::EditorError::RecoverableError(msg))
            }
        }
    }
}
