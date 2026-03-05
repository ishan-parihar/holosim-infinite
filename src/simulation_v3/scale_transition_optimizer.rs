//! Scale Transition Optimizer
//!
//! From MASTER_R&D_ROADMAP.md Week 21-24:
//! - Optimize scale transition to <50ms
//! - Implement smooth zoom interpolation
//! - Implement holographic continuity preservation
//! - Implement scale-specific rendering pipelines
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4.5:
//! "Optimize scale transitions through pre-loading, progressive refinement, and caching"
//!
//! This module implements:
//! 1. ScaleTransitionOptimizer - Main optimization orchestrator
//! 2. Transition cache for pre-loaded scale data
//! 3. Progressive refinement during transitions
//! 4. Holographic continuity preservation
//! 5. Performance metrics (target: <50ms)

use super::fractal_cache::FractalData;
use super::multiscale_camera::{InterpolatedScale, InterpolationMode, ScaleLevel};
use super::predictive_loading::PredictiveLoader;
use crate::types::Float;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Scale Transition Optimizer
///
/// From MASTER_R&D_ROADMAP.md Week 21-24:
/// "Optimize scale transition to <50ms"
///
/// Optimizes scale transitions through:
/// - Pre-loading target scale data before transition starts
/// - Progressive refinement during transition (LOD)
/// - Caching of transition paths
/// - Holographic continuity preservation
///
/// Performance target: <50ms for scale transitions
pub struct ScaleTransitionOptimizer {
    /// Transition cache for pre-loaded scale data
    transition_cache: HashMap<TransitionKey, TransitionCacheEntry>,

    /// Predictive loader for pre-loading data
    predictive_loader: PredictiveLoader,

    /// Active transition
    active_transition: Option<OptimizedTransition>,

    /// Maximum transition cache size
    max_cache_entries: usize,

    /// Target transition duration (<50ms for instant feel)
    target_duration: Duration,

    /// Progressive refinement enabled
    progressive_refinement: bool,

    /// Holographic continuity strength (0.0 to 1.0)
    continuity_strength: Float,

    /// Performance statistics
    stats: TransitionOptimizerStatistics,

    /// Pre-load radius (how much data to pre-load around target position)
    pre_load_radius: usize,
}

/// Transition Cache Key
///
/// Uniquely identifies a transition path between two scales.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransitionKey {
    /// Source scale level
    pub from_scale: ScaleLevel,

    /// Target scale level
    pub to_scale: ScaleLevel,

    /// Position index (quantized)
    pub position_idx: (i64, i64, i64),
}

/// Transition Cache Entry
///
/// Contains pre-loaded data for a transition path.
#[derive(Debug, Clone)]
pub struct TransitionCacheEntry {
    /// Source scale data
    from_data: Option<FractalData>,

    /// Target scale data
    to_data: Option<FractalData>,

    /// Intermediate scale data (for progressive refinement)
    #[allow(dead_code)]
    intermediate_data: Vec<Option<FractalData>>,

    /// Last access time
    last_access: Instant,

    /// Access count
    access_count: usize,

    /// Cache hit flag
    is_hit: bool,
}

/// Optimized Transition
///
/// Represents an active scale transition with optimizations.
#[derive(Debug, Clone)]
pub struct OptimizedTransition {
    /// Source scale
    pub from_scale: ScaleLevel,

    /// Target scale
    pub to_scale: ScaleLevel,

    /// Transition start time
    start_time: Instant,

    /// Target duration
    target_duration: Duration,

    /// Interpolation mode
    interpolation: InterpolationMode,

    /// Current progress (0.0 to 1.0)
    progress: Float,

    /// Transition cache entry (if pre-loaded)
    cache_entry: Option<TransitionCacheEntry>,

    /// Progressive refinement state
    refinement_state: RefinementState,

    /// Is the transition complete?
    complete: bool,

    /// Performance metrics
    metrics: TransitionMetrics,
}

