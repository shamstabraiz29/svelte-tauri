// tauri-plugin-subscriber-client/rust/src/lifecycle/init.rs
use crate::{config, desktop, specta_builder_wry, PLUGIN_NAME};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Wry,
};

/// Initializes the plugin.
pub fn init() -> TauriPlugin<Wry, config::SubscriberClientConfig> {
    let builder = specta_builder_wry!();

    Builder::<Wry, config::SubscriberClientConfig>::new(PLUGIN_NAME)
        .invoke_handler(builder.invoke_handler())
        .setup(move |app, api| {
            builder.mount_events(app);

            let subscriber_client_config = api.config().clone();
            app.manage(subscriber_client_config);

            #[cfg(desktop)]
            let subscriber_client = desktop::init(app, api)?;
            app.manage(subscriber_client);

            Ok(())
        })
        .build()
}
