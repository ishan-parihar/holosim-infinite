//! Fractal Caching System
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4.3:
//! "Implement O(1) access multi-scale caching system"
//!
//! From MASTER_R&D_ROADMAP.md Week 17-18:
//! - Implement FractalCache with multi-scale entries
//! - Cache at multiple scales (level_0 to level_7)
//! - Implement fractal refinement (coarse to fine)
//! - Implement cache hit/miss statistics
//!
//! This module implements:
//! 1. FractalCache - O(1) access multi-scale cache
//! 2. FractalCacheEntry - 8-level fractal data storage
//! 3. FractalRefinement - Coarse-to-fine refinement logic
//! 4. FractalCacheStatistics - Hit/miss tracking
//! 5. Holographic continuity across all scales

use super::multiscale_camera::ScaleLevel;
use crate::types::Float;
use std::collections::HashMap;

/// Fractal Cache
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4.3:
/// "Implement O(1) access multi-scale caching system"
///
/// The fractal cache stores data at multiple resolution levels simultaneously,
/// enabling O(1) access time through HashMap lookups. Each cache entry contains
/// the same data at 8 different scale levels (level_0 to level_7), following
/// the holographic principle that each scale contains the whole.
///
/// Key features:
/// - O(1) access time via HashMap
/// - 8-level fractal caching (level_0 = finest, level_7 = coarsest)
/// - Fractal refinement from coarse to fine
/// - Holographic continuity across all scales
/// - Cache hit/miss statistics
pub struct FractalCache {
    /// Cache entries stored by key (O(1) access)
    entries: HashMap<FractalCacheKey, FractalCacheEntry>,

    /// Cache statistics
    stats: FractalCacheStatistics,

    /// Maximum cache size (number of entries)
    max_entries: usize,

    /// Current number of entries in cache
    current_entries: usize,

    /// Cache generation counter (for LRU eviction)
    generation: u64,
}

/// Fractal Cache Entry
///
/// From MASTER_R&D_ROADMAP.md Week 17-18:
/// "Cache at multiple scales (level_0 to level_7)"
///
/// Each cache entry contains the same data at 8 different scale levels,
/// enabling progressive refinement from coarse to fine.
///
/// Structure:
/// - level_0: Finest detail (quantum scale)
/// - level_1: Coarser (cellular scale)
/// - level_2: Even coarser (biological scale)
/// - level_3: Planetary scale
/// - level_4: Stellar scale
/// - level_5: Galactic scale
/// - level_6: Cosmic scale
/// - level_7: Universal scale (coarsest)
#[derive(Debug, Clone)]
pub struct FractalCacheEntry {
    /// Level 0: Finest detail (quantum scale)
    pub level_0: Option<FractalData>,

    /// Level 1: Coarser (cellular scale)
    pub level_1: Option<FractalData>,

    /// Level 2: Even coarser (biological scale)
    pub level_2: Option<FractalData>,

    /// Level 3: Planetary scale
    pub level_3: Option<FractalData>,

    /// Level 4: Stellar scale
    pub level_4: Option<FractalData>,

    /// Level 5: Galactic scale
    pub level_5: Option<FractalData>,

    /// Level 6: Cosmic scale
    pub level_6: Option<FractalData>,

    /// Level 7: Universal scale (coarsest)
    pub level_7: Option<FractalData>,

    /// Last access generation (for LRU eviction)
    pub last_access: u64,

    /// Access count (for LFU statistics)
    pub access_count: usize,
}

/// Fractal Data
///
/// Represents cached data at a specific scale level.
#[derive(Debug, Clone)]
pub struct FractalData {
    /// Data values
    pub data: Vec<Float>,

    /// Data shape (dimensions)
    pub shape: Vec<usize>,

    /// Quality metric (0.0 = coarse, 1.0 = finest)
    pub quality: Float,

    /// Compression ratio (original size / compressed size)
    pub compression_ratio: Float,

    /// Data size in bytes
    pub size_bytes: usize,
}

/// Fractal Cache Key
///
/// Unique identifier for cache entries.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FractalCacheKey {
    /// Entity or region identifier
    pub entity_id: u64,

    /// Scale level
    pub scale_level: usize,

    /// Position index (quantized)
    pub position_idx: (i64, i64, i64),

    /// Data type identifier
    pub data_type: u32,
}

/// Fractal Refinement
///
/// From MASTER_R&D_ROADMAP.md Week 17-18:
/// "Implement fractal refinement (coarse to fine)"
///
/// Provides progressive refinement from coarse scale data to fine scale data,
/// enabling smooth transitions and LOD (Level of Detail) management.
#[derive(Debug, Clone)]
pub struct FractalRefinement {
    /// Current refinement level (0 = coarsest, 7 = finest)
    pub current_level: usize,

    /// Target refinement level
    pub target_level: usize,

