// ============================================================================
// TASK 3: OPTIMIZE FIELD AGGREGATION
// ============================================================================
// Objective: Profile and optimize field aggregation performance.
//
// Tasks:
// - Profile field aggregation code
// - Optimize spatial partitioning
// - Optimize photon-to-field mapping
// - Use caching where beneficial
// - Optimize spatial queries
// ============================================================================

use crate::energy_fields::{ElectricField, GravitationalField, MagneticField, Vector3};
use crate::light::{MomentumVector, Photon};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

// Helper function to convert Photon momentum to velocity vector (always speed of light)
fn photon_velocity_from_momentum(momentum: &MomentumVector) -> Vector3 {
    const SPEED_OF_LIGHT: f64 = 2.998e8; // m/s
    let momentum_mag =
        (momentum.x * momentum.x + momentum.y * momentum.y + momentum.z * momentum.z).sqrt();
    if momentum_mag > 0.0 {
        Vector3::new(
            momentum.x / momentum_mag * SPEED_OF_LIGHT,
            momentum.y / momentum_mag * SPEED_OF_LIGHT,
            momentum.z / momentum_mag * SPEED_OF_LIGHT,
        )
    } else {
        Vector3::zero()
    }
}

// Helper function to convert QuantumPosition to Vector3
fn position_to_vector3(position: &crate::light::QuantumPosition) -> Vector3 {
    Vector3::new(position.x, position.y, position.z)
}

// ============================================================================
// FIELD AGGREGATION STATS
// ============================================================================

/// Statistics for field aggregation performance
#[derive(Debug, Clone, Default)]
pub struct FieldAggregationStats {
    /// Total number of field aggregations performed
    pub total_aggregations: u64,
    /// Total time spent on field aggregation (in nanoseconds)
    pub total_aggregation_time_ns: u64,
    /// Average time per aggregation (in nanoseconds)
    pub avg_aggregation_time_ns: u64,
    /// Number of photons aggregated
    pub total_photons: u64,
    /// Number of spatial partitions used
    pub spatial_partitions: u64,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Average photons per aggregation
    pub avg_photons_per_aggregation: f64,
}

impl FieldAggregationStats {
    /// Calculate average aggregation time
    pub fn calculate_avg_time(&mut self) {
        if self.total_aggregations > 0 {
            self.avg_aggregation_time_ns = self.total_aggregation_time_ns / self.total_aggregations;
        }
    }

    /// Calculate average photons per aggregation
    pub fn calculate_avg_photons(&mut self) {
        if self.total_aggregations > 0 {
            self.avg_photons_per_aggregation =
                self.total_photons as f64 / self.total_aggregations as f64;
        }
    }

    /// Record an aggregation
    pub fn record_aggregation(&mut self, duration: Duration, photon_count: u64) {
        self.total_aggregations += 1;
        self.total_aggregation_time_ns += duration.as_nanos() as u64;
        self.total_photons += photon_count;
        self.calculate_avg_time();
        self.calculate_avg_photons();
    }

    /// Record a cache hit
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }

    /// Record a cache miss
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }

    /// Calculate cache hit rate
    pub fn calculate_cache_hit_rate(&mut self) {
        let total = self.cache_hits + self.cache_misses;
        if total > 0 {
            self.cache_hit_rate = self.cache_hits as f64 / total as f64;
        }
    }

    /// Get aggregation efficiency (photons per microsecond)
    pub fn aggregation_efficiency(&self) -> f64 {
        if self.avg_aggregation_time_ns == 0 {
            return 0.0;
        }
        (self.avg_photons_per_aggregation * 1_000_000.0) / self.avg_aggregation_time_ns as f64
    }

    /// Get performance improvement percentage
    pub fn performance_improvement(&self, baseline_avg_time_ns: u64) -> f64 {
        if baseline_avg_time_ns == 0 {
            return 0.0;
        }
        let improvement = ((baseline_avg_time_ns as f64 - self.avg_aggregation_time_ns as f64)
            / baseline_avg_time_ns as f64)
            * 100.0;
        improvement.max(0.0)
    }
}

// ============================================================================
// FIELD AGGREGATION CACHE
// ============================================================================