/// Refinement State
///
/// Tracks progressive refinement during transition.
#[derive(Debug, Clone)]
pub struct RefinementState {
    /// Current refinement level (0 = coarsest, 7 = finest)
    current_level: usize,

    /// Target refinement level
    target_level: usize,

    /// Refinement progress (0.0 to 1.0)
    progress: Float,

    /// Refinement steps completed
    steps_completed: usize,

    /// Total refinement steps
    total_steps: usize,
}

/// Transition Metrics
///
/// Tracks performance metrics for a transition.
#[derive(Debug, Clone)]
pub struct TransitionMetrics {
    /// Transition start time
    pub start_time: Instant,

    /// Pre-load start time
    pub pre_load_start: Option<Instant>,

    /// Pre-load duration
    pub pre_load_duration: Option<Duration>,

    /// Transition duration
    pub transition_duration: Option<Duration>,

    /// Cache hit flag
    pub cache_hit: bool,

    /// Progressive refinement steps
    pub refinement_steps: usize,

    /// Data size loaded (bytes)
    pub data_size_loaded: usize,

    /// Frame count during transition
    pub frame_count: usize,
}

impl Default for TransitionMetrics {
    fn default() -> Self {
        TransitionMetrics {
            start_time: Instant::now(),
            pre_load_start: None,
            pre_load_duration: None,
            transition_duration: None,
            cache_hit: false,
            refinement_steps: 0,
            data_size_loaded: 0,
            frame_count: 0,
        }
    }
}

/// Transition Optimizer Statistics
///
/// From MASTER_R&D_ROADMAP.md Week 21-24:
/// "Performance tests for scale transitions (<50ms)"
///
/// Tracks performance metrics for the transition optimizer.
#[derive(Debug, Clone, Default)]
pub struct TransitionOptimizerStatistics {
    /// Total transitions initiated
    pub total_transitions: usize,

    /// Transitions completed within target time
    pub on_target_transitions: usize,

    /// Transitions that exceeded target time
    pub off_target_transitions: usize,

    /// Total transition duration
    pub total_transition_duration: Duration,

    /// Average transition duration
    pub average_transition_duration: Duration,

    /// Cache hits
    pub cache_hits: usize,

    /// Cache misses
    pub cache_misses: usize,

    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: Float,

    /// Total pre-load time
    pub total_pre_load_time: Duration,

    /// Average pre-load time
    pub average_pre_load_time: Duration,

    /// Progressive refinements
    pub progressive_refinements: usize,

    /// Holographic continuity violations
    pub continuity_violations: usize,
}

/// Scale Transition Optimizer Error
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionOptimizerError {
    /// Invalid scale transition
    InvalidTransition(String),

    /// Cache error
    CacheError(String),

    /// Transition in progress
    TransitionInProgress,

    /// Pre-load failed
    PreLoadFailed(String),

    /// Timeout exceeded
    TimeoutExceeded(Duration),
}

impl ScaleTransitionOptimizer {
    /// Create a new scale transition optimizer
    ///
    /// # Arguments
    /// - `predictive_loader` - Predictive loader for pre-loading data
    /// - `target_duration` - Target transition duration (<50ms recommended)
    pub fn new(predictive_loader: PredictiveLoader, target_duration: Duration) -> Self {
        ScaleTransitionOptimizer {
            transition_cache: HashMap::new(),
            predictive_loader,
            active_transition: None,
            max_cache_entries: 100,
            target_duration,
            progressive_refinement: true,
            continuity_strength: 1.0,
            stats: TransitionOptimizerStatistics::default(),
            pre_load_radius: 3,
        }
    }

    /// Create with default configuration
    pub fn new_with_defaults() -> Self {
        let predictive_loader = PredictiveLoader::new_with_defaults();
        let target_duration = Duration::from_millis(50);

        Self::new(predictive_loader, target_duration)
    }

    /// Set target transition duration
    pub fn set_target_duration(&mut self, duration: Duration) {
        self.target_duration = duration;
    }

    /// Set progressive refinement
    pub fn set_progressive_refinement(&mut self, enabled: bool) {
        self.progressive_refinement = enabled;
    }

