const COMMANDS: &[&str] = &[
    "create_branch",
    "get_commit_history",
    "get_repo",
    "remove_branch_properties",
    "remove_branch",
    "rename_branch",
    "set_repo",
    "upsert_branch_properties",
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
