// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{App, Builder, Manager};

use lib::state;
use lib::cli;
use lib::commands;

/// Open devtools on debug builds
fn devtools(app: &App) {
    #[cfg(debug_assertions)]
    {
        let window = app.get_webview_window("main").unwrap();
        window.open_devtools();
    }
}

fn main() {
    Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            state::setup(app);  // Setup app state
            cli::cli(app);      // Setup command line interface
            devtools(app);      // Open devtools on debug builds
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet,  // Greet command
            commands::add,    // Add command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