    /// Set holographic continuity strength
    pub fn set_continuity_strength(&mut self, strength: Float) {
        self.continuity_strength = strength.clamp(0.0, 1.0);
    }

    /// Set pre-load radius
    pub fn set_pre_load_radius(&mut self, radius: usize) {
        self.pre_load_radius = radius.max(1);
    }

    /// Set maximum cache entries
    pub fn set_max_cache_entries(&mut self, max: usize) {
        self.max_cache_entries = max;
    }

    /// Begin an optimized scale transition
    ///
    /// From MASTER_R&D_ROADMAP.md Week 21-24:
    /// "Optimize scale transition to <50ms"
    ///
    /// # Arguments
    /// - `from_scale` - Source scale level
    /// - `to_scale` - Target scale level
    /// - `position_log` - Position in log space
    /// - `interpolation` - Interpolation mode
    pub fn begin_transition(
        &mut self,
        from_scale: ScaleLevel,
        to_scale: ScaleLevel,
        position_log: Float,
        interpolation: InterpolationMode,
    ) -> Result<(), TransitionOptimizerError> {
        if self.active_transition.is_some() {
            return Err(TransitionOptimizerError::TransitionInProgress);
        }

        if from_scale == to_scale {
            return Err(TransitionOptimizerError::InvalidTransition(
                "Source and target scales are the same".to_string(),
            ));
        }

        self.stats.total_transitions += 1;

        let transition_key = TransitionKey {
            from_scale,
            to_scale,
            position_idx: self.quantize_position(position_log),
        };

        let cache_entry = self.get_or_load_transition_data(&transition_key, position_log)?;

        let scale_distance = (to_scale.index() as i32 - from_scale.index() as i32).unsigned_abs() as usize;
        let total_steps = if self.progressive_refinement {
            scale_distance * 2
        } else {
            1
        };

        let start_time = Instant::now();
        let cache_hit = cache_entry.is_hit;

        self.active_transition = Some(OptimizedTransition {
            from_scale,
            to_scale,
            start_time,
            target_duration: self.target_duration,
            interpolation,
            progress: 0.0,
            cache_entry: Some(cache_entry),
            refinement_state: RefinementState {
                current_level: from_scale.index(),
                target_level: to_scale.index(),
                progress: 0.0,
                steps_completed: 0,
                total_steps,
            },
            complete: false,
            metrics: TransitionMetrics {
                start_time,
                pre_load_start: None,
                pre_load_duration: None,
                transition_duration: None,
                cache_hit,
                refinement_steps: 0,
                data_size_loaded: 0,
                frame_count: 0,
            },
        });

        Ok(())
    }

    /// Update the active transition
    ///
    /// Should be called each frame to update transition progress.
    pub fn update_transition(
        &mut self,
        _delta: Duration,
    ) -> Result<InterpolatedScale, TransitionOptimizerError> {
        let transition =
            self.active_transition
                .as_mut()
                .ok_or(TransitionOptimizerError::InvalidTransition(
                    "No active transition".to_string(),
                ))?;

        if transition.complete {
            return Ok(InterpolatedScale::new(
                transition.from_scale,
                transition.to_scale,
                1.0,
            ));
        }

        let elapsed = transition.start_time.elapsed();
        let progress = elapsed.as_secs_f64() / transition.target_duration.as_secs_f64();
        transition.progress = progress.clamp(0.0, 1.0);

        transition.metrics.frame_count += 1;

        if self.progressive_refinement {
            // Inline progressive refinement to avoid borrow conflict
            let refinement = &mut transition.refinement_state;
            let progress = transition.progress;

            if refinement.steps_completed < refinement.total_steps {
                let target_step = (progress * refinement.total_steps as Float)
                    .min(refinement.total_steps as Float)
                    as usize;

                while refinement.steps_completed < target_step {
                    refinement.steps_completed += 1;
                    transition.metrics.refinement_steps += 1;
                    self.stats.progressive_refinements += 1;

                    let level_fraction =
                        refinement.steps_completed as Float / refinement.total_steps as Float;
                    let level_delta = (refinement.target_level as Float
                        - refinement.current_level as Float)
                        * level_fraction;
                    refinement.current_level =
                        (refinement.current_level as Float + level_delta) as usize;
                    refinement.progress = level_fraction;
                }
            }
        }

        if transition.progress >= 1.0 {
            transition.complete = true;
            transition.metrics.transition_duration = Some(elapsed);

            if elapsed <= self.target_duration {
                self.stats.on_target_transitions += 1;
            } else {
                self.stats.off_target_transitions += 1;
            }

            self.stats.total_transition_duration += elapsed;
            let count = self.stats.on_target_transitions + self.stats.off_target_transitions;
            if count > 0 {
                self.stats.average_transition_duration =
                    self.stats.total_transition_duration / count as u32;
            }
        }

        let interpolated_progress = transition.interpolation.apply(transition.progress);

        Ok(InterpolatedScale::new(
            transition.from_scale,
            transition.to_scale,
            interpolated_progress,
        ))
    }

