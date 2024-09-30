// src/commands/get_subscriber.rs
use common_libs_subscriber_api_client::SubscriberApiClient;
use frontend_tauri_plugins_common::{
    error::{Error, Result},
    types::bearer_tokens::BearerTokens,
};
use log::{error, info};
use tauri::{State, Wry};

use crate::{
    config::SubscriberClientConfig,
    models::get_subscriber::{request::SubscriberRequest, response::SubscriberResponse},
};

#[tauri::command]
#[specta::specta]
pub async fn get_subscriber(
    config: State<'_, SubscriberClientConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    request: SubscriberRequest,
) -> Result<SubscriberResponse> {
    info!("Getting subscriber for request ID: {}", request.req_id);

    let access_token = bearer_tokens.access_token().ok_or_else(|| {
        let err = Error {
            req_id: request.req_id.clone(),
            message: "No access token found".to_string(),
        };
        error!("Authentication error: {}", err.message);
        err
    })?;

    let subscriber_api_client = SubscriberApiClient::new(&config.url, config.api_path.clone())
        .map_err(|e| {
            let err = Error {
                req_id: request.req_id.clone(),
                message: format!("Error creating subscriber API client: {}", e),
            };
            error!("API client creation error: {}", err.message);
            err
        })?;

    info!("Subscriber API client created successfully");
    get_subscriber_inner(subscriber_api_client, access_token, request).await
}

async fn get_subscriber_inner(
    subscriber_api_client: SubscriberApiClient,
    access_token: String,
    request: SubscriberRequest,
) -> Result<SubscriberResponse> {
    info!(
        "Attempting to load subscriber data for request ID: {}",
        request.req_id
    );

    subscriber_api_client
        .load(&access_token)
        .await
        .map(|dto| {
            info!(
                "Subscriber data loaded successfully for request ID: {}",
                request.req_id
            );
            SubscriberResponse {
                req_id: request.req_id.clone(),
                subscriber_detail: dto.into(),
            }
        })
        .map_err(|e| {
            let err = Error {
                req_id: request.req_id.clone(),
                message: format!("Error loading subscriber: {}", e),
            };
            error!("Failed to load subscriber data: {}", err.message);
            err
        })
}
