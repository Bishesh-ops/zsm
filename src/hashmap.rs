use std::hash::{Hash, Hasher};

pub struct HashMap<K, V>
where
    K: Eq + Hash,
{
    buckets: Vec<Option<((K, V), usize)>>,
    len: usize,
}

impl<K: Eq + Hash, V> HashMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        buckets.resize_with(capacity, || None);
        HashMap { buckets, len: 0 }
    }

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn find_bucket_borrowed<Q>(&self, key: &Q) -> Option<(usize, bool)>
    where
        K: std::borrow::Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.buckets.is_empty() {
            return None;
        }
        let mut hasher = std::hash::DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_val = hasher.finish();
        let start_idx = (hash_val as usize) % self.buckets.len();

        let mut idx = start_idx;
        let mut dist = 0;
        loop {
            match &self.buckets[idx] {
                Some(((k, _), d)) => {
                    if k.borrow() == key {
                        return Some((idx, true));
                    }
                    if *d < dist {
                        return Some((idx, false));
                    }
                }
                None => {
                    return Some((idx, false));
                }
            }
            idx = (idx + 1) % self.buckets.len();
            dist += 1;
            if idx == start_idx {
                return None;
            }
        }
    }

    fn find_bucket(&self, key: &K) -> Option<(usize, bool)> {
        self.find_bucket_borrowed(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.len * 10 > self.buckets.len() * 7 {
            let new_cap = std::cmp::max(self.buckets.len() * 2, 8);
            self.resize(new_cap);
        }

        let (mut idx, found) = self.find_bucket(&key).expect("table full");
        if found {
            if let Some(((_, ref mut v), _)) = self.buckets[idx] {
                return Some(std::mem::replace(v, value));
            }
            unreachable!();
        }

        let ideal = self.hash(&key) as usize % self.buckets.len();
        let mut probe_dist =
            (idx as isize - ideal as isize).rem_euclid(self.buckets.len() as isize) as usize;
        let mut new_entry = (key, value);

        loop {
            if let Some(ref mut slot) = self.buckets[idx] {
                if probe_dist > slot.1 {
                    std::mem::swap(&mut new_entry.0, &mut slot.0 .0);
                    std::mem::swap(&mut new_entry.1, &mut slot.0 .1);
                    std::mem::swap(&mut probe_dist, &mut slot.1);
                }
            } else {
                self.buckets[idx] = Some((new_entry, probe_dist));
                self.len += 1;
                return None;
            }
            idx = (idx + 1) % self.buckets.len();
            probe_dist += 1;
        }
    }

    fn resize(&mut self, new_capacity: usize) {
        let mut new_buckets: Vec<Option<((K, V), usize)>> = Vec::with_capacity(new_capacity);
        for _ in 0..new_capacity {
            new_buckets.push(None);
        }
        let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);
        self.len = 0;
        for bucket in old_buckets {
            if let Some(((k, v), _)) = bucket {
                self.insert(k, v);
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if let Some((idx, true)) = self.find_bucket_borrowed(key) {
            if let Some(((_, v), _)) = &self.buckets[idx] {
                return Some(v);
            }
        }
        None
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if let Some((idx, true)) = self.find_bucket_borrowed(key) {
            if let Some(((_, v), _)) = &mut self.buckets[idx] {
                return Some(v);
            }
        }
        None
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets
            .iter()
            .filter_map(|opt| opt.as_ref().map(|((k, v), _)| (k, v)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut map = HashMap::new(4);
        map.insert("hello", 42);
        assert_eq!(map.get(&"hello"), Some(&42));
    }

    #[test]
    fn update_existing() {
        let mut map = HashMap::new(4);
        map.insert("x", 1);
        assert_eq!(map.insert("x", 2), Some(1));
        assert_eq!(map.get(&"x"), Some(&2));
    }

    #[test]
    fn collision_and_robin_hood() {
        let mut map = HashMap::new(4);
        for i in 0..20 {
            map.insert(format!("key{}", i), i);
        }
        for i in 0..20 {
            assert_eq!(map.get(&format!("key{}", i)), Some(&i));
        }
    }

    #[test]
    fn missing_key() {
        let mut map = HashMap::new(4);
        map.insert("a", 1);
        assert_eq!(map.get(&"b"), None);
    }
}
