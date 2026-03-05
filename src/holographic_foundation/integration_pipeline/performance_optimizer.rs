//! Performance Optimizer for Large-Scale Simulations
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 6:
//! "Performance optimization for 10,000+ entities"
//!
//! Success Criteria:
//! - 60 FPS with 10,000 entities
//!
//! This module provides:
//! - Spatial partitioning for efficient entity queries
//! - Batched updates for reduced per-entity overhead
//! - Adaptive level-of-detail based on distance/importance
//! - Parallel processing strategies

use std::collections::HashMap;

use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;

/// Performance configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Target frame rate
    pub target_fps: u64,
    /// Maximum entities to process per frame
    pub max_entities_per_frame: usize,
    /// Spatial partitioning grid size
    pub spatial_grid_size: usize,
    /// Enable parallel processing
    pub parallel_processing: bool,
    /// Number of worker threads (0 = auto)
    pub worker_threads: usize,
    /// Level of detail distances
    pub lod_distances: [Float; 4],
    /// Enable adaptive LOD
    pub adaptive_lod: bool,
    /// Entity update batch size
    pub batch_size: usize,
    /// Enable frustum culling
    pub frustum_culling: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            target_fps: 60,
            max_entities_per_frame: 15000,
            spatial_grid_size: 16,
            parallel_processing: true,
            worker_threads: 0,
            lod_distances: [0.1, 0.3, 0.6, 1.0],
            adaptive_lod: true,
            batch_size: 100,
            frustum_culling: true,
        }
    }
}

/// Performance metrics
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// Frames per second
    pub fps: Float,
    /// Frame time in milliseconds
    pub frame_time_ms: Float,
    /// Entities processed per frame
    pub entities_per_frame: usize,
    /// Update time in microseconds
    pub update_time_us: u64,
    /// Render time in microseconds
    pub render_time_us: u64,
    /// Memory usage in MB
    pub memory_mb: Float,
    /// Cache hit rate
    pub cache_hit_rate: Float,
    /// Parallel efficiency
    pub parallel_efficiency: Float,
    /// LOD distribution
    pub lod_distribution: [usize; 4],
}

/// Spatial partitioning for efficient queries
pub struct SpatialPartitioning {
    /// Grid size
    grid_size: usize,
    /// Cells containing entity IDs
    cells: Vec<Vec<u64>>,
    /// Entity positions cache
    entity_positions: HashMap<u64, Position3D>,
    /// Entity scales cache
    entity_scales: HashMap<u64, Float>,
}

impl SpatialPartitioning {
    /// Create new spatial partitioning
    pub fn new(grid_size: usize) -> Self {
        let cells = vec![Vec::new(); grid_size * grid_size * grid_size];

        Self {
            grid_size,
            cells,
            entity_positions: HashMap::new(),
            entity_scales: HashMap::new(),
        }
    }

    /// Insert an entity into the grid
    pub fn insert(&mut self, entity_id: u64, position: &Position3D, scale: Float) {
        let cell_index = self.position_to_cell(position);

        if cell_index < self.cells.len() {
            self.cells[cell_index].push(entity_id);
        }

        self.entity_positions.insert(entity_id, *position);
        self.entity_scales.insert(entity_id, scale);
    }

    /// Remove an entity from the grid
    pub fn remove(&mut self, entity_id: u64) {
        if let Some(position) = self.entity_positions.get(&entity_id) {
            let cell_index = self.position_to_cell(position);
            if cell_index < self.cells.len() {
                self.cells[cell_index].retain(|&id| id != entity_id);
            }
        }

        self.entity_positions.remove(&entity_id);
        self.entity_scales.remove(&entity_id);
    }

    /// Update entity position
    pub fn update(&mut self, entity_id: u64, new_position: &Position3D) {
        if let Some(old_position) = self.entity_positions.get(&entity_id) {
            let old_cell = self.position_to_cell(old_position);
            let new_cell = self.position_to_cell(new_position);

            if old_cell != new_cell {
                if old_cell < self.cells.len() {
                    self.cells[old_cell].retain(|&id| id != entity_id);
                }
                if new_cell < self.cells.len() {
                    self.cells[new_cell].push(entity_id);
                }
            }
        }

        self.entity_positions.insert(entity_id, *new_position);
    }