    /// Update progressive refinement
    #[allow(dead_code)]
    fn update_progressive_refinement(
        &mut self,
        transition: &mut OptimizedTransition,
    ) -> Result<(), TransitionOptimizerError> {
        let refinement = &mut transition.refinement_state;

        if refinement.steps_completed >= refinement.total_steps {
            return Ok(());
        }

        let progress = transition.progress;
        let target_step = (progress * refinement.total_steps as Float)
            .min(refinement.total_steps as Float) as usize;

        while refinement.steps_completed < target_step {
            refinement.steps_completed += 1;
            transition.metrics.refinement_steps += 1;
            self.stats.progressive_refinements += 1;

            let level_fraction =
                refinement.steps_completed as Float / refinement.total_steps as Float;
            let level_delta = (refinement.target_level as Float
                - refinement.current_level as Float)
                * level_fraction;
            refinement.current_level = (refinement.current_level as Float + level_delta) as usize;
            refinement.progress = level_fraction;
        }

        Ok(())
    }

    /// Get or load transition data
    fn get_or_load_transition_data(
        &mut self,
        key: &TransitionKey,
        position_log: Float,
    ) -> Result<TransitionCacheEntry, TransitionOptimizerError> {
        if let Some(entry) = self.transition_cache.get_mut(key) {
            entry.last_access = Instant::now();
            entry.access_count += 1;
            entry.is_hit = true;
            self.stats.cache_hits += 1;
            return Ok(entry.clone());
        }

        self.stats.cache_misses += 1;

        let from_data = self.load_scale_data(key.from_scale, position_log)?;
        let to_data = self.load_scale_data(key.to_scale, position_log)?;

        let scale_distance =
            (key.to_scale.index() as i32 - key.from_scale.index() as i32).unsigned_abs() as usize;
        let intermediate_levels = scale_distance.saturating_sub(1).min(5);
        let mut intermediate_data = Vec::with_capacity(intermediate_levels);

        for i in 1..=intermediate_levels {
            let fraction = i as Float / (intermediate_levels + 1) as Float;
            let level_index = key.from_scale.index()
                + ((key.to_scale.index() as i32 - key.from_scale.index() as i32) as Float
                    * fraction) as usize;
            let intermediate_scale = ScaleLevel::from_index(level_index);

            if let Some(scale) = intermediate_scale {
                let data = self.load_scale_data(scale, position_log)?;
                intermediate_data.push(Some(data));
            } else {
                intermediate_data.push(None);
            }
        }

        let entry = TransitionCacheEntry {
            from_data: Some(from_data),
            to_data: Some(to_data),
            intermediate_data,
            last_access: Instant::now(),
            access_count: 1,
            is_hit: false,
        };

        if self.transition_cache.len() >= self.max_cache_entries {
            self.evict_oldest_entry();
        }

        self.transition_cache.insert(key.clone(), entry.clone());

        Ok(entry)
    }

