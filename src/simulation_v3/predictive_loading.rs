//! Predictive Loading System
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4.4:
//! "Implement predictive loading for seamless multi-scale navigation"
//!
//! From MASTER_R&D_ROADMAP.md Week 19-20:
//! - Implement player position prediction
//! - Pre-load data at predicted positions
//! - Implement scale-aware pre-loading
//! - Implement cache eviction policy
//!
//! This module implements:
//! 1. PredictiveLoader - Main predictive loading orchestrator
//! 2. Position prediction using velocity and movement patterns
//! 3. Scale-aware pre-loading strategies
//! 4. Load queue management and prioritization
//! 5. Cache eviction policies (LRU, LFU, adaptive)
//! 6. Performance statistics and monitoring

use super::fractal_cache::{
    EvictionPolicy, FractalCache, FractalCacheKey, FractalCacheStatistics, FractalData,
};
use super::multiscale_camera::{MultiScaleCamera, ScaleLevel};
use super::multiscale_field::{HolographicView, MultiScaleField};
use crate::types::Float;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

/// Predictive Loader
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4.4:
/// "Predict where player will be in next N frames and pre-load data"
///
/// The predictive loader anticipates player movement and scale transitions,
/// pre-loading relevant data to ensure seamless navigation across all scales.
///
/// Key features:
/// - Velocity-based position prediction
/// - Scale transition prediction
/// - Prioritized load queue
/// - Adaptive cache eviction
/// - Performance statistics
pub struct PredictiveLoader {
    /// Fractal cache for storing loaded data
    cache: FractalCache,

    /// Multi-scale field for data access
    field: MultiScaleField,

    /// Multi-scale camera for tracking player state
    camera: MultiScaleCamera,

    /// Current player position (log10 of meters)
    player_position: (Float, Float, Float),

    /// Current player velocity (log10 of meters per second)
    player_velocity: (Float, Float, Float),

    /// Movement history for pattern recognition
    movement_history: VecDeque<MovementSample>,

    /// Load queue with prioritized entries
    load_queue: VecDeque<LoadTask>,

    /// Current load tasks in progress
    active_loads: HashMap<LoadTaskId, LoadProgress>,

    /// Load task ID counter
    next_load_id: u64,

    /// Eviction policy
    eviction_policy: EvictionPolicy,

    /// Maximum queue size
    max_queue_size: usize,

    /// Maximum concurrent loads
    max_concurrent_loads: usize,

    /// Predictive lookahead (frames)
    lookahead_frames: usize,

    /// Prediction confidence threshold
    confidence_threshold: Float,

    /// Scale prediction weights
    scale_prediction_weights: [Float; 7],

    /// Statistics
    stats: PredictiveLoaderStatistics,
}

/// Movement Sample
///
/// Stores a snapshot of player movement for pattern recognition.
#[derive(Debug, Clone)]
struct MovementSample {
    /// Timestamp of this sample
    #[allow(dead_code)]
    timestamp: Instant,

    /// Position (log10 of meters)
    #[allow(dead_code)]
    position: (Float, Float, Float),

    /// Velocity (log10 of meters per second)
    velocity: (Float, Float, Float),

    /// Scale level
    #[allow(dead_code)]
    scale: ScaleLevel,

    /// Scale transition progress (0.0 to 1.0)
    #[allow(dead_code)]
    scale_progress: Float,
}

/// Load Task
///
/// Represents a pending load operation.
#[derive(Debug, Clone)]
struct LoadTask {
    /// Unique task identifier
    id: LoadTaskId,

    /// Cache key for the data to load
    cache_key: FractalCacheKey,

    /// Position to load (log10 of meters)
    position: (Float, Float, Float),

    /// Scale level
    scale: ScaleLevel,

    /// Load priority
    priority: LoadPriority,

    /// Prediction confidence (0.0 to 1.0)
    #[allow(dead_code)]
    confidence: Float,

    /// Expected frame when this data will be needed
    #[allow(dead_code)]
    expected_frame: usize,

    /// Data type to load
    #[allow(dead_code)]
    data_type: u32,
}

/// Load Task ID
///
/// Unique identifier for load tasks.
type LoadTaskId = u64;

/// Load Priority
///
/// Priority levels for load tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoadPriority {
    /// Critical - needed immediately
    Critical = 0,

    /// High - needed in next few frames
    High = 1,

    /// Medium - needed soon
    Medium = 2,

    /// Low - background loading
    Low = 3,

    /// Background - lowest priority
    Background = 4,
}

