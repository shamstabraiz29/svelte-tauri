const COMMANDS: &[&str] = &[
    "logout",
    "mfa_login",
    "mfa_setup",
    "pw_login",
    "refresh_login",
    "store_refresh_token",
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
