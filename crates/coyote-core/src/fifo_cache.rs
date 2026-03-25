use std::{
    collections::{HashMap, VecDeque},
    time::{Duration, Instant},
};

pub struct FifoCache<V> {
    map: HashMap<String, (Instant, V)>,
    order: VecDeque<(String, Instant)>,
    capacity: usize,
}

impl<V> FifoCache<V> {
    /// Creates a new `FifoCache` that holds at most `capacity` entries.
    ///
    /// The goal was low maintenance, so it doesn't clear stale keys on its own, but instead only
    /// clears keys once capacity is reached (and only on insert).
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::new(),
            capacity,
        }
    }

    /// Returns the value for `key` if it exists and was inserted within `ttl` ago.
    /// Returns `None` if the key is absent or the entry has expired.
    pub fn get(&self, key: &str, ttl: Duration) -> Option<&V> {
        self.map
            .get(key)
            .filter(|(inserted_at, _)| inserted_at.elapsed() < ttl)
            .map(|(_, v)| v)
    }

    /// Inserts or updates `key` with `value`, resetting its TTL and moving it to the back of the
    /// eviction queue. If the cache is at capacity, the oldest entry is evicted first.
    pub fn put(&mut self, key: String, value: V) {
        let now = Instant::now();
        self.order.push_back((key.clone(), now));
        self.map.insert(key, (now, value));

        while self.map.len() > self.capacity {
            while let Some((oldest_key, oldest_time)) = self.order.pop_front() {
                match self.map.get(&oldest_key) {
                    Some((map_time, _)) if *map_time == oldest_time => {
                        self.map.remove(&oldest_key);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_inserted_value() {
        let mut cache = FifoCache::new(10);
        cache.put("a".to_string(), 1);
        assert_eq!(cache.get("a", Duration::from_secs(60)), Some(&1));
    }

    #[test]
    fn get_returns_none_for_missing_key() {
        let cache = FifoCache::<i32>::new(10);
        assert_eq!(cache.get("missing", Duration::from_secs(60)), None);
    }

    #[test]
    fn get_returns_none_after_ttl_expires() {
        let mut cache = FifoCache::new(10);
        cache.put("a".to_string(), 1);
        assert_eq!(cache.get("a", Duration::ZERO), None);
    }

    #[test]
    fn evicts_oldest_when_at_capacity() {
        let mut cache = FifoCache::new(2);
        cache.put("a".to_string(), 1);
        cache.put("b".to_string(), 2);
        cache.put("c".to_string(), 3);
        assert_eq!(cache.get("a", Duration::from_secs(60)), None);
        assert_eq!(cache.get("b", Duration::from_secs(60)), Some(&2));
        assert_eq!(cache.get("c", Duration::from_secs(60)), Some(&3));
    }

    #[test]
    fn update_refreshes_value_and_eviction_order() {
        let mut cache = FifoCache::new(2);
        cache.put("a".to_string(), 1);
        cache.put("b".to_string(), 2);
        // Re-insert "a" — it should now be evicted last
        cache.put("a".to_string(), 10);
        cache.put("c".to_string(), 3);
        // "b" is now the oldest live entry and should be evicted
        assert_eq!(cache.get("b", Duration::from_secs(60)), None);
        assert_eq!(cache.get("a", Duration::from_secs(60)), Some(&10));
        assert_eq!(cache.get("c", Duration::from_secs(60)), Some(&3));
    }
}