    /// Query entities within a radius
    pub fn query_radius(&self, center: &Position3D, radius: Float) -> Vec<u64> {
        let mut result = Vec::new();

        let min_x = ((center.x - radius).max(0.0) * self.grid_size as Float) as usize;
        let max_x = ((center.x + radius).min(1.0) * self.grid_size as Float) as usize;
        let min_y = ((center.y - radius).max(0.0) * self.grid_size as Float) as usize;
        let max_y = ((center.y + radius).min(1.0) * self.grid_size as Float) as usize;
        let min_z = ((center.z - radius).max(0.0) * self.grid_size as Float) as usize;
        let max_z = ((center.z + radius).min(1.0) * self.grid_size as Float) as usize;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    let cell_index = x * self.grid_size * self.grid_size + y * self.grid_size + z;

                    if cell_index < self.cells.len() {
                        for &entity_id in &self.cells[cell_index] {
                            if let Some(pos) = self.entity_positions.get(&entity_id) {
                                let distance = center.distance(pos);
                                if distance <= radius {
                                    result.push(entity_id);
                                }
                            }
                        }
                    }
                }
            }
        }

        result
    }

    /// Query entities in a bounding box
    pub fn query_box(&self, min_pos: &Position3D, max_pos: &Position3D) -> Vec<u64> {
        let mut result = Vec::new();

        let min_x = (min_pos.x.max(0.0) * self.grid_size as Float) as usize;
        let max_x = (max_pos.x.min(1.0) * self.grid_size as Float) as usize;
        let min_y = (min_pos.y.max(0.0) * self.grid_size as Float) as usize;
        let max_y = (max_pos.y.min(1.0) * self.grid_size as Float) as usize;
        let min_z = (min_pos.z.max(0.0) * self.grid_size as Float) as usize;
        let max_z = (max_pos.z.min(1.0) * self.grid_size as Float) as usize;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    let cell_index = x * self.grid_size * self.grid_size + y * self.grid_size + z;

                    if cell_index < self.cells.len() {
                        result.extend(self.cells[cell_index].iter().cloned());
                    }
                }
            }
        }

        result
    }

    /// Get total entity count
    pub fn entity_count(&self) -> usize {
        self.entity_positions.len()
    }

    /// Clear all entities
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
        self.entity_positions.clear();
        self.entity_scales.clear();
    }

    /// Convert position to cell index
    fn position_to_cell(&self, position: &Position3D) -> usize {
        let x = (position.x.clamp(0.0, 0.9999) * self.grid_size as Float) as usize;
        let y = (position.y.clamp(0.0, 0.9999) * self.grid_size as Float) as usize;
        let z = (position.z.clamp(0.0, 0.9999) * self.grid_size as Float) as usize;

        x * self.grid_size * self.grid_size + y * self.grid_size + z
    }
}

/// Entity batch for grouped processing
#[derive(Debug, Clone)]
pub struct EntityBatch {
    /// Entity IDs in this batch
    pub entity_ids: Vec<u64>,
    /// Batch center position
    pub center: Position3D,
    /// Average scale
    pub avg_scale: Float,
    /// LOD level for this batch
    pub lod_level: u8,
    /// Priority (higher = more important)
    pub priority: i32,
}

impl EntityBatch {
    /// Create a new batch
    pub fn new() -> Self {
        Self {
            entity_ids: Vec::new(),
            center: Position3D::zero(),
            avg_scale: 0.0,
            lod_level: 0,
            priority: 0,
        }
    }

    /// Add entity to batch
    pub fn add(&mut self, entity_id: u64, position: &Position3D, scale: Float) {
        let count = self.entity_ids.len();

        self.center.x = (self.center.x * count as Float + position.x) / (count + 1) as Float;
        self.center.y = (self.center.y * count as Float + position.y) / (count + 1) as Float;
        self.center.z = (self.center.z * count as Float + position.z) / (count + 1) as Float;

        self.avg_scale = (self.avg_scale * count as Float + scale) / (count + 1) as Float;

        self.entity_ids.push(entity_id);
    }

