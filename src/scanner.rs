use crate::frecent::FrecentDB;
use std::fs;
use std::path::Path;

fn fuzzy_score(query: &str, candidate: &str) -> Option<u32> {
    if query.is_empty() {
        return Some(0);
    }

    let query_chars: Vec<char> = query.chars().collect();
    let mut q_idx = 0;
    let mut score: u32 = 0;
    let mut prev_match_idx: i32 = -2;
    let mut consecutive_bonus_given = false;

    for (c_idx, c) in candidate.chars().enumerate() {
        if q_idx >= query_chars.len() {
            break;
        }

        let query_char = query_chars[q_idx];
        if c.to_lowercase().eq(query_char.to_lowercase()) {
            score += 10;

            if c_idx as i32 == prev_match_idx + 1 {
                score += 15;
                consecutive_bonus_given = true;
            }
            if c_idx == 0
                || candidate.as_bytes()[c_idx - 1] == b'_'
                || candidate.as_bytes()[c_idx - 1] == b'-'
                || candidate.as_bytes()[c_idx - 1] == b'.'
                || (c_idx > 0
                    && candidate.as_bytes()[c_idx - 1].is_ascii_lowercase()
                    && c.is_uppercase())
            {
                score += 20;
            }

            prev_match_idx = c_idx as i32;
            q_idx += 1;
        } else {
            if !consecutive_bonus_given {
                score = score.saturating_sub(1);
            }
            consecutive_bonus_given = false;
        }
    }

    if q_idx == query_chars.len() {
        Some(score)
    } else {
        None
    }
}
pub fn find_project(base_dir: &str, alias: &str, db: &FrecentDB, now: u64) -> Option<String> {
    let dir_iter = fs::read_dir(base_dir).ok()?;

    let mut best_score: i64 = -1;
    let mut best_path: Option<String> = None;

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

        if let Some(fuzzy) = fuzzy_score(alias, name_str) {
            let full_path = Path::new(base_dir).join(name_str);
            let full_path_str = full_path.to_string_lossy().into_owned();

            let frecency = db.frecency_score(&full_path_str, now);
            let total = fuzzy as i64 + frecency as i64;

            if total > best_score {
                best_score = total;
                best_path = Some(full_path_str);
            }
        }
    }

    best_path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frecent::FrecentDB;
    use std::env;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn setup_test_dirs(subdirs: &[&str]) -> String {
        let mut temp = env::temp_dir();
        temp.push("zsm_test");
        temp.push(format!(
            "{:x}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp).unwrap();
        for dir in subdirs {
            fs::create_dir(temp.join(dir)).unwrap();
        }
        temp.to_string_lossy().into_owned()
    }
    fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    #[test]
    fn test_find_exact_match() {
        let base = setup_test_dirs(&["my_project", "other_project"]);
        let db = FrecentDB::load(); // empty DB
        let now = now_secs();
        let result = find_project(&base, "my_project", &db, now);
        assert!(result.is_some());
        assert!(result.unwrap().ends_with("my_project"));
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn test_case_insensitive_match() {
        let base = setup_test_dirs(&["MyProject"]);
        let db = FrecentDB::load();
        let now = now_secs();
        let result = find_project(&base, "myproject", &db, now);
        assert!(result.is_some());
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn test_no_match() {
        let base = setup_test_dirs(&["project"]);
        let db = FrecentDB::load();
        let now = now_secs();
        let result = find_project(&base, "nonexistent", &db, now);
        assert!(result.is_none());
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn test_skip_hidden() {
        let base = setup_test_dirs(&[".hidden_project", "visible"]);
        let db = FrecentDB::load();
        let now = now_secs();
        let result = find_project(&base, "hidden", &db, now);
        assert!(result.is_none());
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn test_fuzzy_ordering() {
        let base = setup_test_dirs(&["dotfiles-bspwm", "bspwm-dotfiles", "database-migration"]);
        let db = FrecentDB::load();
        let now = now_secs();
        let result = find_project(&base, "dbm", &db, now);
        assert!(result.is_some());
        let path = result.unwrap();
        assert!(path.contains("dotfiles-bspwm") || path.contains("database-migration"));
        assert!(!path.contains("bspwm-dotfiles"));
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn test_fuzzy_no_match_wrong_order() {
        let base = setup_test_dirs(&["abcdef"]);
        let db = FrecentDB::load();
        let now = now_secs();
        let result = find_project(&base, "fa", &db, now);
        assert!(result.is_none());
        fs::remove_dir_all(base).unwrap();
    }
}
