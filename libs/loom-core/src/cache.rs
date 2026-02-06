use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Configuration for cache behavior
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries (None = unlimited)
    pub capacity: usize,

    /// Time-to-live for entries (None = no expiry)
    pub ttl: Option<Duration>,
}

impl CacheConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            capacity: 100,
            ttl: None,
        }
    }
}

/// Entry stored in the cache
struct CacheEntry<V> {
    value: V,
    inserted_at: Instant,
}

/// Thread-safe cache with TTL and size limits
///
/// # Example
///
/// ```
/// use std::time::Duration;
/// use loom_core::{Cache, CacheConfig};
///
/// let cache: Cache<String, i32> = Cache::new(
///     CacheConfig::new()
///         .with_max_entries(100)
///         .with_ttl(Duration::from_secs(60))
/// );
///
/// cache.insert("key".to_string(), 42);
/// assert_eq!(cache.get(&"key".to_string()), Some(42));
/// ```
pub struct Cache<K, V> {
    entries: RwLock<HashMap<K, CacheEntry<V>>>,
    config: CacheConfig,
}

impl<K: Eq + Hash, V: Clone> Cache<K, V> {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            config,
        }
    }

    /// Get value if present and not expired
    pub fn get(&self, key: &K) -> Option<V> {
        let entries = self.entries.read().ok()?;
        let entry = entries.get(key)?;

        // Check TTL
        if let Some(ttl) = self.config.ttl {
            if entry.inserted_at.elapsed() > ttl {
                return None; // Expired
            }
        }

        Some(entry.value.clone())
    }

    /// Check if key exists and is not expired
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Insert value, evicting oldest if at capacity
    pub fn insert(&self, key: K, value: V)
    where
        K: Clone,
    {
        let mut entries = match self.entries.write() {
            Ok(e) => e,
            Err(_) => return,
        };

        // Evict if at capacity
        if entries.len() >= self.config.capacity && !entries.contains_key(&key) {
            // Remove oldest entry
            if let Some(oldest_key) = Self::find_oldest(&entries) {
                entries.remove(&oldest_key);
            }
        }

        entries.insert(
            key,
            CacheEntry {
                value,
                inserted_at: Instant::now(),
            },
        );
    }

    /// Get existing or compute and insert
    pub fn get_or_insert_with<F>(&self, key: K, f: F) -> V
    where
        F: FnOnce() -> V,
        K: Clone,
    {
        if let Some(v) = self.get(&key) {
            return v;
        }

        let value = f();
        self.insert(key, value.clone());
        value
    }

    /// Get existing or try to compute and insert
    pub fn get_or_try_insert_with<F, E>(&self, key: K, f: F) -> Result<V, E>
    where
        F: FnOnce() -> Result<V, E>,
        K: Clone,
    {
        if let Some(v) = self.get(&key) {
            return Ok(v);
        }

        let value = f()?;
        self.insert(key, value.clone());
        Ok(value)
    }

    /// Remove entry
    pub fn remove(&self, key: &K) -> Option<V> {
        self.entries.write().ok()?.remove(key).map(|e| e.value)
    }

    /// Clear all entries
    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.write() {
            entries.clear();
        }
    }

    /// Remove expired entries
    pub fn evict_expired(&self) {
        if self.config.ttl.is_none() {
            return;
        }

        let ttl = self.config.ttl.unwrap();
        if let Ok(mut entries) = self.entries.write() {
            entries.retain(|_, entry| entry.inserted_at.elapsed() <= ttl);
        }
    }

    /// Number of entries (including possibly expired)
    pub fn len(&self) -> usize {
        self.entries.read().map(|e| e.len()).unwrap_or(0)
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get cache configuration
    pub fn config(&self) -> &CacheConfig {
        &self.config
    }

    fn find_oldest(entries: &HashMap<K, CacheEntry<V>>) -> Option<K>
    where
        K: Clone,
    {
        entries
            .iter()
            .min_by_key(|(_, e)| e.inserted_at)
            .map(|(k, _)| k.clone())
    }
}

