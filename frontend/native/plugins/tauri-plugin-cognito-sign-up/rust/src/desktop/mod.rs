// frontend/tauri/plugins/tauri-plugin-cognito-sign-up/rust/src/desktop/mod.rs

pub mod cognito_sign_up;

use log::info;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use self::cognito_sign_up::CognitoSignUp;
use crate::config::SignupConfig;
use crate::error::Result;

pub(crate) fn init<R: Runtime>(
    app: &AppHandle<R>,
    api: PluginApi<R, SignupConfig>,
) -> Result<CognitoSignUp<R>> {
    info!("Initializing desktop module for Cognito Sign Up");

    let config = api.config().clone();
    let cognito_sign_up = CognitoSignUp::new(app.clone(), config);
    Ok(cognito_sign_up)
}
