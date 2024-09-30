pub mod cognito_login;

use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use self::cognito_login::CognitoLogin;

pub(crate) fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> CognitoLogin<R> {
    CognitoLogin(app.clone())
}
