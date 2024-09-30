use auth_login::cognito_login::{CognitoLogin, MfaLoginTransitionResult};
use frontend_tauri_plugins_common::{
    error::{Error, Result},
    types::bearer_tokens::BearerTokens,
};
use tauri::{State, Wry};

use crate::{
    config::LoginConfig,
    models::mfa_login::{
        request::MfaLoginRequest,
        response::{MfaLoginResponse, MfaLoginState},
    },
};

use super::logout_inner;

#[tauri::command]
#[specta::specta]
pub async fn mfa_login(
    config: State<'_, LoginConfig>,
    bearer_tokens: State<'_, BearerTokens<Wry>>,
    request: MfaLoginRequest,
) -> Result<MfaLoginResponse> {
    log::info!("Starting MFA login process for user: {}", request.username);
    log::debug!("MFA login request details: {:?}", request);

    log::debug!("Logging out current user");
    logout_inner(&bearer_tokens);

    let user_pool_id = config.user_pool_id.as_str();
    let client_id = config.client_id.as_str();
    let region = config.region.as_str();

    log::debug!(
        "Initializing CognitoLogin with client_id: {} and region: {}",
        client_id,
        region
    );
    let cognito_login = CognitoLogin::new(client_id, region).await;

    log::info!("Attempting MFA login for user: {}", request.username);
    let mfa_login_result = cognito_login
        .mfa_login(
            client_id,
            &request.session,
            &request.username,
            &request.mfa_code,
        )
        .await;

    log::debug!("MFA login result: {:?}", mfa_login_result);

    match mfa_login_result {
        MfaLoginTransitionResult::Authenticated(auth_data) => {
            log::info!("MFA login successful for user: {}", request.username);
            log::debug!("Setting bearer tokens");
            bearer_tokens
                .set_tokens(
                    user_pool_id,
                    client_id,
                    region,
                    auth_data.access_token(),
                    auth_data.id_token(),
                    auth_data.refresh_token(),
                )
                .await;

            Ok(MfaLoginResponse {
                req_id: request.req_id,
                state: MfaLoginState::Authenticated,
            })
        }
        MfaLoginTransitionResult::EmailConfirmationRequired(_) => {
            log::info!("Email confirmation required for user: {}", request.username);
            Ok(MfaLoginResponse {
                req_id: request.req_id,
                state: MfaLoginState::EmailConfirmationRequired,
            })
        }
        MfaLoginTransitionResult::ExternalServiceError(err) => {
            log::error!("External service error during MFA login: {:?}", err);
            Err(Error {
                req_id: request.req_id,
                message: "External service error".to_string(),
            })
        }
        MfaLoginTransitionResult::Forbidden(err) => {
            log::warn!("Forbidden error during MFA login: {:?}", err);
            Ok(MfaLoginResponse {
                req_id: request.req_id,
                state: MfaLoginState::Forbidden,
            })
        }
        MfaLoginTransitionResult::InternalAppError(err) => {
            log::error!("Internal app error during MFA login: {:?}", err);
            Err(Error {
                req_id: request.req_id,
                message: "Internal app error".to_string(),
            })
        }
        MfaLoginTransitionResult::MfaLoginFailed(err) => {
            log::warn!("MFA login failed for user {}: {:?}", request.username, err);
            Ok(MfaLoginResponse {
                req_id: request.req_id,
                state: MfaLoginState::MfaLoginFailed,
            })
        }
        MfaLoginTransitionResult::MfaNotSetup(err) => {
            log::warn!("MFA not set up for user {}: {:?}", request.username, err);
            Ok(MfaLoginResponse {
                req_id: request.req_id,
                state: MfaLoginState::MfaNotSetup,
            })
        }
        MfaLoginTransitionResult::NotAuthorized(err) => {
            log::warn!("User {} not authorized: {:?}", request.username, err);
            Ok(MfaLoginResponse {
                req_id: request.req_id,
                state: MfaLoginState::NotAuthorized,
            })
        }
        MfaLoginTransitionResult::UserNotFound(err) => {
            log::warn!("User {} not found: {:?}", request.username, err);
            Ok(MfaLoginResponse {
                req_id: request.req_id,
                state: MfaLoginState::UserNotFound,
            })
        }
        MfaLoginTransitionResult::PasswordResetRequired(err) => {
            log::info!(
                "Password reset required for user {}: {:?}",
                request.username,
                err
            );
            Ok(MfaLoginResponse {
                req_id: request.req_id,
                state: MfaLoginState::PasswordResetRequired,
            })
        }
        MfaLoginTransitionResult::ResourceNotFound(err) => {
            log::error!("Resource not found during MFA login: {:?}", err);
            Err(Error {
                req_id: request.req_id,
                message: "Resource not found".to_string(),
            })
        }
    }
}
