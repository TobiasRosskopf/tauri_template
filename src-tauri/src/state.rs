use std::sync::Mutex;

use tauri::{App, Manager};

#[derive(Default)]
pub struct AppState {
    pub num1: i32,
    pub num2: i32,
    pub sum: i32,
}

pub fn setup(app: &App) {
    app.manage(Mutex::new(AppState::default()));
}
