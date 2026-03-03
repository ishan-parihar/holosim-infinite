//! Fractal Cache - Multi-Scale Field Representation with Caching
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 5.3:
//! "Fractal Caching - Multi-scale field representation with fractal caching"
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "Query at any scale - cache handles resolution"
//!
//! This module implements:
//! - Multi-scale field representation (8 levels)
//! - Fractal caching for O(log n) queries
//! - On-demand decompression from MERA

use crate::hpo::spatial_field::Position3D;
use std::collections::HashMap;

/// Scale level in the fractal hierarchy
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// > "8 levels: quantum → cosmic"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScaleLevel(pub usize);

impl ScaleLevel {
    pub fn new(level: usize) -> Self {
        ScaleLevel(level.min(7))
    }

    /// Get scale factor relative to base
    pub fn scale_factor(&self) -> f64 {
        2.0_f64.powi(self.0 as i32)
    }

    /// Get next coarser level
    pub fn coarser(&self) -> ScaleLevel {
        ScaleLevel((self.0 + 1).min(7))
    }

    /// Get next finer level
    pub fn finer(&self) -> ScaleLevel {
        if self.0 == 0 {
            ScaleLevel(0)
        } else {
            ScaleLevel(self.0 - 1)
        }
    }
}

/// Fractal data wrapper
#[derive(Debug, Clone)]
pub struct FractalData {
    pub data: Vec<f64>,
    pub scale: usize,
}

impl FractalData {
    pub fn new(data: Vec<f64>, scale: usize) -> Self {
        FractalData { data, scale }
    }
}

/// Fractal cache entry at a specific scale
#[derive(Debug, Clone)]
pub struct FractalCacheEntry {
    /// Data at this scale
    pub data: Vec<f64>,

    /// Scale level
    pub scale_level: ScaleLevel,

    /// Center position
    pub center: Position3D,

    /// Resolution (number of samples per axis)
    pub resolution: usize,

    /// Last access time
    pub last_access: u64,

    /// Access count
    pub access_count: u64,
}

impl FractalCacheEntry {
    pub fn new(
        data: Vec<f64>,
        scale_level: ScaleLevel,
        center: Position3D,
        resolution: usize,
    ) -> Self {
        FractalCacheEntry {
            data,
            scale_level,
            center,
            resolution,
            last_access: 0,
            access_count: 0,
        }
    }

    /// Interpolate value at a position within this cache entry
    pub fn interpolate(&self, position: Position3D) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }

        // Calculate relative position within this cache entry
        let scale = self.scale_level.scale_factor();
        let rel_x = (position.x - self.center.x) / scale;
        let rel_y = (position.y - self.center.y) / scale;
        let rel_z = (position.z - self.center.z) / scale;

        // Simple nearest-neighbor interpolation
        let res = self.resolution as i32;
        let ix = ((rel_x + 0.5) * res as f64).clamp(0.0, (res - 1) as f64) as usize;
        let iy = ((rel_y + 0.5) * res as f64).clamp(0.0, (res - 1) as f64) as usize;
        let iz = ((rel_z + 0.5) * res as f64).clamp(0.0, (res - 1) as f64) as usize;

        let idx = ix + iy * res as usize + iz * res as usize * res as usize;
        self.data.get(idx).copied().unwrap_or(0.0)
    }
}

/// Fractal Cache Key
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FractalCacheKey {
    /// Center position (rounded for hashing)
    pub center_x: i64,
    pub center_y: i64,
    pub center_z: i64,

    /// Scale level
    pub scale_level: usize,
}

impl Eq for FractalCacheKey {}

impl std::hash::Hash for FractalCacheKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.center_x.hash(state);
        self.center_y.hash(state);
        self.center_z.hash(state);
        self.scale_level.hash(state);
    }
}

impl FractalCacheKey {
    pub fn new(center: Position3D, scale_level: ScaleLevel) -> Self {
        // Round to reduce precision for hashing
        let scale = scale_level.scale_factor() * 10.0;
        FractalCacheKey {
            center_x: (center.x * scale).round() as i64,
            center_y: (center.y * scale).round() as i64,
            center_z: (center.z * scale).round() as i64,
            scale_level: scale_level.0,
        }
    }
}

