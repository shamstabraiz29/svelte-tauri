// frontend/tauri/plugins/tauri-plugin-cognito-sign-up/rust/src/commands/confirm_email.rs

use auth_signup::cognito_signup::{CognitoSignup, ConfirmTransition};
use frontend_tauri_plugins_common::error::{Error, Result};
use log::{debug, error, info};
use tauri::State;
use thiserror::Error;

use crate::{
    config::SignupConfig,
    models::confirm_email::{request::ConfirmEmailRequest, response::ConfirmEmailResponse},
};

#[derive(Debug, Error)]
pub enum ConfirmEmailError {
    #[error("Email confirmation failed: {0}")]
    EmailConfirmationError(String),
}

#[tauri::command]
#[specta::specta]
pub async fn confirm_email(
    config: State<'_, SignupConfig>,
    request: ConfirmEmailRequest,
) -> Result<ConfirmEmailResponse> {
    debug!("Confirming email for request ID: {}", request.req_id);

    let client_id = config.client_id.clone();
    let region = config.region.clone();

    let cognito_signup = CognitoSignup::new(client_id, region).await;

    info!("Attempting to confirm email for: {}", request.email);

    let confirm_response = cognito_signup
        .confirm_email(request.email.clone(), request.confirmation_code.clone())
        .await;

    match confirm_response {
        ConfirmTransition::UserConfirmed(_) => {
            info!("Email confirmed successfully for: {}", request.email);
            Ok(ConfirmEmailResponse {
                req_id: request.req_id,
            })
        }
        error_transition => {
            let error_message = match error_transition {
                ConfirmTransition::CodeFailure(e) => e.message(),
                ConfirmTransition::ExternalServiceError(e) => e.message(),
                ConfirmTransition::InternalAppError(e) => e.message(),
                ConfirmTransition::NotAuthorized(e) => e.message(),
                ConfirmTransition::UserExists(e) => e.message(),
                ConfirmTransition::UserNotFound(e) => e.message(),
                ConfirmTransition::ResourceLimit(e) => e.message(),
                _ => "Unknown error occurred".to_string(),
            };

            error!(
                "Email confirmation failed for {}: {}",
                request.email, error_message
            );

            Err(Error {
                req_id: request.req_id,
                message: ConfirmEmailError::EmailConfirmationError(error_message).to_string(),
            })
        }
    }
}