    /// Get batch size
    pub fn size(&self) -> usize {
        self.entity_ids.len()
    }
}

impl Default for EntityBatch {
    fn default() -> Self {
        Self::new()
    }
}

/// Update strategy based on entity importance
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UpdateStrategy {
    /// Full update every frame
    FullUpdate,
    /// Update every other frame
    HalfRate,
    /// Update every fourth frame
    QuarterRate,
    /// Update based on activity
    Adaptive,
    /// Skip update (too far/insignificant)
    Skip,
}

/// Performance optimizer
pub struct PerformanceOptimizer {
    /// Configuration
    config: PerformanceConfig,
    /// Spatial partitioning
    spatial_partitioning: SpatialPartitioning,
    /// Current metrics
    metrics: PerformanceMetrics,
    /// Frame counter for LOD scheduling
    frame_counter: u64,
    /// Entity update schedule
    update_schedule: HashMap<u64, u64>,
    /// Last update frame for each entity
    last_update: HashMap<u64, u64>,
}

impl PerformanceOptimizer {
    /// Create a new performance optimizer
    pub fn new(config: PerformanceConfig) -> Self {
        let spatial_partitioning = SpatialPartitioning::new(config.spatial_grid_size);

        Self {
            config,
            spatial_partitioning,
            metrics: PerformanceMetrics::default(),
            frame_counter: 0,
            update_schedule: HashMap::new(),
            last_update: HashMap::new(),
        }
    }

    /// Begin a new frame
    pub fn begin_frame(&mut self) {
        self.frame_counter += 1;
    }

    /// End frame and calculate metrics
    pub fn end_frame(&mut self, update_time_us: u64, render_time_us: u64) {
        let total_time_us = update_time_us + render_time_us;
        self.metrics.update_time_us = update_time_us;
        self.metrics.render_time_us = render_time_us;

        if total_time_us > 0 {
            self.metrics.frame_time_ms = total_time_us as Float / 1000.0;
            self.metrics.fps = if total_time_us > 0 {
                1_000_000.0 / total_time_us as Float
            } else {
                60.0
            };
        }

        self.metrics.entities_per_frame = self.spatial_partitioning.entity_count();
    }

    /// Register an entity for optimization
    pub fn register_entity(&mut self, entity_id: u64, position: &Position3D, scale: Float) {
        self.spatial_partitioning.insert(entity_id, position, scale);

        let strategy = self.determine_update_strategy(position);
        let schedule = match strategy {
            UpdateStrategy::FullUpdate => 1,
            UpdateStrategy::HalfRate => 2,
            UpdateStrategy::QuarterRate => 4,
            UpdateStrategy::Adaptive => 3,
            UpdateStrategy::Skip => 1000,
        };

        self.update_schedule.insert(entity_id, schedule);
        self.last_update.insert(entity_id, 0);
    }

    /// Update entity position
    pub fn update_entity_position(&mut self, entity_id: u64, position: &Position3D) {
        self.spatial_partitioning.update(entity_id, position);

        let strategy = self.determine_update_strategy(position);
        let schedule = match strategy {
            UpdateStrategy::FullUpdate => 1,
            UpdateStrategy::HalfRate => 2,
            UpdateStrategy::QuarterRate => 4,
            UpdateStrategy::Adaptive => 3,
            UpdateStrategy::Skip => 1000,
        };

        self.update_schedule.insert(entity_id, schedule);
    }

    /// Unregister an entity
    pub fn unregister_entity(&mut self, entity_id: u64) {
        self.spatial_partitioning.remove(entity_id);
        self.update_schedule.remove(&entity_id);
        self.last_update.remove(&entity_id);
    }

