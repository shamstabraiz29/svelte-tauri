const COMMANDS: &[&str] = &[
    "archive_repo",
    "create_folder",
    "create_repo",
    "get_account_summary",
    "move_folder",
    "move_repo",
    "remove_folder_properties",
    "remove_folder",
    "remove_repo_properties",
    "rename_folder",
    "rename_repo",
    "set_account",
    "upsert_folder_properties",
    "upsert_repo_properties",
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
