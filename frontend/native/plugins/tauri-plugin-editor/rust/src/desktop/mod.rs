// frontend/tauri/plugins/tauri-plugin-editor/rust/src/desktop/mod.rs
pub mod editor_client;

use log::{debug, info};
use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use self::editor_client::EditorClient;

pub(crate) fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<EditorClient<R>> {
    info!("Initializing EditorClient for desktop");
    // debug!("AppHandle details: {:?}", app); // Looks like there is a lot of binary data in the AppHandle

    let editor_client = EditorClient(app.clone());
    debug!("EditorClient created successfully");

    info!("EditorClient initialization completed");
    Ok(editor_client)
}