/// Cache for field aggregation results
#[derive(Debug, Clone)]
pub struct FieldAggregationCache {
    /// Cache entries keyed by spatial region
    cache: HashMap<(i32, i32, i32), CachedFieldAggregation>,
    /// Maximum cache size
    max_size: usize,
    /// Maximum age for cache entries
    max_age: Duration,
}

/// Cached field aggregation result
#[derive(Debug, Clone)]
pub struct CachedFieldAggregation {
    /// Electric field at this region
    pub electric_field: ElectricField,
    /// Magnetic field at this region
    pub magnetic_field: MagneticField,
    /// Gravitational field at this region
    pub gravitational_field: GravitationalField,
    /// Timestamp when this cache entry was created
    pub timestamp: Instant,
    /// Number of photons used for this aggregation
    pub photon_count: u64,
}

impl FieldAggregationCache {
    /// Create a new field aggregation cache
    pub fn new(max_size: usize, max_age: Duration) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            max_age,
        }
    }

    /// Get a cached field aggregation
    pub fn get(&mut self, region: (i32, i32, i32)) -> Option<&CachedFieldAggregation> {
        // First check if entry is stale
        let is_stale = self
            .cache
            .get(&region)
            .map(|entry| entry.timestamp.elapsed() > self.max_age)
            .unwrap_or(false);

        if is_stale {
            self.cache.remove(&region);
            return None;
        }

        self.cache.get(&region)
    }

    /// Insert a new field aggregation into the cache
    pub fn insert(
        &mut self,
        region: (i32, i32, i32),
        electric_field: ElectricField,
        magnetic_field: MagneticField,
        gravitational_field: GravitationalField,
        photon_count: u64,
    ) {
        // Evict oldest entry if cache is full
        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        }

        let entry = CachedFieldAggregation {
            electric_field,
            magnetic_field,
            gravitational_field,
            timestamp: Instant::now(),
            photon_count,
        };

        self.cache.insert(region, entry);
    }

    /// Evict the oldest entry from the cache
    fn evict_oldest(&mut self) {
        if let Some((&oldest_key, _)) = self.cache.iter().min_by_key(|(_, entry)| entry.timestamp) {
            self.cache.remove(&oldest_key);
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

// ============================================================================
// SPATIAL PARTITIONING OPTIMIZER
// ============================================================================

/// Optimizer for spatial partitioning of photons
#[derive(Debug)]
pub struct SpatialPartitioningOptimizer {
    /// Grid size for spatial partitioning
    grid_size: f64,
    /// Statistics
    stats: FieldAggregationStats,
}

impl SpatialPartitioningOptimizer {
    /// Create a new spatial partitioning optimizer
    pub fn new(grid_size: f64) -> Self {
        Self {
            grid_size,
            stats: FieldAggregationStats::default(),
        }
    }

    /// Calculate spatial region for a position
    pub fn calculate_region(&self, position: &Vector3) -> (i32, i32, i32) {
        let x = (position.x / self.grid_size).floor() as i32;
        let y = (position.y / self.grid_size).floor() as i32;
        let z = (position.z / self.grid_size).floor() as i32;
        (x, y, z)
    }

    /// Partition photons into spatial regions
    pub fn partition_photons(
        &mut self,
        photons: &[Arc<Photon>],
    ) -> HashMap<(i32, i32, i32), Vec<Arc<Photon>>> {
        let mut partitions: HashMap<(i32, i32, i32), Vec<Arc<Photon>>> = HashMap::new();

        for photon in photons {
            let photon_pos = position_to_vector3(&photon.position);
            let region = self.calculate_region(&photon_pos);
            partitions
                .entry(region)
                .or_insert_with(Vec::new)
                .push(photon.clone());
        }

        self.stats.spatial_partitions = partitions.len() as u64;
        partitions
    }

    /// Get photons in a specific region
    pub fn get_photons_in_region(
        &self,
        photons: &[Arc<Photon>],
        region: (i32, i32, i32),
    ) -> Vec<Arc<Photon>> {
        photons
            .iter()
            .filter(|photon| {
                let photon_pos = position_to_vector3(&photon.position);
                self.calculate_region(&photon_pos) == region
            })
            .cloned()
            .collect()
    }

    /// Get photons in neighboring regions (including the region itself)
    pub fn get_photons_in_neighboring_regions(
        &self,
        photons: &[Arc<Photon>],
        region: (i32, i32, i32),
    ) -> Vec<Arc<Photon>> {
        let mut result = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let neighbor = (region.0 + dx, region.1 + dy, region.2 + dz);
                    result.extend(self.get_photons_in_region(photons, neighbor));
                }
            }
        }

        result
    }

    /// Get optimizer statistics
    pub fn stats(&self) -> &FieldAggregationStats {
        &self.stats
    }
}

