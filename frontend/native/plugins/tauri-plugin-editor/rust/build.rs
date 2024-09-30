const COMMANDS: &[&str] = &[
    "custom_cursor",
    "evaluate_cloud_pattern",
    "get_cloud_patterns",
    "load_model",
    "unload_model",
    "viewport_create",
    "viewport_upsert_properties",
    "viewport_remove_properties",
    "viewport_remove",
    "viewport_item_upsert_properties",
    "viewport_item_remove_properties",
    "viewport_item_remove",
    "model_node_upsert_properties",
    "model_node_remove_properties",
    "save_updates",
];

fn main() {
    let output = std::process::Command::new(std::env::var("CARGO").unwrap())
        .args(["metadata", "--format-version", "1"])
        .output()
        .expect("Failed to execute cargo metadata");

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata");

    let base_path = metadata["metadata"]["ts_bindings_base_path"]
        .as_str()
        .expect("bindings_base_path not set in workspace Cargo.toml");

    println!("cargo:rustc-env=TS_BINDINGS_BASE_PATH={}", base_path);
    println!("cargo:rerun-if-changed=../../../../../Cargo.toml");

    tauri_plugin::Builder::new(COMMANDS).build()
}
