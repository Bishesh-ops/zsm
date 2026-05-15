use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct FrecentEntry {
    pub frequency: u32,
    pub last_visited: u64,
}

pub struct FrecentDB {
    entries: HashMap<String, FrecentEntry>,
}

impl FrecentDB {
    fn file_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("~/.local/share"));
        path.push("zsm");
        fs::create_dir_all(&path).ok();
        path.push("history.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::file_path();
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(entries) = serde_json::from_str::<HashMap<String, FrecentEntry>>(&data) {
                return FrecentDB { entries };
            }
        }
        FrecentDB {
            entries: HashMap::new(),
        }
    }

    pub fn save(&self) {
        let path = Self::file_path();
        if let Ok(json) = serde_json::to_string(&self.entries) {
            fs::write(path, json).ok();
        }
    }

    pub fn record_visit(&mut self, path: &str, now_unix_secs: u64) {
        let entry = self.entries.entry(path.to_owned()).or_insert(FrecentEntry {
            frequency: 0,
            last_visited: now_unix_secs,
        });
        entry.frequency += 1;
        entry.last_visited = now_unix_secs;
    }

    pub fn frecency_score(&self, path: &str, now_unix_secs: u64) -> u32 {
        if let Some(entry) = self.entries.get(path) {
            let age_secs = now_unix_secs.saturating_sub(entry.last_visited);
            let age_hours = age_secs / 3600;
            let recency_bonus = 100 / (age_hours + 1);
            entry.frequency * (1 + recency_bonus) as u32
        } else {
            0
        }
    }

    #[cfg(test)]
    pub fn new() -> Self {
        FrecentDB {
            entries: HashMap::new(),
        }
    }
}
