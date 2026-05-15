use std::env;
use std::process;

mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: No search alias provided.\nUsage: zsm <alias>");
        process::exit(1);
    }
    let target_alias = &args[1];

    // Validation
    if target_alias.is_empty()
        || target_alias.starts_with('.')
        || target_alias.starts_with('/')
        || target_alias
            .chars()
            .any(|c| c == '/' || c == '\\' || c.is_control())
    {
        eprintln!("Error: Alias contains illegal characters or starts with '.' or '/'.");
        process::exit(1);
    }

    let base = env::var("ZSM_BASE").unwrap_or_else(|_| "/home/bisheshshrestha/Dev".to_string());

    match scanner::find_project(&base, target_alias) {
        Some(path) => println!("{}", path),
        None => {
            eprintln!(
                "Error: No project matching '{}' found in '{}'.",
                target_alias, base
            );
            process::exit(1);
        }
    }
}
