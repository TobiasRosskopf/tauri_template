// Help under: https://v2.tauri.app/develop/calling-rust/

use tauri::State;

use crate::state::AppState;

/// Greet a user
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Add two numbers
#[tauri::command]
pub fn add(state: State<'_, AppState>, num1: i32, num2: i32) -> i32 {
    // Get state
    let mut state = state.lock().unwrap();

    // Update state
    state.num1 = num1;
    state.num2 = num2;
    state.sum = state.num1 + state.num2;

    // Return sum
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
