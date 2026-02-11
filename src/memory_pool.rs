//! Memory Pool Implementation
//!
//! This module provides object pooling for frequently allocated objects
//! to reduce memory allocation overhead in hot paths.
//!
//! # Purpose
//! The holographic state management creates many small objects (StateChange,
//! EnergyFlowUpdate) repeatedly during simulation. Object pooling reduces
//! allocation overhead by reusing objects instead of allocating new ones.
//!
//! # Thread Safety
//! The memory pool is thread-safe and can be used with parallel processing.

use crate::types::Float;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Statistics about memory pool usage
#[derive(Debug, Clone)]
pub struct MemoryPoolStats {
    /// Total number of objects created
    pub total_created: usize,

    /// Total number of objects acquired from pool
    pub total_acquired: usize,

    /// Total number of objects returned to pool
    pub total_returned: usize,

    /// Current number of objects in pool
    pub current_pool_size: usize,

    /// Maximum pool size reached
    pub max_pool_size: usize,

    /// Number of times pool was empty (had to create new object)
    pub pool_misses: usize,

    /// Number of times pool had object available (cache hit)
    pub pool_hits: usize,
}

impl MemoryPoolStats {
    /// Calculate pool hit rate (0.0 to 1.0)
    pub fn hit_rate(&self) -> Float {
        if self.total_acquired == 0 {
            0.0
        } else {
            self.pool_hits as Float / self.total_acquired as Float
        }
    }

    /// Calculate efficiency (how many objects were reused)
    pub fn efficiency(&self) -> Float {
        if self.total_created == 0 {
            0.0
        } else {
            self.total_returned as Float / self.total_created as Float
        }
    }
}

/// Generic memory pool for reusable objects
///
/// # Type Parameters
/// * `T` - The type of object to pool (must implement Clone)
pub struct MemoryPool<T>
where
    T: Clone,
{
    /// Pool of available objects
    pool: Arc<Mutex<VecDeque<T>>>,

    /// Factory function to create new objects
    factory: fn() -> T,

    /// Maximum pool size (prevents unbounded growth)
    max_size: usize,

    /// Statistics
    stats: Arc<Mutex<MemoryPoolStats>>,
}

impl<T> std::fmt::Debug for MemoryPool<T>
where
    T: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryPool")
            .field("max_size", &self.max_size)
            .field("stats", &self.stats)
            .finish()
    }
}

