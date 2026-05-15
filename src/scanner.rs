use std::fs;
use std::path::Path;

pub fn find_project(base_dir: &str, alias: &str) -> Option<String> {
    let dir_iter = fs::read_dir(base_dir).ok()?;
    let lower_alias = alias.to_lowercase();

    for entry in dir_iter {
        let entry = entry.ok()?;
        let file_type = entry.file_type().ok()?;
        if !file_type.is_dir() {
            continue;
        }

        let name = entry.file_name();
        let name_str = name.to_str()?;

        if name_str.starts_with('.') {
            continue;
        }

        if name_str.to_lowercase().contains(&lower_alias) {
            let full_path = Path::new(base_dir).join(name_str);
            return Some(full_path.to_string_lossy().into_owned());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    fn setup_test_dirs(subdirs: &[&str]) -> String {
        let mut temp = env::temp_dir();
        temp.push("zsm_test");
        temp.push(format!(
            "{:x}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        fs::create_dir_all(&temp).unwrap();
        for dir in subdirs {
            fs::create_dir(temp.join(dir)).unwrap();
        }
        temp.to_string_lossy().into_owned()
    }

    #[test]
    fn test_find_exact_match() {
        let base = setup_test_dirs(&["my_project", "other_project"]);
        let result = find_project(&base, "my_project");
        assert!(result.is_some());
        assert!(result.unwrap().ends_with("my_project"));
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn test_case_insensitive_match() {
        let base = setup_test_dirs(&["MyProject"]);
        let result = find_project(&base, "myproject");
        assert!(result.is_some());
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn test_no_match() {
        let base = setup_test_dirs(&["project"]);
        let result = find_project(&base, "nonexistent");
        assert!(result.is_none());
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn test_skip_hidden() {
        let base = setup_test_dirs(&[".hidden_project", "visible"]);
        let result = find_project(&base, "hidden");
        assert!(result.is_none());
        fs::remove_dir_all(base).unwrap();
    }
}