    /// Refinement progress (0.0 to 1.0)
    pub progress: Float,

    /// Refinement mode
    pub mode: RefinementMode,
}

/// Refinement mode for fractal data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefinementMode {
    /// Immediate refinement (jump to target level)
    Immediate,

    /// Progressive refinement (smooth transition)
    Progressive,

    /// Adaptive refinement (based on camera distance)
    Adaptive,
}

/// Fractal Cache Statistics
///
/// From MASTER_R&D_ROADMAP.md Week 17-18:
/// "Implement cache hit/miss statistics"
///
/// Tracks cache performance metrics for optimization.
#[derive(Debug, Clone, Default)]
pub struct FractalCacheStatistics {
    /// Total cache hits
    pub total_hits: usize,

    /// Total cache misses
    pub total_misses: usize,

    /// Cache hit rate (0.0 to 1.0)
    pub hit_rate: Float,

    /// Total bytes cached
    pub total_bytes_cached: usize,

    /// Total bytes served from cache
    pub total_bytes_served: usize,

    /// Average access time (nanoseconds)
    pub average_access_time_ns: Float,

    /// Number of evictions
    pub evictions: usize,

    /// Number of cache invalidations
    pub invalidations: usize,

    /// Peak cache size (entries)
    pub peak_entries: usize,

    /// Current cache size (entries)
    pub current_entries: usize,
}

/// Fractal Cache Error
#[derive(Debug, Clone, PartialEq)]
pub enum FractalCacheError {
    /// Invalid scale level
    InvalidScaleLevel(usize),

    /// Cache entry not found
    EntryNotFound(FractalCacheKey),

    /// Cache full
    CacheFull,

    /// Invalid data
    InvalidData,
}

/// Eviction Policy
///
/// Strategy for evicting cache entries when cache is full.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least Recently Used
    Lru,

    /// Least Frequently Used
    Lfu,

    /// Adaptive (combines LRU and LFU)
    Adaptive,

    /// Scale-aware (evict entries from distant scales first)
    ScaleAware,
}

impl FractalCache {
    /// Create a new fractal cache
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4.3:
    /// "Implement O(1) access multi-scale caching system"
    ///
    /// # Arguments
    /// * `max_entries` - Maximum number of cache entries
    pub fn new(max_entries: usize) -> Self {
        FractalCache {
            entries: HashMap::new(),
            stats: FractalCacheStatistics::default(),
            max_entries,
            current_entries: 0,
            generation: 0,
        }
    }

    /// Insert data into the cache at a specific scale level
    ///
    /// This is an O(1) operation thanks to HashMap storage.
    ///
    /// # Arguments
    /// * `key` - Cache key
    /// * `level` - Scale level (0-7)
    /// * `data` - Fractal data to cache
    pub fn insert(
        &mut self,
        key: FractalCacheKey,
        level: usize,
        data: FractalData,
    ) -> Result<(), FractalCacheError> {
        if level > 7 {
            return Err(FractalCacheError::InvalidScaleLevel(level));
        }

        if data.data.is_empty() || data.shape.is_empty() {
            return Err(FractalCacheError::InvalidData);
        }

        self.generation += 1;

        let size_bytes = data.size_bytes;
        let mut entry = if let Some(mut existing) = self.entries.remove(&key) {
            existing.last_access = self.generation;
            existing.access_count += 1;
            existing
        } else {
            if self.current_entries >= self.max_entries {
                self.evict_lru();
            }
            self.current_entries += 1;
            self.stats.total_bytes_cached += size_bytes;

            FractalCacheEntry {
                level_0: None,
                level_1: None,
                level_2: None,
                level_3: None,
                level_4: None,
                level_5: None,
                level_6: None,
                level_7: None,
                last_access: self.generation,
                access_count: 1,
            }
        };

        match level {
            0 => entry.level_0 = Some(data),
            1 => entry.level_1 = Some(data),
            2 => entry.level_2 = Some(data),
            3 => entry.level_3 = Some(data),
            4 => entry.level_4 = Some(data),
            5 => entry.level_5 = Some(data),
            6 => entry.level_6 = Some(data),
            7 => entry.level_7 = Some(data),
            _ => return Err(FractalCacheError::InvalidScaleLevel(level)),
        }

        self.entries.insert(key, entry);

        if self.current_entries > self.stats.peak_entries {
            self.stats.peak_entries = self.current_entries;
        }
        self.stats.current_entries = self.current_entries;

        Ok(())
    }

