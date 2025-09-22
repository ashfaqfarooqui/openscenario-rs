//! Performance optimization utilities for the builder system
//!
//! This module provides memory optimization, caching, and performance monitoring
//! capabilities to ensure the builder system scales efficiently for large scenarios.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::borrow::Cow;

/// Memory pool for frequently allocated objects
pub struct MemoryPool<T> {
    pool: Vec<T>,
    factory: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> MemoryPool<T> {
    /// Create a new memory pool with a factory function
    pub fn new<F>(factory: F) -> Self 
    where 
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            pool: Vec::new(),
            factory: Box::new(factory),
        }
    }

    /// Get an object from the pool or create a new one
    pub fn get(&mut self) -> T {
        self.pool.pop().unwrap_or_else(|| (self.factory)())
    }

    /// Return an object to the pool for reuse
    pub fn put(&mut self, item: T) {
        if self.pool.len() < 100 { // Limit pool size
            self.pool.push(item);
        }
    }
}

/// Copy-on-write wrapper for large data structures
#[derive(Debug, Clone)]
pub struct CowData<T> {
    data: Arc<T>,
    modified: Option<Box<T>>,
}

impl<T: Clone> CowData<T> {
    /// Create new copy-on-write data
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(data),
            modified: None,
        }
    }

    /// Get immutable reference to data
    pub fn get(&self) -> &T {
        self.modified.as_ref().map(|m| m.as_ref()).unwrap_or(&self.data)
    }

    /// Get mutable reference to data (triggers copy if shared)
    pub fn get_mut(&mut self) -> &mut T {
        if self.modified.is_none() {
            self.modified = Some(Box::new((*self.data).clone()));
        }
        self.modified.as_mut().unwrap()
    }

    /// Check if data has been modified
    pub fn is_modified(&self) -> bool {
        self.modified.is_some()
    }
}

/// Performance metrics collector
#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    build_times: Vec<Duration>,
    memory_usage: Vec<usize>,
    validation_times: Vec<Duration>,
    entity_counts: Vec<usize>,
}

impl PerformanceMetrics {
    /// Record a build operation
    pub fn record_build(&mut self, duration: Duration, entity_count: usize) {
        self.build_times.push(duration);
        self.entity_counts.push(entity_count);
    }

    /// Record memory usage
    pub fn record_memory(&mut self, bytes: usize) {
        self.memory_usage.push(bytes);
    }

    /// Record validation time
    pub fn record_validation(&mut self, duration: Duration) {
        self.validation_times.push(duration);
    }

    /// Get average build time
    pub fn avg_build_time(&self) -> Option<Duration> {
        if self.build_times.is_empty() {
            None
        } else {
            let total: Duration = self.build_times.iter().sum();
            Some(total / self.build_times.len() as u32)
        }
    }

    /// Get peak memory usage
    pub fn peak_memory(&self) -> Option<usize> {
        self.memory_usage.iter().max().copied()
    }

    /// Check if performance targets are met
    pub fn meets_targets(&self) -> bool {
        // Target: <5s for 1000 entities
        if let Some(avg_time) = self.avg_build_time() {
            if let Some(&max_entities) = self.entity_counts.iter().max() {
                if max_entities >= 1000 {
                    return avg_time < Duration::from_secs(5);
                }
            }
        }
        true // No data or small scenarios
    }
}

/// Global performance metrics instance
static METRICS: Mutex<PerformanceMetrics> = Mutex::new(PerformanceMetrics {
    build_times: Vec::new(),
    memory_usage: Vec::new(),
    validation_times: Vec::new(),
    entity_counts: Vec::new(),
});

/// Record performance metrics globally
pub fn record_build_metrics(duration: Duration, entity_count: usize) {
    if let Ok(mut metrics) = METRICS.lock() {
        metrics.record_build(duration, entity_count);
    }
}

/// Get current performance metrics
pub fn get_metrics() -> PerformanceMetrics {
    METRICS.lock().unwrap_or_else(|_| PerformanceMetrics::default()).clone()
}

/// Validation cache for expensive operations
pub struct ValidationCache {
    cache: HashMap<String, bool>,
    max_size: usize,
}

