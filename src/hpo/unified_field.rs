//! Unified Field Equation (Phase 1)
//!
//! From REFACTOR_ROADMAP_COMPREHENSIVE_2026.md:
//! "The unified field equation integrates the three distortions as terms in a partial 
//! differential equation governing field evolution."
//!
//! This module implements the Three Primal Distortions as unified field dynamics:
//! - Free Will: Stochastic perturbation term (not entity attribute!)
//! - Love/Light: Attractive potential (coherence field)
//! - Light: Wave propagation dynamics
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Three Primal Distortions emanate from the ONE: Free Will (stochastic), 
//! Love/Light (attractive), Light (propagating)"

use super::field_state::{Complex, DensityBand, FieldNodeData, Float, OctreeNode};
use std::collections::HashMap;

/// Configuration for the unified field equation
#[derive(Debug, Clone)]
pub struct UnifiedFieldConfig {
    /// Free Will: Amplitude of stochastic perturbations (0.0 - 1.0)
    pub free_will_amplitude: Float,
    
    /// Free Will: Spatial correlation scale (0.0 = uncorrelated, 1.0 = fully correlated)
    pub free_will_correlation: Float,
    
    /// Love/Light: Attraction strength (0.0 - 1.0)
    pub love_attraction_strength: Float,
    
    /// Love/Light: Attraction range (in spatial units)
    pub love_attraction_range: Float,
    
    /// Light: Wave propagation speed
    pub light_propagation_speed: Float,
    
    /// Light: Damping factor (0.0 = no damping, 1.0 = complete damping)
    pub light_damping: Float,
    
    /// Time step for field evolution
    pub time_step: Float,
    
    /// Number of density bands
    pub density_band_count: usize,
}

impl Default for UnifiedFieldConfig {
    fn default() -> Self {
        UnifiedFieldConfig {
            free_will_amplitude: 0.1,
            free_will_correlation: 0.0,
            love_attraction_strength: 0.5,
            love_attraction_range: 100.0,
            light_propagation_speed: 1.0,
            light_damping: 0.01,
            time_step: 0.01,
            density_band_count: 8,
        }
    }
}

/// The Three Primal Distortions as field terms
#[derive(Debug, Clone)]
pub struct PrimalDistortionTerms {
    /// Free Will: Stochastic perturbations
    pub free_will: FreeWillTerm,
    
    /// Love/Light: Attractive potential
    pub love: LoveTerm,
    
    /// Light: Wave propagation
    pub light: LightTerm,
}

/// Free Will term: Stochastic perturbation kernel
/// From COSMOLOGICAL-ARCHITECTURE.md: "Free Will emanates from the ONE as the first distortion"
#[derive(Debug, Clone)]
pub struct FreeWillTerm {
    /// Amplitude of perturbations
    pub amplitude: Float,
    
    /// Spatial correlation
    pub correlation: Float,
}

impl FreeWillTerm {
    pub fn new(amplitude: Float, correlation: Float) -> Self {
        FreeWillTerm {
            amplitude,
            correlation,
        }
    }

    /// Apply stochastic perturbation to field node
    /// This represents Free Will breaking the perfect symmetry
    pub fn apply(&mut self, node_data: &mut FieldNodeData, position: &[Float; 3], time: Float) {
        // Use position and time as seed for pseudo-random behavior
        let seed = ((position[0] * 1000.0 + position[1] * 100.0 + position[2] * 10.0 + time * 100.0) as i64).abs() as u64;
        
        for density_idx in 0..8 {
            // Generate pseudo-stochastic perturbation
            let noise = (((seed.wrapping_mul(density_idx as u64 + 1)) % 1000) as Float / 500.0 - 1.0);
            
            // Scale by amplitude
            let perturbation = noise * self.amplitude * 0.1;
            
            // Apply to real part of amplitude
            node_data.density_amplitudes[density_idx].re += perturbation;
            
            // Keep amplitude bounded
            let mag = node_data.density_amplitudes[density_idx].magnitude();
            if mag > 1.0 {
                let scale = 1.0 / mag;
                node_data.density_amplitudes[density_idx] = node_data.density_amplitudes[density_idx].scale(scale);
            }
        }
        
        // Free Will also affects coherence - entities have freedom to change their coherence
        let coherence_change = ((seed % 100) as Float / 2500.0 - 0.02);
        node_data.coherence = (node_data.coherence + coherence_change * self.amplitude).clamp(0.0, 1.0);
    }
}

