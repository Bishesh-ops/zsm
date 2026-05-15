use crate::hashmap::HashMap as MyHashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap as StdHashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrecentEntry {
    pub frequency: u32,
    pub last_visited: u64,
}

pub struct FrecentDB {
    entries: MyHashMap<String, FrecentEntry>,
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
            if let Ok(std_map) = serde_json::from_str::<StdHashMap<String, FrecentEntry>>(&data) {
                // Convert to our custom HashMap
                let mut map = MyHashMap::new(std_map.len().max(8));
                for (k, v) in std_map {
                    map.insert(k, v);
                }
                return FrecentDB { entries: map };
            }
        }
        FrecentDB {
            entries: MyHashMap::new(8),
        }
    }

    pub fn save(&self) {
        let path = Self::file_path();
        let std_map: StdHashMap<&String, &FrecentEntry> = self.entries.iter().collect();
        if let Ok(json) = serde_json::to_string(&std_map) {
            fs::write(path, json).ok();
        }
    }

    pub fn record_visit(&mut self, path: &str, now_unix_secs: u64) {
        if let Some(entry) = self.entries.get_mut(path) {
            entry.frequency += 1;
            entry.last_visited = now_unix_secs;
        } else {
            self.entries.insert(
                path.to_owned(),
                FrecentEntry {
                    frequency: 1,
                    last_visited: now_unix_secs,
                },
            );
        }
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
            entries: MyHashMap::new(8),
        }
    }
}
