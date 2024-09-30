// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cloudcad_ui2::ui_logger::*;
use log::LevelFilter;

fn main() {
    env_logger::Builder::new()
        .filter_module("commands_tests", LevelFilter::Trace)
        .filter_module("auth_", LevelFilter::Trace)
        .filter_module("cloud_", LevelFilter::Trace)
        .filter_module("common_", LevelFilter::Trace)
        .filter_module("tauri_plugin_", LevelFilter::Trace)
        .filter_module("verified_token", LevelFilter::Trace)
        .filter_module("frontend_tauri_plugins_common", LevelFilter::Trace)
        .filter(None, LevelFilter::Debug)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_cognito_sign_up::lifecycle::init())
        .plugin(tauri_plugin_cognito_login::lifecycle::init())
        .plugin(tauri_plugin_account_client::lifecycle::init())
        .plugin(tauri_plugin_repo_client::lifecycle::init())
        .plugin(tauri_plugin_subscriber_client::lifecycle::init())
        .plugin(tauri_plugin_editor::lifecycle::init())
        .invoke_handler(tauri::generate_handler![log_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
