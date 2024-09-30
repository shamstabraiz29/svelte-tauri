// src/desktop/mod.rs
pub mod subscriber_client;

use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use self::subscriber_client::SubscriberClient;

use anyhow::Result;

pub(crate) fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<SubscriberClient<R>> {
    Ok(SubscriberClient::new(app.clone()))
}