    /// Load scale data
    fn load_scale_data(
        &mut self,
        scale: ScaleLevel,
        position_log: Float,
    ) -> Result<FractalData, TransitionOptimizerError> {
        let view = self
            .predictive_loader
            .get_view(scale, position_log, (5.0, 5.0, 5.0))
            .map_err(|e| TransitionOptimizerError::PreLoadFailed(format!("{:?}", e)))?;

        Ok(FractalData {
            data: view.field_data.clone(),
            shape: view.field_shape.clone(),
            quality: view.quality,
            compression_ratio: 1.0,
            size_bytes: view.field_data.len() * std::mem::size_of::<Float>(),
        })
    }

    /// Evict oldest cache entry
    fn evict_oldest_entry(&mut self) {
        if self.transition_cache.is_empty() {
            return;
        }

        let mut oldest_key = None;
        let mut oldest_time = Instant::now();

        for (key, entry) in self.transition_cache.iter() {
            if entry.last_access < oldest_time {
                oldest_time = entry.last_access;
                oldest_key = Some(key.clone());
            }
        }

        if let Some(key) = oldest_key {
            self.transition_cache.remove(&key);
        }
    }

    /// Quantize position for cache key
    fn quantize_position(&self, position_log: Float) -> (i64, i64, i64) {
        const QUANTUM: Float = 0.1;
        let x = (position_log / QUANTUM).floor() as i64;
        (x, 0, 0)
    }

    /// Check if transition is in progress
    pub fn is_transitioning(&self) -> bool {
        self.active_transition.is_some()
            && !self
                .active_transition
                .as_ref()
                .map(|t| t.complete)
                .unwrap_or(true)
    }

    /// Get current transition progress (0.0 to 1.0)
    pub fn transition_progress(&self) -> Float {
        self.active_transition
            .as_ref()
            .map(|t| t.progress)
            .unwrap_or(0.0)
    }

    /// Get current transition metrics
    pub fn transition_metrics(&self) -> Option<TransitionMetrics> {
        self.active_transition.as_ref().map(|t| t.metrics.clone())
    }

    /// Get transition optimizer statistics
    pub fn statistics(&self) -> TransitionOptimizerStatistics {
        let mut stats = self.stats.clone();

        let total_cache_ops = stats.cache_hits + stats.cache_misses;
        if total_cache_ops > 0 {
            stats.cache_hit_rate = stats.cache_hits as Float / total_cache_ops as Float;
        }

        stats
    }

    /// Reset statistics
    pub fn reset_statistics(&mut self) {
        self.stats = TransitionOptimizerStatistics::default();
    }

    /// Clear transition cache
    pub fn clear_cache(&mut self) {
        self.transition_cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.transition_cache.len()
    }

    /// Check holographic continuity
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Each entity contains within it all densities and sub-densities of the octave"
    ///
    /// Validates that holographic continuity is maintained during transition.
    pub fn check_holographic_continuity(&self) -> bool {
        if let Some(ref transition) = self.active_transition {
            if let Some(ref cache_entry) = transition.cache_entry {
                let from_size = cache_entry
                    .from_data
                    .as_ref()
                    .map(|d| d.data.len())
                    .unwrap_or(0);
                let to_size = cache_entry
                    .to_data
                    .as_ref()
                    .map(|d| d.data.len())
                    .unwrap_or(0);

                let size_ratio = if from_size > 0 && to_size > 0 {
                    (from_size as Float / to_size as Float)
                        .min(to_size as Float / from_size as Float)
                } else {
                    0.0
                };

                return size_ratio >= self.continuity_strength;
            }
        }

        true
    }

    /// Pre-load transition for future use
    ///
    /// Pre-loads transition data into cache for faster future transitions.
    pub fn pre_load_transition(
        &mut self,
        from_scale: ScaleLevel,
        to_scale: ScaleLevel,
        position_log: Float,
    ) -> Result<(), TransitionOptimizerError> {
        let key = TransitionKey {
            from_scale,
            to_scale,
            position_idx: self.quantize_position(position_log),
        };

        self.get_or_load_transition_data(&key, position_log)?;
        Ok(())
    }