    /// Retrieve data from the cache at a specific scale level
    ///
    /// This is an O(1) operation thanks to HashMap storage.
    ///
    /// # Arguments
    /// * `key` - Cache key
    /// * `level` - Scale level (0-7)
    pub fn get(
        &mut self,
        key: &FractalCacheKey,
        level: usize,
    ) -> Result<Option<FractalData>, FractalCacheError> {
        if level > 7 {
            return Err(FractalCacheError::InvalidScaleLevel(level));
        }

        let start_time = std::time::Instant::now();

        if let Some(entry) = self.entries.get_mut(key) {
            entry.last_access = self.generation;
            entry.access_count += 1;

            let data = match level {
                0 => entry.level_0.clone(),
                1 => entry.level_1.clone(),
                2 => entry.level_2.clone(),
                3 => entry.level_3.clone(),
                4 => entry.level_4.clone(),
                5 => entry.level_5.clone(),
                6 => entry.level_6.clone(),
                7 => entry.level_7.clone(),
                _ => return Err(FractalCacheError::InvalidScaleLevel(level)),
            };

            self.stats.total_hits += 1;
            if let Some(ref d) = data {
                self.stats.total_bytes_served += d.size_bytes;
            }
            self.update_hit_rate();

            let elapsed = start_time.elapsed();
            let elapsed_ns = elapsed.as_nanos() as Float;
            let total_time =
                self.stats.average_access_time_ns * (self.stats.total_hits - 1) as Float;
            self.stats.average_access_time_ns =
                (total_time + elapsed_ns) / self.stats.total_hits as Float;

            Ok(data)
        } else {
            self.stats.total_misses += 1;
            self.update_hit_rate();
            Ok(None)
        }
    }

    /// Get data with fractal refinement (coarse to fine)
    ///
    /// From MASTER_R&D_ROADMAP.md Week 17-18:
    /// "Implement fractal refinement (coarse to fine)"
    ///
    /// # Arguments
    /// * `key` - Cache key
    /// * `refinement` - Refinement specification
    pub fn get_with_refinement(
        &mut self,
        key: &FractalCacheKey,
        refinement: &FractalRefinement,
    ) -> Result<Option<FractalData>, FractalCacheError> {
        let target_level = refinement.target_level;

        if target_level > 7 {
            return Err(FractalCacheError::InvalidScaleLevel(target_level));
        }

        match refinement.mode {
            RefinementMode::Immediate => self.get(key, target_level),

            RefinementMode::Progressive => {
                let current_level = refinement.current_level;

                if current_level > target_level {
                    self.get(key, target_level)
                } else if let Some(data) = self.get(key, current_level)? {
                    if current_level == target_level {
                        Ok(Some(data))
                    } else {
                        self.interpolate_data(
                            data,
                            current_level,
                            target_level,
                            refinement.progress,
                        )
                    }
                } else {
                    Ok(None)
                }
            }

            RefinementMode::Adaptive => {
                let adaptive_level = if refinement.progress < 0.33 {
                    target_level.min(3)
                } else if refinement.progress < 0.66 {
                    target_level.min(5)
                } else {
                    target_level
                };
                self.get(key, adaptive_level)
            }
        }
    }

    /// Invalidate a cache entry
    ///
    /// # Arguments
    /// * `key` - Cache key to invalidate
    pub fn invalidate(&mut self, key: &FractalCacheKey) {
        if self.entries.remove(key).is_some() {
            self.current_entries -= 1;
            self.stats.invalidations += 1;
            self.stats.current_entries = self.current_entries;
        }
    }

