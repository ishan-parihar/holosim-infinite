//! Predictive Cache - Predict Field Regions Needed Based on Entity Movement
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 5.2:
//! "Predict what field regions will be needed and preload"
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "Predictive loading enables 60 FPS with 10,000+ entities"
//!
//! This module implements:
//! - Movement pattern prediction for entities
//! - Preloading field regions before they're needed
//! - Cache invalidation based on entity trajectories

use crate::hpo::spatial_field::Position3D;
use std::collections::HashMap;
use std::collections::VecDeque;

/// Configuration for predictive cache
#[derive(Debug, Clone)]
pub struct PredictiveCacheConfig {
    /// Number of steps to predict ahead
    pub prediction_horizon: usize,

    /// Maximum cache size
    pub max_cache_size: usize,

    /// Minimum confidence for prediction
    pub min_confidence: f64,

    /// Update frequency (steps)
    pub update_frequency: usize,

    /// Enable trajectory tracking
    pub enable_trajectory_tracking: bool,
}

impl Default for PredictiveCacheConfig {
    fn default() -> Self {
        PredictiveCacheConfig {
            prediction_horizon: 10,
            max_cache_size: 1000,
            min_confidence: 0.5,
            update_frequency: 1,
            enable_trajectory_tracking: true,
        }
    }
}

/// Cache key for field region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheKey {
    /// Position in space (rounded for hashing)
    pub pos_x: i64,
    pub pos_y: i64,
    pub pos_z: i64,

    /// Scale level (0-7)
    pub scale_level: usize,

    /// Spectrum position (rounded)
    pub spectrum_position: i64,
}

impl std::hash::Hash for CacheKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos_x.hash(state);
        self.pos_y.hash(state);
        self.pos_z.hash(state);
        self.scale_level.hash(state);
        self.spectrum_position.hash(state);
    }
}

impl CacheKey {
    pub fn new(position: Position3D, scale_level: usize, spectrum_position: f64) -> Self {
        let scale = 1000.0; // Reduce precision for hashing
        CacheKey {
            pos_x: (position.x * scale).round() as i64,
            pos_y: (position.y * scale).round() as i64,
            pos_z: (position.z * scale).round() as i64,
            scale_level,
            spectrum_position: (spectrum_position * scale).round() as i64,
        }
    }
}

/// Entity trajectory for prediction
#[derive(Debug, Clone)]
pub struct EntityTrajectory {
    /// Entity ID
    pub entity_id: u64,

    /// Recent positions (history)
    pub position_history: VecDeque<Position3D>,

    /// Velocity vector
    pub velocity: Position3D,

    /// Acceleration (for prediction)
    pub acceleration: Position3D,

    /// Prediction confidence
    pub confidence: f64,

    /// History size limit
    pub max_history: usize,
}

impl EntityTrajectory {
    pub fn new(entity_id: u64, max_history: usize) -> Self {
        EntityTrajectory {
            entity_id,
            position_history: VecDeque::new(),
            velocity: Position3D::new(0.0, 0.0, 0.0),
            acceleration: Position3D::new(0.0, 0.0, 0.0),
            confidence: 1.0,
            max_history,
        }
    }

    /// Update trajectory with new position
    pub fn update(&mut self, new_position: Position3D) {
        let now = self.position_history.len();

        // Calculate velocity if we have history
        if let Some(old_pos) = self.position_history.back() {
            self.velocity.x = new_position.x - old_pos.x;
            self.velocity.y = new_position.y - old_pos.y;
            self.velocity.z = new_position.z - old_pos.z;

            // Calculate acceleration if we have enough history
            if self.position_history.len() >= 2 {
                if let Some(old_vel) = self.position_history.get(self.position_history.len() - 2) {
                    self.acceleration.x = self.velocity.x - (old_pos.x - old_vel.x);
                    self.acceleration.y = self.velocity.y - (old_pos.y - old_vel.y);
                    self.acceleration.z = self.velocity.z - (old_pos.z - old_vel.z);
                }
            }
        }

        // Add to history
        self.position_history.push_back(new_position);

        // Limit history size
        while self.position_history.len() > self.max_history {
            self.position_history.pop_front();
        }

        // Update confidence based on trajectory smoothness
        self.update_confidence();
    }