    /// Pre-load all transitions from current scale
    ///
    /// Pre-loads transitions to adjacent scales for smoother navigation.
    pub fn pre_load_adjacent_scales(
        &mut self,
        current_scale: ScaleLevel,
        position_log: Float,
    ) -> Result<(), TransitionOptimizerError> {
        if let Some(next_scale) = current_scale.next() {
            self.pre_load_transition(current_scale, next_scale, position_log)?;
        }

        if let Some(prev_scale) = current_scale.prev() {
            self.pre_load_transition(current_scale, prev_scale, position_log)?;
        }

        Ok(())
    }
}

impl Default for ScaleTransitionOptimizer {
    fn default() -> Self {
        Self::new_with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_optimizer_creation() {
        let optimizer = ScaleTransitionOptimizer::new_with_defaults();

        assert_eq!(optimizer.target_duration, Duration::from_millis(50));
        assert!(!optimizer.is_transitioning());
        assert_eq!(optimizer.transition_progress(), 0.0);
    }

    #[test]
    fn test_transition_key_creation() {
        let key = TransitionKey {
            from_scale: ScaleLevel::Biological,
            to_scale: ScaleLevel::Planetary,
            position_idx: (10, 0, 0),
        };

        assert_eq!(key.from_scale, ScaleLevel::Biological);
        assert_eq!(key.to_scale, ScaleLevel::Planetary);
        assert_eq!(key.position_idx, (10, 0, 0));
    }

    #[test]
    fn test_begin_transition() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        let result = optimizer.begin_transition(
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            0.0,
            InterpolationMode::Smooth,
        );

        assert!(result.is_ok());
        assert!(optimizer.is_transitioning());
        assert!(optimizer.transition_progress() >= 0.0);
    }

