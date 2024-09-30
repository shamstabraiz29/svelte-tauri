use auth_login::cognito_login::{CognitoLogin, PwLoginTransitionResult};
use frontend_tauri_plugins_common::{
    error::{Error, Result},
    types::bearer_tokens::BearerTokens,
};
use log::{debug, error, info, warn};
use tauri::{State, Wry};

use crate::{
    config::LoginConfig,
    models::pw_login::{
        request::PwLoginRequest,
        response::{PwLoginResponse, PwLoginState},
    },
};

use super::logout_inner;

#[tauri::command]
#[specta::specta]
pub async fn pw_login(
    config: State<'_, LoginConfig>,
    bearer_token: State<'_, BearerTokens<Wry>>,
    request: PwLoginRequest,
) -> Result<PwLoginResponse> {
    info!(
        "Starting password login process for request ID: {}",
        request.req_id
    );
    debug!("Login attempt for email: {}", request.email);

    // Always perform logout before a new login attempt
    debug!("Performing logout before new login attempt");
    logout_inner(&bearer_token);

    let client_id = config.client_id.as_str();
    let region = config.region.as_str();

    debug!(
        "Initializing CognitoLogin with client_id: {} and region: {}",
        client_id, region
    );
    let cognito_login = CognitoLogin::new(client_id, region).await;

    info!(
        "Attempting password login for request ID: {}",
        request.req_id
    );
    match cognito_login
        .pw_login(&request.email, &request.password)
        .await
    {
        PwLoginTransitionResult::Authenticated(_) => {
            info!("Login successful for request ID: {}", request.req_id);
            Ok(PwLoginResponse {
                req_id: request.req_id,
                state: PwLoginState::Authenticated,
            })
        }
        PwLoginTransitionResult::MfaChallenge(challenge) => {
            info!("MFA required for request ID: {}", request.req_id);
            Ok(PwLoginResponse {
                req_id: request.req_id,
                state: PwLoginState::MfaRequired {
                    session: challenge.session_id().to_string(),
                },
            })
        }
        PwLoginTransitionResult::MfaSetupRequired(mfa_setup_required) => {
            info!("MFA setup required for request ID: {}", request.req_id);
            let totp_uri = mfa_setup_required.mfa_totp_uri();
            let session = mfa_setup_required.session_id();

            Ok(PwLoginResponse {
                req_id: request.req_id,
                state: PwLoginState::MfaSetupRequired {
                    totp_uri: totp_uri.to_string(),
                    session: session.to_string(),
                },
            })
        }
        PwLoginTransitionResult::PasswordResetRequired(_e) => {
            warn!("Password reset required for request ID: {}", request.req_id);
            Ok(PwLoginResponse {
                req_id: request.req_id,
                state: PwLoginState::PasswordResetRequired,
            })
        }
        PwLoginTransitionResult::EmailConfirmationRequired(_e) => {
            warn!(
                "Email confirmation required for request ID: {}",
                request.req_id
            );
            Ok(PwLoginResponse {
                req_id: request.req_id,
                state: PwLoginState::EmailConfirmationRequired,
            })
        }
        PwLoginTransitionResult::ExternalServiceError(e) => {
            error!(
                "External service error during login for request ID: {}. Error: {}",
                request.req_id,
                e.message()
            );
            Err(Error {
                req_id: request.req_id,
                message: e.message().to_string(),
            })
        }
        PwLoginTransitionResult::Forbidden(e) => {
            error!(
                "Forbidden error during login for request ID: {}. Error: {}",
                request.req_id,
                e.message()
            );
            Err(Error {
                req_id: request.req_id,
                message: e.message().to_string(),
            })
        }
        PwLoginTransitionResult::InternalAppError(e) => {
            error!(
                "Internal app error during login for request ID: {}. Error: {}",
                request.req_id,
                e.message()
            );
            Err(Error {
                req_id: request.req_id,
                message: e.message().to_string(),
            })
        }
        PwLoginTransitionResult::NotAuthorized(_e) => {
            warn!(
                "Not authorized login attempt for request ID: {}",
                request.req_id
            );
            Ok(PwLoginResponse {
                req_id: request.req_id,
                state: PwLoginState::NotAuthorized,
            })
        }
        PwLoginTransitionResult::UserNotFound(_e) => {
            warn!("User not found for request ID: {}", request.req_id);
            Ok(PwLoginResponse {
                req_id: request.req_id,
                state: PwLoginState::UserNotFound,
            })
        }
    }
}
