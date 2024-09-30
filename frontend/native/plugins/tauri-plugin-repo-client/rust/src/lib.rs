// tauri-plugin-repo-client/rust/src/lib.rs
pub mod commands;
mod config;
mod error;
pub mod ext;
pub mod lifecycle;
mod models;

#[cfg(desktop)]
pub mod desktop;

pub mod types;

pub use error::{RepoError, Result};

pub const PLUGIN_NAME: &str = "repo-client";

#[macro_export]
macro_rules! specta_builder_wry {
    () => {
        tauri_specta::Builder::<tauri::Wry>::new()
            .plugin_name($crate::PLUGIN_NAME)
            .events(tauri_specta::collect_events![
                frontend_tauri_plugins_common::events::commit_info_event::CommitInfoEvent,
                frontend_tauri_plugins_common::events::repo_detail_event::RepoDetailEvent
            ])
            .commands(tauri_specta::collect_commands![
                $crate::commands::get_repo,
                $crate::commands::get_commit_history,
                $crate::commands::create_branch,
                $crate::commands::remove_branch_properties,
                $crate::commands::remove_branch,
                $crate::commands::rename_branch,
                $crate::commands::upsert_branch_properties,
                $crate::commands::set_repo
            ])
    };
}
