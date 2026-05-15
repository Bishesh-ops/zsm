use crate::hashmap::HashMap; // <--- new

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
            entries: HashMap::new(8),
        }
    }

    pub fn record_visit(&mut self, path: &str, now_unix_secs: u64) {
        if let Some(entry) = self.entries.get_mut(&path.to_owned()) {
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
        if let Some(entry) = self.entries.get(&path.to_owned()) {
            let age_secs = now_unix_secs.saturating_sub(entry.last_visited);
            let age_hours = age_secs / 3600;
            let recency_bonus = 100 / (age_hours + 1);
            entry.frequency * (1 + recency_bonus) as u32
        } else {
            0
        }
    }
}