    /// Clear all cache entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_entries = 0;
        self.stats.current_entries = 0;
        self.stats.total_bytes_cached = 0;
    }

    /// Get cache statistics
    pub fn statistics(&self) -> &FractalCacheStatistics {
        &self.stats
    }

    /// Get current number of entries
    pub fn len(&self) -> usize {
        self.current_entries
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.current_entries == 0
    }

    /// Get maximum number of entries
    pub fn max_entries(&self) -> usize {
        self.max_entries
    }

    /// Update hit rate
    fn update_hit_rate(&mut self) {
        let total = self.stats.total_hits + self.stats.total_misses;
        if total > 0 {
            self.stats.hit_rate = self.stats.total_hits as Float / total as Float;
        }
    }

    /// Evict least recently used entry
    fn evict_lru(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        let mut lru_key = None;
        let mut min_access = u64::MAX;

        for (key, entry) in self.entries.iter() {
            if entry.last_access < min_access {
                min_access = entry.last_access;
                lru_key = Some(key.clone());
            }
        }

        if let Some(key) = lru_key {
            self.entries.remove(&key);
            self.current_entries -= 1;
            self.stats.evictions += 1;
        }
    }

    /// Interpolate data between scale levels for progressive refinement
    fn interpolate_data(
        &self,
        data: FractalData,
        from_level: usize,
        to_level: usize,
        progress: Float,
    ) -> Result<Option<FractalData>, FractalCacheError> {
        if from_level == to_level {
            return Ok(Some(data));
        }

        let interpolated_size_factor = 2.0_f64.powi((to_level - from_level) as i32);
        let new_shape: Vec<usize> = data
            .shape
            .iter()
            .map(|&s| (s as Float * interpolated_size_factor).ceil() as usize)
            .collect();

        let mut interpolated_data = Vec::with_capacity(new_shape.iter().product());

        let progress_clamped = progress.clamp(0.0, 1.0);

        if new_shape.len() == 1 {
            for i in 0..new_shape[0] {
                let source_idx = ((i as Float) / interpolated_size_factor) as usize;
                if source_idx < data.data.len() {
                    interpolated_data.push(data.data[source_idx]);
                } else {
                    interpolated_data.push(0.0);
                }
            }
        } else if new_shape.len() == 2 {
            for j in 0..new_shape[1] {
                for i in 0..new_shape[0] {
                    let source_j = ((j as Float) / interpolated_size_factor) as usize;
                    let source_i = ((i as Float) / interpolated_size_factor) as usize;
                    let source_idx = source_j * data.shape[0] + source_i;
                    if source_idx < data.data.len() {
                        interpolated_data.push(data.data[source_idx]);
                    } else {
                        interpolated_data.push(0.0);
                    }
                }
            }
        } else {
            for i in 0..new_shape.iter().product::<usize>() {
                let source_idx =
                    (i as Float / interpolated_size_factor.powi(new_shape.len() as i32)) as usize;
                if source_idx < data.data.len() {
                    interpolated_data.push(data.data[source_idx]);
                } else {
                    interpolated_data.push(0.0);
                }
            }
        }

        let size_bytes = interpolated_data.len() * std::mem::size_of::<Float>();
        let new_quality = data.quality * progress_clamped + (1.0 - progress_clamped) * 0.5;

        Ok(Some(FractalData {
            data: interpolated_data,
            shape: new_shape,
            quality: new_quality,
            compression_ratio: data.compression_ratio,
            size_bytes,
        }))
    }

    /// Get all scale levels for a cache entry
    ///
    /// Useful for debugging and validation.
    pub fn get_all_levels(&self, key: &FractalCacheKey) -> Option<Vec<Option<FractalData>>> {
        self.entries.get(key).map(|entry| {
            vec![
                entry.level_0.clone(),
                entry.level_1.clone(),
                entry.level_2.clone(),
                entry.level_3.clone(),
                entry.level_4.clone(),
                entry.level_5.clone(),
                entry.level_6.clone(),
                entry.level_7.clone(),
            ]
        })
    }

    /// Check if a cache entry exists
    pub fn contains(&self, key: &FractalCacheKey) -> bool {
        self.entries.contains_key(key)
    }

    /// Check if cache key exists (alias for contains)
    pub fn contains_key(&self, key: &FractalCacheKey) -> bool {
        self.contains(key)
    }

    /// Check if cache is full
    pub fn is_full(&self) -> bool {
        self.current_entries >= self.max_entries
    }

    /// Evict entries based on policy
    ///
    /// Returns the number of entries evicted
    pub fn evict(&mut self, policy: &EvictionPolicy) -> usize {
        if self.entries.is_empty() {
            return 0;
        }

        match policy {
            EvictionPolicy::Lru => {
                self.evict_lru();
                1
            }
            EvictionPolicy::Lfu => {
                self.evict_lfu();
                1
            }
            EvictionPolicy::Adaptive => {
                self.evict_adaptive();
                1
            }
            EvictionPolicy::ScaleAware => {
                self.evict_scale_aware();
                1
            }
        }
    }

    /// Evict least frequently used entry
    fn evict_lfu(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        let mut lfu_key = None;
        let mut min_access = usize::MAX;

        for (key, entry) in self.entries.iter() {
            if entry.access_count < min_access {
                min_access = entry.access_count;
                lfu_key = Some(key.clone());
            }
        }

        if let Some(key) = lfu_key {
            self.entries.remove(&key);
            self.current_entries -= 1;
            self.stats.evictions += 1;
        }
    }

    /// Evict entry using adaptive policy (combination of LRU and LFU)
    fn evict_adaptive(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        let mut worst_key = None;
        let mut worst_score = f64::MAX;

        for (key, entry) in self.entries.iter() {
            let age = self.generation.saturating_sub(entry.last_access);
            let score = (age as f64) / (entry.access_count as f64 + 1.0);

            if score < worst_score {
                worst_score = score;
                worst_key = Some(key.clone());
            }
        }

        if let Some(key) = worst_key {
            self.entries.remove(&key);
            self.current_entries -= 1;
            self.stats.evictions += 1;
        }
    }

    /// Evict entry from distant scale (scale-aware eviction)
    fn evict_scale_aware(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        let current_scale_level = 3;
        let mut furthest_key = None;
        let mut max_distance = 0;

        for (key, entry) in self.entries.iter() {
            let distance = (key.scale_level as i32 - current_scale_level).unsigned_abs() as usize;

            if distance > max_distance {
                max_distance = distance;
                furthest_key = Some(key.clone());
            } else if distance == max_distance
                && entry.last_access
                    < furthest_key
                        .as_ref()
                        .and_then(|k| self.entries.get(k).map(|e| e.last_access))
                        .unwrap_or(u64::MAX)
                {
                    furthest_key = Some(key.clone());
                }
        }

        if let Some(key) = furthest_key {
            self.entries.remove(&key);
            self.current_entries -= 1;
            self.stats.evictions += 1;
        }
    }

    /// Get the number of populated levels for a cache entry
    pub fn get_populated_levels(&self, key: &FractalCacheKey) -> Option<usize> {
        self.entries.get(key).map(|entry| {
            [
                entry.level_0.is_some(),
                entry.level_1.is_some(),
                entry.level_2.is_some(),
                entry.level_3.is_some(),
                entry.level_4.is_some(),
                entry.level_5.is_some(),
                entry.level_6.is_some(),
                entry.level_7.is_some(),
            ]
            .iter()
            .filter(|&&x| x)
            .count()
        })
    }
}

