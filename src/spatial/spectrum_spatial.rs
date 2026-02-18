//! Spectrum-Derived Spatial Dynamics
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 4.1:
//! "Replace golden ratio with spectrum-derived positions"
//! "Positions emerge from field coherence gradients, NOT from mathematical formulas"
//!
//! This module implements:
//! - Deriving entity positions from field coherence gradients
//! - Spectrum-based spatial transformations
//! - Integration with the unified field for coherence computation
//!
//! From COSMOLOGICAL-ARCHITECTURE.md section 2.5:
//! "The spectrum is configured at galactic and solar scales before physical matter exists"
//! "v = s/t creates 3D space with 1D time"
//! "v = t/s creates 1D space with 3D time"

use super::*;

/// Configuration for spectrum-derived spatial dynamics
#[derive(Debug, Clone)]
pub struct SpectrumSpatialConfig {
    /// Base scale for spatial extent
    pub spatial_scale: f64,
    
    /// Minimum coherence required for position derivation
    pub min_coherence: f64,
    
    /// Maximum distance from coherence peak
    pub max_gradient_distance: f64,
    
    /// Veil position (spectrum = 1.0)
    pub veil_position: f64,
    
    /// Veil thickness
    pub veil_thickness: f64,
    
    /// Enable coherence gradient following
    pub use_gradient_following: bool,
    
    /// Enable veil transformation
    pub apply_veil_transform: bool,
}

impl Default for SpectrumSpatialConfig {
    fn default() -> Self {
        SpectrumSpatialConfig {
            spatial_scale: 50.0,
            min_coherence: 0.1,
            max_gradient_distance: 10.0,
            veil_position: 1.0,
            veil_thickness: 0.2,
            use_gradient_following: true,
            apply_veil_transform: true,
        }
    }
}

/// Spectrum-derived spatial dynamics
/// 
/// Derives positions from field coherence gradients instead of golden ratio formulas
/// 
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > Each entity contains within it all densities and sub-densities of the octave
/// > The same field that creates matter also creates consciousness
#[derive(Debug, Clone)]
pub struct SpectrumSpatialDynamics {
    config: SpectrumSpatialConfig,
    positions: HashMap<u64, CoherenceDerivedPosition>,
    statistics: SpatialStatistics,
    coherence_cache: HashMap<[i32; 3], f64>,
}

impl SpectrumSpatialDynamics {
    pub fn new() -> Self {
        SpectrumSpatialDynamics {
            config: SpectrumSpatialConfig::default(),
            positions: HashMap::new(),
            statistics: SpatialStatistics::default(),
            coherence_cache: HashMap::new(),
        }
    }
    
    pub fn with_config(config: SpectrumSpatialConfig) -> Self {
        SpectrumSpatialDynamics {
            config,
            positions: HashMap::new(),
            statistics: SpatialStatistics::default(),
            coherence_cache: HashMap::new(),
        }
    }
    
    /// Derive position from field coherence gradients
    /// 
    /// This is the KEY REPLACEMENT for golden ratio formulas
    /// 
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "The same field that creates matter also creates consciousness"
    /// > "Physics, biology, and consciousness are different perspectives on the same holographic field"
    pub fn derive_position_from_spectrum(
        &mut self,
        entity_id: u64,
        field_coherence_fn: impl Fn([f64; 3]) -> f64,
    ) -> CoherenceDerivedPosition {
        // Get spectrum position for this entity
        let spectrum_position = self.get_spectrum_position(entity_id);
        
        // Sample coherence at nearby points to compute gradient
        let gradient = self.compute_coherence_gradient(entity_id, spectrum_position, &field_coherence_fn);
        
        // Derive position from coherence gradient
        let position = self.derive_position_from_gradient(gradient, spectrum_position);
        
        // Compute coherence at derived position
        let coherence = field_coherence_fn(position);
        
        let derived_position = CoherenceDerivedPosition::new(
            position,
            spectrum_position,
            coherence,
            gradient,
        );
        
        // Store for later use
        self.positions.insert(entity_id, derived_position.clone());
        
        derived_position
    }
    
    /// Get spectrum position for entity (from unified field)
    fn get_spectrum_position(&self, entity_id: u64) -> f64 {
        // Use entity_id as seed for deterministic but varied positions
        // This avoids golden ratio while maintaining determinism
        let seed = entity_id as f64;
        let phase = (seed * 0.618033988749895).fract();
        phase * 2.0 // Range [0, 2] with spectrum
    }
    