impl<T> MemoryPool<T>
where
    T: Clone,
{
    /// Create a new memory pool
    ///
    /// # Arguments
    /// * `factory` - Function to create new objects when pool is empty
    /// * `max_size` - Maximum pool size (default: 1000)
    ///
    /// # Returns
    /// * A new MemoryPool instance
    pub fn new(factory: fn() -> T, max_size: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(VecDeque::with_capacity(100))),
            factory,
            max_size,
            stats: Arc::new(Mutex::new(MemoryPoolStats {
                total_created: 0,
                total_acquired: 0,
                total_returned: 0,
                current_pool_size: 0,
                max_pool_size: 0,
                pool_misses: 0,
                pool_hits: 0,
            })),
        }
    }

    /// Create a new memory pool with default max size (1000)
    pub fn with_defaults(factory: fn() -> T) -> Self {
        Self::new(factory, 1000)
    }

    /// Acquire an object from the pool
    ///
    /// If pool has an object, returns it (cache hit).
    /// If pool is empty, creates a new object (cache miss).
    ///
    /// # Returns
    /// * An object from the pool or a newly created object
    pub fn acquire(&self) -> T {
        let mut pool = self.pool.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        stats.total_acquired += 1;

        if let Some(obj) = pool.pop_front() {
            // Cache hit
            stats.pool_hits += 1;
            stats.current_pool_size = pool.len();
            obj
        } else {
            // Cache miss - create new object
            stats.pool_misses += 1;
            stats.total_created += 1;
            drop(pool); // Release lock before calling factory
            (self.factory)()
        }
    }

    /// Return an object to the pool
    ///
    /// If pool is not at max size, the object is returned for reuse.
    /// If pool is at max size, the object is dropped.
    ///
    /// # Arguments
    /// * `obj` - The object to return
    pub fn release(&self, obj: T) {
        let mut pool = self.pool.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        if pool.len() < self.max_size {
            pool.push_back(obj);
            stats.total_returned += 1;
            stats.current_pool_size = pool.len();

            // Update max pool size if needed
            if pool.len() > stats.max_pool_size {
                stats.max_pool_size = pool.len();
            }
        }
        // If pool is full, object is dropped (not added to stats)
    }

    /// Get pool statistics
    ///
    /// # Returns
    /// * A copy of the current pool statistics
    pub fn get_stats(&self) -> MemoryPoolStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Reset pool statistics
    pub fn reset_stats(&self) {
        let mut stats = self.stats.lock().unwrap();
        *stats = MemoryPoolStats {
            total_created: 0,
            total_acquired: 0,
            total_returned: 0,
            current_pool_size: self.pool.lock().unwrap().len(),
            max_pool_size: 0,
            pool_misses: 0,
            pool_hits: 0,
        };
    }

    /// Clear the pool (remove all objects)
    ///
    /// This is useful for memory cleanup between simulations
    pub fn clear(&self) {
        let mut pool = self.pool.lock().unwrap();
        pool.clear();
        let mut stats = self.stats.lock().unwrap();
        stats.current_pool_size = 0;
    }

    /// Pre-populate the pool with objects
    ///
    /// This can improve performance by avoiding cache misses during initial warmup
    ///
    /// # Arguments
    /// * `count` - Number of objects to add to pool
    pub fn prepopulate(&self, count: usize) {
        let mut pool = self.pool.lock().unwrap();
        for _ in 0..count {
            if pool.len() < self.max_size {
                pool.push_back((self.factory)());
            }
        }

        let mut stats = self.stats.lock().unwrap();
        stats.total_created += count;
        stats.current_pool_size = pool.len();
        if pool.len() > stats.max_pool_size {
            stats.max_pool_size = pool.len();
        }
    }
}

impl<T> Clone for MemoryPool<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pool: Arc::clone(&self.pool),
            factory: self.factory,
            max_size: self.max_size,
            stats: Arc::clone(&self.stats),
        }
    }
}

/// Specialized memory pool for StateChange objects
///
/// This is a convenience wrapper around MemoryPool<StateChange>
/// that provides StateChange-specific factory and reset methods.
#[derive(Debug)]
pub struct StateChangePool {
    pool: MemoryPool<crate::fractal_holographic_structure::StateChange>,
}

impl StateChangePool {
    /// Create a new StateChange pool
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: MemoryPool::new(Self::create_state_change, max_size),
        }
    }

    /// Create a new StateChange pool with default max size (1000)
    pub fn with_defaults() -> Self {
        Self::new(1000)
    }

    /// Factory function to create a new StateChange
    fn create_state_change() -> crate::fractal_holographic_structure::StateChange {
        use crate::fractal_holographic_structure::{ChangeType, StateChange};
        StateChange {
            change_type: ChangeType::Activation,
            magnitude: 0.0,
            affected_attributes: Vec::new(),
        }
    }

    /// Acquire a StateChange from the pool
    pub fn acquire(&self) -> crate::fractal_holographic_structure::StateChange {
        self.pool.acquire()
    }

    /// Return a StateChange to the pool
    ///
    /// Note: The StateChange will be reset to default values before being returned
    pub fn release(&self, mut obj: crate::fractal_holographic_structure::StateChange) {
        // Reset object to default state before returning to pool
        obj.affected_attributes.clear();
        obj.magnitude = 0.0;
        obj.change_type = crate::fractal_holographic_structure::ChangeType::Activation;
        self.pool.release(obj);
    }

    /// Get pool statistics
    pub fn get_stats(&self) -> MemoryPoolStats {
        self.pool.get_stats()
    }

    /// Reset pool statistics
    pub fn reset_stats(&self) {
        self.pool.reset_stats();
    }

    /// Clear the pool
    pub fn clear(&self) {
        self.pool.clear();
    }

    /// Pre-populate the pool
    pub fn prepopulate(&self, count: usize) {
        self.pool.prepopulate(count);
    }
}

