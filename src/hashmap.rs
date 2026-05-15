use std::hash::{Hash, Hasher};

pub struct HashMap<K, V>
where
    K: Eq + Hash,
{
    buckets: Vec<Option<(K, V)>>,
    len: usize,
}

impl<K: Eq + Hash, V> HashMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(None);
        }
        HashMap { buckets, len: 0 }
    }

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn find_bucket(&self, key: &K) -> Option<usize> {
        if self.buckets.is_empty() {
            return None;
        }
        let mut idx = (self.hash(key) as usize) % self.buckets.len();
        let start = idx;
        loop {
            match &self.buckets[idx] {
                Some((k, _)) if k == key => return Some(idx),
                None => return Some(idx),
                _ => {}
            }
            idx = (idx + 1) % self.buckets.len();
            if idx == start {
                return None;
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Resize if load factor > 0.7
        if self.len * 10 > self.buckets.len() * 7 {
            let new_cap = std::cmp::max(self.buckets.len() * 2, 8);
            self.resize(new_cap);
        }
        if let Some(bucket_idx) = self.find_bucket(&key) {
            if let Some(ref mut slot) = self.buckets[bucket_idx] {
                // existing key, replace
                let old_val = std::mem::replace(&mut slot.1, value);
                Some(old_val)
            } else {
                // empty slot
                self.buckets[bucket_idx] = Some((key, value));
                self.len += 1;
                None
            }
        } else {
            panic!("HashMap full – should not happen after resize");
        }
    }

    fn resize(&mut self, new_capacity: usize) {
        let mut new_buckets = Vec::with_capacity(new_capacity);
        for _ in 0..new_capacity {
            new_buckets.push(None);
        }
        let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);
        self.len = 0;
        for bucket in old_buckets {
            if let Some((k, v)) = bucket {
                self.insert(k, v);
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(idx) = self.find_bucket(key) {
            if let Some((_, ref v)) = self.buckets[idx] {
                return Some(v);
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if let Some(idx) = self.find_bucket(key) {
            if let Some((_, ref mut v)) = self.buckets[idx] {
                return Some(v);
            }
        }
        None
    }
}