    #[test]
    fn test_begin_transition_same_scale() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        let result = optimizer.begin_transition(
            ScaleLevel::Biological,
            ScaleLevel::Biological,
            0.0,
            InterpolationMode::Smooth,
        );

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(TransitionOptimizerError::InvalidTransition(_))
        ));
    }

    #[test]
    fn test_begin_transition_in_progress() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        let _ = optimizer.begin_transition(
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            0.0,
            InterpolationMode::Smooth,
        );

        let result = optimizer.begin_transition(
            ScaleLevel::Planetary,
            ScaleLevel::Stellar,
            0.0,
            InterpolationMode::Smooth,
        );

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(TransitionOptimizerError::TransitionInProgress)
        ));
    }

    #[test]
    fn test_update_transition() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        let _ = optimizer.begin_transition(
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            0.0,
            InterpolationMode::Smooth,
        );

        let delta = Duration::from_millis(10);
        let result = optimizer.update_transition(delta);

        assert!(result.is_ok());
        assert!(optimizer.transition_progress() >= 0.0);
    }

    #[test]
    fn test_update_transition_no_active() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        let delta = Duration::from_millis(10);
        let result = optimizer.update_transition(delta);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(TransitionOptimizerError::InvalidTransition(_))
        ));
    }

    #[test]
    fn test_position_quantization() {
        let optimizer = ScaleTransitionOptimizer::new_with_defaults();

        let pos1 = optimizer.quantize_position(1.05);
        assert_eq!(pos1, (10, 0, 0));

        let pos2 = optimizer.quantize_position(1.00);
        assert_eq!(pos2, (10, 0, 0));
    }

    #[test]
    fn test_statistics_initialization() {
        let stats = TransitionOptimizerStatistics::default();

        assert_eq!(stats.total_transitions, 0);
        assert_eq!(stats.on_target_transitions, 0);
        assert_eq!(stats.off_target_transitions, 0);
        assert_eq!(stats.cache_hits, 0);
        assert_eq!(stats.cache_misses, 0);
        assert_eq!(stats.cache_hit_rate, 0.0);
    }

    #[test]
    fn test_refinement_state_creation() {
        let state = RefinementState {
            current_level: 2,
            target_level: 3,
            progress: 0.0,
            steps_completed: 0,
            total_steps: 2,
        };

        assert_eq!(state.current_level, 2);
        assert_eq!(state.target_level, 3);
        assert_eq!(state.progress, 0.0);
        assert_eq!(state.steps_completed, 0);
        assert_eq!(state.total_steps, 2);
    }

    #[test]
    fn test_transition_metrics_creation() {
        let metrics = TransitionMetrics::default();

        assert!(metrics.pre_load_start.is_none());
        assert!(metrics.pre_load_duration.is_none());
        assert!(metrics.transition_duration.is_none());
        assert!(!metrics.cache_hit);
        assert_eq!(metrics.refinement_steps, 0);
    }

    #[test]
    fn test_target_duration_setting() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        optimizer.set_target_duration(Duration::from_millis(30));
        assert_eq!(optimizer.target_duration, Duration::from_millis(30));
    }

    #[test]
    fn test_progressive_refinement_setting() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        optimizer.set_progressive_refinement(false);
        assert!(!optimizer.progressive_refinement);

        optimizer.set_progressive_refinement(true);
        assert!(optimizer.progressive_refinement);
    }

    #[test]
    fn test_continuity_strength_setting() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        optimizer.set_continuity_strength(0.8);
        assert_eq!(optimizer.continuity_strength, 0.8);

        optimizer.set_continuity_strength(1.5);
        assert_eq!(optimizer.continuity_strength, 1.0);

        optimizer.set_continuity_strength(-0.5);
        assert_eq!(optimizer.continuity_strength, 0.0);
    }

    #[test]
    fn test_pre_load_radius_setting() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        optimizer.set_pre_load_radius(5);
        assert_eq!(optimizer.pre_load_radius, 5);

        optimizer.set_pre_load_radius(0);
        assert_eq!(optimizer.pre_load_radius, 1);
    }

    #[test]
    fn test_max_cache_entries_setting() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        optimizer.set_max_cache_entries(200);
        assert_eq!(optimizer.max_cache_entries, 200);
    }

    #[test]
    fn test_cache_operations() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        assert_eq!(optimizer.cache_size(), 0);

        optimizer.clear_cache();
        assert_eq!(optimizer.cache_size(), 0);
    }

    #[test]
    fn test_holographic_continuity_check() {
        let optimizer = ScaleTransitionOptimizer::new_with_defaults();

        let result = optimizer.check_holographic_continuity();
        assert!(result);
    }

    #[test]
    fn test_statistics_computation() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        optimizer.stats.total_transitions = 100;
        optimizer.stats.on_target_transitions = 95;
        optimizer.stats.off_target_transitions = 5;
        optimizer.stats.cache_hits = 80;
        optimizer.stats.cache_misses = 20;

        let stats = optimizer.statistics();

        assert_eq!(stats.total_transitions, 100);
        assert_eq!(stats.on_target_transitions, 95);
        assert_eq!(stats.off_target_transitions, 5);
        assert_eq!(stats.cache_hit_rate, 0.8);
    }

    #[test]
    fn test_statistics_reset() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        optimizer.stats.total_transitions = 100;
        optimizer.stats.cache_hits = 50;

        optimizer.reset_statistics();

        assert_eq!(optimizer.stats.total_transitions, 0);
        assert_eq!(optimizer.stats.cache_hits, 0);
    }

    #[test]
    fn test_pre_load_adjacent_scales() {
        let mut optimizer = ScaleTransitionOptimizer::new_with_defaults();

        let result = optimizer.pre_load_adjacent_scales(ScaleLevel::Biological, 0.0);

        assert!(result.is_ok());
    }

    #[test]
    fn test_interpolated_scale_creation() {
        let interpolated =
            InterpolatedScale::new(ScaleLevel::Biological, ScaleLevel::Planetary, 0.5);

        assert_eq!(interpolated.from_scale(), ScaleLevel::Biological);
        assert_eq!(interpolated.to_scale(), ScaleLevel::Planetary);
        assert_eq!(interpolated.factor(), 0.5);
    }
}
