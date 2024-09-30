use frontend_tauri_plugins_common::{error::Result, types::bearer_tokens::BearerTokens};
use log::{debug, info, warn};
use tauri::{AppHandle, Runtime, State};

#[tauri::command]
#[specta::specta]
pub async fn logout<R: Runtime>(
    app: AppHandle<R>,
    bearer_tokens: State<'_, BearerTokens<R>>,
) -> Result<()> {
    debug!("Logout function called");

    // get the email from the id token
    let mut email: Option<String> = None;

    if bearer_tokens.is_token_set() {
        debug!("Bearer token is set, attempting to retrieve email from id token");
        email = bearer_tokens.id_token_claims().and_then(|id_claims| {
            id_claims
                .get("email")
                .and_then(|value| value.as_str().map(|email| email.to_string()))
        });

        match &email {
            Some(e) => debug!("Email retrieved from id token: {}", e),
            None => warn!("Failed to retrieve email from id token"),
        }
    } else {
        debug!("Bearer token is not set");
    }

    if let Some(email) = email {
        let app_id = &app.config().identifier;
        debug!(
            "Attempting to delete secret for app_id: {} and email: {}",
            app_id, email
        );

        match auth_secure_store::delete_secret(app_id, &email) {
            Ok(_) => info!(
                "Successfully removed token from keystore for email: {}",
                email
            ),
            Err(e) => {
                warn!(
                    "Error removing token from keystore for email {}: {:?}",
                    email, e
                );
            }
        }
    } else {
        debug!("No email found, skipping keystore deletion");
    }

    debug!("Calling logout_inner function");
    logout_inner(&bearer_tokens);

    info!("Logout process completed successfully");
    Ok(())
}

pub(crate) fn logout_inner<R: Runtime>(bearer_token: &State<'_, BearerTokens<R>>) {
    debug!("logout_inner function called");
    bearer_token.clear_tokens();
    debug!("Bearer tokens cleared");
}