impl Default for FractalCache {
    fn default() -> Self {
        FractalCache::new(1000)
    }
}

impl FractalCacheEntry {
    /// Create a new empty cache entry
    pub fn new() -> Self {
        FractalCacheEntry {
            level_0: None,
            level_1: None,
            level_2: None,
            level_3: None,
            level_4: None,
            level_5: None,
            level_6: None,
            level_7: None,
            last_access: 0,
            access_count: 0,
        }
    }

    /// Get data at a specific level
    pub fn get_level(&self, level: usize) -> Option<&FractalData> {
        match level {
            0 => self.level_0.as_ref(),
            1 => self.level_1.as_ref(),
            2 => self.level_2.as_ref(),
            3 => self.level_3.as_ref(),
            4 => self.level_4.as_ref(),
            5 => self.level_5.as_ref(),
            6 => self.level_6.as_ref(),
            7 => self.level_7.as_ref(),
            _ => None,
        }
    }

    /// Set data at a specific level
    pub fn set_level(&mut self, level: usize, data: FractalData) -> Result<(), FractalCacheError> {
        match level {
            0 => self.level_0 = Some(data),
            1 => self.level_1 = Some(data),
            2 => self.level_2 = Some(data),
            3 => self.level_3 = Some(data),
            4 => self.level_4 = Some(data),
            5 => self.level_5 = Some(data),
            6 => self.level_6 = Some(data),
            7 => self.level_7 = Some(data),
            _ => return Err(FractalCacheError::InvalidScaleLevel(level)),
        }
        Ok(())
    }

    /// Check if any level is populated
    pub fn is_populated(&self) -> bool {
        self.level_0.is_some()
            || self.level_1.is_some()
            || self.level_2.is_some()
            || self.level_3.is_some()
            || self.level_4.is_some()
            || self.level_5.is_some()
            || self.level_6.is_some()
            || self.level_7.is_some()
    }

    /// Get the finest populated level
    pub fn get_finest_level(&self) -> Option<usize> {
        for level in 0..=7 {
            if self.get_level(level).is_some() {
                return Some(level);
            }
        }
        None
    }

    /// Get the coarsest populated level
    pub fn get_coarsest_level(&self) -> Option<usize> {
        (0..=7).rev().find(|&level| self.get_level(level).is_some())
    }
}

impl Default for FractalCacheEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl FractalData {
    /// Create new fractal data
    pub fn new(data: Vec<Float>, shape: Vec<usize>) -> Self {
        let size_bytes = data.len() * std::mem::size_of::<Float>();
        FractalData {
            data,
            shape,
            quality: 1.0,
            compression_ratio: 1.0,
            size_bytes,
        }
    }

    /// Create with quality and compression ratio
    pub fn with_quality(
        data: Vec<Float>,
        shape: Vec<usize>,
        quality: Float,
        compression_ratio: Float,
    ) -> Self {
        let size_bytes = data.len() * std::mem::size_of::<Float>();
        FractalData {
            data,
            shape,
            quality: quality.clamp(0.0, 1.0),
            compression_ratio: compression_ratio.max(1.0),
            size_bytes,
        }
    }

    /// Get number of elements
    pub fn num_elements(&self) -> usize {
        self.data.len()
    }

    /// Get rank (number of dimensions)
    pub fn rank(&self) -> usize {
        self.shape.len()
    }
}

impl FractalRefinement {
    /// Create a new refinement specification
    pub fn new(current_level: usize, target_level: usize, mode: RefinementMode) -> Self {
        FractalRefinement {
            current_level,
            target_level,
            progress: 0.0,
            mode,
        }
    }

