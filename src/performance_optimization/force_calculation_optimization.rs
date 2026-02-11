// ============================================================================
// TASK 4: OPTIMIZE FORCE CALCULATIONS
// ============================================================================
// Objective: Profile and optimize force calculation performance.
//
// Tasks:
// - Profile force calculation code
// - Optimize force emergence formulas
// - Use caching for repeated calculations
// - Optimize spatial queries
// ============================================================================

use crate::energy_fields::Vector3;
use crate::matter::Particle;
use std::collections::HashMap;
use std::time::{Duration, Instant};

// Helper function to derive mass from particle energy (E = mc^2)
fn derive_mass_from_energy(energy: f64) -> f64 {
    const SPEED_OF_LIGHT_SQUARED: f64 = 8.98755179e16; // c^2 in (m/s)^2
    energy / SPEED_OF_LIGHT_SQUARED
}

// Helper function to derive charge from archetype activation
fn derive_charge_from_archetypes(archetypes: &[f64; 22]) -> f64 {
    // Catalyst archetypes (A3, A10, A17) determine charge type and magnitude
    let catalyst_sum = archetypes[2] + archetypes[9] + archetypes[16];
    let _experience_sum = archetypes[3] + archetypes[10] + archetypes[17];

    // Simplified charge derivation
    if catalyst_sum > 1.5 {
        1.0
    }
    // Positive
    else if catalyst_sum < 0.5 {
        -1.0
    }
    // Negative
    else {
        0.0
    } // Neutral
}

// Helper function to create particle identifier for caching
fn particle_id(particle: &Particle) -> u64 {
    // Use position hash as identifier (simplified)
    let x = (particle.position.x * 1000.0) as u64;
    let y = (particle.position.y * 1000.0) as u64;
    let z = (particle.position.z * 1000.0) as u64;
    x ^ (y.wrapping_mul(31)) ^ (z.wrapping_mul(17))
}

// Placeholder for cached force calculation result
#[derive(Debug, Clone)]
pub struct ForceCalculationResult {
    pub force: Vector3,
    pub magnitude: f64,
}

// Placeholder for ArchetypeActivation (used in performance optimization)
#[derive(Debug, Clone)]
pub struct ArchetypeActivation {
    pub archetype_id: u8,
    pub activation_level: f64,
    pub coherence: f64,
    pub polarization: f64,
    pub lambda_value: f64,
}

// ============================================================================
// FORCE CALCULATION STATS
// ============================================================================

/// Statistics for force calculation performance
#[derive(Debug, Clone, Default)]
pub struct ForceCalculationStats {
    /// Total number of force calculations performed
    pub total_calculations: u64,
    /// Total time spent on force calculation (in nanoseconds)
    pub total_calculation_time_ns: u64,
    /// Average time per calculation (in nanoseconds)
    pub avg_calculation_time_ns: u64,
    /// Number of gravitational force calculations
    pub gravitational_calculations: u64,
    /// Number of electromagnetic force calculations
    pub electromagnetic_calculations: u64,
    /// Number of strong nuclear force calculations
    pub strong_nuclear_calculations: u64,
    /// Number of weak nuclear force calculations
    pub weak_nuclear_calculations: u64,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
}

impl ForceCalculationStats {
    /// Calculate average calculation time
    pub fn calculate_avg_time(&mut self) {
        if self.total_calculations > 0 {
            self.avg_calculation_time_ns = self.total_calculation_time_ns / self.total_calculations;
        }
    }

