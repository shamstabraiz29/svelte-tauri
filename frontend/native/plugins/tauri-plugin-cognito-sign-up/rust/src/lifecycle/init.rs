use tauri::plugin::{Builder, TauriPlugin};

use tauri::{Manager, Wry};

use crate::{config, desktop, specta_builder_wry, PLUGIN_NAME};

/// Initializes the plugin.
pub fn init() -> TauriPlugin<Wry, config::SignupConfig> {
    let builder = specta_builder_wry!();

    Builder::<Wry, config::SignupConfig>::new(PLUGIN_NAME)
        .invoke_handler(builder.invoke_handler())
        .setup(move |app, api| {
            let sign_up_config = api.config().clone();
            app.manage(sign_up_config);

            #[cfg(desktop)]
            let cognito_sign_up = desktop::init(app, api)?;
            app.manage(cognito_sign_up);

            Ok(())
        })
        .build()
}