/// Load Progress
///
/// Tracks progress of active load operations.
#[derive(Debug, Clone)]
struct LoadProgress {
    /// Load task
    #[allow(dead_code)]
    task: LoadTask,

    /// Start time
    #[allow(dead_code)]
    start_time: Instant,

    /// Bytes loaded
    #[allow(dead_code)]
    bytes_loaded: usize,

    /// Total bytes
    #[allow(dead_code)]
    total_bytes: usize,

    /// Completion percentage (0.0 to 1.0)
    #[allow(dead_code)]
    progress: Float,
}

/// Predictive Loader Statistics
///
/// Tracks performance metrics for the predictive loader.
#[derive(Debug, Clone, Default)]
pub struct PredictiveLoaderStatistics {
    /// Total predictions made
    pub total_predictions: usize,

    /// Correct predictions
    pub correct_predictions: usize,

    /// Total loads initiated
    pub total_loads: usize,

    /// Successful loads
    pub successful_loads: usize,

    /// Failed loads
    pub failed_loads: usize,

    /// Cache hits from pre-loading
    pub cache_hits: usize,

    /// Cache misses (data not pre-loaded)
    pub cache_misses: usize,

    /// Total bytes loaded
    pub total_bytes_loaded: usize,

    /// Average load time (milliseconds)
    pub average_load_time_ms: Float,

    /// Prediction accuracy (0.0 to 1.0)
    pub prediction_accuracy: Float,

    /// Cache hit rate from pre-loading (0.0 to 1.0)
    pub cache_hit_rate: Float,

    /// Eviction count
    pub evictions: usize,
}

/// Predictive Loader Error
///
/// Errors that can occur during predictive loading.
#[derive(Debug, Clone, PartialEq)]
pub enum PredictiveLoaderError {
    /// Cache error
    CacheError(String),

    /// Field error
    FieldError(String),

    /// Invalid position
    InvalidPosition(String),

    /// Invalid scale
    InvalidScale(String),

    /// Load queue full
    LoadQueueFull,

    /// Too many concurrent loads
    TooManyConcurrentLoads,

    /// Prediction failed
    PredictionFailed(String),
}

impl PredictiveLoader {
    /// Create a new predictive loader
    ///
    /// # Arguments
    /// - `cache` - Fractal cache for storing loaded data
    /// - `field` - Multi-scale field for data access
    /// - `camera` - Multi-scale camera for tracking player state
    pub fn new(cache: FractalCache, field: MultiScaleField, camera: MultiScaleCamera) -> Self {
        PredictiveLoader {
            cache,
            field,
            camera,
            player_position: (0.0, 0.0, 0.0),
            player_velocity: (0.0, 0.0, 0.0),
            movement_history: VecDeque::with_capacity(100),
            load_queue: VecDeque::new(),
            active_loads: HashMap::new(),
            next_load_id: 0,
            eviction_policy: EvictionPolicy::Adaptive,
            max_queue_size: 1000,
            max_concurrent_loads: 10,
            lookahead_frames: 10,
            confidence_threshold: 0.5,
            scale_prediction_weights: [0.3; 7],
            stats: PredictiveLoaderStatistics::default(),
        }
    }

    /// Create a new predictive loader with default components
    pub fn new_with_defaults() -> Self {
        let cache = FractalCache::new(1000);
        let mut field = MultiScaleField::new();
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);

        // Initialize the field with default data for testing
        // From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md: "The field is initialized at the finest scale"
        let base_data = vec![1.0; 1000]; // Default holographic field data
        let base_shape = vec![10, 10, 10]; // 3D field
        let _ = field.initialize(base_data, base_shape); // Ignore initialization errors for tests

