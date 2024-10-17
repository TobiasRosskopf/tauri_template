use std::sync::Mutex;

use tauri::State;

use super::state::AppState;

/// Greet a user
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Add two numbers
#[tauri::command]
pub fn add(state: State<'_, Mutex<AppState>>, num1: i32, num2: i32) -> i32 {
    let mut state = state.lock().unwrap();
    state.num1 = num1;
    state.num2 = num2;
    state.sum = state.num1 + state.num2;
    state.sum
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("world"), "Hello, world! You've been greeted from Rust!");
    }
}