/// Love/Light term: Attractive potential that pulls toward coherence
/// From COSMOLOGICAL-ARCHITECTURE.md: "Love/Logos creates structure through attractive potential"
#[derive(Debug, Clone)]
pub struct LoveTerm {
    /// Attraction strength
    pub strength: Float,
    
    /// Attraction range
    pub range: Float,
}

impl LoveTerm {
    pub fn new(strength: Float, range: Float) -> Self {
        LoveTerm {
            strength,
            range,
        }
    }

    /// Apply attractive force toward higher coherence regions
    /// This is analogous to gravity but for consciousness/coherence
    pub fn apply(&mut self, node: &mut OctreeNode) {
        let local_coherence = node.field_data.coherence;
        
        // Attractive force pulls toward higher coherence
        // F = -strength * gradient(coherence)
        let attraction = self.strength * local_coherence * 0.1;
        
        // Apply attraction to energy field
        node.field_data.energy *= (1.0 + attraction * 0.01);
        
        // Love also increases coherence locally
        node.field_data.coherence = (node.field_data.coherence + attraction * 0.01).min(1.0);
        
        // Cross-density attraction: Love pulls all densities toward Green (density 3)
        // This represents the unifying nature of Love
        let green_mag = node.field_data.density_amplitudes[3].magnitude();
        
        // Collect current magnitudes first to avoid borrow issues
        let mut mags: [Float; 8] = [0.0; 8];
        for i in 0..8 {
            mags[i] = node.field_data.density_amplitudes[i].magnitude();
        }
        
        // Now apply transfers
        for i in 0..8 {
            if i != 3 && mags[i] > 0.001 {
                let transfer = mags[i] * self.strength * 0.001;
                node.field_data.density_amplitudes[3].re += transfer;
            }
        }
        
        // Ensure Green maintains balance
        if green_mag > 0.0 && green_mag < 0.01 {
            node.field_data.density_amplitudes[3].re = 0.01;
        }
    }
}

/// Light term: Wave propagation dynamics
/// From COSMOLOGICAL-ARCHITECTURE.md: "Light propagates as wave dynamics allowing 
/// information/energy to travel through the field"
#[derive(Debug, Clone)]
pub struct LightTerm {
    /// Propagation speed
    pub propagation_speed: Float,
    
    /// Damping factor
    pub damping: Float,
    
    /// Previous field state for wave equation
    prev_state: Vec<(Float, Float, Float, Float)>, // x, y, z, energy
}

impl LightTerm {
    pub fn new(propagation_speed: Float, damping: Float) -> Self {
        LightTerm {
            propagation_speed,
            damping,
            prev_state: Vec::new(),
        }
    }

    /// Apply wave propagation: ∂²ψ/∂t² = c²∇²ψ - damping ∂ψ/∂t
    /// This is the wave equation with damping
    pub fn apply(&mut self, node: &mut OctreeNode, dt: Float) {
        let center = node.bounds.center();
        let size = node.bounds.size();
        
        // Calculate spatial derivatives (simplified Laplacian)
        let dx = if size[0] > 0.001 { size[0] } else { 1.0 };
        let dy = if size[1] > 0.001 { size[1] } else { 1.0 };
        let dz = if size[2] > 0.001 { size[2] } else { 1.0 };
        
        let current_energy = node.field_data.energy;
        
        // Get previous energy (or use current if not available)
        let mut prev_energy = current_energy;
        for (px, py, pz, pe) in self.prev_state.iter() {
            if (px - center[0]).abs() < 0.1 && (py - center[1]).abs() < 0.1 && (pz - center[2]).abs() < 0.1 {
                prev_energy = *pe;
                break;
            }
        }
        
        // Wave equation: acceleration = c² * Laplacian - damping * velocity
        let laplacian = (current_energy - prev_energy) / (dx * dx + dy * dy + dz * dz);
        let velocity = current_energy - prev_energy;
        
        let acceleration = self.propagation_speed * self.propagation_speed * laplacian 
            - self.damping * velocity / dt;
        
        // Update energy with wave dynamics
        node.field_data.energy += acceleration * dt * dt;
        
        // Light creates oscillation in density amplitudes
        let oscillation = (center[0] * 0.1 + center[1] * 0.1 + center[2] * 0.1).sin();
        for amp in node.field_data.density_amplitudes.iter_mut() {
            let light_oscillation = oscillation * self.propagation_speed * 0.01;
            amp.re += light_oscillation;
        }
        
        // Store current state for next iteration
        self.prev_state.push((center[0], center[1], center[2], current_energy));
        
        // Keep only last 100 states to prevent memory growth
        if self.prev_state.len() > 100 {
            self.prev_state.remove(0);
        }
    }
}

