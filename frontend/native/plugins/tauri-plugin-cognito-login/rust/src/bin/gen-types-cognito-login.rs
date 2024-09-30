// frontend/native/plugins/tauri-plugin-cognito-login/rust/src/bin/gen-types-cognito-login.rs
use frontend_tauri_plugins_common::generate_typescript_bindings_main;
use tauri_plugin_cognito_login::specta_builder_wry;

generate_typescript_bindings_main!(specta_builder_wry!());
