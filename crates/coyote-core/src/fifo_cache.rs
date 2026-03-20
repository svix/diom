use std::time::{Duration, Instant};

pub struct FifoCache<V> {
    map: indexmap::IndexMap<String, (Instant, V)>,
    capacity: usize,
}

impl<V> FifoCache<V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            map: indexmap::IndexMap::new(),
            capacity,
        }
    }

    pub fn get(&self, key: &str, ttl: Duration) -> Option<&V> {
        self.map
            .get(key)
            .filter(|(inserted_at, _)| inserted_at.elapsed() < ttl)
            .map(|(_, v)| v)
    }

    pub fn put(&mut self, key: String, value: V) {
        self.map.insert(key, (Instant::now(), value));
        if self.map.len() > self.capacity {
            self.map.shift_remove_index(0);
        }
    }
}
