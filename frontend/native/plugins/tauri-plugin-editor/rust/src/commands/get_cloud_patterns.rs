// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/get_cloud_patterns.rs
use std::collections::HashMap;

use common_dtos::editor_client::cloud_patterns_get::response::CloudPatternsGetStatus;
use common_libs_editor_api_client::EditorApiClient;
use frontend_tauri_plugins_common::{
    error::{Error, Result},
    notifying_sync::notifying_state::NotifyingState,
    types::bearer_tokens::BearerTokens,
};
use log::{debug, error, info, warn};
use tauri::{State, Wry};

use crate::{
    models::get_cloud_patterns::{
        request::CloudPatternsGetRequest, response::UiCloudPatternsGetResponse,
    },
    types::{
        cloud_pattern_evaluator_state::CloudPatternEvaluatorState,
        cloud_patterns_meta_state::CloudPatternsMetaState,
    },
};

#[tauri::command]
#[specta::specta]
pub async fn get_cloud_patterns(
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    editor_api_client: State<'_, EditorApiClient>,
    cloud_patterns_meta_state: State<'_, CloudPatternsMetaState>,
    cloud_pattern_evaluator_state: State<'_, CloudPatternEvaluatorState>,
    request: CloudPatternsGetRequest,
) -> Result<UiCloudPatternsGetResponse> {
    info!(
        "Starting get_cloud_patterns with request ID: {}",
        request.req_id
    );
    debug!("Received get cloud patterns request: {:?}", request);

    let access_token = match bearer_tokens.access_token() {
        Some(token) => {
            debug!("Access token found");
            token
        }
        None => {
            warn!("No access token found for request ID: {}", request.req_id);
            return Err(Error {
                req_id: request.req_id,
                message: "No access token found".to_string(),
            });
        }
    };

    if cloud_patterns_meta_state.read_state_value(|_| {}) {
        info!("Cloud patterns meta state already exists. Returning early.");
        return Ok(UiCloudPatternsGetResponse {
            req_id: request.req_id,
        });
    }

    info!("Fetching cloud patterns from editor API");
    let repo_response = editor_api_client.get(&access_token).await;

    match repo_response {
        Ok(response) => {
            let req_id = request.req_id;

            let cloud_patterns_meta = match response.status {
                CloudPatternsGetStatus::Success { cloud_patterns } => {
                    info!("Successfully fetched cloud patterns");
                    cloud_patterns
                }
                CloudPatternsGetStatus::Failure(message) => {
                    error!("Failed to fetch cloud patterns: {}", message);
                    return Err(Error { req_id, message });
                }
            };

            debug!("Flattening cloud patterns meta");
            let cloud_patterns_meta_flattened = cloud_patterns_meta.values().cloned().fold(
                HashMap::new(),
                |mut acc, cloud_patterns_meta| {
                    for cloud_pattern_meta in cloud_patterns_meta {
                        acc.insert(cloud_pattern_meta.id.to_owned(), cloud_pattern_meta);
                    }
                    acc
                },
            );

            info!("Setting cloud patterns meta in evaluator state");
            if let Err(e) =
                cloud_pattern_evaluator_state.set_cloud_patterns_meta(cloud_patterns_meta_flattened)
            {
                error!(
                    "Failed to set cloud patterns meta in evaluator state: {}",
                    e
                );
                return Err(Error {
                    req_id,
                    message: e.to_string(),
                });
            }

            info!("Setting cloud patterns meta in state");
            debug!("Cloud patterns meta: {:#?}", cloud_patterns_meta);
            cloud_patterns_meta_state.set_state_value(Some(cloud_patterns_meta));

            info!(
                "Successfully processed cloud patterns for request ID: {}",
                req_id
            );
            Ok(UiCloudPatternsGetResponse { req_id })
        }
        Err(e) => {
            error!(
                "Error fetching cloud patterns for request ID: {}. Error: {}",
                request.req_id, e
            );
            Err(Error {
                req_id: request.req_id,
                message: e.to_string(),
            })
        }
    }
}