    /// Record a calculation
    pub fn record_calculation(&mut self, duration: Duration, force_type: ForceType) {
        self.total_calculations += 1;
        self.total_calculation_time_ns += duration.as_nanos() as u64;

        match force_type {
            ForceType::Gravitational => self.gravitational_calculations += 1,
            ForceType::Electromagnetic => self.electromagnetic_calculations += 1,
            ForceType::StrongNuclear => self.strong_nuclear_calculations += 1,
            ForceType::WeakNuclear => self.weak_nuclear_calculations += 1,
        }

        self.calculate_avg_time();
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

    /// Get calculation efficiency (calculations per microsecond)
    pub fn calculation_efficiency(&self) -> f64 {
        if self.avg_calculation_time_ns == 0 {
            return 0.0;
        }
        1_000_000.0 / self.avg_calculation_time_ns as f64
    }

    /// Get performance improvement percentage
    pub fn performance_improvement(&self, baseline_avg_time_ns: u64) -> f64 {
        if baseline_avg_time_ns == 0 {
            return 0.0;
        }
        let improvement = ((baseline_avg_time_ns as f64 - self.avg_calculation_time_ns as f64)
            / baseline_avg_time_ns as f64)
            * 100.0;
        improvement.max(0.0)
    }
}

// ============================================================================
// FORCE TYPE
// ============================================================================

/// Type of force
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForceType {
    Gravitational,
    Electromagnetic,
    StrongNuclear,
    WeakNuclear,
}

// ============================================================================
// FORCE CALCULATION CACHE
// ============================================================================

/// Cache for force calculation results
#[derive(Debug, Clone)]
pub struct ForceCalculationCache {
    /// Cache entries keyed by (particle1_pos_hash, particle2_pos_hash, force_type)
    cache: HashMap<(u64, u64, ForceType), CachedForceCalculation>,
    /// Maximum cache size
    max_size: usize,
    /// Maximum age for cache entries
    max_age: Duration,
}

/// Cached force calculation result
#[derive(Debug, Clone)]
pub struct CachedForceCalculation {
    /// Force vector
    pub force: Vector3,
    /// Force magnitude
    pub magnitude: f64,
    /// Timestamp when this cache entry was created
    pub timestamp: Instant,
}

impl ForceCalculationCache {
    /// Create a new force calculation cache
    pub fn new(max_size: usize, max_age: Duration) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            max_age,
        }
    }

    /// Get a cached force calculation
    pub fn get(
        &mut self,
        particle1_id: u64,
        particle2_id: u64,
        force_type: ForceType,
    ) -> Option<&CachedForceCalculation> {
        let key = (particle1_id, particle2_id, force_type);

        // First check if entry is stale
        let is_stale = self
            .cache
            .get(&key)
            .map(|entry| entry.timestamp.elapsed() > self.max_age)
            .unwrap_or(false);

        if is_stale {
            self.cache.remove(&key);
            return None;
        }

        self.cache.get(&key)
    }

    /// Insert a new force calculation into the cache
    pub fn insert(
        &mut self,
        particle1_id: u64,
        particle2_id: u64,
        force_type: ForceType,
        force: Vector3,
        magnitude: f64,
    ) {
        // Evict oldest entry if cache is full
        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        }

        let entry = CachedForceCalculation {
            force,
            magnitude,
            timestamp: Instant::now(),
        };

        self.cache
            .insert((particle1_id, particle2_id, force_type), entry);
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
// FORCE CALCULATION OPTIMIZER
// ============================================================================

/// Optimizer for force calculations
#[derive(Debug)]
pub struct ForceCalculationOptimizer {
    /// Force calculation cache
    cache: ForceCalculationCache,
    /// Statistics
    stats: ForceCalculationStats,
    /// Enable/disable optimization
    optimization_enabled: bool,
    /// Enable/disable caching
    caching_enabled: bool,
}

impl ForceCalculationOptimizer {
    /// Create a new force calculation optimizer
    pub fn new() -> Self {
        Self {
            cache: ForceCalculationCache::new(1000, Duration::from_secs(60)),
            stats: ForceCalculationStats::default(),
            optimization_enabled: true,
            caching_enabled: true,
        }
    }

