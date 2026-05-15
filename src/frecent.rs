use std::collections::HashMap;

pub struct FrecentEntry {
    pub frequency: u32,
    pub last_visited: u64,
}

pub struct FrecentDB {
    entries: HashMap<String, FrecentEntry>,
}

impl FrecentDB {
    pub fn new() -> Self {
        FrecentDB {
            entries: HashMap::new(),
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
}
