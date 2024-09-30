// tauri-plugin-account-client/rust/src/lib.rs
pub mod commands;
mod config;
mod error;
pub mod ext;
pub mod lifecycle;
pub mod models;

#[cfg(desktop)]
pub mod desktop;

pub mod types;

pub use error::{AccountError, Result};

pub const PLUGIN_NAME: &str = "account-client";

#[macro_export]
macro_rules! specta_builder_wry {
    () => {
        tauri_specta::Builder::<tauri::Wry>::new()
            .plugin_name($crate::PLUGIN_NAME)
            .events(tauri_specta::collect_events![
                frontend_tauri_plugins_common::events::acct_detail_event::AcctDetailEvent,
                frontend_tauri_plugins_common::events::commit_info_event::CommitInfoEvent
            ])
            .commands(tauri_specta::collect_commands![
                $crate::commands::create_folder,
                $crate::commands::create_repo,
                $crate::commands::get_account_summary,
                $crate::commands::move_folder,
                $crate::commands::move_repo,
                $crate::commands::remove_folder,
                $crate::commands::archive_repo,
                $crate::commands::remove_folder_properties,
                $crate::commands::remove_repo_properties,
                $crate::commands::rename_folder,
                $crate::commands::rename_repo,
                $crate::commands::set_account,
                $crate::commands::upsert_folder_properties,
                $crate::commands::upsert_repo_properties
            ])
    };
}