impl Clone for StateChangePool {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

/// Specialized memory pool for EnergyFlowUpdate objects
///
/// This is a convenience wrapper around MemoryPool<EnergyFlowUpdate>
/// that provides EnergyFlowUpdate-specific factory method.
#[derive(Debug)]
pub struct EnergyFlowUpdatePool {
    pool: MemoryPool<crate::dual_dimensional_integration::EnergyFlowUpdate>,
}

impl EnergyFlowUpdatePool {
    /// Create a new EnergyFlowUpdate pool
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: MemoryPool::new(Self::create_energy_flow_update, max_size),
        }
    }

    /// Create a new EnergyFlowUpdate pool with default max size (1000)
    pub fn with_defaults() -> Self {
        Self::new(1000)
    }

    /// Factory function to create a new EnergyFlowUpdate
    fn create_energy_flow_update() -> crate::dual_dimensional_integration::EnergyFlowUpdate {
        crate::dual_dimensional_integration::EnergyFlowUpdate::new(0.0, 0.0, 0, 1)
    }

    /// Acquire an EnergyFlowUpdate from the pool
    pub fn acquire(&self) -> crate::dual_dimensional_integration::EnergyFlowUpdate {
        self.pool.acquire()
    }

    /// Return an EnergyFlowUpdate to the pool
    pub fn release(&self, obj: crate::dual_dimensional_integration::EnergyFlowUpdate) {
        self.pool.release(obj);
    }

    /// Get pool statistics
    pub fn get_stats(&self) -> MemoryPoolStats {
        self.pool.get_stats()
    }

    /// Reset pool statistics
    pub fn reset_stats(&self) {
        self.pool.reset_stats();
    }

    /// Clear the pool
    pub fn clear(&self) {
        self.pool.clear();
    }

    /// Pre-populate the pool
    pub fn prepopulate(&self, count: usize) {
        self.pool.prepopulate(count);
    }
}

impl Clone for EnergyFlowUpdatePool {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fractal_holographic_structure::{ChangeType, StateChange};

    #[test]
    fn test_memory_pool_creation() {
        let pool = MemoryPool::new(|| 42, 10);
        assert_eq!(pool.get_stats().total_created, 0);
    }

    #[test]
    fn test_memory_pool_acquire_miss() {
        let pool = MemoryPool::new(|| 42, 10);
        let value = pool.acquire();

        assert_eq!(value, 42);
        let stats = pool.get_stats();
        assert_eq!(stats.total_acquired, 1);
        assert_eq!(stats.total_created, 1);
        assert_eq!(stats.pool_misses, 1);
        assert_eq!(stats.pool_hits, 0);
    }

    #[test]
    fn test_memory_pool_acquire_hit() {
        let pool = MemoryPool::new(|| 42, 10);

        // First acquire - cache miss
        pool.release(pool.acquire());

        // Second acquire - cache hit
        let value = pool.acquire();

        assert_eq!(value, 42);
        let stats = pool.get_stats();
        assert_eq!(stats.total_acquired, 2);
        assert_eq!(stats.total_created, 1);
        assert_eq!(stats.pool_misses, 1);
        assert_eq!(stats.pool_hits, 1);
    }

    #[test]
    fn test_memory_pool_release() {
        let pool = MemoryPool::new(|| 42, 10);

        let obj = pool.acquire();
        pool.release(obj);

        let stats = pool.get_stats();
        assert_eq!(stats.total_returned, 1);
        assert_eq!(stats.current_pool_size, 1);
    }