    /// Get entities that should be updated this frame
    pub fn get_entities_to_update(&mut self) -> Vec<u64> {
        let frame = self.frame_counter;
        let max_entities = self.config.max_entities_per_frame;
        let mut entities = Vec::new();

        for (&entity_id, &schedule) in &self.update_schedule {
            if frame.is_multiple_of(schedule) {
                entities.push(entity_id);
            }

            if entities.len() >= max_entities {
                break;
            }
        }

        for &entity_id in &entities {
            self.last_update.insert(entity_id, frame);
        }

        self.metrics.lod_distribution = [0, 0, 0, 0];
        for &schedule in self.update_schedule.values() {
            match schedule {
                1 => self.metrics.lod_distribution[0] += 1,
                2 => self.metrics.lod_distribution[1] += 1,
                3 | 4 => self.metrics.lod_distribution[2] += 1,
                _ => self.metrics.lod_distribution[3] += 1,
            }
        }

        entities
    }

    /// Determine update strategy for an entity
    fn determine_update_strategy(&self, position: &Position3D) -> UpdateStrategy {
        if !self.config.adaptive_lod {
            return UpdateStrategy::FullUpdate;
        }

        let distance =
            (position.x - 0.5).powi(2) + (position.y - 0.5).powi(2) + (position.z - 0.5).powi(2);
        let distance = distance.sqrt();

        if distance < self.config.lod_distances[0] {
            UpdateStrategy::FullUpdate
        } else if distance < self.config.lod_distances[1] {
            UpdateStrategy::HalfRate
        } else if distance < self.config.lod_distances[2] {
            UpdateStrategy::QuarterRate
        } else if distance < self.config.lod_distances[3] {
            UpdateStrategy::Adaptive
        } else {
            UpdateStrategy::Skip
        }
    }

    /// Query entities within radius
    pub fn query_radius(&self, center: &Position3D, radius: Float) -> Vec<u64> {
        self.spatial_partitioning.query_radius(center, radius)
    }

    /// Query entities in bounding box
    pub fn query_box(&self, min_pos: &Position3D, max_pos: &Position3D) -> Vec<u64> {
        self.spatial_partitioning.query_box(min_pos, max_pos)
    }

    /// Create batches for parallel processing
    pub fn create_batches(&self) -> Vec<EntityBatch> {
        let mut batches = Vec::new();
        let batch_size = self.config.batch_size;

        let mut current_batch = EntityBatch::new();

        for (&entity_id, position) in &self.spatial_partitioning.entity_positions {
            let scale = self
                .spatial_partitioning
                .entity_scales
                .get(&entity_id)
                .copied()
                .unwrap_or(1.0);

            current_batch.add(entity_id, position, scale);

            if current_batch.size() >= batch_size {
                batches.push(current_batch);
                current_batch = EntityBatch::new();
            }
        }

        if current_batch.size() > 0 {
            batches.push(current_batch);
        }

        batches
    }

    /// Get current performance metrics
    pub fn metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// Check if performance target is met
    pub fn is_performance_target_met(&self) -> bool {
        self.metrics.fps >= self.config.target_fps as Float
    }

