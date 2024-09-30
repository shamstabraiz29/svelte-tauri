// /tauri-plugin-subscriber-client/rust/src/lib.rs
pub mod commands;
mod config;
mod error;
mod models;
mod types;

pub mod ext;
pub mod lifecycle;

#[cfg(desktop)]
pub mod desktop;

pub use error::SubscriberError;

use tauri::plugin::TauriPlugin;
use tauri::Wry;

pub const PLUGIN_NAME: &str = "subscriber-client";

#[macro_export]
macro_rules! specta_builder_wry {
    () => {
        tauri_specta::Builder::<tauri::Wry>::new()
            .plugin_name($crate::PLUGIN_NAME)
            .commands(tauri_specta::collect_commands![
                $crate::commands::get_subscriber
            ])
    };
}

pub fn init() -> TauriPlugin<Wry, config::SubscriberClientConfig> {
    lifecycle::init::init()
}