impl<K: Eq + Hash, V: Clone> Default for Cache<K, V> {
    fn default() -> Self {
        Self::new(CacheConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn insert_and_get() {
        let cache: Cache<String, i32> = Cache::default();

        cache.insert("key".to_string(), 42);
        assert_eq!(cache.get(&"key".to_string()), Some(42));
    }

    #[test]
    fn get_missing_returns_none() {
        let cache: Cache<String, i32> = Cache::default();
        assert_eq!(cache.get(&"missing".to_string()), None);
    }

    #[test]
    fn ttl_expiration() {
        let cache: Cache<String, i32> =
            Cache::new(CacheConfig::new().with_ttl(Duration::from_millis(50)));

        cache.insert("key".to_string(), 42);
        assert_eq!(cache.get(&"key".to_string()), Some(42));

        thread::sleep(Duration::from_millis(60));
        assert_eq!(cache.get(&"key".to_string()), None);
    }

    #[test]
    fn max_entries_eviction() {
        let cache: Cache<i32, i32> = Cache::new(CacheConfig::new().with_capacity(2));

        cache.insert(1, 10);
        thread::sleep(Duration::from_millis(1));
        cache.insert(2, 20);
        thread::sleep(Duration::from_millis(1));
        cache.insert(3, 30); // Should evict key 1 (oldest)

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(20));
        assert_eq!(cache.get(&3), Some(30));
    }

    #[test]
    fn get_or_insert_with_cache_hit() {
        let cache: Cache<String, i32> = Cache::default();
        cache.insert("key".to_string(), 42);

        let mut called = false;
        let value = cache.get_or_insert_with("key".to_string(), || {
            called = true;
            99
        });

        assert_eq!(value, 42);
        assert!(!called);
    }

    #[test]
    fn get_or_insert_with_cache_miss() {
        let cache: Cache<String, i32> = Cache::default();

        let value = cache.get_or_insert_with("key".to_string(), || 42);

        assert_eq!(value, 42);
        assert_eq!(cache.get(&"key".to_string()), Some(42));
    }

    #[test]
    fn get_or_try_insert_with_success() {
        let cache: Cache<String, i32> = Cache::default();

        let result: Result<i32, &str> = cache.get_or_try_insert_with("key".to_string(), || Ok(42));

        assert_eq!(result, Ok(42));
        assert_eq!(cache.get(&"key".to_string()), Some(42));
    }

    #[test]
    fn get_or_try_insert_with_error() {
        let cache: Cache<String, i32> = Cache::default();

        let result: Result<i32, &str> =
            cache.get_or_try_insert_with("key".to_string(), || Err("failed"));

        assert_eq!(result, Err("failed"));
        assert_eq!(cache.get(&"key".to_string()), None);
    }

    #[test]
    fn remove_entry() {
        let cache: Cache<String, i32> = Cache::default();

        cache.insert("key".to_string(), 42);
        assert_eq!(cache.remove(&"key".to_string()), Some(42));
        assert_eq!(cache.get(&"key".to_string()), None);
    }

    #[test]
    fn clear_entries() {
        let cache: Cache<String, i32> = Cache::default();

        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        assert_eq!(cache.len(), 2);

        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn evict_expired() {
        let cache: Cache<String, i32> =
            Cache::new(CacheConfig::new().with_ttl(Duration::from_millis(50)));

        cache.insert("key".to_string(), 42);
        assert_eq!(cache.len(), 1);

        thread::sleep(Duration::from_millis(60));
        cache.evict_expired();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn contains_key() {
        let cache: Cache<String, i32> = Cache::default();

        assert!(!cache.contains(&"key".to_string()));
        cache.insert("key".to_string(), 42);
        assert!(cache.contains(&"key".to_string()));
    }

    #[test]
    fn concurrent_access() {
        use std::sync::Arc;

        let cache = Arc::new(Cache::<i32, i32>::default());
        let mut handles = vec![];

        // Spawn multiple threads that read and write
        for i in 0..10 {
            let cache = cache.clone();
            handles.push(thread::spawn(move || {
                for j in 0..100 {
                    cache.insert(i * 100 + j, j);
                    cache.get(&(i * 100 + j));
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert!(cache.len() > 0);
    }

    #[test]
    fn update_existing_key_resets_timestamp() {
        let cache: Cache<String, i32> =
            Cache::new(CacheConfig::new().with_ttl(Duration::from_millis(100)));

        cache.insert("key".to_string(), 1);
        thread::sleep(Duration::from_millis(60));

        // Update resets the timestamp
        cache.insert("key".to_string(), 2);
        thread::sleep(Duration::from_millis(60));

        // Should still be valid because we updated it
        assert_eq!(cache.get(&"key".to_string()), Some(2));
    }
}
