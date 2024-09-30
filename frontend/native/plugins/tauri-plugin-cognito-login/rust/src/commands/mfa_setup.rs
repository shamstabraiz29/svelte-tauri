use auth_login::cognito_login::{CognitoLogin, MfaSetupTransitionResult};
use frontend_tauri_plugins_common::{
    error::{Error, Result},
    types::bearer_tokens::BearerTokens,
};
use log::{debug, error, info, warn};
use tauri::{State, Wry};

use crate::{
    config::LoginConfig,
    models::mfa_setup::{
        request::MfaSetupRequest,
        response::{MfaSetupResponse, MfaSetupState},
    },
};

use super::logout_inner;

#[tauri::command]
#[specta::specta]
pub async fn mfa_setup(
    config: State<'_, LoginConfig>,
    bearer_token: State<'_, BearerTokens<Wry>>,
    request: MfaSetupRequest,
) -> Result<MfaSetupResponse> {
    info!(
        "Starting MFA setup process for request ID: {}",
        request.req_id
    );

    // if the user is already authenticated, then lets log them out first
    debug!("Logging out user if already authenticated");
    logout_inner(&bearer_token);

    let client_id = config.client_id.as_str();
    let region = config.region.as_str();

    debug!(
        "Initializing CognitoLogin with client_id: {} and region: {}",
        client_id, region
    );
    let cognito_login = CognitoLogin::new(client_id, region).await;

    info!("Verifying MFA token for session");
    let verify_result = cognito_login
        .mfa_verify_sw_token(&request.session, &request.mfa_code)
        .await;

    match verify_result {
        MfaSetupTransitionResult::MfaSetupSucceeded(_) => {
            info!("MFA setup succeeded for request ID: {}", request.req_id);
            Ok(MfaSetupResponse {
                req_id: request.req_id,
                state: MfaSetupState::MfaSetupSucceeded,
            })
        }
        MfaSetupTransitionResult::ExternalServiceError(e) => {
            error!("External service error during MFA setup: {}", e.message());
            Err(Error {
                req_id: request.req_id,
                message: e.message().to_string(),
            })
        }
        MfaSetupTransitionResult::EmailConfirmationRequired(_e) => {
            info!(
                "Email confirmation required for request ID: {}",
                request.req_id
            );
            Ok(MfaSetupResponse {
                req_id: request.req_id,
                state: MfaSetupState::EmailConfirmationRequired,
            })
        }
        MfaSetupTransitionResult::Forbidden(_e) => {
            warn!(
                "Forbidden access attempt for request ID: {}",
                request.req_id
            );
            Ok(MfaSetupResponse {
                req_id: request.req_id,
                state: MfaSetupState::Forbidden,
            })
        }
        MfaSetupTransitionResult::InternalAppError(e) => {
            error!(
                "Internal application error during MFA setup: {}",
                e.message()
            );
            Err(Error {
                req_id: request.req_id,
                message: e.message().to_string(),
            })
        }
        MfaSetupTransitionResult::MfaSetupFailed(_e) => {
            warn!("MFA setup failed for request ID: {}", request.req_id);
            Ok(MfaSetupResponse {
                req_id: request.req_id,
                state: MfaSetupState::MfaSetupFailed,
            })
        }
        MfaSetupTransitionResult::NotAuthorized(_e) => {
            warn!(
                "Unauthorized MFA setup attempt for request ID: {}",
                request.req_id
            );
            Ok(MfaSetupResponse {
                req_id: request.req_id,
                state: MfaSetupState::NotAuthorized,
            })
        }
        MfaSetupTransitionResult::PasswordResetRequired(_e) => {
            info!("Password reset required for request ID: {}", request.req_id);
            Ok(MfaSetupResponse {
                req_id: request.req_id,
                state: MfaSetupState::PasswordResetRequired,
            })
        }
        MfaSetupTransitionResult::ResourceNotFound(e) => {
            error!("Resource not found during MFA setup: {}", e.message());
            Err(Error {
                req_id: request.req_id,
                message: e.message().to_string(),
            })
        }
        MfaSetupTransitionResult::UserNotFound(_e) => {
            warn!(
                "User not found during MFA setup for request ID: {}",
                request.req_id
            );
            Ok(MfaSetupResponse {
                req_id: request.req_id,
                state: MfaSetupState::UserNotFound,
            })
        }
    }
}