    /// Create with progress
    pub fn with_progress(
        current_level: usize,
        target_level: usize,
        progress: Float,
        mode: RefinementMode,
    ) -> Self {
        FractalRefinement {
            current_level,
            target_level,
            progress: progress.clamp(0.0, 1.0),
            mode,
        }
    }

    /// Check if refinement is complete
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0 || self.current_level == self.target_level
    }

    /// Update progress
    pub fn update_progress(&mut self, delta: Float) {
        self.progress = (self.progress + delta).clamp(0.0, 1.0);
    }

    /// Get current level based on progress
    pub fn get_current_level(&self) -> usize {
        if self.is_complete() {
            self.target_level
        } else {
            let level_range = (self.target_level - self.current_level) as Float;
            let current = self.current_level as Float + level_range * self.progress;
            current.floor() as usize
        }
    }
}

impl FractalCacheKey {
    /// Create a new cache key
    pub fn new(entity_id: u64, scale_level: usize, position_idx: (i64, i64, i64)) -> Self {
        FractalCacheKey {
            entity_id,
            scale_level,
            position_idx,
            data_type: 0,
        }
    }

    /// Create with data type
    pub fn with_data_type(
        entity_id: u64,
        scale_level: usize,
        position_idx: (i64, i64, i64),
        data_type: u32,
    ) -> Self {
        FractalCacheKey {
            entity_id,
            scale_level,
            position_idx,
            data_type,
        }
    }

    /// Create from scale level enum
    pub fn from_scale_level(
        entity_id: u64,
        scale: ScaleLevel,
        position_idx: (i64, i64, i64),
    ) -> Self {
        FractalCacheKey {
            entity_id,
            scale_level: scale.index(),
            position_idx,
            data_type: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractal_cache_creation() {
        let cache = FractalCache::new(100);
        assert_eq!(cache.max_entries(), 100);
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_fractal_cache_default() {
        let cache = FractalCache::default();
        assert_eq!(cache.max_entries(), 1000);
    }

    #[test]
    fn test_fractal_cache_insert_and_get() {
        let mut cache = FractalCache::new(100);

        let key = FractalCacheKey::new(1, 0, (100, 100, 100));
        let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);

        cache.insert(key.clone(), 0, data.clone()).unwrap();

        let result = cache.get(&key, 0).unwrap();
        assert!(result.is_some());
        let retrieved = result.unwrap();
        assert_eq!(retrieved.data, data.data);
        assert_eq!(retrieved.shape, data.shape);
    }

    #[test]
    fn test_fractal_cache_hit_miss() {
        let mut cache = FractalCache::new(100);

        let key = FractalCacheKey::new(1, 0, (100, 100, 100));
        let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);

        cache.insert(key.clone(), 0, data).unwrap();

        let _ = cache.get(&key, 0).unwrap();
        let stats = cache.statistics();
        assert_eq!(stats.total_hits, 1);
        assert_eq!(stats.total_misses, 0);
        assert_eq!(stats.hit_rate, 1.0);

        let _ = cache
            .get(&FractalCacheKey::new(2, 0, (100, 100, 100)), 0)
            .unwrap();
        let stats = cache.statistics();
        assert_eq!(stats.total_hits, 1);
        assert_eq!(stats.total_misses, 1);
        assert_eq!(stats.hit_rate, 0.5);
    }

    #[test]
    fn test_fractal_cache_multiple_levels() {
        let mut cache = FractalCache::new(100);

        let key = FractalCacheKey::new(1, 0, (100, 100, 100));

        cache
            .insert(key.clone(), 0, FractalData::new(vec![1.0; 4], vec![2, 2]))
            .unwrap();
        cache
            .insert(key.clone(), 1, FractalData::new(vec![2.0; 2], vec![2]))
            .unwrap();
        cache
            .insert(key.clone(), 2, FractalData::new(vec![3.0], vec![1]))
            .unwrap();

        let level_0 = cache.get(&key, 0).unwrap().unwrap();
        let level_1 = cache.get(&key, 1).unwrap().unwrap();
        let level_2 = cache.get(&key, 2).unwrap().unwrap();

        assert_eq!(level_0.data[0], 1.0);
        assert_eq!(level_1.data[0], 2.0);
        assert_eq!(level_2.data[0], 3.0);
    }

    #[test]
    fn test_fractal_cache_invalidate() {
        let mut cache = FractalCache::new(100);

        let key = FractalCacheKey::new(1, 0, (100, 100, 100));
        let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);

        cache.insert(key.clone(), 0, data).unwrap();
        assert!(cache.contains(&key));

        cache.invalidate(&key);
        assert!(!cache.contains(&key));
    }

    #[test]
    fn test_fractal_cache_clear() {
        let mut cache = FractalCache::new(100);

        for i in 0..10 {
            let key = FractalCacheKey::new(i, 0, (100, 100, 100));
            let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);
            cache.insert(key, 0, data).unwrap();
        }

