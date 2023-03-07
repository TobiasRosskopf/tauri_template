// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lib::cli;

/// Greet a user
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Open devtools on debug builds
fn devtools(app: &tauri::App) {
    #[cfg(debug_assertions)]
    {
        use tauri::Manager;
        let window = app.get_window("main").unwrap();
        window.open_devtools();
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            cli::cli(app);
            devtools(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
