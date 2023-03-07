/// Parse command line arguments
pub fn cli(app: &tauri::App) {
    match app.get_cli_matches() {
        // `matches` here is a Struct with { args, subcommand }.
        // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
        // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
        Ok(matches) => {
            // Check if the `help` argument was passed
            if matches.args.contains_key("help") {
                println!(
                    "{}",
                    matches.args.get("help").unwrap().value.as_str().unwrap()
                );
                std::process::exit(0);
            }
            // Check if the `version` argument was passed
            if matches.args.contains_key("version") {
                println!("Version: {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            // Check if the `verbose` argument was passed
            if matches.args.contains_key("verbose") {
                println!(
                    "Verbose mode enabled ({} times)!",
                    matches.args.get("verbose").unwrap().occurrences
                );
                // TODO: Enable verbose mode
            }
        }
        Err(_) => {}
    }
}
