use frontend_tauri_plugins_common::{
    error::{Error, Result},
    types::bearer_tokens::BearerTokens,
};
use log;
use tauri::{AppHandle, Runtime, State};

use crate::models::store_refresh_token::{
    request::StoreRefreshTokenRequest,
    response::{StoreRefreshTokenResponse, StoreRefreshTokenState},
};

#[tauri::command]
#[specta::specta]
pub async fn store_refresh_token<R: Runtime>(
    app: AppHandle<R>,
    bearer_tokens: State<'_, BearerTokens<R>>,
    request: StoreRefreshTokenRequest,
) -> Result<StoreRefreshTokenResponse> {
    log::info!(
        "Starting store_refresh_token for request ID: {}",
        request.req_id
    );

    // Check for access token
    let _access_token = bearer_tokens.access_token().ok_or_else(|| {
        log::warn!("No access token found for request ID: {}", request.req_id);
        Error {
            req_id: request.req_id.clone(),
            message: "No access token found".to_string(),
        }
    })?;

    log::debug!("Access token verified for request ID: {}", request.req_id);

    // Check for refresh token
    let refresh_token = match bearer_tokens.refresh_token() {
        Some(token) => {
            log::debug!("Refresh token found for request ID: {}", request.req_id);
            token
        }
        None => {
            log::warn!("No refresh token found for request ID: {}", request.req_id);
            return Ok(StoreRefreshTokenResponse {
                req_id: request.req_id,
                state: StoreRefreshTokenState::NotAuthenticated,
            });
        }
    };

    let app_id = app.config().identifier.clone();
    log::debug!(
        "Using app identifier: {} for request ID: {}",
        app_id,
        request.req_id
    );

    // Retrieve email from ID token claims
    let email = bearer_tokens
        .id_token_claims()
        .and_then(|id_claims| {
            id_claims
                .get("email")
                .and_then(|value| value.as_str().map(String::from))
        })
        .ok_or_else(|| {
            log::error!(
                "Failed to retrieve email from ID token claims for request ID: {}",
                request.req_id
            );
            Error {
                req_id: request.req_id.clone(),
                message: "Internal Error - inconsistent state".to_string(),
            }
        })?;

    log::debug!("Email retrieved for request ID: {}", request.req_id);

    // Attempt to store the refresh token
    match auth_secure_store::store_secret(&app_id, &email, &refresh_token) {
        Ok(_) => {
            log::info!(
                "Successfully stored refresh token for request ID: {}",
                request.req_id
            );
            Ok(StoreRefreshTokenResponse {
                req_id: request.req_id,
                state: StoreRefreshTokenState::Success,
            })
        }
        Err(e) => {
            log::error!(
                "Failed to store refresh token for request ID: {}. Error: {}",
                request.req_id,
                e
            );
            Err(Error {
                req_id: request.req_id,
                message: format!("Failed to store refresh token: {}", e),
            })
        }
    }
}
