// tauri-plugin-repo-client/rust/src/lifecycle/init.rs
use frontend_tauri_plugins_common::{
    states::ag_commit_state::AgCommitState, types::repo_detail_state::RepoDtoState,
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Wry,
};

use crate::{config::RepoClientConfig, desktop, specta_builder_wry, PLUGIN_NAME};

#[derive(Debug, Clone)]
pub struct RepoCommitInfo;

/// Initializes the plugin.
pub fn init() -> TauriPlugin<Wry, RepoClientConfig> {
    let builder = specta_builder_wry!();

    Builder::<Wry, RepoClientConfig>::new(PLUGIN_NAME)
        .invoke_handler(builder.invoke_handler())
        .setup(move |app, api| {
            builder.mount_events(app);

            let repo_client_config = api.config().clone();
            app.manage(repo_client_config.to_owned());

            let repo_dto_state = RepoDtoState::new(app.clone());
            app.manage(repo_dto_state);

            let repo_commit_state: AgCommitState<RepoCommitInfo> = AgCommitState::new(app.clone());
            app.manage(repo_commit_state);

            #[cfg(desktop)]
            let repo_client = desktop::init(app, api)?;
            app.manage(repo_client);

            Ok(())
        })
        .build()
}