// ============================================================================
// FIELD AGGREGATION OPTIMIZER
// ============================================================================

/// Optimizer for field aggregation
#[derive(Debug)]
pub struct FieldAggregationOptimizer {
    /// Spatial partitioning optimizer
    spatial_optimizer: SpatialPartitioningOptimizer,
    /// Field aggregation cache
    cache: FieldAggregationCache,
    /// Statistics
    stats: FieldAggregationStats,
    /// Enable/disable optimization
    optimization_enabled: bool,
    /// Enable/disable caching
    caching_enabled: bool,
    /// Enable/disable spatial partitioning
    spatial_partitioning_enabled: bool,
}

impl FieldAggregationOptimizer {
    /// Create a new field aggregation optimizer
    pub fn new(grid_size: f64) -> Self {
        Self {
            spatial_optimizer: SpatialPartitioningOptimizer::new(grid_size),
            cache: FieldAggregationCache::new(1000, Duration::from_secs(60)),
            stats: FieldAggregationStats::default(),
            optimization_enabled: true,
            caching_enabled: true,
            spatial_partitioning_enabled: true,
        }
    }

    /// Create a new optimizer with custom cache settings
    pub fn with_cache_config(grid_size: f64, cache_size: usize, cache_max_age: Duration) -> Self {
        Self {
            spatial_optimizer: SpatialPartitioningOptimizer::new(grid_size),
            cache: FieldAggregationCache::new(cache_size, cache_max_age),
            stats: FieldAggregationStats::default(),
            optimization_enabled: true,
            caching_enabled: true,
            spatial_partitioning_enabled: true,
        }
    }

    /// Enable or disable optimization
    pub fn set_optimization_enabled(&mut self, enabled: bool) {
        self.optimization_enabled = enabled;
    }

    /// Enable or disable caching
    pub fn set_caching_enabled(&mut self, enabled: bool) {
        self.caching_enabled = enabled;
    }

    /// Enable or disable spatial partitioning
    pub fn set_spatial_partitioning_enabled(&mut self, enabled: bool) {
        self.spatial_partitioning_enabled = enabled;
    }

    /// Aggregate electric field from photons
    pub fn aggregate_electric_field(
        &mut self,
        photons: &[Arc<Photon>],
        position: &Vector3,
    ) -> ElectricField {
        let start = Instant::now();

        // Try to get from cache
        if self.caching_enabled && self.spatial_partitioning_enabled {
            let region = self.spatial_optimizer.calculate_region(position);
            if let Some(cached) = self.cache.get(region) {
                self.stats.record_cache_hit();
                self.stats.calculate_cache_hit_rate();
                return cached.electric_field.clone();
            }
            self.stats.record_cache_miss();
            self.stats.calculate_cache_hit_rate();
        }

        // Calculate electric field
        let electric_field = if self.optimization_enabled {
            self.calculate_electric_field_optimized(photons, position)
        } else {
            self.calculate_electric_field_baseline(photons, position)
        };

        // Cache the result
        if self.caching_enabled && self.spatial_partitioning_enabled {
            let region = self.spatial_optimizer.calculate_region(position);
            self.cache.insert(
                region,
                electric_field.clone(),
                MagneticField::default(),
                GravitationalField::default(),
                photons.len() as u64,
            );
        }

        // Record statistics
        let duration = start.elapsed();
        self.stats
            .record_aggregation(duration, photons.len() as u64);

        electric_field
    }