        assert_eq!(cache.len(), 10);

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_fractal_cache_lru_eviction() {
        let mut cache = FractalCache::new(5);

        for i in 0..5 {
            let key = FractalCacheKey::new(i, 0, (i as i64, i as i64, i as i64));
            let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);
            cache.insert(key, 0, data).unwrap();
        }

        assert_eq!(cache.len(), 5);

        let key6 = FractalCacheKey::new(6, 0, (6, 6, 6));
        let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);
        cache.insert(key6, 0, data).unwrap();

        assert_eq!(cache.len(), 5);
        assert!(!cache.contains(&FractalCacheKey::new(0, 0, (0, 0, 0))));
    }

    #[test]
    fn test_fractal_cache_invalid_scale_level() {
        let mut cache = FractalCache::new(100);

        let key = FractalCacheKey::new(1, 0, (100, 100, 100));
        let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);

        let result = cache.insert(key, 10, data);
        assert!(matches!(
            result,
            Err(FractalCacheError::InvalidScaleLevel(10))
        ));

        let result = cache.get(&FractalCacheKey::new(1, 0, (100, 100, 100)), 10);
        assert!(matches!(
            result,
            Err(FractalCacheError::InvalidScaleLevel(10))
        ));
    }

    #[test]
    fn test_fractal_cache_invalid_data() {
        let mut cache = FractalCache::new(100);

        let key = FractalCacheKey::new(1, 0, (100, 100, 100));
        let data = FractalData::new(vec![], vec![]);

        let result = cache.insert(key, 0, data);
        assert!(matches!(result, Err(FractalCacheError::InvalidData)));
    }

    #[test]
    fn test_fractal_cache_entry_get_all_levels() {
        let mut cache = FractalCache::new(100);

        let key = FractalCacheKey::new(1, 0, (100, 100, 100));

        cache
            .insert(key.clone(), 0, FractalData::new(vec![1.0; 4], vec![2, 2]))
            .unwrap();
        cache
            .insert(key.clone(), 2, FractalData::new(vec![2.0], vec![1]))
            .unwrap();

        let levels = cache.get_all_levels(&key);
        assert!(levels.is_some());

        let levels = levels.unwrap();
        assert!(levels[0].is_some());
        assert!(levels[1].is_none());
        assert!(levels[2].is_some());
    }

    #[test]
    fn test_fractal_cache_entry_populated_levels() {
        let mut cache = FractalCache::new(100);

        let key = FractalCacheKey::new(1, 0, (100, 100, 100));

        cache
            .insert(key.clone(), 0, FractalData::new(vec![1.0; 4], vec![2, 2]))
            .unwrap();
        cache
            .insert(key.clone(), 2, FractalData::new(vec![2.0], vec![1]))
            .unwrap();

        let populated = cache.get_populated_levels(&key);
        assert_eq!(populated, Some(2));
    }

    #[test]
    fn test_fractal_refinement_creation() {
        let refinement = FractalRefinement::new(0, 3, RefinementMode::Progressive);
        assert_eq!(refinement.current_level, 0);
        assert_eq!(refinement.target_level, 3);
        assert_eq!(refinement.progress, 0.0);
        assert!(!refinement.is_complete());
    }

    #[test]
    fn test_fractal_refinement_with_progress() {
        let refinement = FractalRefinement::with_progress(0, 3, 1.0, RefinementMode::Progressive);
        assert!(refinement.is_complete());
    }

    #[test]
    fn test_fractal_refinement_update_progress() {
        let mut refinement = FractalRefinement::new(0, 3, RefinementMode::Progressive);
        refinement.update_progress(0.5);
        assert_eq!(refinement.progress, 0.5);
        refinement.update_progress(1.0);
        assert_eq!(refinement.progress, 1.0);
        assert!(refinement.is_complete());
    }

    #[test]
    fn test_fractal_refinement_get_current_level() {
        let refinement = FractalRefinement::with_progress(0, 4, 0.5, RefinementMode::Progressive);
        assert_eq!(refinement.get_current_level(), 2);
    }

    #[test]
    fn test_fractal_cache_get_with_refinement_immediate() {
        let mut cache = FractalCache::new(100);

        let key = FractalCacheKey::new(1, 0, (100, 100, 100));
        let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);

        cache.insert(key.clone(), 2, data.clone()).unwrap();

        let refinement = FractalRefinement::new(0, 2, RefinementMode::Immediate);
        let result = cache.get_with_refinement(&key, &refinement).unwrap();

        assert!(result.is_some());
        assert_eq!(result.unwrap().data, data.data);
    }

    #[test]
    fn test_fractal_cache_get_with_refinement_progressive() {
        let mut cache = FractalCache::new(100);

        let key = FractalCacheKey::new(1, 0, (100, 100, 100));
        let data = FractalData::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);

        cache.insert(key.clone(), 0, data.clone()).unwrap();

        let refinement = FractalRefinement::with_progress(0, 1, 0.5, RefinementMode::Progressive);
        let result = cache.get_with_refinement(&key, &refinement).unwrap();

        assert!(result.is_some());
        let refined = result.unwrap();
        assert_eq!(refined.shape.len(), 2);
        assert!(refined.shape[0] >= 2 || refined.shape[1] >= 2);
    }

    #[test]
    fn test_fractal_data_creation() {
        let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);
        assert_eq!(data.data.len(), 3);
        assert_eq!(data.num_elements(), 3);
        assert_eq!(data.rank(), 1);
        assert_eq!(data.quality, 1.0);
        assert_eq!(data.compression_ratio, 1.0);
    }

    #[test]
    fn test_fractal_data_with_quality() {
        let data = FractalData::with_quality(vec![1.0, 2.0, 3.0], vec![3], 0.8, 10.0);
        assert_eq!(data.quality, 0.8);
        assert_eq!(data.compression_ratio, 10.0);
    }

    #[test]
    fn test_fractal_cache_key_creation() {
        let key = FractalCacheKey::new(1, 0, (100, 100, 100));
        assert_eq!(key.entity_id, 1);
        assert_eq!(key.scale_level, 0);
        assert_eq!(key.position_idx, (100, 100, 100));
        assert_eq!(key.data_type, 0);
    }

    #[test]
    fn test_fractal_cache_key_from_scale_level() {
        let key = FractalCacheKey::from_scale_level(1, ScaleLevel::Planetary, (100, 100, 100));
        assert_eq!(key.scale_level, 3);
    }

    #[test]
    fn test_fractal_cache_entry_new() {
        let entry = FractalCacheEntry::new();
        assert!(!entry.is_populated());
        assert_eq!(entry.get_finest_level(), None);
        assert_eq!(entry.get_coarsest_level(), None);
    }

    #[test]
    fn test_fractal_cache_entry_set_level() {
        let mut entry = FractalCacheEntry::new();
        let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);

        entry.set_level(0, data.clone()).unwrap();
        assert!(entry.is_populated());
        assert_eq!(entry.get_finest_level(), Some(0));
        assert_eq!(entry.get_coarsest_level(), Some(0));

        entry.set_level(2, data).unwrap();
        assert_eq!(entry.get_finest_level(), Some(0));
        assert_eq!(entry.get_coarsest_level(), Some(2));
    }

    #[test]
    fn test_fractal_cache_entry_get_level() {
        let mut entry = FractalCacheEntry::new();
        let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);

        entry.set_level(0, data.clone()).unwrap();

        let retrieved = entry.get_level(0);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().data, data.data);

        assert!(entry.get_level(1).is_none());
    }

    #[test]
    fn test_all_refinement_modes() {
        let mut cache = FractalCache::new(100);
        let key = FractalCacheKey::new(1, 0, (100, 100, 100));
        let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);

        cache.insert(key.clone(), 2, data).unwrap();

        let modes = [
            RefinementMode::Immediate,
            RefinementMode::Progressive,
            RefinementMode::Adaptive,
        ];

        for mode in modes {
            let refinement = FractalRefinement::new(0, 2, mode);
            let result = cache.get_with_refinement(&key, &refinement);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_fractal_cache_statistics_default() {
        let stats = FractalCacheStatistics::default();
        assert_eq!(stats.total_hits, 0);
        assert_eq!(stats.total_misses, 0);
        assert_eq!(stats.hit_rate, 0.0);
        assert_eq!(stats.evictions, 0);
        assert_eq!(stats.invalidations, 0);
    }

    #[test]
    fn test_fractal_cache_contains() {
        let mut cache = FractalCache::new(100);
        let key = FractalCacheKey::new(1, 0, (100, 100, 100));
        let data = FractalData::new(vec![1.0, 2.0, 3.0], vec![3]);

        assert!(!cache.contains(&key));

        cache.insert(key.clone(), 0, data).unwrap();

        assert!(cache.contains(&key));
    }

    #[test]
    fn test_fractal_cache_all_8_levels() {
        let mut cache = FractalCache::new(100);
        let key = FractalCacheKey::new(1, 0, (100, 100, 100));

        for level in 0..=7 {
            let data = FractalData::new(vec![level as Float], vec![1]);
            cache.insert(key.clone(), level, data).unwrap();
        }

        let populated = cache.get_populated_levels(&key);
        assert_eq!(populated, Some(8));

        let all_levels = cache.get_all_levels(&key).unwrap();
        assert_eq!(all_levels.len(), 8);

        for level in 0..=7 {
            let result = cache.get(&key, level).unwrap();
            assert!(result.is_some());
        }
    }
}