        Self::new(cache, field, camera)
    }

    /// Set eviction policy
    pub fn set_eviction_policy(&mut self, policy: EvictionPolicy) {
        self.eviction_policy = policy;
    }

    /// Set maximum queue size
    pub fn set_max_queue_size(&mut self, size: usize) {
        self.max_queue_size = size;
    }

    /// Set maximum concurrent loads
    pub fn set_max_concurrent_loads(&mut self, max: usize) {
        self.max_concurrent_loads = max;
    }

    /// Set lookahead frames
    pub fn set_lookahead_frames(&mut self, frames: usize) {
        self.lookahead_frames = frames;
    }

    /// Update player state
    ///
    /// # Arguments
    /// - `position` - Current player position (log10 of meters)
    /// - `velocity` - Current player velocity (log10 of meters per second)
    pub fn update_player_state(
        &mut self,
        position: (Float, Float, Float),
        velocity: (Float, Float, Float),
    ) {
        self.player_position = position;
        self.player_velocity = velocity;

        let scale_progress = self.camera.transition_progress();

        let sample = MovementSample {
            timestamp: Instant::now(),
            position,
            velocity,
            scale: self.camera.current_scale(),
            scale_progress,
        };

        self.movement_history.push_back(sample);

        if self.movement_history.len() > 100 {
            self.movement_history.pop_front();
        }
    }

    /// Update predictive loader
    ///
    /// This should be called each frame to:
    /// 1. Predict future positions
    /// 2. Generate load tasks
    /// 3. Process load queue
    /// 4. Evict cache entries if needed
    pub fn update(&mut self) -> Result<(), PredictiveLoaderError> {
        self.predict_future_positions()?;
        self.generate_load_tasks()?;
        self.process_load_queue()?;
        self.evict_if_needed()?;

        Ok(())
    }

    /// Predict future positions
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4.4:
    /// "Predict where player will be in next N frames"
    fn predict_future_positions(&mut self) -> Result<(), PredictiveLoaderError> {
        self.stats.total_predictions += 1;

        let _current_scale = self.camera.current_scale();
        let _scale_progress = self.camera.transition_progress();

        let predictions = if self.movement_history.len() >= 3 {
            self.predict_from_history()?
        } else {
            self.predict_from_velocity()?
        };

        for (frame_offset, prediction) in predictions.iter().enumerate() {
            let confidence = self.compute_confidence(prediction, frame_offset);

            if confidence < self.confidence_threshold {
                continue;
            }

            let cache_key = FractalCacheKey {
                entity_id: 0,
                scale_level: prediction.scale as usize,
                position_idx: self.quantize_position(prediction.position),
                data_type: 0,
            };

            let priority = self.compute_priority(frame_offset, confidence, prediction.scale);

            let task = LoadTask {
                id: self.next_load_id,
                cache_key,
                position: prediction.position,
                scale: prediction.scale,
                priority,
                confidence,
                expected_frame: frame_offset,
                data_type: 0,
            };

            self.next_load_id += 1;

            if self.load_queue.len() < self.max_queue_size {
                self.load_queue.push_back(task);
            }
        }

        Ok(())
    }

    /// Predict from movement history
    fn predict_from_history(&self) -> Result<Vec<Prediction>, PredictiveLoaderError> {
        let mut predictions = Vec::new();

        let recent_samples: Vec<_> = self.movement_history.iter().rev().take(10).collect();

        if recent_samples.len() < 3 {
            return self.predict_from_velocity();
        }

        let velocity_avg = recent_samples
            .iter()
            .map(|s| s.velocity)
            .fold((0.0, 0.0, 0.0), |acc, v| {
                (acc.0 + v.0, acc.1 + v.1, acc.2 + v.2)
            });

        let velocity_avg = (
            velocity_avg.0 / recent_samples.len() as Float,
            velocity_avg.1 / recent_samples.len() as Float,
            velocity_avg.2 / recent_samples.len() as Float,
        );

        let current_position = self.player_position;
        let _current_scale = self.camera.current_scale();

        for frame in 1..=self.lookahead_frames {
            let dt = frame as Float / 60.0;
            let predicted_position = (
                current_position.0 + velocity_avg.0 * dt,
                current_position.1 + velocity_avg.1 * dt,
                current_position.2 + velocity_avg.2 * dt,
            );

            let predicted_scale = self.predict_scale_change(frame);

            predictions.push(Prediction {
                position: predicted_position,
                scale: predicted_scale,
                frame_offset: frame,
            });
        }

        Ok(predictions)
    }

    /// Predict from velocity
    fn predict_from_velocity(&self) -> Result<Vec<Prediction>, PredictiveLoaderError> {
        let mut predictions = Vec::new();

        let current_position = self.player_position;
        let velocity = self.player_velocity;

        for frame in 1..=self.lookahead_frames {
            let dt = frame as Float / 60.0;
            let predicted_position = (
                current_position.0 + velocity.0 * dt,
                current_position.1 + velocity.1 * dt,
                current_position.2 + velocity.2 * dt,
            );

            let predicted_scale = self.camera.current_scale();

            predictions.push(Prediction {
                position: predicted_position,
                scale: predicted_scale,
                frame_offset: frame,
            });
        }

        Ok(predictions)
    }

    /// Predict scale change
    fn predict_scale_change(&self, _frame_offset: usize) -> ScaleLevel {
        let current_scale = self.camera.current_scale();
        let target_scale = self.camera.target_scale();

        if current_scale != target_scale {
            target_scale
        } else {
            current_scale
        }
    }

    /// Compute confidence for prediction
    fn compute_confidence(&self, prediction: &Prediction, frame_offset: usize) -> Float {
        let distance_factor = 1.0 / (1.0 + frame_offset as Float);
        let scale_match = if prediction.scale == self.camera.current_scale() {
            1.0
        } else {
            0.7
        };

        distance_factor * scale_match * self.scale_prediction_weights[prediction.scale as usize]
    }

    /// Compute priority for load task
    fn compute_priority(
        &self,
        frame_offset: usize,
        confidence: Float,
        _scale: ScaleLevel,
    ) -> LoadPriority {
        if frame_offset == 0 {
            return LoadPriority::Critical;
        }

        if frame_offset <= 2 && confidence > 0.8 {
            return LoadPriority::High;
        }

        if frame_offset <= 5 {
            return LoadPriority::Medium;
        }

        if frame_offset <= 10 {
            return LoadPriority::Low;
        }

        LoadPriority::Background
    }

    /// Quantize position for cache key
    fn quantize_position(&self, position: (Float, Float, Float)) -> (i64, i64, i64) {
        const QUANTUM: Float = 0.1;

        (
            (position.0 / QUANTUM).round() as i64,
            (position.1 / QUANTUM).round() as i64,
            (position.2 / QUANTUM).round() as i64,
        )
    }

    /// Generate load tasks from predictions
    fn generate_load_tasks(&mut self) -> Result<(), PredictiveLoaderError> {
        let tasks_to_remove: Vec<_> = self
            .load_queue
            .iter()
            .filter(|task| self.cache.contains(&task.cache_key))
            .map(|task| task.id)
            .collect();

        for task_id in tasks_to_remove {
            self.load_queue.retain(|t| t.id != task_id);
        }

        Ok(())
    }

    /// Process load queue
    fn process_load_queue(&mut self) -> Result<(), PredictiveLoaderError> {
        self.load_queue
            .make_contiguous()
            .sort_by_key(|t| t.priority);

        while let Some(task) = self.load_queue.pop_front() {
            if self.active_loads.len() >= self.max_concurrent_loads {
                break;
            }

            if self.cache.contains(&task.cache_key) {
                self.stats.cache_hits += 1;
                continue;
            }

            self.start_load(task)?;
        }

        Ok(())
    }

    /// Start a load task
    fn start_load(&mut self, task: LoadTask) -> Result<(), PredictiveLoaderError> {
        self.stats.total_loads += 1;

        let scale_index = match task.scale {
            ScaleLevel::Quantum => 0,
            ScaleLevel::Cellular => 1,
            ScaleLevel::Biological => 2,
            ScaleLevel::Planetary => 3,
            ScaleLevel::Stellar => 4,
            ScaleLevel::Galactic => 5,
            ScaleLevel::Cosmic => 6,
        };

        let view = self
            .field
            .get_view(task.scale, task.position.0, task.position)
            .map_err(|e| PredictiveLoaderError::FieldError(format!("{:?}", e)))?;

        let data = FractalData {
            data: view.field_data.clone(),
            shape: view.field_shape.clone(),
            quality: view.quality,
            compression_ratio: 1.0,
            size_bytes: view.field_data.len() * std::mem::size_of::<Float>(),
        };

        let size_bytes = data.size_bytes;

        if let Err(e) = self.cache.insert(task.cache_key.clone(), scale_index, data) {
            return Err(PredictiveLoaderError::CacheError(format!("{:?}", e)));
        }

        self.stats.successful_loads += 1;
        self.stats.total_bytes_loaded += size_bytes;

        Ok(())
    }

    /// Evict cache entries if needed
    fn evict_if_needed(&mut self) -> Result<(), PredictiveLoaderError> {
        if !self.cache.is_full() {
            return Ok(());
        }

        let evicted = self.cache.evict(&self.eviction_policy);
        self.stats.evictions += evicted;

        Ok(())
    }

    /// Get holographic view at position
    pub fn get_view(
        &mut self,
        scale: ScaleLevel,
        position_log: Float,
        dimensions_log: (Float, Float, Float),
    ) -> Result<HolographicView, PredictiveLoaderError> {
        let cache_key = FractalCacheKey {
            entity_id: 0,
            scale_level: scale as usize,
            position_idx: self.quantize_position((position_log, 0.0, 0.0)),
            data_type: 0,
        };

        if self.cache.contains_key(&cache_key) {
            self.stats.cache_hits += 1;
        } else {
            self.stats.cache_misses += 1;
        }

        self.field
            .get_view(scale, position_log, dimensions_log)
            .map_err(|e| PredictiveLoaderError::FieldError(format!("{:?}", e)))
    }

    /// Get cache statistics
    pub fn cache_statistics(&self) -> FractalCacheStatistics {
        self.cache.statistics().clone()
    }

    /// Get predictive loader statistics
    pub fn statistics(&self) -> PredictiveLoaderStatistics {
        let mut stats = self.stats.clone();

        if stats.total_predictions > 0 {
            stats.prediction_accuracy =
                stats.correct_predictions as Float / stats.total_predictions as Float;
        }

        if stats.total_loads > 0 {
            stats.cache_hit_rate = stats.cache_hits as Float / stats.total_loads as Float;
        }

        stats
    }

    /// Reset statistics
    pub fn reset_statistics(&mut self) {
        self.stats = PredictiveLoaderStatistics::default();
    }
}