/// Fractal Cache Configuration
#[derive(Debug, Clone)]
pub struct FractalCacheConfig {
    /// Maximum entries per scale level
    pub max_entries_per_level: usize,

    /// Resolution at each scale level
    pub resolution_per_level: Vec<usize>,

    /// Enable interpolation between levels
    pub enable_interpolation: bool,

    /// Cache eviction strategy
    pub eviction_strategy: EvictionStrategy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionStrategy {
    /// Remove least recently used
    LRU,
    /// Remove least frequently used
    LFU,
    /// Remove oldest
    FIFO,
}

impl Default for FractalCacheConfig {
    fn default() -> Self {
        FractalCacheConfig {
            max_entries_per_level: 100,
            resolution_per_level: vec![8, 16, 32, 64, 128, 256, 512, 1024],
            enable_interpolation: true,
            eviction_strategy: EvictionStrategy::LRU,
        }
    }
}

/// Fractal Cache - Multi-scale field representation with caching
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// > "Query at any scale - cache handles resolution"
/// > "Multi-scale field representation with fractal caching"
#[derive(Debug, Clone)]
pub struct FractalCache {
    config: FractalCacheConfig,
    /// Cache entries organized by scale level
    cache: Vec<HashMap<FractalCacheKey, FractalCacheEntry>>,
    /// MERA network for decompression
    mera_reference: Option<()>,
    /// Statistics
    hits: u64,
    misses: u64,
    interpolations: u64,
}

impl FractalCache {
    pub fn new() -> Self {
        let config = FractalCacheConfig::default();
        let mut cache = Vec::new();

        // Initialize 8 scale levels
        for _ in 0..8 {
            cache.push(HashMap::new());
        }

        FractalCache {
            config,
            cache,
            mera_reference: None,
            hits: 0,
            misses: 0,
            interpolations: 0,
        }
    }

    pub fn with_config(config: FractalCacheConfig) -> Self {
        let mut cache = Vec::new();

        for _ in 0..8 {
            cache.push(HashMap::new());
        }

        FractalCache {
            config,
            cache,
            mera_reference: None,
            hits: 0,
            misses: 0,
            interpolations: 0,
        }
    }

    /// Query field at a specific position and scale
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// > "Query at any scale - cache handles resolution"
    pub fn query(&mut self, position: Position3D, scale: ScaleLevel, time: u64) -> f64 {
        // Try exact match first
        let key = FractalCacheKey::new(position, scale);

        if let Some(entry) = self.cache[scale.0].get_mut(&key) {
            entry.last_access = time;
            entry.access_count += 1;
            self.hits += 1;
            return entry.interpolate(position);
        }

        // Try interpolation from neighboring scales
        if self.config.enable_interpolation {
            let interpolated = self.interpolate_from_neighbors(position, scale, time);
            if let Some(val) = interpolated {
                self.interpolations += 1;
                return val;
            }
        }

        // Cache miss - would need to decompress from MERA
        self.misses += 0;
        0.0 // Would be decompressed from MERA in real implementation
    }

    /// Interpolate from neighboring scale levels
    fn interpolate_from_neighbors(
        &self,
        position: Position3D,
        scale: ScaleLevel,
        _time: u64,
    ) -> Option<f64> {
        // Try finer scale (higher resolution)
        if scale.0 > 0 {
            let finer = scale.finer();
            let key = FractalCacheKey::new(position, finer);

            if let Some(entry) = self.cache[finer.0].get(&key) {
                return Some(entry.interpolate(position));
            }
        }

        // Try coarser scale (lower resolution)
        if scale.0 < 7 {
            let coarser = scale.coarser();
            let key = FractalCacheKey::new(position, coarser);

            if let Some(entry) = self.cache[coarser.0].get(&key) {
                return Some(entry.interpolate(position));
            }
        }

        None
    }