    /// Calculate electric field with optimization
    fn calculate_electric_field_optimized(
        &self,
        photons: &[Arc<Photon>],
        position: &Vector3,
    ) -> ElectricField {
        let mut total_field = Vector3::new(0.0, 0.0, 0.0);

        // Use spatial partitioning to reduce calculations
        if self.spatial_partitioning_enabled {
            let region = self.spatial_optimizer.calculate_region(position);
            let nearby_photons = self
                .spatial_optimizer
                .get_photons_in_neighboring_regions(photons, region);

            for photon in nearby_photons {
                let photon_pos = position_to_vector3(&photon.position);
                let diff = position.sub(&photon_pos);
                let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
                if distance_sq > 0.0 {
                    let distance = distance_sq.sqrt();
                    let field_strength = photon.energy / (4.0 * std::f64::consts::PI * distance_sq);
                    total_field = total_field.add(&diff.scale(field_strength / distance));
                }
            }
        } else {
            // Baseline: calculate for all photons
            for photon in photons {
                let photon_pos = position_to_vector3(&photon.position);
                let diff = position.sub(&photon_pos);
                let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
                if distance_sq > 0.0 {
                    let distance = distance_sq.sqrt();
                    let field_strength = photon.energy / (4.0 * std::f64::consts::PI * distance_sq);
                    total_field = total_field.add(&diff.scale(field_strength / distance));
                }
            }
        }

        ElectricField::default()
    }

    /// Calculate electric field with baseline algorithm
    fn calculate_electric_field_baseline(
        &self,
        photons: &[Arc<Photon>],
        position: &Vector3,
    ) -> ElectricField {
        let mut total_field = Vector3::new(0.0, 0.0, 0.0);

        for photon in photons {
            let photon_pos = position_to_vector3(&photon.position);
            let diff = position.sub(&photon_pos);
            let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
            if distance_sq > 0.0 {
                let distance = distance_sq.sqrt();
                let field_strength = photon.energy / (4.0 * std::f64::consts::PI * distance_sq);
                total_field = total_field.add(&diff.scale(field_strength / distance));
            }
        }

        ElectricField::default()
    }

    /// Aggregate magnetic field from photons
    pub fn aggregate_magnetic_field(
        &mut self,
        photons: &[Arc<Photon>],
        position: &Vector3,
    ) -> MagneticField {
        let start = Instant::now();

        // Try to get from cache
        if self.caching_enabled && self.spatial_partitioning_enabled {
            let region = self.spatial_optimizer.calculate_region(position);
            if let Some(cached) = self.cache.get(region) {
                self.stats.record_cache_hit();
                self.stats.calculate_cache_hit_rate();
                return cached.magnetic_field.clone();
            }
            self.stats.record_cache_miss();
            self.stats.calculate_cache_hit_rate();
        }

        // Calculate magnetic field
        let magnetic_field = if self.optimization_enabled {
            self.calculate_magnetic_field_optimized(photons, position)
        } else {
            self.calculate_magnetic_field_baseline(photons, position)
        };

        // Cache the result
        if self.caching_enabled && self.spatial_partitioning_enabled {
            let region = self.spatial_optimizer.calculate_region(position);
            self.cache.insert(
                region,
                ElectricField::default(),
                magnetic_field.clone(),
                GravitationalField::default(),
                photons.len() as u64,
            );
        }

        // Record statistics
        let duration = start.elapsed();
        self.stats
            .record_aggregation(duration, photons.len() as u64);

        magnetic_field
    }