impl ValidationCache {
    /// Create new validation cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
        }
    }

    /// Get cached validation result
    pub fn get(&self, key: &str) -> Option<bool> {
        self.cache.get(key).copied()
    }

    /// Cache validation result
    pub fn put(&mut self, key: String, result: bool) {
        if self.cache.len() >= self.max_size {
            // Simple LRU: remove first entry
            if let Some(first_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&first_key);
            }
        }
        self.cache.insert(key, result);
    }

    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

/// Bulk operation helper for efficient batch processing
pub struct BulkOperations<T> {
    items: Vec<T>,
    batch_size: usize,
}

impl<T> BulkOperations<T> {
    /// Create new bulk operations helper
    pub fn new(batch_size: usize) -> Self {
        Self {
            items: Vec::with_capacity(batch_size),
            batch_size,
        }
    }

    /// Add item to batch
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    /// Process batch if full
    pub fn process_if_full<F>(&mut self, mut processor: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(&mut Vec<T>) -> Result<(), Box<dyn std::error::Error>>,
    {
        if self.items.len() >= self.batch_size {
            processor(&mut self.items)?;
            self.items.clear();
        }
        Ok(())
    }

    /// Process remaining items
    pub fn flush<F>(&mut self, mut processor: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(&mut Vec<T>) -> Result<(), Box<dyn std::error::Error>>,
    {
        if !self.items.is_empty() {
            processor(&mut self.items)?;
            self.items.clear();
        }
        Ok(())
    }
}

/// Performance timer for measuring operations
pub struct PerformanceTimer {
    start: Instant,
    name: String,
}

impl PerformanceTimer {
    /// Start timing an operation
    pub fn start(name: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            name: name.into(),
        }
    }

    /// Stop timer and return duration
    pub fn stop(self) -> Duration {
        let duration = self.start.elapsed();
        log::debug!("Operation '{}' took {:?}", self.name, duration);
        duration
    }
}

/// Lazy validation wrapper
pub struct LazyValidation<T, F> {
    data: T,
    validator: F,
    validated: bool,
    valid: bool,
}

impl<T, F> LazyValidation<T, F>
where
    F: Fn(&T) -> bool,
{
    /// Create new lazy validation wrapper
    pub fn new(data: T, validator: F) -> Self {
        Self {
            data,
            validator,
            validated: false,
            valid: false,
        }
    }

    /// Get data reference
    pub fn get(&self) -> &T {
        &self.data
    }

    /// Check if data is valid (validates on first call)
    pub fn is_valid(&mut self) -> bool {
        if !self.validated {
            self.valid = (self.validator)(&self.data);
            self.validated = true;
        }
        self.valid
    }

    /// Force revalidation
    pub fn revalidate(&mut self) -> bool {
        self.valid = (self.validator)(&self.data);
        self.validated = true;
        self.valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new(|| String::new());
        
        let item1 = pool.get();
        assert_eq!(item1, "");
        
        pool.put(String::from("reused"));
        let item2 = pool.get();
        assert_eq!(item2, "reused");
    }

    #[test]
    fn test_cow_data() {
        let mut cow = CowData::new(vec![1, 2, 3]);
        
        // Read access doesn't trigger copy
        assert_eq!(cow.get(), &vec![1, 2, 3]);
        assert!(!cow.is_modified());
        
        // Write access triggers copy
        cow.get_mut().push(4);
        assert!(cow.is_modified());
        assert_eq!(cow.get(), &vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_validation_cache() {
        let mut cache = ValidationCache::new(2);
        
        cache.put("test1".to_string(), true);
        cache.put("test2".to_string(), false);
        
        assert_eq!(cache.get("test1"), Some(true));
        assert_eq!(cache.get("test2"), Some(false));
        
        // Adding third item should evict first
        cache.put("test3".to_string(), true);
        assert_eq!(cache.get("test1"), None);
        assert_eq!(cache.get("test3"), Some(true));
    }

    #[test]
    fn test_lazy_validation() {
        let mut lazy = LazyValidation::new(42, |x| *x > 0);
        
        assert_eq!(lazy.get(), &42);
        assert!(lazy.is_valid());
        
        // Second call should use cached result
        assert!(lazy.is_valid());
    }
}