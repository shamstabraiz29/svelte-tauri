use auth_login::cognito_login::{CognitoLogin, RefreshLoginTransitionResult};

use frontend_tauri_plugins_common::{
    error::{Error, Result},
    types::bearer_tokens::BearerTokens,
};
use log::{debug, error, info, warn};
use tauri::{AppHandle, Runtime, State};

use crate::{
    config::LoginConfig,
    models::refresh_login::{
        request::RefreshLoginRequest,
        response::{RefreshLoginResponse, RefreshLoginState},
    },
};

#[tauri::command]
#[specta::specta]
pub async fn refresh_login<R: Runtime>(
    app: AppHandle<R>,
    config: State<'_, LoginConfig>,
    bearer_tokens: State<'_, BearerTokens<R>>,
    request: RefreshLoginRequest,
) -> Result<RefreshLoginResponse> {
    info!("Initiating login refresh for email: {}", request.email);

    let refresh_token =
        match get_refresh_token(&app, &bearer_tokens, &request.email, &request.req_id).await {
            Ok(token) => token,
            Err(e) => {
                error!("Failed to get refresh token: {}", e.message);
                return Err(e);
            }
        };

    debug!("Obtained refresh token for email: {}", request.email);

    let cognito_login = CognitoLogin::new(config.client_id.as_str(), config.region.as_str()).await;
    info!(
        "Attempting to refresh login with Cognito for email: {}",
        request.email
    );

    match cognito_login.refresh_login(&refresh_token).await {
        RefreshLoginTransitionResult::Authenticated(auth_data) => {
            info!(
                "Successfully authenticated refresh for email: {}",
                request.email
            );
            handle_successful_refresh(&config, &bearer_tokens, &refresh_token, auth_data).await;
            Ok(RefreshLoginResponse {
                req_id: request.req_id,
                state: RefreshLoginState::Authenticated,
            })
        }
        RefreshLoginTransitionResult::ExternalServiceError(e) => {
            error!(
                "External service error during refresh for email {}: {}",
                request.email,
                e.message()
            );
            Err(Error {
                req_id: request.req_id,
                message: e.message().to_string(),
            })
        }
        RefreshLoginTransitionResult::Forbidden(e) => {
            warn!(
                "Refresh token forbidden for email {}: {}",
                request.email,
                e.message()
            );
            Ok(RefreshLoginResponse {
                req_id: request.req_id,
                state: RefreshLoginState::Forbidden,
            })
        }
        RefreshLoginTransitionResult::InternalAppError(e) => {
            error!(
                "Internal app error during refresh for email {}: {}",
                request.email,
                e.message()
            );
            Err(Error {
                req_id: request.req_id,
                message: e.message().to_string(),
            })
        }
        RefreshLoginTransitionResult::NotAuthorized(e) => {
            warn!(
                "Refresh token not authorized for email {}: {}",
                request.email,
                e.message()
            );
            Ok(RefreshLoginResponse {
                req_id: request.req_id,
                state: RefreshLoginState::NotAuthorized,
            })
        }
    }
}

async fn get_refresh_token<R: Runtime>(
    app: &AppHandle<R>,
    bearer_tokens: &State<'_, BearerTokens<R>>,
    email: &str,
    req_id: &str,
) -> Result<String> {
    debug!("Attempting to get refresh token for email: {}", email);

    if let Some(token) = get_token_from_bearer(bearer_tokens, email).await {
        info!("Retrieved refresh token from bearer for email: {}", email);
        return Ok(token);
    }

    debug!(
        "Refresh token not found in bearer, attempting to retrieve from keyring for email: {}",
        email
    );
    get_token_from_keyring(app, email, req_id)
}

async fn get_token_from_bearer<R: Runtime>(
    bearer_tokens: &State<'_, BearerTokens<R>>,
    email: &str,
) -> Option<String> {
    if bearer_tokens.is_token_set() {
        debug!("Bearer tokens are set, checking for matching email");
        let bearer_email = bearer_tokens.id_token_claims().and_then(|id_claims| {
            id_claims
                .get("email")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        });

        if Some(email.to_string()) == bearer_email {
            debug!("Email match found in bearer tokens");
            return bearer_tokens.refresh_token();
        }
    }
    debug!(
        "No matching refresh token found in bearer for email: {}",
        email
    );
    None
}

fn get_token_from_keyring<R: Runtime>(
    app: &AppHandle<R>,
    email: &str,
    req_id: &str,
) -> Result<String> {
    let app_id = &app.config().identifier;
    debug!(
        "Attempting to retrieve token from keyring for email: {}",
        email
    );

    match auth_secure_store::get_secret(app_id, email) {
        Ok(token) => {
            info!(
                "Successfully retrieved token from keyring for email: {}",
                email
            );
            Ok(token)
        }
        Err(e) => match e {
            auth_secure_store::SecureStoreError::KeyringError(keyring::Error::NoEntry) => {
                warn!("No refresh token available in keyring for email: {}", email);
                Err(Error {
                    req_id: req_id.to_string(),
                    message: "No refresh token available".to_string(),
                })
            }
            _ => {
                error!(
                    "Error retrieving token from keyring for email {}: {:?}",
                    email, e
                );
                Err(Error {
                    req_id: req_id.to_string(),
                    message: format!("KeyringError: {:?}", e),
                })
            }
        },
    }
}

async fn handle_successful_refresh<R: Runtime>(
    config: &State<'_, LoginConfig>,
    bearer_tokens: &State<'_, BearerTokens<R>>,
    refresh_token: &str,
    auth_data: CognitoLogin<auth_login::cognito_login::authenticated::Authenticated>,
) {
    info!("Handling successful login refresh");
    bearer_tokens
        .set_tokens(
            config.user_pool_id.as_str(),
            config.client_id.as_str(),
            config.region.as_str(),
            auth_data.access_token(),
            auth_data.id_token(),
            refresh_token,
        )
        .await;
    debug!("Bearer tokens updated after successful refresh");
}