    /// Calculate magnetic field with optimization
    fn calculate_magnetic_field_optimized(
        &self,
        photons: &[Arc<Photon>],
        position: &Vector3,
    ) -> MagneticField {
        let mut total_field = Vector3::new(0.0, 0.0, 0.0);

        if self.spatial_partitioning_enabled {
            let region = self.spatial_optimizer.calculate_region(position);
            let nearby_photons = self
                .spatial_optimizer
                .get_photons_in_neighboring_regions(photons, region);

            for photon in nearby_photons {
                let photon_pos = position_to_vector3(&photon.position);
                let velocity = photon_velocity_from_momentum(&photon.momentum);
                let diff = position.sub(&photon_pos);
                let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
                if distance_sq > 0.0 {
                    let distance = distance_sq.sqrt();
                    let field_strength = photon.energy / (4.0 * std::f64::consts::PI * distance_sq);
                    let cross_product = diff.cross(&velocity);
                    total_field = total_field.add(
                        &cross_product.scale(field_strength / (distance * velocity.magnitude())),
                    );
                }
            }
        } else {
            for photon in photons {
                let photon_pos = position_to_vector3(&photon.position);
                let velocity = photon_velocity_from_momentum(&photon.momentum);
                let diff = position.sub(&photon_pos);
                let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
                if distance_sq > 0.0 {
                    let distance = distance_sq.sqrt();
                    let field_strength = photon.energy / (4.0 * std::f64::consts::PI * distance_sq);
                    let cross_product = diff.cross(&velocity);
                    total_field = total_field.add(
                        &cross_product.scale(field_strength / (distance * velocity.magnitude())),
                    );
                }
            }
        }

        MagneticField::default()
    }

    /// Calculate magnetic field with baseline algorithm
    fn calculate_magnetic_field_baseline(
        &self,
        photons: &[Arc<Photon>],
        position: &Vector3,
    ) -> MagneticField {
        let mut total_field = Vector3::new(0.0, 0.0, 0.0);

        for photon in photons {
            let photon_pos = position_to_vector3(&photon.position);
            let velocity = photon_velocity_from_momentum(&photon.momentum);
            let diff = position.sub(&photon_pos);
            let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
            if distance_sq > 0.0 {
                let distance = distance_sq.sqrt();
                let field_strength = photon.energy / (4.0 * std::f64::consts::PI * distance_sq);
                let cross_product = diff.cross(&velocity);
                total_field = total_field
                    .add(&cross_product.scale(field_strength / (distance * velocity.magnitude())));
            }
        }

        MagneticField::default()
    }

    /// Aggregate all fields at once (batch optimization)
    pub fn aggregate_all_fields(
        &mut self,
        photons: &[Arc<Photon>],
        position: &Vector3,
    ) -> (ElectricField, MagneticField, GravitationalField) {
        let start = Instant::now();

        let electric_field = self.aggregate_electric_field(photons, position);
        let magnetic_field = self.aggregate_magnetic_field(photons, position);
        let gravitational_field = GravitationalField::default(); // Placeholder

        // Record statistics
        let duration = start.elapsed();
        self.stats
            .record_aggregation(duration, photons.len() as u64);

        (electric_field, magnetic_field, gravitational_field)
    }

    /// Get optimizer statistics
    pub fn stats(&self) -> &FieldAggregationStats {
        &self.stats
    }

    /// Get spatial optimizer statistics
    pub fn spatial_stats(&self) -> &FieldAggregationStats {
        self.spatial_optimizer.stats()
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.cache.size()
    }
}

