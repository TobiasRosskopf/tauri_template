use tauri_plugin_cli::CliExt;

use crate::state;

/// Parse command line arguments
pub fn cli(app: &tauri::App) {
    // Get app state to eventually store command line arguments
    let mut state = state::get_state(app);
    
    println!("Parsing command line arguments");
    match app.cli().matches() {
        // `matches` here is a Struct with { args, subcommand }.
        // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
        // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
        Ok(matches) => {
            // Check if the `help` argument was passed
            if matches.args.contains_key("help") {
                println!("{}", matches.args.get("help").unwrap().value.as_str().unwrap());
                std::process::exit(0);
            }
            // Check if the `version` argument was passed
            if matches.args.contains_key("version") {
                println!("Version: {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            // Check if the `ini` argument was passed
            if matches.args.contains_key("ini") {
                state.ini_file = match matches.args.get("ini") {
                    Some(ini_file) => ini_file.value.as_str().unwrap().to_string(),
                    None => "".to_string(),
                };
            }
            // Print the app state in debug builds
            #[cfg(debug_assertions)] {
                println!("{:#?}", state);
            }
        }
        Err(err) => {
            println!("Error parsing command line arguments: {}", err);
        }
    };
}
