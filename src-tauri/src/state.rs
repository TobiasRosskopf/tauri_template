use std::sync::Mutex;

use tauri::{App, Manager};

#[derive(Default, Debug, Clone)]
pub struct AppState {
    pub ini_file: String,
    pub num1: i32,
    pub num2: i32,
    pub sum: i32,
}

pub fn setup(app: &App) {
    app.manage(Mutex::new(AppState::default()));
}

pub fn get_state(app: &App) -> AppState {
    app.state::<Mutex<AppState>>().lock().unwrap().clone()
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let state = AppState::default();
        assert_eq!(state.ini_file, "");
        assert_eq!(state.num1, 0);
        assert_eq!(state.num2, 0);
        assert_eq!(state.sum, 0);
    }
}