    /// Update prediction confidence
    fn update_confidence(&mut self) {
        // More history = higher confidence
        let history_factor =
            (self.position_history.len() as f64 / self.max_history as f64).min(1.0);

        // Lower acceleration = higher confidence
        let accel_mag = (self.acceleration.x.powi(2)
            + self.acceleration.y.powi(2)
            + self.acceleration.z.powi(2))
        .sqrt();
        let accel_factor = (1.0 / (1.0 + accel_mag * 0.1)).min(1.0);

        self.confidence = (history_factor * 0.7 + accel_factor * 0.3).min(1.0);
    }

    /// Predict future position
    pub fn predict_position(&self, steps_ahead: usize) -> Position3D {
        if self.position_history.is_empty() {
            return Position3D::new(0.0, 0.0, 0.0);
        }

        let last_pos = self.position_history.back().unwrap();

        // Use velocity for prediction (more stable than acceleration)
        Position3D::new(
            last_pos.x + self.velocity.x * steps_ahead as f64,
            last_pos.y + self.velocity.y * steps_ahead as f64,
            last_pos.z + self.velocity.z * steps_ahead as f64,
        )
    }
}

/// Predictive Cache
///
/// Predicts which field regions will be needed and preloads them
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// > "Predictive loading enables 60 FPS with 10,000+ entities"
#[derive(Debug, Clone)]
pub struct PredictiveCache {
    config: PredictiveCacheConfig,
    trajectories: HashMap<u64, EntityTrajectory>,
    cached_regions: HashMap<CacheKey, CachedRegion>,
    cache_hits: usize,
    cache_misses: usize,
    predictions_made: usize,
}

#[derive(Debug, Clone)]
pub struct CachedRegion {
    /// Field data
    pub data: Vec<f64>,

    /// When cached (step)
    pub cached_at: usize,

    /// Last accessed (step)
    pub last_accessed: usize,

    /// Access count
    pub access_count: usize,
}

impl PredictiveCache {
    pub fn new() -> Self {
        PredictiveCache {
            config: PredictiveCacheConfig::default(),
            trajectories: HashMap::new(),
            cached_regions: HashMap::new(),
            cache_hits: 0,
            cache_misses: 0,
            predictions_made: 0,
        }
    }

    pub fn with_config(config: PredictiveCacheConfig) -> Self {
        PredictiveCache {
            config,
            trajectories: HashMap::new(),
            cached_regions: HashMap::new(),
            cache_hits: 0,
            cache_misses: 0,
            predictions_made: 0,
        }
    }

    /// Update entity position and trajectory
    pub fn update_entity(&mut self, entity_id: u64, position: Position3D) {
        if self.config.enable_trajectory_tracking {
            let trajectory = self
                .trajectories
                .entry(entity_id)
                .or_insert_with(|| EntityTrajectory::new(entity_id, 20));

            trajectory.update(position);
        }
    }

    /// Predict what regions will be needed
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// > "Predict based on entity movement patterns"
    pub fn predict(&self) -> Vec<CacheKey> {
        let mut predictions = Vec::new();

        for (entity_id, trajectory) in &self.trajectories {
            if trajectory.confidence < self.config.min_confidence {
                continue;
            }

            // Predict positions at each horizon step
            for step in 1..=self.config.prediction_horizon {
                let predicted_pos = trajectory.predict_position(step);

                // Create cache keys at different scales
                for scale in 0..8 {
                    let key = CacheKey::new(predicted_pos, scale, 0.5);
                    predictions.push(key);
                }
            }

            // Also predict current position
            if let Some(current_pos) = trajectory.position_history.back() {
                for scale in 0..8 {
                    let key = CacheKey::new(*current_pos, scale, 0.5);
                    predictions.push(key);
                }
            }
        }

        predictions
    }

