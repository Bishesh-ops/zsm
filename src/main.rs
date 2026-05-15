use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No search aliases provided. \nUsage: zsm <alias>");
        process::exit(1);
    }
    let target_alias = &args[1];

    if target_alias
        .chars()
        .any(|c| c == '/' || c == '\\' || c.is_control())
    {
        eprintln!("Error: Alias contains illegal characters.");
        process::exit(1);
    }

    let base = env::var("ZSM_BASE").unwrap_or_else(|_| "/home/bisheshshrestha/Dev".to_string());

    let dir_iter = match fs::read_dir(&base) {
        Ok(it) => it,
        Err(e) => {
            eprintln!("Error: Cannot open '{}': {}", base, e);
            process::exit(1);
        }
    };

    for entry in dir_iter {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        if !file_type.is_dir() {
            continue;
        }
        let name = entry.file_name();

        let name_str = match name.to_str() {
            Some(s) => s,
            None => continue,
        };

        if name_str.starts_with('.') {
            continue;
        }

        if name_str
            .to_lowercase()
            .contains(&target_alias.to_lowercase())
        {
            println!("{}/{}", base, name_str);
            return;
        }
    }

    eprintln!(
        "Error: No project matching '{}' found in '{}'.",
        target_alias, base
    );
    process::exit(1);
}
