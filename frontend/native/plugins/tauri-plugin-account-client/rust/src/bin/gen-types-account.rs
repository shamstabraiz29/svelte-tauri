// frontend/native/plugins/tauri-plugin-account-client/rust/src/bin/gen-types-account.rs
use frontend_tauri_plugins_common::generate_typescript_bindings_main;
use tauri_plugin_account_client::specta_builder_wry;

generate_typescript_bindings_main!(specta_builder_wry!());
