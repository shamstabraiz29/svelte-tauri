// frontend/tauri/app/cloudcad-ui2/src-tauri/src/main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use cloudcad_ui2::ui_logger::*;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::fs::{create_dir_all, File};
use std::io;
use std::io::Write;
use std::path::PathBuf;

// Use the WORKSPACE_ROOT environment variable set by the build script
const WORKSPACE_ROOT: &str = env!("WORKSPACE_ROOT");

fn main() {
    let mut logger_builder = Builder::new();

    logger_builder
        .filter_module("cloudcad_ui2", LevelFilter::Debug)
        .filter_module("auth_", LevelFilter::Debug)
        .filter_module("cloud_", LevelFilter::Debug)
        .filter_module("common_", LevelFilter::Debug)
        .filter_module("tauri_plugin_", LevelFilter::Debug)
        .filter_module("verified_token", LevelFilter::Debug)
        .filter_module("frontend_tauri_plugins_common", LevelFilter::Debug)
        .filter(None, LevelFilter::Warn);

    // Set up the logging format
    logger_builder.format(|buf, record| {
        writeln!(
            buf,
            "{} [{}] - {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        )
    });

    // Always log to stdout
    logger_builder.target(Target::Stdout);

    #[cfg(debug_assertions)]
    {
        let debug_log_dir = PathBuf::from(WORKSPACE_ROOT).join("target").join("logs");
        if let Err(e) = create_dir_all(&debug_log_dir) {
            eprintln!("Failed to create log directory: {}", e);
        } else {
            let timestamp = Local::now().format("%Y%m%d_%H%M%S");
            let log_file_name = format!("debug_log_{}.log", timestamp);
            let debug_log_path = debug_log_dir.join(log_file_name);

            match File::create(&debug_log_path) {
                Ok(debug_log_file) => {
                    println!("Log file created: {:?}", debug_log_path);

                    // Use a custom writer that writes to both stdout and the file
                    let multi_writer = MultiWriter::new(std::io::stdout(), debug_log_file);
                    logger_builder.target(Target::Pipe(Box::new(multi_writer)));
                }
                Err(e) => eprintln!("Failed to create debug log file: {}", e),
            }
        }
    }

    // Initialize the logger
    logger_builder.init();

    tauri::Builder::default()
        .setup(|_app| {
            // Your setup code here
            Ok(())
        })
        .plugin(tauri_plugin_cognito_sign_up::lifecycle::init())
        .plugin(tauri_plugin_cognito_login::lifecycle::init())
        .plugin(tauri_plugin_account_client::lifecycle::init())
        .plugin(tauri_plugin_repo_client::lifecycle::init())
        .plugin(tauri_plugin_subscriber_client::lifecycle::init())
        .plugin(tauri_plugin_editor::lifecycle::init())
        .invoke_handler(tauri::generate_handler![log_message])
        .run(tauri::generate_context!())
        .expect("Error while running CloudCAD application");
}

// Add this struct to allow writing to multiple outputs

struct MultiWriter<W1: io::Write, W2: io::Write> {
    writer1: W1,
    writer2: W2,
}

impl<W1: io::Write, W2: io::Write> MultiWriter<W1, W2> {
    fn new(writer1: W1, writer2: W2) -> Self {
        MultiWriter { writer1, writer2 }
    }
}

impl<W1: io::Write, W2: io::Write> io::Write for MultiWriter<W1, W2> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer1.write(buf)?;
        self.writer2.write_all(&buf[..len])?;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer1.flush()?;
        self.writer2.flush()
    }
}
