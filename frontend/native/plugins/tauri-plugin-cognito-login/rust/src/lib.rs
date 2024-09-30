// src/lib.rs
pub mod commands;
mod config;
mod error;
pub mod events;
pub mod models;

#[cfg(desktop)]
pub mod desktop;

pub mod lifecycle;
pub mod types;

pub use error::LoginError;

pub const PLUGIN_NAME: &str = "cognito-login";

#[macro_export]
macro_rules! specta_builder_wry {
    () => {
        tauri_specta::Builder::<tauri::Wry>::new()
            .plugin_name($crate::PLUGIN_NAME)
            .events(tauri_specta::collect_events![
                frontend_tauri_plugins_common::events::bearer_token_event::BearerTokenEvent
            ])
            .commands(tauri_specta::collect_commands![
                $crate::commands::logout::<tauri::Wry>,
                $crate::commands::mfa_login,
                $crate::commands::mfa_setup,
                $crate::commands::pw_login,
                $crate::commands::refresh_login::<tauri::Wry>,
                $crate::commands::store_refresh_token::<tauri::Wry>,
            ])
    };
}
