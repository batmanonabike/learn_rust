# Challenge 4: Generic Cache System

**Difficulty:** 🟠 Advanced  
**Topics:** Generics, Lifetimes, Traits, Smart Pointers, Thread Safety

## The Challenge

Build a flexible, generic caching system that can store any type of data with configurable eviction policies. This challenge will test your understanding of generics, lifetimes, and advanced Rust concepts.

## Requirements

Create a caching system that:

1. **Stores any type** that implements specific traits
2. **Supports multiple eviction policies** (LRU, FIFO, TTL)
3. **Provides thread-safe access** for concurrent usage
4. **Tracks cache statistics** (hits, misses, evictions)
5. **Implements custom serialization** for persistence

### Core Structures

```rust
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

trait Cacheable: Clone + Send + Sync + 'static {}

#[derive(Debug, Clone)]
enum EvictionPolicy {
    LRU(usize),           // Least Recently Used with max size
    FIFO(usize),          // First In, First Out with max size
    TTL(Duration),        // Time To Live
    None,                 // No eviction
}

#[derive(Debug, Clone, Default)]
struct CacheStats {
    hits: u64,
    misses: u64,
    evictions: u64,
    total_requests: u64,
}

struct Cache<K, V> 
where 
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Cacheable,
{
    // Implementation details
}
```

### Required Methods

1. **Cache creation and configuration:**
   ```rust
   fn new(policy: EvictionPolicy) -> Self
   fn with_capacity(capacity: usize, policy: EvictionPolicy) -> Self
   ```

2. **Basic operations:**
   ```rust
   fn get(&self, key: &K) -> Option<V>
   fn put(&self, key: K, value: V) -> Option<V>  // Returns old value if existed
   fn remove(&self, key: &K) -> Option<V>
   fn contains_key(&self, key: &K) -> bool
   fn clear(&self)
   ```

3. **Advanced operations:**
   ```rust
   fn get_or_insert_with<F>(&self, key: K, f: F) -> V 
   where F: FnOnce() -> V
   
   fn get_with_expiry(&self, key: &K) -> Option<(V, Instant)>
   fn update_if_present<F>(&self, key: &K, f: F) -> bool
   where F: FnOnce(&mut V)
   ```

4. **Statistics and monitoring:**
   ```rust
   fn stats(&self) -> CacheStats
   fn hit_rate(&self) -> f64
   fn size(&self) -> usize
   fn capacity(&self) -> Option<usize>
   ```

5. **Thread-safe wrapper:**
   ```rust
   fn into_shared(self) -> SharedCache<K, V>
   ```

### Example Usage

```rust
use std::thread;
use std::time::Duration;

// Implement Cacheable for custom types
#[derive(Clone, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

impl Cacheable for User {}

fn main() {
    // Create a cache with LRU eviction
    let cache = Cache::new(EvictionPolicy::LRU(100));
    
    // Basic operations
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    
    cache.put(1, user.clone());
    
    if let Some(cached_user) = cache.get(&1) {
        println!("Found user: {:?}", cached_user);
    }
    
    // Thread-safe usage
    let shared_cache = cache.into_shared();
    let cache_clone = shared_cache.clone();
    
    thread::spawn(move || {
        cache_clone.put(2, User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        });
    });
    
    // Cache with TTL
    let ttl_cache = Cache::new(EvictionPolicy::TTL(Duration::from_secs(60)));
    
    // Statistics
    println!("Cache stats: {:?}", shared_cache.stats());
    println!("Hit rate: {:.2}%", shared_cache.hit_rate() * 100.0);
}
```

## Learning Objectives

- Master complex generic type constraints
- Understand lifetime parameters and bounds
- Implement thread-safe data structures
- Work with smart pointers (`Arc`, `Mutex`)
- Create flexible API designs with traits
- Handle time-based operations and TTL

## Advanced Features to Implement

1. **Custom eviction policies:**
   ```rust
   trait EvictionStrategy<K, V> {
       fn should_evict(&self, cache_size: usize) -> bool;
       fn select_victim(&self, entries: &HashMap<K, CacheEntry<V>>) -> Option<K>;
   }
   ```

2. **Persistence support:**
   ```rust
   fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), CacheError>
   fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, CacheError>
   ```

3. **Cache warming:**
   ```rust
   fn warm_up<I, F>(&self, keys: I, loader: F) 
   where 
       I: Iterator<Item = K>,
       F: Fn(&K) -> Option<V>
   ```

## Hints

<details>
<summary>Click to see hints</summary>

1. **Thread safety:** Use `Arc<Mutex<HashMap<K, CacheEntry<V>>>>` for internal storage

2. **LRU implementation:** Track access order with timestamps or a separate order structure

3. **TTL handling:** Store `Instant` with each entry and check on access

4. **Generic constraints:** Use `where` clauses for complex bounds

5. **Statistics:** Use `AtomicU64` for thread-safe counters

Example cache entry:
```rust
struct CacheEntry<V> {
    value: V,
    created_at: Instant,
    last_accessed: Instant,
    access_count: u64,
}
```

</details>

## Bonus Challenges

1. **Async support:** Create an async version using `tokio::sync::RwLock`
2. **Memory-mapped persistence:** Use memory-mapped files for large caches
3. **Compression:** Compress values automatically for large objects
4. **Cache hierarchies:** Implement L1/L2 cache levels
5. **Metrics integration:** Add Prometheus metrics support
6. **Distributed caching:** Network-aware cache with Redis backend

## Testing Requirements

- Concurrent access safety
- Eviction policy correctness
- TTL expiration handling
- Statistics accuracy
- Memory leak prevention

## Next Steps

After completing this challenge, move on to `challenge_05_concurrent_downloader` to practice async programming and networking!
