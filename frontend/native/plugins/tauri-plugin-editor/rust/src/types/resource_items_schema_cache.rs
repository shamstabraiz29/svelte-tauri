use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use common_dtos::editor_client::{
    resource_items_schemas_get::{
        request::ResourceItemsSchemasGetRequest, response::ResourceItemsSchemasGetStatus,
    },
    types::resource_item_schema::ResourceItemSchema,
};
use common_libs_editor_api_client::EditorApiClient;
use frontend_tauri_plugins_common::types::bearer_tokens::BearerTokens;
use tauri::{AppHandle, Manager, Runtime};

use crate::error::InternalApplicationError;

#[derive(Debug, Default)]
pub struct ResourceItemsSchemasCache {
    // HashMap<resource_item_type, ResourceItemSchema>
    loaded_data: Mutex<HashMap<String, ResourceItemSchema>>,
}

impl ResourceItemsSchemasCache {
    fn get_access_token<R: Runtime>(app: &AppHandle<R>) -> Result<String, crate::EditorError> {
        let bearer_tokens = app.state::<BearerTokens<R>>();

        bearer_tokens
            .access_token()
            .ok_or_else(|| InternalApplicationError::BearerTokensAccessTokenNotPresent.into())
    }

    pub(crate) async fn get_resource_items_schemas<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        // HashSet<resource_item_type>
        resource_items_types: HashSet<String>,
    ) -> Result<HashMap<String, ResourceItemSchema>, crate::EditorError> {
        // HashMap<viewport_type, HashMap<resource_item_type, ResourceItemSchema>>
        let mut requested_schemas: HashMap<String, ResourceItemSchema> = HashMap::new();

        // HashMap<viewport_type, Vec<resource_item_type>>
        let mut non_loaded_static_data: HashSet<String> = HashSet::new();

        {
            let loaded_data = self
                .loaded_data
                .lock()
                .map_err(|_| InternalApplicationError::ResourceItemsSchemasLock)?;

            for resource_item_type in resource_items_types.iter() {
                match loaded_data.get(resource_item_type) {
                    Some(schema) => {
                        requested_schemas.insert(resource_item_type.to_owned(), schema.to_owned());
                    }
                    None => {
                        non_loaded_static_data.insert(resource_item_type.to_owned());
                    }
                }
            }
        }

        if non_loaded_static_data.is_empty() {
            return Ok(requested_schemas);
        }

        let editor_api_client = app.state::<EditorApiClient>();
        let access_token = Self::get_access_token(app)?;
        let payload = ResourceItemsSchemasGetRequest {
            resource_items_types: non_loaded_static_data.to_owned(),
        };
        let node_info_get_response = editor_api_client
            .get_resource_items_schemas(&access_token, payload)
            .await?;

        match node_info_get_response.status {
            ResourceItemsSchemasGetStatus::Success {
                resource_items_schemas,
            } => {
                {
                    let mut loaded_data = self
                        .loaded_data
                        .lock()
                        .map_err(|_| InternalApplicationError::ResourceItemsSchemasLock)?;

                    for (resource_item_type, schema) in resource_items_schemas {
                        loaded_data.insert(resource_item_type.to_owned(), schema.to_owned());

                        requested_schemas.insert(resource_item_type.to_owned(), schema);
                    }
                }
                Ok(requested_schemas)
            }
            ResourceItemsSchemasGetStatus::Failure(msg) => {
                Err(crate::EditorError::RecoverableError(msg))
            }
        }
    }
}