impl PrimalDistortionTerms {
    pub fn new(config: &UnifiedFieldConfig) -> Self {
        PrimalDistortionTerms {
            free_will: FreeWillTerm::new(config.free_will_amplitude, config.free_will_correlation),
            love: LoveTerm::new(config.love_attraction_strength, config.love_attraction_range),
            light: LightTerm::new(config.light_propagation_speed, config.light_damping),
        }
    }
}

/// Unified Field Equation
/// Combines all three primal distortions into a single evolution equation
pub struct UnifiedFieldEquation {
    /// Configuration
    config: UnifiedFieldConfig,
    
    /// The three distortion terms
    distortions: PrimalDistortionTerms,
    
    /// Current simulation time
    time: Float,
    
    /// Statistics
    pub statistics: UnifiedFieldStatistics,
}

/// Statistics for unified field evolution
#[derive(Debug, Clone, Default)]
pub struct UnifiedFieldStatistics {
    /// Total Free Will perturbations applied
    pub free_will_applications: usize,
    
    /// Total Love attractions applied
    pub love_applications: usize,
    
    /// Total Light wave propagations
    pub light_applications: usize,
    
    /// Average field coherence
    pub average_coherence: Float,
    
    /// Total field energy
    pub total_energy: Float,
}

impl UnifiedFieldEquation {
    pub fn new(config: UnifiedFieldConfig) -> Self {
        UnifiedFieldEquation {
            config: config.clone(),
            distortions: PrimalDistortionTerms::new(&config),
            time: 0.0,
            statistics: UnifiedFieldStatistics::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(UnifiedFieldConfig::default())
    }

    /// Evolve the field by one time step
    /// Applies all three primal distortions in sequence
    pub fn evolve(&mut self, root: &mut OctreeNode) {
        let dt = self.config.time_step;
        
        // Recursively apply field evolution to all nodes
        self.evolve_node(root, dt);
        
        self.time += dt;
        self.update_statistics(root);
    }

    /// Evolve a single node and its children
    fn evolve_node(&mut self, node: &mut OctreeNode, dt: Float) {
        let position = node.bounds.center();
        
        // Apply Free Will (stochastic perturbation)
        // This must be applied first as it breaks symmetry
        self.distortions.free_will.apply(&mut node.field_data, &position, self.time);
        self.statistics.free_will_applications += 1;
        
        // Apply Love/Light (attractive potential)
        // This creates structure through coherence
        self.distortions.love.apply(node);
        self.statistics.love_applications += 1;
        
        // Apply Light (wave propagation)
        // This allows information to travel through the field
        self.distortions.light.apply(node, dt);
        self.statistics.light_applications += 1;
        
        // Recursively evolve children
        if let Some(ref mut children) = node.children {
            for child in children.iter_mut() {
                self.evolve_node(child, dt);
            }
        }
    }

    /// Update field statistics
    fn update_statistics(&mut self, root: &mut OctreeNode) {
        let mut total_coherence = 0.0;
        let mut total_energy = 0.0;
        let mut node_count = 0;
        
        self.collect_stats(root, &mut total_coherence, &mut total_energy, &mut node_count);
        
        if node_count > 0 {
            self.statistics.average_coherence = total_coherence / node_count as Float;
            self.statistics.total_energy = total_energy;
        }
    }

    /// Collect statistics recursively
    fn collect_stats(&self, node: &OctreeNode, total_coherence: &mut Float, total_energy: &mut Float, count: &mut usize) {
        *total_coherence += node.field_data.coherence;
        *total_energy += node.field_data.energy;
        *count += 1;
        
        if let Some(ref children) = node.children {
            for child in children.iter() {
                self.collect_stats(child, total_coherence, total_energy, count);
            }
        }
    }

    /// Get current simulation time
    pub fn get_time(&self) -> Float {
        self.time
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &UnifiedFieldStatistics {
        &self.statistics
    }

    /// Get configuration
    pub fn get_config(&self) -> &UnifiedFieldConfig {
        &self.config
    }

    /// Modify Free Will amplitude at runtime
    pub fn set_free_will_amplitude(&mut self, amplitude: Float) {
        self.config.free_will_amplitude = amplitude;
        self.distortions.free_will.amplitude = amplitude;
    }

    /// Modify Love attraction strength at runtime
    pub fn set_love_strength(&mut self, strength: Float) {
        self.config.love_attraction_strength = strength;
        self.distortions.love.strength = strength;
    }

    /// Modify Light propagation speed at runtime
    pub fn set_light_speed(&mut self, speed: Float) {
        self.config.light_propagation_speed = speed;
        self.distortions.light.propagation_speed = speed;
    }
}

/// Builder for UnifiedFieldEquation
pub struct UnifiedFieldBuilder {
    config: UnifiedFieldConfig,
}

impl UnifiedFieldBuilder {
    pub fn new() -> Self {
        UnifiedFieldBuilder {
            config: UnifiedFieldConfig::default(),
        }
    }

    pub fn with_free_will(mut self, amplitude: Float, correlation: Float) -> Self {
        self.config.free_will_amplitude = amplitude;
        self.config.free_will_correlation = correlation;
        self
    }

    pub fn with_love(mut self, strength: Float, range: Float) -> Self {
        self.config.love_attraction_strength = strength;
        self.config.love_attraction_range = range;
        self
    }

    pub fn with_light(mut self, speed: Float, damping: Float) -> Self {
        self.config.light_propagation_speed = speed;
        self.config.light_damping = damping;
        self
    }

    pub fn with_time_step(mut self, dt: Float) -> Self {
        self.config.time_step = dt;
        self
    }

    pub fn build(self) -> UnifiedFieldEquation {
        UnifiedFieldEquation::new(self.config)
    }
}

impl Default for UnifiedFieldBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::field_state::{FieldBounds, HolographicFieldState};

    #[test]
    fn test_unified_field_creation() {
        let field = UnifiedFieldEquation::with_defaults();
        assert!(field.get_time() >= 0.0);
    }

    #[test]
    fn test_field_builder() {
        let field = UnifiedFieldBuilder::new()
            .with_free_will(0.2, 0.1)
            .with_love(0.6, 150.0)
            .with_light(2.0, 0.02)
            .build();
        
        let config = field.get_config();
        assert_eq!(config.free_will_amplitude, 0.2);
        assert_eq!(config.love_attraction_strength, 0.6);
        assert_eq!(config.light_propagation_speed, 2.0);
    }

    #[test]
    fn test_field_evolution() {
        let mut field = UnifiedFieldEquation::with_defaults();
        let mut state = HolographicFieldState::with_defaults();
        
        // Add some energy to the field
        state.add_energy_at([0.0, 0.0, 0.0], 3, 1.0);
        
        // Evolve the field
        field.evolve(&mut state.root);
        
        // Field should have evolved (time should have advanced)
        assert!(field.get_time() > 0.0);
    }

    #[test]
    fn test_free_will_modification() {
        let mut field = UnifiedFieldEquation::with_defaults();
        
        field.set_free_will_amplitude(0.5);
        assert_eq!(field.get_config().free_will_amplitude, 0.5);
    }
}