    /// Preload predicted regions
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// > "Preload predicted regions"
    pub fn preload<F>(&mut self, predictions: Vec<CacheKey>, loader: &F)
    where
        F: Fn(CacheKey) -> Vec<f64>,
    {
        for key in predictions {
            // Skip if already cached
            if self.cached_regions.contains_key(&key) {
                continue;
            }

            // Skip if cache is full
            if self.cached_regions.len() >= self.config.max_cache_size {
                self.evict_oldest();
            }

            // Load and cache
            let data = loader(key);
            self.cached_regions.insert(
                key,
                CachedRegion {
                    data,
                    cached_at: 0,
                    last_accessed: 0,
                    access_count: 0,
                },
            );

            self.predictions_made += 1;
        }
    }

    /// Get cached region
    pub fn get(&mut self, key: &CacheKey, current_step: usize) -> Option<&Vec<f64>> {
        if let Some(region) = self.cached_regions.get_mut(key) {
            region.last_accessed = current_step;
            region.access_count += 1;
            self.cache_hits += 1;
            Some(&region.data)
        } else {
            self.cache_misses += 1;
            None
        }
    }

    /// Evict oldest cache entry
    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self
            .cached_regions
            .iter()
            .min_by_key(|(_, v)| v.last_accessed)
            .map(|(k, _)| k.clone())
        {
            self.cached_regions.remove(&oldest_key);
        }
    }

    /// Update predictions (called each simulation step)
    pub fn update(&mut self, current_step: usize) {
        if current_step % self.config.update_frequency != 0 {
            return;
        }

        // Clean up stale trajectories
        self.trajectories.retain(|_, t| t.confidence > 0.1);

        // Evict old cache entries periodically
        if current_step % 100 == 0 {
            self.cleanup_cache(current_step);
        }
    }

    /// Cleanup stale cache entries
    fn cleanup_cache(&mut self, current_step: usize) {
        let max_age = 1000; // steps
        self.cached_regions
            .retain(|_, v| current_step - v.cached_at < max_age);
    }

    /// Get cache statistics
    pub fn get_statistics(&self) -> PredictiveCacheStats {
        let total_requests = self.cache_hits + self.cache_misses;
        let hit_rate = if total_requests > 0 {
            self.cache_hits as f64 / total_requests as f64
        } else {
            0.0
        };

        PredictiveCacheStats {
            cache_hits: self.cache_hits,
            cache_misses: self.cache_misses,
            hit_rate,
            predictions_made: self.predictions_made,
            cached_regions: self.cached_regions.len(),
            tracked_entities: self.trajectories.len(),
        }
    }

    /// Clear cache
    pub fn clear(&mut self) {
        self.cached_regions.clear();
        self.cache_hits = 0;
        self.cache_misses = 0;
        self.predictions_made = 0;
    }
}

impl Default for PredictiveCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for predictive cache
#[derive(Debug, Clone)]
pub struct PredictiveCacheStats {
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub hit_rate: f64,
    pub predictions_made: usize,
    pub cached_regions: usize,
    pub tracked_entities: usize,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trajectory_prediction() {
        let mut trajectory = EntityTrajectory::new(1, 10);

        // Add positions
        trajectory.update(Position3D::new(0.0, 0.0, 0.0));
        trajectory.update(Position3D::new(1.0, 0.0, 0.0));
        trajectory.update(Position3D::new(2.0, 0.0, 0.0));

        // Predict next position
        let predicted = trajectory.predict_position(1);

        // Should continue in same direction
        assert!(predicted.x > 2.0);
    }

    #[test]
    fn test_cache_operations() {
        let mut cache = PredictiveCache::new();

        let key = CacheKey::new(Position3D::new(0.0, 0.0, 0.0), 0, 0.5);

        // Miss
        let result = cache.get(&key, 0);
        assert!(result.is_none());

        // Preload
        cache.preload(vec![key], &|_| vec![1.0, 2.0, 3.0]);

        // Hit
        let result = cache.get(&key, 1);
        assert!(result.is_some());
    }
}