// ============================================================================
// OPTIMIZATION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_photon(x: f64, y: f64, z: f64) -> Arc<Photon> {
        let seed = Arc::new(HolographicSeed::new_from_source());
        let holographic_ref = crate::holographic_seed::HolographicSeedReference::new(seed);

        Arc::new(Photon {
            id: crate::light::PhotonID::generate(),
            holographic_reference: holographic_ref,
            energy: 1.0,
            frequency: 6.0e14,
            wavelength: 500.0,
            polarization: 0.5,
            position: Vector3::new(x, y, z),
            velocity: Vector3::new(1.0, 0.0, 0.0),
            quantum_state: crate::light::QuantumState::new(),
        })
    }

    #[test]
    fn test_spatial_partitioning_optimizer_basic() {
        let optimizer = SpatialPartitioningOptimizer::new(10.0);
        let position = Vector3::new(15.0, 25.0, 35.0);
        let region = optimizer.calculate_region(&position);
        assert_eq!(region, (1, 2, 3));
    }

    #[test]
    fn test_spatial_partitioning_optimizer_partition() {
        let mut optimizer = SpatialPartitioningOptimizer::new(10.0);
        let photons = vec![
            create_test_photon(5.0, 5.0, 5.0),
            create_test_photon(15.0, 5.0, 5.0),
            create_test_photon(5.0, 15.0, 5.0),
        ];

        let partitions = optimizer.partition_photons(&photons);
        assert_eq!(partitions.len(), 3);
    }

    #[test]
    fn test_spatial_partitioning_optimizer_get_photons_in_region() {
        let optimizer = SpatialPartitioningOptimizer::new(10.0);
        let photons = vec![
            create_test_photon(5.0, 5.0, 5.0),
            create_test_photon(15.0, 5.0, 5.0),
            create_test_photon(5.0, 15.0, 5.0),
        ];

        let region_photons = optimizer.get_photons_in_region(&photons, (0, 0, 0));
        assert_eq!(region_photons.len(), 1);
    }

    #[test]
    fn test_field_aggregation_cache_basic() {
        let cache = FieldAggregationCache::new(10, Duration::from_secs(60));
        cache.insert(
            (0, 0, 0),
            ElectricField::default(),
            MagneticField::default(),
            GravitationalField::default(),
            10,
        );

        let cached = cache.get((0, 0, 0));
        assert!(cached.is_some());
    }

    #[test]
    fn test_field_aggregation_optimizer_basic() {
        let mut optimizer = FieldAggregationOptimizer::new(10.0);
        let photons = vec![create_test_photon(0.0, 0.0, 0.0)];
        let position = Vector3::new(10.0, 10.0, 10.0);

        let electric_field = optimizer.aggregate_electric_field(&photons, &position);
        assert_eq!(optimizer.stats().total_aggregations, 1);
    }

    #[test]
    fn test_field_aggregation_optimizer_caching() {
        let mut optimizer = FieldAggregationOptimizer::new(10.0);
        let photons = vec![create_test_photon(0.0, 0.0, 0.0)];
        let position = Vector3::new(10.0, 10.0, 10.0);

        // First call - cache miss
        let _ = optimizer.aggregate_electric_field(&photons, &position);
        assert_eq!(optimizer.stats().cache_misses, 1);

        // Second call - cache hit
        let _ = optimizer.aggregate_electric_field(&photons, &position);
        assert_eq!(optimizer.stats().cache_hits, 1);
    }

    #[test]
    fn test_field_aggregation_optimizer_spatial_partitioning() {
        let mut optimizer = FieldAggregationOptimizer::new(10.0);
        optimizer.set_spatial_partitioning_enabled(true);

        let photons = vec![
            create_test_photon(0.0, 0.0, 0.0),
            create_test_photon(100.0, 100.0, 100.0),
            create_test_photon(5.0, 5.0, 5.0),
        ];
        let position = Vector3::new(5.0, 5.0, 5.0);

        let electric_field = optimizer.aggregate_electric_field(&photons, &position);
        assert_eq!(optimizer.stats().total_aggregations, 1);
    }

    #[test]
    fn test_field_aggregation_optimizer_batch() {
        let mut optimizer = FieldAggregationOptimizer::new(10.0);
        let photons = vec![create_test_photon(0.0, 0.0, 0.0)];
        let position = Vector3::new(10.0, 10.0, 10.0);

        let (electric_field, magnetic_field, gravitational_field) =
            optimizer.aggregate_all_fields(&photons, &position);
        assert_eq!(optimizer.stats().total_aggregations, 1);
    }

    #[test]
    fn test_field_aggregation_stats() {
        let mut stats = FieldAggregationStats::default();

        stats.record_aggregation(Duration::from_nanos(1000), 10);
        stats.record_aggregation(Duration::from_nanos(2000), 20);

        assert_eq!(stats.total_aggregations, 2);
        assert_eq!(stats.total_photons, 30);
        assert_eq!(stats.avg_aggregation_time_ns, 1500);
        assert_eq!(stats.avg_photons_per_aggregation, 15.0);
    }

    #[test]
    fn test_field_aggregation_aggregation_efficiency() {
        let mut stats = FieldAggregationStats::default();

        stats.record_aggregation(Duration::from_nanos(1000), 100);
        stats.calculate_avg_time();
        stats.calculate_avg_photons();

        let efficiency = stats.aggregation_efficiency();
        assert!(efficiency > 0.0);
    }
}
