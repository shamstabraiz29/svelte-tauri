// frontend/native/plugins/common/src/ts_bindings/mod.rs
use anyhow::{Context, Result};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use specta_typescript::Typescript;
use std::env;
use std::fs;
use std::path::{Component, PathBuf};

pub fn generate_typescript_bindings(
    builder: tauri_specta::Builder<tauri::Wry>,
    crate_name: &str,
    base_path: &str,
) -> Result<()> {
    // Initialize the logger
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .context("Failed to initialize logger")?;

    info!("Starting type generation script");
    info!("Crate name: {}", crate_name);

    // Ensure we're running from Cargo
    if env::var("CARGO").is_err() {
        anyhow::bail!("This script must be run from Cargo");
    }

    // Get the workspace root
    let workspace_root = find_workspace_root()?;
    info!("Workspace root: {:?}", workspace_root);

    // Sanitize and validate the base_path
    let sanitized_base_path = sanitize_path(base_path)?;
    info!("Sanitized base path: {:?}", sanitized_base_path);

    // Construct the full output path
    let mut binding_out_file = workspace_root.join(&sanitized_base_path);
    binding_out_file.push(crate_name);
    binding_out_file.push("bindings");
    binding_out_file.push("index.mts");

    info!("Output file path: {:?}", binding_out_file);

    // Ensure the directory exists
    if let Some(parent) = binding_out_file.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        info!("Ensured directory exists: {:?}", parent);
    }

    info!("Exporting TypeScript bindings");
    builder
        .export(
            Typescript::default().formatter(specta_typescript::formatter::prettier),
            &binding_out_file,
        )
        .with_context(|| {
            format!(
                "Failed to export TypeScript bindings to {:?}",
                binding_out_file
            )
        })?;

    info!(
        "Successfully exported TypeScript bindings to {:?}",
        binding_out_file
    );
    Ok(())
}

fn sanitize_path(path: &str) -> Result<PathBuf> {
    let path = PathBuf::from(path);

    // Normalize the path and remove any `.` or `..` components
    let cleaned: PathBuf = path
        .components()
        .filter(|component| matches!(component, Component::Normal(_)))
        .collect();

    if cleaned.to_str().unwrap_or("").is_empty() {
        anyhow::bail!("Invalid or empty path after sanitization");
    }

    Ok(cleaned)
}

fn find_workspace_root() -> Result<PathBuf> {
    let mut current_dir = env::current_dir()?;
    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            let contents = fs::read_to_string(cargo_toml)?;
            if contents.contains("[workspace]") {
                return Ok(current_dir);
            }
        }
        if !current_dir.pop() {
            anyhow::bail!("Could not find workspace root");
        }
    }
}

#[macro_export]
macro_rules! generate_typescript_bindings_main {
    ($specta_builder:expr) => {
        fn main() -> ::anyhow::Result<()> {
            let crate_name = env!("CARGO_PKG_NAME");
            let base_path = env!("TS_BINDINGS_BASE_PATH");
            $crate::ts_bindings::generate_typescript_bindings(
                $specta_builder,
                crate_name,
                base_path,
            )
        }
    };
}