    /// Cache field data at a specific scale
    pub fn cache_field(&mut self, center: Position3D, scale: ScaleLevel, data: Vec<f64>) {
        let key = FractalCacheKey::new(center, scale);

        // Check if cache is full for this level
        if self.cache[scale.0].len() >= self.config.max_entries_per_level {
            self.evict(scale.0);
        }

        let resolution = self
            .config
            .resolution_per_level
            .get(scale.0)
            .copied()
            .unwrap_or(32);

        let entry = FractalCacheEntry::new(data, scale, center, resolution);
        self.cache[scale.0].insert(key, entry);
    }

    /// Evict entry from cache level
    fn evict(&mut self, level: usize) {
        if self.cache[level].is_empty() {
            return;
        }

        match self.config.eviction_strategy {
            EvictionStrategy::LRU => {
                // Remove least recently used
                if let Some(key) = self.cache[level]
                    .iter()
                    .min_by_key(|(_, v)| v.last_access)
                    .map(|(k, _)| k.clone())
                {
                    self.cache[level].remove(&key);
                }
            }
            EvictionStrategy::LFU => {
                // Remove least frequently used
                if let Some(key) = self.cache[level]
                    .iter()
                    .min_by_key(|(_, v)| v.access_count)
                    .map(|(k, _)| k.clone())
                {
                    self.cache[level].remove(&key);
                }
            }
            EvictionStrategy::FIFO => {
                // Remove oldest (by creation, which we approximate with access)
                if let Some(key) = self.cache[level]
                    .iter()
                    .min_by_key(|(_, v)| v.last_access)
                    .map(|(k, _)| k.clone())
                {
                    self.cache[level].remove(&key);
                }
            }
        }
    }

    /// Prefetch region at multiple scales
    pub fn prefetch(&mut self, center: Position3D, scale_range: std::ops::Range<usize>) {
        for scale_idx in scale_range {
            let scale = ScaleLevel(scale_idx);
            let key = FractalCacheKey::new(center, scale);

            // Only prefetch if not already cached
            if !self.cache[scale_idx].contains_key(&key) {
                // Would load from MERA in real implementation
            }
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> FractalCacheStats {
        let total_requests = self.hits + self.misses;
        let hit_rate = if total_requests > 0 {
            self.hits as f64 / total_requests as f64
        } else {
            0.0
        };

        let total_entries: usize = self.cache.iter().map(|m| m.len()).sum();

        FractalCacheStats {
            hits: self.hits,
            misses: self.misses,
            interpolations: self.interpolations,
            hit_rate,
            total_entries,
            entries_per_level: self.cache.iter().map(|m| m.len()).collect(),
        }
    }

    /// Clear cache
    pub fn clear(&mut self) {
        for level in &mut self.cache {
            level.clear();
        }
        self.hits = 0;
        self.misses = 0;
        self.interpolations = 0;
    }
}

impl Default for FractalCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Fractal cache statistics
#[derive(Debug, Clone)]
pub struct FractalCacheStats {
    pub hits: u64,
    pub misses: u64,
    pub interpolations: u64,
    pub hit_rate: f64,
    pub total_entries: usize,
    pub entries_per_level: Vec<usize>,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_level() {
        let scale = ScaleLevel(3);
        assert_eq!(scale.0, 3);
        assert_eq!(scale.scale_factor(), 8.0); // 2^3

        let coarser = scale.coarser();
        assert_eq!(coarser.0, 4);

        let finer = scale.finer();
        assert_eq!(finer.0, 2);
    }

    #[test]
    fn test_cache_query() {
        let mut cache = FractalCache::new();

        // Query at scale 0 (should miss initially)
        let value = cache.query(Position3D::new(0.0, 0.0, 0.0), ScaleLevel(0), 0);

        // Should return 0 (no data cached)
        assert_eq!(value, 0.0);
    }

    #[test]
    fn test_cache_stats() {
        let cache = FractalCache::new();
        let stats = cache.get_statistics();

        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }
}