    #[test]
    fn test_memory_pool_max_size() {
        let pool = MemoryPool::new(|| 42, 3);

        // Pre-populate 5 objects (max is 3, so only 3 should be added)
        pool.prepopulate(5);

        let stats = pool.get_stats();
        assert_eq!(stats.current_pool_size, 3);
        assert_eq!(stats.max_pool_size, 3);
    }

    #[test]
    fn test_memory_pool_prepopulate() {
        let pool = MemoryPool::new(|| 42, 10);

        pool.prepopulate(5);

        let stats = pool.get_stats();
        assert_eq!(stats.total_created, 5);
        assert_eq!(stats.current_pool_size, 5);

        // Next 5 acquires should be cache hits
        for _ in 0..5 {
            pool.acquire();
        }

        let stats = pool.get_stats();
        assert_eq!(stats.pool_hits, 5);
        assert_eq!(stats.pool_misses, 0);
    }

    #[test]
    fn test_memory_pool_clear() {
        let pool = MemoryPool::new(|| 42, 10);

        pool.prepopulate(5);
        pool.clear();

        let stats = pool.get_stats();
        assert_eq!(stats.current_pool_size, 0);
    }

    #[test]
    fn test_memory_pool_reset_stats() {
        let pool = MemoryPool::new(|| 42, 10);

        pool.acquire();
        pool.release(pool.acquire());
        pool.reset_stats();

        let stats = pool.get_stats();
        assert_eq!(stats.total_acquired, 0);
        assert_eq!(stats.total_created, 0);
        assert_eq!(stats.total_returned, 0);
    }

    #[test]
    fn test_memory_pool_stats_hit_rate() {
        let pool = MemoryPool::new(|| 42, 10);

        // Pre-populate for cache hits
        pool.prepopulate(5);

        // Acquire 5 times (all hits)
        for _ in 0..5 {
            pool.acquire();
        }

        let stats = pool.get_stats();
        assert_eq!(stats.hit_rate(), 1.0);
    }

    #[test]
    fn test_memory_pool_stats_efficiency() {
        let pool = MemoryPool::new(|| 42, 10);

        // Pre-populate 2 objects
        pool.prepopulate(2);

        // Acquire and return both objects
        for _ in 0..2 {
            pool.release(pool.acquire());
        }

        let stats = pool.get_stats();
        assert_eq!(stats.total_created, 2);
        assert_eq!(stats.total_returned, 2);
        assert_eq!(stats.efficiency(), 1.0);
    }

    #[test]
    fn test_memory_pool_clone() {
        let pool1 = MemoryPool::new(|| 42, 10);
        pool1.release(pool1.acquire());

        let pool2 = pool1.clone();

        // Both pools share the same underlying pool
        let value = pool2.acquire();
        assert_eq!(value, 42);

        let stats = pool2.get_stats();
        assert_eq!(stats.pool_hits, 1);
    }

    #[test]
    fn test_state_change_pool() {
        let pool = StateChangePool::with_defaults();

        let mut change = pool.acquire();
        change.change_type = ChangeType::Lambda;
        change.magnitude = 0.5;
        change.affected_attributes.push("test".to_string());

        pool.release(change);

        // Acquire again - should get a reset object
        let change2 = pool.acquire();
        assert_eq!(change2.change_type, ChangeType::Activation);
        assert_eq!(change2.magnitude, 0.0);
        assert!(change2.affected_attributes.is_empty());

        let stats = pool.get_stats();
        assert_eq!(stats.pool_hits, 1);
    }

    #[test]
    fn test_state_change_pool_prepopulate() {
        let pool = StateChangePool::with_defaults();
        pool.prepopulate(10);

        let stats = pool.get_stats();
        assert_eq!(stats.total_created, 10);
        assert_eq!(stats.current_pool_size, 10);
    }

    #[test]
    fn test_energy_flow_update_pool() {
        let pool = EnergyFlowUpdatePool::with_defaults();

        let update = pool.acquire();
        pool.release(update);

        // Acquire again - should get a cached object
        let update2 = pool.acquire();

        let stats = pool.get_stats();
        assert_eq!(stats.pool_hits, 1);
    }
}