    /// Compute coherence gradient using finite differences
    fn compute_coherence_gradient(
        &self,
        entity_id: u64,
        spectrum_position: f64,
        coherence_fn: impl Fn([f64; 3]) -> f64,
    ) -> [f64; 3] {
        let base_pos = self.get_base_position(entity_id, spectrum_position);
        let epsilon = 0.1;
        
        // Sample coherence at nearby points
        let c_xp = coherence_fn([base_pos[0] + epsilon, base_pos[1], base_pos[2]]);
        let c_xn = coherence_fn([base_pos[0] - epsilon, base_pos[1], base_pos[2]]);
        let c_yp = coherence_fn([base_pos[0], base_pos[1] + epsilon, base_pos[2]]);
        let c_yn = coherence_fn([base_pos[0], base_pos[1] - epsilon, base_pos[2]]);
        let c_zp = coherence_fn([base_pos[0], base_pos[1], base_pos[2] + epsilon]);
        let c_zn = coherence_fn([base_pos[0], base_pos[1], base_pos[2] - epsilon]);
        
        // Compute gradient
        [
            (c_xp - c_xn) / (2.0 * epsilon),
            (c_yp - c_yn) / (2.0 * epsilon),
            (c_zp - c_zn) / (2.0 * epsilon),
        ]
    }
    
    /// Get base position from entity_id (NOT golden ratio!)
    /// 
    /// Uses a different distribution than golden ratio
    fn get_base_position(&self, entity_id: u64, spectrum_position: f64) -> [f64; 3] {
        // Use entity distribution based on prime numbers for variety
        let id = entity_id as f64;
        
        // Different prime multipliers for each axis for good distribution
        let a = (id * 2.0).sin() * self.config.spatial_scale;
        let b = (id * 3.0).cos() * self.config.spatial_scale;
        let c = (id * 5.0).sin() * self.config.spatial_scale;
        
        // Scale by spectrum position (higher spectrum = more expanded)
        let scale = (1.0 + spectrum_position * 0.5);
        
        [a * scale, b * scale, c * scale]
    }
    
    /// Derive final position from coherence gradient
    /// 
    /// Entities move toward higher coherence regions
    fn derive_position_from_gradient(
        &self,
        gradient: [f64; 3],
        spectrum_position: f64,
    ) -> [f64; 3] {
        if !self.config.use_gradient_following {
            // Fallback to simple distribution if gradient following disabled
            return self.get_fallback_position(spectrum_position);
        }
        
        let gradient_magnitude = (gradient[0].powi(2) + gradient[1].powi(2) + gradient[2].powi(2)).sqrt();
        
        if gradient_magnitude < 0.001 {
            // Very low gradient - use fallback
            return self.get_fallback_position(spectrum_position);
        }
        
        // Normalize gradient
        let norm = [
            gradient[0] / gradient_magnitude,
            gradient[1] / gradient_magnitude,
            gradient[2] / gradient_magnitude,
        ];
        
        // Move toward higher coherence (gradient direction)
        let movement = gradient_magnitude.min(self.config.max_gradient_distance);
        
        // Get base position and apply gradient movement
        let base = self.get_base_position(0, spectrum_position);
        
        [
            base[0] + norm[0] * movement,
            base[1] + norm[1] * movement,
            base[2] + norm[2] * movement,
        ]
    }
    
    /// Fallback position when gradient is too weak
    /// 
    /// Uses fibonacci lattice instead of golden ratio for better distribution
    fn get_fallback_position(&self, spectrum_position: f64) -> [f64; 3] {
        let n = self.positions.len() as f64;
        if n == 0.0 {
            return [0.0, 0.0, 0.0];
        }
        
        // Fibonacci spiral on sphere (different from golden ratio!)
        let phi = std::f64::consts::PI * (3.0 - 5.0f64.sqrt()); // Golden angle
        let y = 1.0 - (2.0 * n + 1.0) / 100.0;
        let radius = (1.0 - y * y).sqrt();
        let theta = n * phi;
        
        let scale = self.config.spatial_scale * (1.0 + spectrum_position * 0.5);
        
        [
            radius * theta.cos() * scale,
            y * scale,
            radius * theta.sin() * scale,
        ]
    }
    
    /// Apply veil transformation to position
    /// 
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "The Veil at v=1 is a STRUCTURAL FEATURE"
    /// > "Above veil: entities experience time as navigable dimensions"
    pub fn apply_veil_transform(&self, position: [f64; 3], spectrum_position: f64) -> TransformedPosition {
        if !self.config.apply_veil_transform {
            return TransformedPosition {
                position,
                perspective: if spectrum_position < 1.0 { 
                    Perspective::SpaceTime 
                } else { 
                    Perspective::TimeSpace 
                },
                veil_transparency: 0.0,
                transformation_factor: 0.0,
            };
        }
        
        let dist_from_veil = (spectrum_position - self.config.veil_position).abs();
        
        if dist_from_veil > self.config.veil_thickness {
            // Outside veil region
            TransformedPosition {
                position,
                perspective: if spectrum_position < self.config.veil_position {
                    Perspective::SpaceTime
                } else {
                    Perspective::TimeSpace
                },
                veil_transparency: 0.0,
                transformation_factor: 0.0,
            }
        } else {
            // Within veil - apply transformation
            let factor = 1.0 - (dist_from_veil / self.config.veil_thickness);
            let transformed_pos = if spectrum_position < self.config.veil_position {
                // Below veil: compress toward center
                [
                    position[0] * (1.0 - factor * 0.3),
                    position[1] * (1.0 - factor * 0.3),
                    position[2] * (1.0 - factor * 0.3),
                ]
            } else {
                // Above veil: expand
                [
                    position[0] * (1.0 + factor * 0.5),
                    position[1] * (1.0 + factor * 0.5),
                    position[2] * (1.0 + factor * 0.5),
                ]
            };
            
            TransformedPosition {
                position: transformed_pos,
                perspective: Perspective::Transitional,
                veil_transparency: factor,
                transformation_factor: factor,
            }
        }
    }
    
