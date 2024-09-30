pub mod commands;
mod config;
mod error;
pub mod ext;
pub mod lifecycle;
mod models;

#[cfg(desktop)]
pub mod desktop;
pub mod types;

pub use error::{Result, SignUpError};

pub const PLUGIN_NAME: &str = "cognito-sign-up";

#[macro_export]
macro_rules! specta_builder_wry {
    () => {
        tauri_specta::Builder::<tauri::Wry>::new()
            .plugin_name($crate::PLUGIN_NAME)
            .commands(tauri_specta::collect_commands![
                $crate::commands::confirm_email,
                $crate::commands::resend_confirmation,
                $crate::commands::sign_up
            ])
    };
}
