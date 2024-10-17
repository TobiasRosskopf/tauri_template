/// Greet a user
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Add two numbers
#[tauri::command]
pub fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
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