/// Prediction
///
/// Represents a predicted future state.
#[derive(Debug, Clone)]
struct Prediction {
    position: (Float, Float, Float),
    scale: ScaleLevel,
    #[allow(dead_code)]
    frame_offset: usize,
}

#[cfg(test)]
mod tests {
    use super::super::fractal_cache::EvictionPolicy;
    use super::*;

    #[test]
    fn test_predictive_loader_creation() {
        let cache = FractalCache::new(1000);
        let field = MultiScaleField::new();
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);

        let loader = PredictiveLoader::new(cache, field, camera);

        assert_eq!(loader.lookahead_frames, 10);
        assert_eq!(loader.confidence_threshold, 0.5);
    }

    #[test]
    fn test_predictive_loader_creation_with_defaults() {
        let loader = PredictiveLoader::new_with_defaults();

        assert_eq!(loader.lookahead_frames, 10);
        assert_eq!(loader.confidence_threshold, 0.5);
        assert_eq!(loader.max_queue_size, 1000);
    }

    #[test]
    fn test_update_player_state() {
        let cache = FractalCache::new(1000);
        let field = MultiScaleField::new();
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        let mut loader = PredictiveLoader::new(cache, field, camera);

        loader.update_player_state((1.0, 2.0, 3.0), (0.1, 0.2, 0.3));

        assert_eq!(loader.player_position, (1.0, 2.0, 3.0));
        assert_eq!(loader.player_velocity, (0.1, 0.2, 0.3));
    }

    #[test]
    fn test_eviction_policy_setting() {
        let cache = FractalCache::new(1000);
        let field = MultiScaleField::new();
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        let mut loader = PredictiveLoader::new(cache, field, camera);

        loader.set_eviction_policy(EvictionPolicy::Lfu);
        assert_eq!(loader.eviction_policy, EvictionPolicy::Lfu);

        loader.set_eviction_policy(EvictionPolicy::ScaleAware);
        assert_eq!(loader.eviction_policy, EvictionPolicy::ScaleAware);
    }

    #[test]
    fn test_priority_computation() {
        let cache = FractalCache::new(1000);
        let field = MultiScaleField::new();
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        let loader = PredictiveLoader::new(cache, field, camera);

        let critical = loader.compute_priority(0, 1.0, ScaleLevel::Biological);
        assert_eq!(critical, LoadPriority::Critical);

        let high = loader.compute_priority(1, 0.9, ScaleLevel::Biological);
        assert_eq!(high, LoadPriority::High);

        let low = loader.compute_priority(7, 0.5, ScaleLevel::Biological);
        assert_eq!(low, LoadPriority::Low);

        let background = loader.compute_priority(15, 0.5, ScaleLevel::Biological);
        assert_eq!(background, LoadPriority::Background);
    }

    #[test]
    fn test_position_quantization() {
        let cache = FractalCache::new(1000);
        let field = MultiScaleField::new();
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        let loader = PredictiveLoader::new(cache, field, camera);

        let pos1 = loader.quantize_position((1.05, 2.03, 3.07));
        // 1.05 / 0.1 = 10.5 -> round() = 11
        // 2.03 / 0.1 = 20.3 -> round() = 20
        // 3.07 / 0.1 = 30.7 -> round() = 31
        assert_eq!(pos1, (11, 20, 31));

        let pos2 = loader.quantize_position((1.00, 2.00, 3.00));
        assert_eq!(pos2, (10, 20, 30));
    }

    #[test]
    fn test_load_priority_ordering() {
        assert!(LoadPriority::Critical < LoadPriority::High);
        assert!(LoadPriority::High < LoadPriority::Medium);
        assert!(LoadPriority::Medium < LoadPriority::Low);
        assert!(LoadPriority::Low < LoadPriority::Background);
    }

    #[test]
    fn test_eviction_policy_equality() {
        assert_eq!(EvictionPolicy::Lru, EvictionPolicy::Lru);
        assert_eq!(EvictionPolicy::Lfu, EvictionPolicy::Lfu);
        assert_eq!(EvictionPolicy::Adaptive, EvictionPolicy::Adaptive);
        assert_eq!(EvictionPolicy::ScaleAware, EvictionPolicy::ScaleAware);

        assert_ne!(EvictionPolicy::Lru, EvictionPolicy::Lfu);
        assert_ne!(EvictionPolicy::Adaptive, EvictionPolicy::ScaleAware);
    }

    #[test]
    fn test_statistics_initialization() {
        let stats = PredictiveLoaderStatistics::default();

        assert_eq!(stats.total_predictions, 0);
        assert_eq!(stats.correct_predictions, 0);
        assert_eq!(stats.total_loads, 0);
        assert_eq!(stats.successful_loads, 0);
        assert_eq!(stats.failed_loads, 0);
        assert_eq!(stats.cache_hits, 0);
        assert_eq!(stats.cache_misses, 0);
        assert_eq!(stats.total_bytes_loaded, 0);
        assert_eq!(stats.evictions, 0);
    }

    #[test]
    fn test_scale_prediction_weights() {
        let cache = FractalCache::new(1000);
        let field = MultiScaleField::new();
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        let loader = PredictiveLoader::new(cache, field, camera);

        assert_eq!(loader.scale_prediction_weights.len(), 7);
        for &weight in &loader.scale_prediction_weights {
            assert!((0.0..=1.0).contains(&weight));
        }
    }

    #[test]
    fn test_max_configurations() {
        let cache = FractalCache::new(1000);
        let field = MultiScaleField::new();
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        let mut loader = PredictiveLoader::new(cache, field, camera);

        loader.set_max_queue_size(500);
        assert_eq!(loader.max_queue_size, 500);

        loader.set_max_concurrent_loads(5);
        assert_eq!(loader.max_concurrent_loads, 5);

        loader.set_lookahead_frames(20);
        assert_eq!(loader.lookahead_frames, 20);
    }

    #[test]
    fn test_statistics_computation() {
        let cache = FractalCache::new(1000);
        let field = MultiScaleField::new();
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        let mut loader = PredictiveLoader::new(cache, field, camera);

        loader.stats.total_predictions = 100;
        loader.stats.correct_predictions = 80;
        loader.stats.total_loads = 50;
        loader.stats.cache_hits = 40;

        let stats = loader.statistics();

        assert_eq!(stats.prediction_accuracy, 0.8);
        assert_eq!(stats.cache_hit_rate, 0.8);
    }

    #[test]
    fn test_statistics_reset() {
        let cache = FractalCache::new(1000);
        let field = MultiScaleField::new();
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        let mut loader = PredictiveLoader::new(cache, field, camera);

        loader.stats.total_predictions = 100;
        loader.stats.total_loads = 50;

        loader.reset_statistics();

        assert_eq!(loader.stats.total_predictions, 0);
        assert_eq!(loader.stats.total_loads, 0);
    }
}