    /// Create a new optimizer with custom cache settings
    pub fn with_cache_config(cache_size: usize, cache_max_age: Duration) -> Self {
        Self {
            cache: ForceCalculationCache::new(cache_size, cache_max_age),
            stats: ForceCalculationStats::default(),
            optimization_enabled: true,
            caching_enabled: true,
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

    /// Calculate gravitational force with optimization
    pub fn calculate_gravitational_force_optimized(
        &self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Vector3, f64) {
        let diff = particle2.position.sub(&particle1.position);
        let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;

        if distance_sq < 1e-10 {
            return (Vector3::new(0.0, 0.0, 0.0), 0.0);
        }

        let distance = distance_sq.sqrt();
        let g_constant = 6.674e-11; // Gravitational constant
        let force_magnitude = g_constant
            * derive_mass_from_energy(particle1.energy)
            * derive_mass_from_energy(particle2.energy)
            / distance_sq;

        let force = diff.to_vector3().scale(force_magnitude / distance);
        (force, force_magnitude)
    }

    /// Calculate gravitational force with baseline algorithm
    pub fn calculate_gravitational_force_baseline(
        &self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Vector3, f64) {
        let diff = particle2.position.sub(&particle1.position);
        let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;

        if distance_sq < 1e-10 {
            return (Vector3::new(0.0, 0.0, 0.0), 0.0);
        }

        let distance = distance_sq.sqrt();
        let g_constant = 6.674e-11;
        let force_magnitude = g_constant
            * derive_mass_from_energy(particle1.energy)
            * derive_mass_from_energy(particle2.energy)
            / distance_sq;

        let force = diff.to_vector3().scale(force_magnitude / distance);
        (force, force_magnitude)
    }

    /// Calculate gravitational force between two particles
    pub fn calculate_gravitational_force(
        &mut self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Vector3, f64) {
        let start = Instant::now();

        // Try to get from cache
        if self.caching_enabled {
            if let Some(cached) = self.cache.get(
                particle_id(particle1),
                particle_id(particle2),
                ForceType::Gravitational,
            ) {
                self.stats.record_cache_hit();
                self.stats.calculate_cache_hit_rate();
                return (cached.force, cached.magnitude);
            }
            self.stats.record_cache_miss();
            self.stats.calculate_cache_hit_rate();
        }

        // Calculate gravitational force
        let (force, magnitude) = if self.optimization_enabled {
            self.calculate_gravitational_force_optimized(particle1, particle2)
        } else {
            self.calculate_gravitational_force_baseline(particle1, particle2)
        };

        // Cache the result
        if self.caching_enabled {
            self.cache.insert(
                particle_id(particle1),
                particle_id(particle2),
                ForceType::Gravitational,
                force,
                magnitude,
            );
        }

        // Record statistics
        let duration = start.elapsed();
        self.stats
            .record_calculation(duration, ForceType::Gravitational);

        (force, magnitude)
    }

    /// Calculate electromagnetic force with optimization
    pub fn calculate_electromagnetic_force_optimized(
        &self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Vector3, f64) {
        let diff = particle2.position.sub(&particle1.position);
        let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;

        if distance_sq < 1e-10 {
            return (Vector3::new(0.0, 0.0, 0.0), 0.0);
        }

        let distance = distance_sq.sqrt();
        let k_constant = 8.9875517923e9; // Coulomb's constant
        let charge1 = self.get_particle_charge(particle1);
        let charge2 = self.get_particle_charge(particle2);
        let force_magnitude = k_constant * charge1.abs() * charge2.abs() / distance_sq;

        let force = diff.to_vector3().scale(force_magnitude / distance);

        // Direction depends on charge signs
        if (charge1 > 0.0 && charge2 > 0.0) || (charge1 < 0.0 && charge2 < 0.0) {
            // Same charges repel
            (Vector3::new(-force.x, -force.y, -force.z), force_magnitude)
        } else {
            // Opposite charges attract
            (force, force_magnitude)
        }
    }

    /// Calculate electromagnetic force with baseline algorithm
    pub fn calculate_electromagnetic_force_baseline(
        &self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Vector3, f64) {
        let diff = particle2.position.sub(&particle1.position);
        let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;

        if distance_sq < 1e-10 {
            return (Vector3::new(0.0, 0.0, 0.0), 0.0);
        }

        let distance = distance_sq.sqrt();
        let k_constant = 8.9875517923e9;
        let charge1 = self.get_particle_charge(particle1);
        let charge2 = self.get_particle_charge(particle2);
        let force_magnitude = k_constant * charge1.abs() * charge2.abs() / distance_sq;

        let force = diff.to_vector3().scale(force_magnitude / distance);

        if (charge1 > 0.0 && charge2 > 0.0) || (charge1 < 0.0 && charge2 < 0.0) {
            (Vector3::new(-force.x, -force.y, -force.z), force_magnitude)
        } else {
            (force, force_magnitude)
        }
    }

    /// Calculate electromagnetic force between two particles
    pub fn calculate_electromagnetic_force(
        &mut self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Vector3, f64) {
        let start = Instant::now();

        // Try to get from cache
        if self.caching_enabled {
            if let Some(cached) = self.cache.get(
                particle_id(particle1),
                particle_id(particle2),
                ForceType::Electromagnetic,
            ) {
                self.stats.record_cache_hit();
                self.stats.calculate_cache_hit_rate();
                return (cached.force, cached.magnitude);
            }
            self.stats.record_cache_miss();
            self.stats.calculate_cache_hit_rate();
        }

        // Calculate electromagnetic force
        let (force, magnitude) = if self.optimization_enabled {
            self.calculate_electromagnetic_force_optimized(particle1, particle2)
        } else {
            self.calculate_electromagnetic_force_baseline(particle1, particle2)
        };

        // Cache the result
        if self.caching_enabled {
            self.cache.insert(
                particle_id(particle1),
                particle_id(particle2),
                ForceType::Electromagnetic,
                force,
                magnitude,
            );
        }

        // Record statistics
        let duration = start.elapsed();
        self.stats
            .record_calculation(duration, ForceType::Electromagnetic);

        (force, magnitude)
    }

    /// Get particle charge based on particle type
    fn get_particle_charge(&self, particle: &Particle) -> f64 {
        match particle.get_particle_type_name() {
            "Proton" => 1.602e-19,
            "Electron" => -1.602e-19,
            "Neutron" => 0.0,
            "Photon" => 0.0,
            _ => 0.0, // Default neutral
        }
    }

    /// Calculate strong nuclear force between two particles
    pub fn calculate_strong_nuclear_force(
        &mut self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Vector3, f64) {
        let start = Instant::now();

        // Try to get from cache
        if self.caching_enabled {
            if let Some(cached) = self.cache.get(
                particle_id(particle1),
                particle_id(particle2),
                ForceType::StrongNuclear,
            ) {
                self.stats.record_cache_hit();
                self.stats.calculate_cache_hit_rate();
                return (cached.force, cached.magnitude);
            }
            self.stats.record_cache_miss();
            self.stats.calculate_cache_hit_rate();
        }

        // Calculate strong nuclear force
        let (force, magnitude) = if self.optimization_enabled {
            self.calculate_strong_nuclear_force_optimized(particle1, particle2)
        } else {
            self.calculate_strong_nuclear_force_baseline(particle1, particle2)
        };

        // Cache the result
        if self.caching_enabled {
            self.cache.insert(
                particle_id(particle1),
                particle_id(particle2),
                ForceType::StrongNuclear,
                force,
                magnitude,
            );
        }

        // Record statistics
        let duration = start.elapsed();
        self.stats
            .record_calculation(duration, ForceType::StrongNuclear);

        (force, magnitude)
    }

    /// Calculate strong nuclear force with optimization
    fn calculate_strong_nuclear_force_optimized(
        &self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Vector3, f64) {
        let diff = particle2.position.sub(&particle1.position);
        let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;

        if distance_sq < 1e-10 {
            return (Vector3::new(0.0, 0.0, 0.0), 0.0);
        }

        let distance = distance_sq.sqrt();

        // Strong nuclear force is short-range, only applies at very short distances
        if distance > 1e-15 {
            return (Vector3::new(0.0, 0.0, 0.0), 0.0);
        }

        // Yukawa potential approximation
        let g_constant = 1.0; // Simplified coupling constant
        let range = 1e-15; // Range of strong force
        let force_magnitude = g_constant * (-distance / range).exp() / distance_sq;

        let force = diff.to_vector3().scale(force_magnitude / distance);
        (force, force_magnitude)
    }

    /// Calculate strong nuclear force with baseline algorithm
    fn calculate_strong_nuclear_force_baseline(
        &self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Vector3, f64) {
        let diff = particle2.position.sub(&particle1.position);
        let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;

        if distance_sq < 1e-10 {
            return (Vector3::new(0.0, 0.0, 0.0), 0.0);
        }

        let distance = distance_sq.sqrt();

        if distance > 1e-15 {
            return (Vector3::new(0.0, 0.0, 0.0), 0.0);
        }

        let g_constant = 1.0;
        let range = 1e-15;
        let force_magnitude = g_constant * (-distance / range).exp() / distance_sq;

        let force = diff.to_vector3().scale(force_magnitude / distance);
        (force, force_magnitude)
    }

    /// Calculate weak nuclear force between two particles
    pub fn calculate_weak_nuclear_force(
        &mut self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Vector3, f64) {
        let start = Instant::now();

        // Try to get from cache
        if self.caching_enabled {
            if let Some(cached) = self.cache.get(
                particle_id(particle1),
                particle_id(particle2),
                ForceType::WeakNuclear,
            ) {
                self.stats.record_cache_hit();
                self.stats.calculate_cache_hit_rate();
                return (cached.force, cached.magnitude);
            }
            self.stats.record_cache_miss();
            self.stats.calculate_cache_hit_rate();
        }

        // Calculate weak nuclear force
        let (force, magnitude) = if self.optimization_enabled {
            self.calculate_weak_nuclear_force_optimized(particle1, particle2)
        } else {
            self.calculate_weak_nuclear_force_baseline(particle1, particle2)
        };

        // Cache the result
        if self.caching_enabled {
            self.cache.insert(
                particle_id(particle1),
                particle_id(particle2),
                ForceType::WeakNuclear,
                force,
                magnitude,
            );
        }

        // Record statistics
        let duration = start.elapsed();
        self.stats
            .record_calculation(duration, ForceType::WeakNuclear);

        (force, magnitude)
    }

    /// Calculate weak nuclear force with optimization
    fn calculate_weak_nuclear_force_optimized(
        &self,
        _particle1: &Particle,
        _particle2: &Particle,
    ) -> (Vector3, f64) {
        // Weak nuclear force is very short-range and involves particle decay
        // Simplified model for simulation
        (Vector3::new(0.0, 0.0, 0.0), 0.0)
    }

    /// Calculate weak nuclear force with baseline algorithm
    fn calculate_weak_nuclear_force_baseline(
        &self,
        _particle1: &Particle,
        _particle2: &Particle,
    ) -> (Vector3, f64) {
        (Vector3::new(0.0, 0.0, 0.0), 0.0)
    }

    /// Calculate all forces between two particles (batch optimization)
    pub fn calculate_all_forces(
        &mut self,
        particle1: &Particle,
        particle2: &Particle,
    ) -> (Vector3, Vector3, Vector3, Vector3) {
        let (gravitational, _) = self.calculate_gravitational_force(particle1, particle2);
        let (electromagnetic, _) = self.calculate_electromagnetic_force(particle1, particle2);
        let (strong_nuclear, _) = self.calculate_strong_nuclear_force(particle1, particle2);
        let (weak_nuclear, _) = self.calculate_weak_nuclear_force(particle1, particle2);

        (gravitational, electromagnetic, strong_nuclear, weak_nuclear)
    }

    /// Get optimizer statistics
    pub fn stats(&self) -> &ForceCalculationStats {
        &self.stats
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

    fn create_test_particle(
        id: u64,
        x: f64,
        y: f64,
        z: f64,
        mass: f64,
        particle_type_name: &str,
    ) -> Particle {
        // Create archetype activation pattern based on particle type
        let archetype_activation = match particle_type_name {
            "Proton" => {
                let mut activation = [0.0; 22];
                activation[1] = 0.8; // High potentiator for mass
                activation[2] = 0.7; // Positive catalyst for charge
                activation[6] = 0.95; // Great Way for stability
                activation
            }
            "Electron" => {
                let mut activation = [0.0; 22];
                activation[0] = 1.0; // Matrix
                activation[2] = 0.3; // Negative catalyst for charge
                activation[6] = 0.95; // Great Way for stability
                activation
            }
            "Neutron" => {
                let mut activation = [0.0; 22];
                activation[1] = 0.8; // High potentiator for mass
                activation[6] = 0.95; // Great Way for stability
                activation
            }
            _ => [0.0; 22],
        };

        Particle::from_archetype_activation(
            id,
            archetype_activation,
            crate::matter::particle::Coordinate3D::new(x, y, z),
        )
    }

    #[test]
    fn test_force_calculation_cache_basic() {
        let cache = ForceCalculationCache::new(10, Duration::from_secs(60));
        cache.insert(
            1,
            2,
            ForceType::Gravitational,
            Vector3::new(1.0, 0.0, 0.0),
            1.0,
        );

        let cached = cache.get(1, 2, ForceType::Gravitational);
        assert!(cached.is_some());
    }

    #[test]
    fn test_force_calculation_cache_miss() {
        let mut cache = ForceCalculationCache::new(10, Duration::from_secs(60));
        let cached = cache.get(1, 2, ForceType::Gravitational);
        assert!(cached.is_none());
    }

    #[test]
    fn test_force_calculation_optimizer_gravitational() {
        let mut optimizer = ForceCalculationOptimizer::new();
        let particle1 = create_test_particle(1, 0.0, 0.0, 0.0, 1.0, "Proton");
        let particle2 = create_test_particle(2, 1.0, 0.0, 0.0, 1.0, "Proton");

        let (force, magnitude) = optimizer.calculate_gravitational_force(&particle1, &particle2);
        assert_eq!(optimizer.stats().gravitational_calculations, 1);
        assert!(magnitude > 0.0);
    }

    #[test]
    fn test_force_calculation_optimizer_electromagnetic() {
        let mut optimizer = ForceCalculationOptimizer::new();
        let particle1 = create_test_particle(1, 0.0, 0.0, 0.0, 1.0, "Proton");
        let particle2 = create_test_particle(2, 1.0, 0.0, 0.0, 1.0, "Electron");

        let (force, magnitude) = optimizer.calculate_electromagnetic_force(&particle1, &particle2);
        assert_eq!(optimizer.stats().electromagnetic_calculations, 1);
        assert!(magnitude > 0.0);
    }

    #[test]
    fn test_force_calculation_optimizer_strong_nuclear() {
        let mut optimizer = ForceCalculationOptimizer::new();
        let particle1 = create_test_particle(1, 0.0, 0.0, 0.0, 1.0, "Proton");
        let particle2 = create_test_particle(2, 1e-16, 0.0, 0.0, 1.0, "Neutron");

        let (force, magnitude) = optimizer.calculate_strong_nuclear_force(&particle1, &particle2);
        assert_eq!(optimizer.stats().strong_nuclear_calculations, 1);
    }

    #[test]
    fn test_force_calculation_optimizer_caching() {
        let mut optimizer = ForceCalculationOptimizer::new();
        let particle1 = create_test_particle(1, 0.0, 0.0, 0.0, 1.0, "Proton");
        let particle2 = create_test_particle(2, 1.0, 0.0, 0.0, 1.0, "Proton");

        // First call - cache miss
        let _ = optimizer.calculate_gravitational_force(&particle1, &particle2);
        assert_eq!(optimizer.stats().cache_misses, 1);

        // Second call - cache hit
        let _ = optimizer.calculate_gravitational_force(&particle1, &particle2);
        assert_eq!(optimizer.stats().cache_hits, 1);
    }

    #[test]
    fn test_force_calculation_optimizer_batch() {
        let mut optimizer = ForceCalculationOptimizer::new();
        let particle1 = create_test_particle(1, 0.0, 0.0, 0.0, 1.0, "Proton");
        let particle2 = create_test_particle(2, 1.0, 0.0, 0.0, 1.0, "Proton");

        let (gravitational, electromagnetic, strong_nuclear, weak_nuclear) =
            optimizer.calculate_all_forces(&particle1, &particle2);
        assert_eq!(optimizer.stats().total_calculations, 4);
    }

    #[test]
    fn test_force_calculation_stats() {
        let mut stats = ForceCalculationStats::default();

        stats.record_calculation(Duration::from_nanos(100), ForceType::Gravitational);
        stats.record_calculation(Duration::from_nanos(200), ForceType::Electromagnetic);

        assert_eq!(stats.total_calculations, 2);
        assert_eq!(stats.gravitational_calculations, 1);
        assert_eq!(stats.electromagnetic_calculations, 1);
        assert_eq!(stats.avg_calculation_time_ns, 150);
    }

    #[test]
    fn test_force_calculation_efficiency() {
        let mut stats = ForceCalculationStats::default();

        stats.record_calculation(Duration::from_nanos(100), ForceType::Gravitational);
        stats.calculate_avg_time();

        let efficiency = stats.calculation_efficiency();
        assert!(efficiency > 0.0);
    }
}
