// frontend/tauri/plugins/tauri-plugin-cognito-sign-up/rust/src/commands/resend_confirmation.rs

use auth_signup::cognito_signup::{CognitoSignup, ResendTransition};
use frontend_tauri_plugins_common::error::{Error, Result};
use log::{debug, error, info};
use tauri::State;
use thiserror::Error;

use crate::{
    config::SignupConfig,
    models::resend_conf::{request::ResendConfRequest, response::ResendConfResponse},
};

#[derive(Debug, Error)]
pub enum ResendConfirmationError {
    #[error("Resend confirmation failed: {0}")]
    ResendConfirmationFailed(String),
}

#[tauri::command]
#[specta::specta]
pub async fn resend_confirmation(
    config: State<'_, SignupConfig>,
    request: ResendConfRequest,
) -> Result<ResendConfResponse> {
    debug!("Resending confirmation for request ID: {}", request.req_id);

    let client_id = config.client_id.clone();
    let region = config.region.clone();

    let cognito_signup = CognitoSignup::new(client_id, region).await;

    info!(
        "Attempting to resend confirmation code for: {}",
        request.email
    );

    let resend_response = cognito_signup.resend_code(request.email.clone()).await;

    match resend_response {
        ResendTransition::CodeResent(_) => {
            info!(
                "Confirmation code resent successfully for: {}",
                request.email
            );
            Ok(ResendConfResponse {
                req_id: request.req_id,
            })
        }
        error_transition => {
            let error_message = match error_transition {
                ResendTransition::CodeDeliveryFailure(e) => e.message(),
                ResendTransition::CodeFailure(e) => e.message(),
                ResendTransition::ExternalServiceError(e) => e.message(),
                ResendTransition::Forbidden(e) => e.message(),
                ResendTransition::InternalAppError(e) => e.message(),
                ResendTransition::NotAuthorized(e) => e.message(),
                ResendTransition::UserNotFound(e) => e.message(),
                ResendTransition::ResourceLimit(e) => e.message(),
                _ => "Unknown error occurred".to_string(),
            };

            error!(
                "Resend confirmation failed for {}: {}",
                request.email, error_message
            );

            Err(Error {
                req_id: request.req_id,
                message: ResendConfirmationError::ResendConfirmationFailed(error_message)
                    .to_string(),
            })
        }
    }
}