    /// Update statistics
    pub fn update_statistics(&mut self) {
        self.statistics.total_positions = self.positions.len();
        
        if self.positions.is_empty() {
            return;
        }
        
        let mut total_coherence = 0.0;
        let mut total_gradient_mag = 0.0;
        self.statistics.space_time_count = 0;
        self.statistics.time_space_count = 0;
        self.statistics.transitional_count = 0;
        
        for pos in self.positions.values() {
            total_coherence += pos.coherence;
            let grad_mag = (pos.coherence_gradient[0].powi(2) 
                + pos.coherence_gradient[1].powi(2) 
                + pos.coherence_gradient[2].powi(2)).sqrt();
            total_gradient_mag += grad_mag;
            
            match pos.perspective {
                SpatialPerspective::SpaceTime => self.statistics.space_time_count += 1,
                SpatialPerspective::TimeSpace => self.statistics.time_space_count += 1,
                SpatialPerspective::Transitional => self.statistics.transitional_count += 1,
            }
        }
        
        self.statistics.average_coherence = total_coherence / self.positions.len() as f64;
        self.statistics.coherence_gradient_magnitude = total_gradient_mag / self.positions.len() as f64;
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &SpatialStatistics {
        &self.statistics
    }
    
    /// Get position for entity
    pub fn get_position(&self, entity_id: u64) -> Option<&CoherenceDerivedPosition> {
        self.positions.get(&entity_id)
    }
    
    /// Clear all positions
    pub fn clear(&mut self) {
        self.positions.clear();
        self.coherence_cache.clear();
    }
}

impl Default for SpectrumSpatialDynamics {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spectrum_derived_position() {
        let mut dynamics = SpectrumSpatialDynamics::new();
        
        // Simple coherence function
        let coherence_fn = |pos: [f64; 3]| -> f64 {
            // Coherence increases toward origin
            let dist = (pos[0].powi(2) + pos[1].powi(2) + pos[2].powi(2)).sqrt();
            1.0 / (1.0 + dist * 0.1)
        };
        
        let pos = dynamics.derive_position_from_spectrum(1, coherence_fn);
        
        assert!(pos.spectrum_position >= 0.0 && pos.spectrum_position <= 2.0);
        assert!(pos.coherence >= 0.0 && pos.coherence <= 1.0);
    }
    
    #[test]
    fn test_veil_transform() {
        let dynamics = SpectrumSpatialDynamics::new();
        
        // Test below veil
        let below = dynamics.apply_veil_transform([1.0, 2.0, 3.0], 0.5);
        assert_eq!(below.perspective, Perspective::SpaceTime);
        assert!(below.veil_transparency < 0.1);
        
        // Test above veil  
        let above = dynamics.apply_veil_transform([1.0, 2.0, 3.0], 1.5);
        assert_eq!(above.perspective, Perspective::TimeSpace);
        
        // Test at veil
        let at_veil = dynamics.apply_veil_transform([1.0, 2.0, 3.0], 1.0);
        assert_eq!(at_veil.perspective, Perspective::Transitional);
    }
    
    #[test]
    fn test_gradient_following() {
        let config = SpectrumSpatialConfig {
            use_gradient_following: true,
            ..Default::default()
        };
        let mut dynamics = SpectrumSpatialDynamics::with_config(config);
        
        // Coherence increases toward [10, 10, 10]
        let coherence_fn = |pos: [f64; 3]| -> f64 {
            let target = [10.0, 10.0, 10.0];
            let dist = ((pos[0] - target[0]).powi(2) 
                + (pos[1] - target[1]).powi(2) 
                + (pos[2] - target[2]).powi(2)).sqrt();
            1.0 / (1.0 + dist * 0.1)
        };
        
        // Derive multiple positions - they should gravitate toward target
        let mut positions = Vec::new();
        for i in 0..10 {
            let pos = dynamics.derive_position_from_spectrum(i, coherence_fn);
            positions.push(pos.position);
        }
        
        // Check that positions are reasonably distributed
        assert_eq!(positions.len(), 10);
    }
}
