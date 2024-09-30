// tauri-plugin-account-client/rust/src/lifecycle/init.rs

use frontend_tauri_plugins_common::{
    states::ag_commit_state::AgCommitState, types::account_dto_state::AccountDtoState,
};

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Wry,
};

use crate::{config, desktop, specta_builder_wry, PLUGIN_NAME};

#[derive(Debug, Clone)]
pub struct AcctCommitInfo;

/// Initializes the plugin.
pub fn init() -> TauriPlugin<Wry, config::AccountClientConfig> {
    let builder = specta_builder_wry!().plugin_name(PLUGIN_NAME);

    Builder::<Wry, config::AccountClientConfig>::new(PLUGIN_NAME)
        .invoke_handler(builder.invoke_handler())
        .setup(move |app, api| {
            builder.mount_events(app);

            let account_client_config = api.config().clone();
            app.manage(account_client_config.to_owned());

            let acct_dto_state = AccountDtoState::new(app.clone());
            app.manage(acct_dto_state);

            let acct_commit_state: AgCommitState<AcctCommitInfo> = AgCommitState::new(app.clone());
            app.manage(acct_commit_state);

            #[cfg(desktop)]
            let account_client = desktop::init(app, api)?;
            app.manage(account_client);

            Ok(())
        })
        .build()
}
