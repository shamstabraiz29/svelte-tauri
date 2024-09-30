// frontend/tauri/app/cloudcad-ui2/tauri-backend/build.rs
use std::env;
use std::process::Command;

fn main() {
    // Run `cargo metadata` to get workspace root
    let output = Command::new(env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()))
        .args(["metadata", "--format-version", "1"])
        .output()
        .expect("Failed to execute cargo metadata");

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata");

    let workspace_root = metadata["workspace_root"]
        .as_str()
        .expect("Failed to get workspace root from cargo metadata");

    // Print the workspace root as a compile-time environment variable
    println!("cargo:rustc-env=WORKSPACE_ROOT={}", workspace_root);

    // Run the Tauri 2 beta build process
    tauri_build::build()
}