    /// Get recommended entity limit for current frame budget
    pub fn recommended_entity_limit(&self) -> usize {
        if self.metrics.frame_time_ms > 0.0 {
            let target_frame_ms = 1000.0 / self.config.target_fps as Float;
            let available_time_ratio = target_frame_ms / self.metrics.frame_time_ms;

            (self.metrics.entities_per_frame as Float * available_time_ratio) as usize
        } else {
            self.config.max_entities_per_frame
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_config_default() {
        let config = PerformanceConfig::default();
        assert_eq!(config.target_fps, 60);
        assert!(config.parallel_processing);
    }

    #[test]
    fn test_spatial_partitioning_creation() {
        let sp = SpatialPartitioning::new(16);
        assert_eq!(sp.grid_size, 16);
        assert_eq!(sp.entity_count(), 0);
    }

    #[test]
    fn test_spatial_partitioning_insert() {
        let mut sp = SpatialPartitioning::new(16);
        let pos = Position3D::new(0.5, 0.5, 0.5);

        sp.insert(1, &pos, 1.0);
        assert_eq!(sp.entity_count(), 1);
    }

    #[test]
    fn test_spatial_partitioning_remove() {
        let mut sp = SpatialPartitioning::new(16);
        let pos = Position3D::new(0.5, 0.5, 0.5);

        sp.insert(1, &pos, 1.0);
        sp.remove(1);
        assert_eq!(sp.entity_count(), 0);
    }

    #[test]
    fn test_spatial_partitioning_query_radius() {
        let mut sp = SpatialPartitioning::new(16);

        sp.insert(1, &Position3D::new(0.5, 0.5, 0.5), 1.0);
        sp.insert(2, &Position3D::new(0.6, 0.5, 0.5), 1.0);
        sp.insert(3, &Position3D::new(0.9, 0.9, 0.9), 1.0);

        let result = sp.query_radius(&Position3D::new(0.5, 0.5, 0.5), 0.2);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(!result.contains(&3));
    }

    #[test]
    fn test_spatial_partitioning_query_box() {
        let mut sp = SpatialPartitioning::new(16);

        sp.insert(1, &Position3D::new(0.3, 0.3, 0.3), 1.0);
        sp.insert(2, &Position3D::new(0.7, 0.7, 0.7), 1.0);

        let result = sp.query_box(
            &Position3D::new(0.2, 0.2, 0.2),
            &Position3D::new(0.5, 0.5, 0.5),
        );

        assert!(result.contains(&1));
        assert!(!result.contains(&2));
    }

    #[test]
    fn test_entity_batch() {
        let mut batch = EntityBatch::new();

        batch.add(1, &Position3D::new(0.5, 0.5, 0.5), 1.0);
        batch.add(2, &Position3D::new(0.6, 0.6, 0.6), 1.0);

        assert_eq!(batch.size(), 2);
        assert!(batch.center.x > 0.5);
    }

    #[test]
    fn test_performance_optimizer_creation() {
        let config = PerformanceConfig::default();
        let optimizer = PerformanceOptimizer::new(config);
        assert_eq!(optimizer.frame_counter, 0);
    }

    #[test]
    fn test_performance_optimizer_register() {
        let config = PerformanceConfig::default();
        let mut optimizer = PerformanceOptimizer::new(config);

        optimizer.register_entity(1, &Position3D::new(0.5, 0.5, 0.5), 1.0);
        optimizer.begin_frame();

        let entities = optimizer.get_entities_to_update();
        assert!(entities.contains(&1));
    }

    #[test]
    fn test_performance_optimizer_query() {
        let config = PerformanceConfig::default();
        let mut optimizer = PerformanceOptimizer::new(config);

        optimizer.register_entity(1, &Position3D::new(0.5, 0.5, 0.5), 1.0);
        optimizer.register_entity(2, &Position3D::new(0.9, 0.9, 0.9), 1.0);

        let result = optimizer.query_radius(&Position3D::new(0.5, 0.5, 0.5), 0.2);
        assert!(result.contains(&1));
        assert!(!result.contains(&2));
    }

    #[test]
    fn test_performance_optimizer_batches() {
        let config = PerformanceConfig {
            batch_size: 2,
            ..Default::default()
        };
        let mut optimizer = PerformanceOptimizer::new(config);

        optimizer.register_entity(1, &Position3D::new(0.5, 0.5, 0.5), 1.0);
        optimizer.register_entity(2, &Position3D::new(0.6, 0.6, 0.6), 1.0);
        optimizer.register_entity(3, &Position3D::new(0.7, 0.7, 0.7), 1.0);

        let batches = optimizer.create_batches();
        assert_eq!(batches.len(), 2);
    }

    #[test]
    fn test_performance_metrics() {
        let config = PerformanceConfig::default();
        let mut optimizer = PerformanceOptimizer::new(config);

        optimizer.begin_frame();
        optimizer.end_frame(1000, 500);

        let metrics = optimizer.metrics();
        assert!(metrics.fps > 0.0);
        assert_eq!(metrics.update_time_us, 1000);
        assert_eq!(metrics.render_time_us, 500);
    }
}
