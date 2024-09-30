// frontend/tauri/plugins/tauri-plugin-cognito-sign-up/rust/src/commands/sign_up.rs

use auth_signup::cognito_signup::{CognitoSignup, SignupTransition, SignupValues};
use frontend_tauri_plugins_common::error::{Error, Result};
use log::{debug, error, info};
use tauri::State;
use thiserror::Error;

use crate::{
    config::SignupConfig,
    models::signup::{
        request::SignUpRequest,
        response::{SignUpResponse, SignUpStatus},
    },
};

#[derive(Debug, Error)]
pub enum SignUpError {
    #[error("Sign up failed: {0}")]
    SignUpFailed(String),
}

#[tauri::command]
#[specta::specta]
pub async fn sign_up(
    config: State<'_, SignupConfig>,
    request: SignUpRequest,
) -> Result<SignUpResponse> {
    debug!("Processing sign up for request ID: {}", request.req_id);

    let client_id = config.client_id.clone();
    let region = config.region.clone();

    let cognito_signup = CognitoSignup::new(client_id, region).await;

    let sign_up_values = SignupValues {
        email_address: request.email.clone(),
        password: request.password.clone(),
        given_name: request.first_name.clone(),
        family_name: request.last_name.clone(),
        middle_name: request.middle_name.clone(),
    };

    info!("Attempting to sign up user: {}", request.email);

    let sign_up_response = cognito_signup.sign_up(sign_up_values).await;

    match sign_up_response {
        SignupTransition::EmailConfirmationRequired(_) => {
            info!(
                "Sign up successful, email confirmation required for: {}",
                request.email
            );
            Ok(SignUpResponse {
                req_id: request.req_id,
                status: SignUpStatus::ConfirmEmail,
            })
        }
        SignupTransition::UserConfirmed(_) => {
            info!("Sign up successful, user confirmed for: {}", request.email);
            Ok(SignUpResponse {
                req_id: request.req_id,
                status: SignUpStatus::UserConfirmed,
            })
        }
        error_transition => {
            let error_message = match error_transition {
                SignupTransition::ExternalServiceError(e) => e.message(),
                SignupTransition::Forbidden(e) => e.message(),
                SignupTransition::InvalidUserInput(e) => e.message(),
                SignupTransition::InternalAppError(e) => e.message(),
                SignupTransition::NotAuthorized(e) => e.message(),
                SignupTransition::UserExists(e) => e.message(),
                SignupTransition::SignUpFailed(e) => e.message(),
                _ => "Unknown error occurred".to_string(),
            };

            error!("Sign up failed for {}: {}", request.email, error_message);

            Err(Error {
                req_id: request.req_id,
                message: SignUpError::SignUpFailed(error_message).to_string(),
            })
        }
    }
}
