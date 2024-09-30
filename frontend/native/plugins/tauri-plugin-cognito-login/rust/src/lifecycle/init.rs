// src/lifecycle/init.rs

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Wry,
};

use crate::{config, specta_builder_wry, PLUGIN_NAME};

#[cfg(desktop)]
use crate::desktop;

/// Initializes the plugin.
pub fn init() -> TauriPlugin<Wry, config::LoginConfig> {
    let builder = specta_builder_wry!();

    Builder::<Wry, config::LoginConfig>::new(PLUGIN_NAME)
        .invoke_handler(builder.invoke_handler())
        .setup(move |app, api| {
            builder.mount_events(app);

            let sign_up_config = api.config().clone();
            app.manage(sign_up_config);

            let bearer_tokens =
                frontend_tauri_plugins_common::types::bearer_tokens::BearerTokens::new(app.clone());
            app.manage(bearer_tokens);

            #[cfg(desktop)]
            let cognito_sign_up = desktop::init(app, api);
            app.manage(cognito_sign_up);

            Ok(())
        })
        .build()
}
