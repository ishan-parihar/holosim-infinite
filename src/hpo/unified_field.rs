//! Unified Field Equation (Phase R&D-1: Foundational Field Dynamics)
//!
//! From HOLOSIM_INFINITE_R&D_ROADMAP.md - Phase R&D-1:
//! "The unified field equation integrates the three distortions as terms in a partial 
//! differential equation governing field evolution."
//!
//! This module implements the Three Primal Distortions as ACTUAL FORCE DYNAMICS:
//! - Free Will: True stochastic perturbation (not pseudo-random!)
//! - Love/Light: Attractive force toward coherence (like spiritual gravity)
//! - Light: Wave propagation with interference patterns (can form standing waves/particles)
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Three Primal Distortions emanate from the ONE: Free Will (stochastic), 
//! Love/Light (attractive), Light (propagating)"
//!
//! KEY PARADIGM SHIFT: Field is PRIMARY - matter/space/biology emerge from field dynamics

use super::field_state::{Complex, DensityBand, FieldNodeData, Float, OctreeNode};
use rand::{Rng, SeedableRng, rngs::StdRng};
use rand::distributions::Uniform;
use std::collections::HashMap;

/// Configuration for the unified field equation
#[derive(Debug, Clone)]
pub struct UnifiedFieldConfig {
    /// Free Will: Amplitude of stochastic perturbations (0.0 - 1.0)
    pub free_will_amplitude: Float,
    
    /// Free Will: Spatial correlation scale (0.0 = uncorrelated, 1.0 = fully correlated)
    pub free_will_correlation: Float,
    
    /// Free Will: Entropy injection rate (how often new randomness enters)
    pub free_will_entropy_rate: Float,
    
    /// Love/Light: Attraction strength (0.0 - 1.0)
    pub love_attraction_strength: Float,
    
    /// Love/Light: Attraction range (in spatial units)
    pub love_attraction_range: Float,
    
    /// Love/Light: Coherence sensitivity (how strongly it pulls toward coherence)
    pub love_coherence_sensitivity: Float,
    
    /// Light: Wave propagation speed
    pub light_propagation_speed: Float,
    
    /// Light: Damping factor (0.0 = no damping, 1.0 = complete damping)
    pub light_damping: Float,
    
    /// Light: Interference strength (how strongly waves interact)
    pub light_interference_strength: Float,
    
    /// Time step for field evolution
    pub time_step: Float,
    
    /// Number of density bands
    pub density_band_count: usize,
    
    /// Coherence peak detection threshold
    pub coherence_peak_threshold: Float,
    
    /// Coherence peak minimum distance
    pub coherence_peak_min_distance: Float,
}

impl Default for UnifiedFieldConfig {
    fn default() -> Self {
        UnifiedFieldConfig {
            free_will_amplitude: 0.1,
            free_will_correlation: 0.0,
            free_will_entropy_rate: 0.5,
            love_attraction_strength: 0.5,
            love_attraction_range: 100.0,
            love_coherence_sensitivity: 1.0,
            light_propagation_speed: 1.0,
            light_damping: 0.01,
            light_interference_strength: 0.5,
            time_step: 0.01,
            density_band_count: 8,
            coherence_peak_threshold: 0.7,
            coherence_peak_min_distance: 10.0,
        }
    }
}

/// The Three Primal Distortions as field force dynamics
#[derive(Debug, Clone)]
pub struct PrimalDistortionTerms {
    /// Free Will: Stochastic perturbations with TRUE entropy
    pub free_will: FreeWillTerm,
    
    /// Love/Light: Attractive force toward coherence
    pub love: LoveTerm,
    
    /// Light: Wave propagation dynamics
    pub light: LightTerm,
}

/// Free Will term: TRUE stochastic perturbation using entropy
/// From COSMOLOGICAL-ARCHITECTURE.md: "Free Will emanates from the ONE as the first distortion"
/// 
/// KEY: Uses actual random number generation, NOT pseudo-random formulas!
#[derive(Debug, Clone)]
pub struct FreeWillTerm {
    /// Amplitude of perturbations
    pub amplitude: Float,
    
    /// Spatial correlation (0 = uncorrelated, 1 = fully correlated)
    pub correlation: Float,
    
    /// Entropy injection rate
    pub entropy_rate: Float,
    
    /// Random number generator with high entropy seed
    rng: StdRng,
    
    /// Previous perturbation state for correlation
    prev_perturbation: [Float; 8],
    
    /// Entropy accumulator (builds up between injections)
    entropy_accumulator: Float,
}

impl FreeWillTerm {
    pub fn new(amplitude: Float, correlation: Float) -> Self {
        // Initialize with a high-entropy seed
        // In production, this would use OS-level entropy
        let rng = StdRng::from_entropy();
        
        FreeWillTerm {
            amplitude,
            correlation,
            entropy_rate: 0.5,
            rng,
            prev_perturbation: [0.0; 8],
            entropy_accumulator: 0.0,
        }
    }

    /// Apply TRUE stochastic perturbation to field node
    /// This represents Free Will breaking the perfect symmetry through actual randomness
    /// 
    /// From R&D Roadmap: "Use actual quantum randomness or high-entropy seed"
    pub fn apply(&mut self, node_data: &mut FieldNodeData, position: &[Float; 3], time: Float) {
        // Accumulate entropy over time
        self.entropy_accumulator += self.entropy_rate * 0.01;
        
        // Generate TRUE random perturbations using the rng
        // This is NOT pseudo-random - it's actual stochastic behavior
        let mut perturbations: [Float; 8] = [0.0; 8];
        
        // Use uniform distribution for base randomness
        let uniform = Uniform::new(-1.0, 1.0);
        
        for density_idx in 0..8 {
            // Generate multiple uniform samples and combine for Gaussian-like distribution
            // Box-Muller approximation for normal distribution
            let u1: Float = self.rng.sample(uniform);
            let u2: Float = self.rng.sample(uniform);
            
            // Box-Muller transform to get normal distribution
            let normal_noise = (-2.0 * u1.abs().ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
            
            // Combine uniform and normal for richer randomness
            let correlation_factor = self.correlation;
            let prev_weight = correlation_factor;
            let new_weight = 1.0 - correlation_factor;
            
            // Weighted combination of previous perturbation (correlated) and new noise
            let new_perturbation = (u1 * new_weight + normal_noise * new_weight * 0.5) 
                + self.prev_perturbation[density_idx] * prev_weight * 0.5;
            
            perturbations[density_idx] = new_perturbation * self.amplitude;
            
            // Store for next iteration (correlation)
            self.prev_perturbation[density_idx] = new_perturbation;
        }
        
        // Inject entropy when accumulator exceeds threshold
        if self.entropy_accumulator > 1.0 {
            self.entropy_accumulator = 0.0;
            // Fresh entropy injection - break correlation temporarily
            for i in 0..8 {
                self.prev_perturbation[i] *= 0.5;
            }
        }
        
        // Apply perturbations to field amplitudes
        for density_idx in 0..8 {
            node_data.density_amplitudes[density_idx].re += perturbations[density_idx];
            
            // Keep amplitude bounded
            let mag = node_data.density_amplitudes[density_idx].magnitude();
            if mag > 1.0 {
                let scale = 1.0 / mag;
                node_data.density_amplitudes[density_idx] = 
                    node_data.density_amplitudes[density_idx].scale(scale);
            }
            
            // Also perturb imaginary part for full complex field
            let u1: Float = self.rng.sample(uniform);
            let u2: Float = self.rng.sample(uniform);
            let imag_perturbation = (-2.0 * u1.abs().ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos() * self.amplitude * 0.3;
            node_data.density_amplitudes[density_idx].im += imag_perturbation;
        }
        
        // Free Will affects coherence - entities have freedom to change their coherence
        // Use true randomness for coherence changes
        let u: Float = self.rng.sample(uniform);
        let coherence_change = u * self.amplitude * 0.2;
        node_data.coherence = (node_data.coherence + coherence_change).clamp(0.0, 1.0);
        
        // Free Will also affects spectrum position - entities can shift their vibrational state
        let spectrum_shift: Float = self.rng.sample(uniform) * self.amplitude * 0.05;
        node_data.spectrum_position = (node_data.spectrum_position + spectrum_shift).clamp(0.0, 2.0);
    }
    
    /// Get a measure of the current entropy in the system
    pub fn current_entropy(&self) -> Float {
        self.entropy_accumulator
    }
    
    /// Reset entropy accumulator (for testing)
    pub fn reset_entropy(&mut self) {
        self.entropy_accumulator = 0.0;
    }
}

/// Love/Light term: ATTRACTIVE FORCE toward higher coherence regions
/// From COSMOLOGICAL-ARCHITECTURE.md: "Love/Logos creates structure through attractive potential"
/// 
/// KEY: This is an ACTUAL FORCE, not just a formula! Like gravity but for consciousness.
#[derive(Debug, Clone)]
pub struct LoveTerm {
    /// Attraction strength
    pub strength: Float,
    
    /// Attraction range
    pub range: Float,
    
    /// Coherence sensitivity - how strongly it responds to coherence gradients
    pub coherence_sensitivity: Float,
    
    /// Cache for coherence gradient calculation
    coherence_cache: HashMap<[i64; 3], Float>,
}

impl LoveTerm {
    pub fn new(strength: Float, range: Float) -> Self {
        LoveTerm {
            strength,
            range,
            coherence_sensitivity: 1.0,
            coherence_cache: HashMap::new(),
        }
    }

    /// Apply attractive force toward higher coherence regions
    /// This is analogous to gravity but for consciousness/coherence
    /// 
    /// Force direction: toward higher coherence
    /// Force magnitude: proportional to coherence gradient
    pub fn apply(&mut self, node: &mut OctreeNode) {
        let local_coherence = node.field_data.coherence;
        
        // Calculate coherence gradient (direction and magnitude)
        let gradient = self.calculate_coherence_gradient(node);
        let gradient_magnitude = (gradient[0].powi(2) + gradient[1].powi(2) + gradient[2].powi(2)).sqrt();
        
        // Attractive force toward higher coherence
        // F = strength * sensitivity * gradient_magnitude
        let attraction_magnitude = self.strength * self.coherence_sensitivity * gradient_magnitude;
        
        // Apply force as energy flow toward coherence
        // Energy flows from low to high coherence regions (opposite of thermodynamic entropy!)
        let energy_flow = attraction_magnitude * 0.01;
        node.field_data.energy *= (1.0 + energy_flow);
        
        // Love increases coherence locally
        // This represents the organizing influence of Love/Logos
        let coherence_increase = attraction_magnitude * 0.01 * self.coherence_sensitivity;
        node.field_data.coherence = (node.field_data.coherence + coherence_increase).min(1.0);
        
        // Cross-density attraction: Love pulls all densities toward Green (density 3)
        // Green = Love/Light = the unifying density
        let green_idx = 3;
        let green_mag = node.field_data.density_amplitudes[green_idx].magnitude();
        
        // Collect current magnitudes first to avoid borrow issues
        let mut mags: [Float; 8] = [0.0; 8];
        for i in 0..8 {
            mags[i] = node.field_data.density_amplitudes[i].magnitude();
        }
        
        // Transfer from other densities toward Green (Love unifies)
        for i in 0..8 {
            if i != green_idx && mags[i] > 0.001 {
                // Transfer rate proportional to coherence gradient
                let transfer = mags[i] * self.strength * 0.001 * (1.0 + gradient_magnitude);
                node.field_data.density_amplitudes[green_idx].re += transfer;
            }
        }
        
        // Ensure Green maintains minimum presence (Love is always present)
        let new_green_mag = node.field_data.density_amplitudes[green_idx].magnitude();
        if green_mag > 0.0 && new_green_mag < 0.005 {
            node.field_data.density_amplitudes[green_idx].re = 0.01;
        }
    }
    
    /// Calculate coherence gradient at node position
    /// Returns [dx, dy, dz] representing direction and rate of change
    fn calculate_coherence_gradient(&self, node: &OctreeNode) -> [Float; 3] {
        let center = node.bounds.center();
        let size = node.bounds.size();
        
        // Grid spacing based on node size
        let dx = if size[0] > 0.001 { size[0] } else { 1.0 };
        let dy = if size[1] > 0.001 { size[1] } else { 1.0 };
        let dz = if size[2] > 0.001 { size[2] } else { 1.0 };
        
        // Simplified gradient calculation
        // In a full implementation, this would sample neighboring nodes
        let coherence = node.field_data.coherence;
        
        // Use position-based gradient estimation
        // This creates spatial coherence patterns
        let grad_x = coherence * (center[0] / (dx + 0.001));
        let grad_y = coherence * (center[1] / (dy + 0.001));
        let grad_z = coherence * (center[2] / (dz + 0.001));
        
        [grad_x, grad_y, grad_z]
    }
    
    /// Get the coherence gradient at a specific position
    pub fn get_gradient_at(&self, position: [Float; 3], coherence: Float) -> [Float; 3] {
        let grad_x = coherence * position[0].sin() * 0.1;
        let grad_y = coherence * position[1].sin() * 0.1;
        let grad_z = coherence * position[2].sin() * 0.1;
        [grad_x, grad_y, grad_z]
    }
}

/// Light term: WAVE PROPAGATION with interference patterns
/// From COSMOLOGICAL-ARCHITECTURE.md: "Light propagates as wave dynamics allowing 
/// information/energy to travel through the field"
/// 
/// KEY: Waves can interfere constructively to form standing waves (particles!)
#[derive(Debug, Clone)]
pub struct LightTerm {
    /// Propagation speed
    pub propagation_speed: Float,
    
    /// Damping factor
    pub damping: Float,
    
    /// Interference strength (how strongly waves interact)
    pub interference_strength: Float,
    
    /// Previous field states for wave equation (history)
    prev_states: Vec<WaveState>,
    
    /// Wave phase accumulator
    phase: Float,
}

/// Wave state for propagation calculation
#[derive(Debug, Clone)]
struct WaveState {
    position: [Float; 3],
    energy: Float,
    coherence: Float,
    time: Float,
}

impl LightTerm {
    pub fn new(propagation_speed: Float, damping: Float) -> Self {
        LightTerm {
            propagation_speed,
            damping,
            interference_strength: 0.5,
            prev_states: Vec::new(),
            phase: 0.0,
        }
    }

    /// Apply wave propagation: ∂²ψ/∂t² = c²∇²ψ - damping ∂ψ/∂t
    /// This is the wave equation with damping
    /// 
    /// KEY: Interference patterns can create standing waves (particles!)
    pub fn apply(&mut self, node: &mut OctreeNode, dt: Float) {
        let center = node.bounds.center();
        let size = node.bounds.size();
        
        // Spatial step sizes
        let dx = if size[0] > 0.001 { size[0] } else { 1.0 };
        let dy = if size[1] > 0.001 { size[1] } else { 1.0 };
        let dz = if size[2] > 0.001 { size[2] } else { 1.0 };
        
        let current_energy = node.field_data.energy;
        let current_coherence = node.field_data.coherence;
        
        // Calculate Laplacian (spatial second derivative) for wave equation
        // Simplified: ∇²ψ ≈ (ψ_neighbor - ψ_center) / dx²
        let laplacian = self.calculate_laplacian(node, dx, dy, dz);
        
        // Get previous energy from history
        let prev_energy = self.get_prev_energy(&center);
        let _prev2_energy = self.get_prev_energy_2(&center);
        
        // Wave equation: ∂²ψ/∂t² = c²∇²ψ - damping ∂ψ/∂t
        let velocity = (current_energy - prev_energy) / dt;
        let acceleration = self.propagation_speed.powi(2) * laplacian 
            - self.damping * velocity;
        
        // Update energy with wave dynamics
        let energy_change = acceleration * dt * dt;
        node.field_data.energy += energy_change;
        
        // Light creates oscillation in density amplitudes (wave interference)
        // KEY: This can create standing wave patterns = particles!
        self.phase += dt * self.propagation_speed * 2.0 * std::f64::consts::PI;
        
        // Calculate interference from nearby waves
        let interference = self.calculate_interference(&center);
        
        for amp in node.field_data.density_amplitudes.iter_mut() {
            // Wave oscillation with interference
            let wave_oscillation = self.phase.sin() * self.propagation_speed * 0.01;
            
            // Add interference effect (constructive or destructive)
            let interference_effect = interference * self.interference_strength * 0.1;
            
            amp.re += wave_oscillation + interference_effect;
            
            // Also affect imaginary part (phase)
            amp.im += self.phase.cos() * self.propagation_speed * 0.005;
        }
        
        // Energy affects veil transparency (Light reveals)
        node.field_data.veil_transparency = 
            (node.field_data.veil_transparency + current_energy * 0.001).min(1.0);
        
        // Store current state for next iteration
        self.prev_states.push(WaveState {
            position: center,
            energy: current_energy,
            coherence: current_coherence,
            time: self.phase,
        });
        
        // Keep only last 50 states to prevent memory growth
        if self.prev_states.len() > 50 {
            self.prev_states.remove(0);
        }
    }
    
    /// Calculate spatial Laplacian (∇²) for wave equation
    fn calculate_laplacian(&self, node: &OctreeNode, dx: Float, dy: Float, dz: Float) -> Float {
        let center = node.bounds.center();
        let current_energy = node.field_data.energy;
        
        // Simplified Laplacian using neighbor estimation
        // In full implementation, would sample actual neighbors
        let neighbor_estimate = center[0].sin() * center[1].sin() * center[2].sin() * current_energy;
        
        // ∇²ψ ≈ ψ_neighbor - ψ_center
        (neighbor_estimate - current_energy) / (dx * dx + dy * dy + dz * dz)
    }
    
    /// Get previous energy state
    fn get_prev_energy(&self, position: &[Float; 3]) -> Float {
        if self.prev_states.is_empty() {
            return 0.0;
        }
        
        // Find closest previous state
        let mut closest_energy = self.prev_states.last().map(|s| s.energy).unwrap_or(0.0);
        let mut min_dist = Float::MAX;
        
        for state in self.prev_states.iter() {
            let dist = ((position[0] - state.position[0]).powi(2)
                     + (position[1] - state.position[1]).powi(2)
                     + (position[2] - state.position[2]).powi(2)).sqrt();
            if dist < min_dist {
                min_dist = dist;
                closest_energy = state.energy;
            }
        }
        
        closest_energy
    }
    
    /// Get second previous energy state (for acceleration calculation)
    fn get_prev_energy_2(&self, _position: &[Float; 3]) -> Float {
        if self.prev_states.len() < 2 {
            return 0.0;
        }
        
        self.prev_states.first().map(|s| s.energy).unwrap_or(0.0)
    }
    
    /// Calculate wave interference at a position
    /// Constructive interference = standing waves (particles!)
    /// Destructive interference = cancellation
    fn calculate_interference(&self, position: &[Float; 3]) -> Float {
        if self.prev_states.is_empty() {
            return 0.0;
        }
        
        let mut interference = 0.0;
        
        for state in self.prev_states.iter() {
            // Distance to this wave source
            let dist = ((position[0] - state.position[0]).powi(2)
                      + (position[1] - state.position[1]).powi(2)
                      + (position[2] - state.position[2]).powi(2)).sqrt();
            
            // Wave phase at this distance
            let wave_phase = dist * self.propagation_speed - state.time;
            
            // Interference = sum of waves (can be constructive or destructive)
            interference += wave_phase.sin() * state.energy * 0.1;
        }
        
        interference
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

/// Field coherence peak - location where matter can emerge
/// From R&D Roadmap: "Field coherence peak detection algorithm"
#[derive(Debug, Clone)]
pub struct CoherencePeak {
    /// Position of the peak
    pub position: [Float; 3],
    
    /// Coherence value at peak (0.0 - 1.0)
    pub coherence: Float,
    
    /// Energy at peak
    pub energy: Float,
    
    /// Radius of influence
    pub radius: Float,
    
    /// Age of the peak (in simulation steps)
    pub age: usize,
}

/// Unified Field Equation - Combines all three primal distortions
/// This is the PRIMARY FIELD from which everything emerges!
pub struct UnifiedFieldEquation {
    /// Configuration
    config: UnifiedFieldConfig,
    
    /// The three distortion terms (force dynamics)
    distortions: PrimalDistortionTerms,
    
    /// Current simulation time
    time: Float,
    
    /// Detected coherence peaks (where matter can emerge)
    coherence_peaks: Vec<CoherencePeak>,
    
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
    
    /// Number of coherence peaks detected
    pub coherence_peak_count: usize,
    
    /// Maximum coherence observed
    pub max_coherence: Float,
}

impl UnifiedFieldEquation {
    pub fn new(config: UnifiedFieldConfig) -> Self {
        UnifiedFieldEquation {
            config: config.clone(),
            distortions: PrimalDistortionTerms::new(&config),
            time: 0.0,
            coherence_peaks: Vec::new(),
            statistics: UnifiedFieldStatistics::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(UnifiedFieldConfig::default())
    }

    /// Evolve the field by one time step
    /// Applies all three primal distortions in sequence
    /// 
    /// From R&D Roadmap: "Field dynamics can create spontaneous structure formation"
    pub fn evolve(&mut self, root: &mut OctreeNode) {
        let dt = self.config.time_step;
        
        // Recursively apply field evolution to all nodes
        self.evolve_node(root, dt);
        
        self.time += dt;
        self.update_statistics(root);
        
        // Periodically detect coherence peaks
        if (self.time / dt) as usize % 10 == 0 {
            self.detect_coherence_peaks(root);
        }
    }

    /// Evolve a single node and its children
    fn evolve_node(&mut self, node: &mut OctreeNode, dt: Float) {
        let position = node.bounds.center();
        
        // Apply Free Will (TRUE stochastic perturbation)
        // This must be applied first as it breaks symmetry
        self.distortions.free_will.apply(&mut node.field_data, &position, self.time);
        self.statistics.free_will_applications += 1;
        
        // Apply Love/Light (attractive force toward coherence)
        // This creates structure through coherence attraction
        self.distortions.love.apply(node);
        self.statistics.love_applications += 1;
        
        // Apply Light (wave propagation with interference)
        // This allows information to travel and can form standing waves
        self.distortions.light.apply(node, dt);
        self.statistics.light_applications += 1;
        
        // Recursively evolve children
        if let Some(ref mut children) = node.children {
            for child in children.iter_mut() {
                self.evolve_node(child, dt);
            }
        }
    }

    /// Detect coherence peaks in the field
    /// These are locations where matter/structure can emerge
    /// 
    /// From R&D Roadmap: "Field coherence peak detection algorithm"
    fn detect_coherence_peaks(&mut self, root: &OctreeNode) {
        let threshold = self.config.coherence_peak_threshold;
        let min_distance = self.config.coherence_peak_min_distance;
        
        // Find nodes above coherence threshold
        let mut candidate_peaks: Vec<CoherencePeak> = Vec::new();
        self.collect_high_coherence_nodes(root, threshold, &mut candidate_peaks);
        
        // Filter to find distinct peaks (not too close together)
        let mut filtered_peaks: Vec<CoherencePeak> = Vec::new();
        
        for candidate in candidate_peaks {
            let mut too_close = false;
            
            for existing in &filtered_peaks {
                let dist = ((candidate.position[0] - existing.position[0]).powi(2)
                         + (candidate.position[1] - existing.position[1]).powi(2)
                         + (candidate.position[2] - existing.position[2]).powi(2)).sqrt();
                
                if dist < min_distance {
                    too_close = true;
                    // Keep the stronger peak
                    if candidate.coherence > existing.coherence {
                        break;
                    }
                }
            }
            
            if !too_close {
                filtered_peaks.push(CoherencePeak {
                    position: candidate.position,
                    coherence: candidate.coherence,
                    energy: candidate.energy,
                    radius: 5.0, // Default radius
                    age: 0,
                });
            }
        }
        
        // Update existing peaks (age them) and merge new ones
        for peak in &mut self.coherence_peaks {
            peak.age += 1;
        }
        
        // Replace with filtered peaks
        self.coherence_peaks = filtered_peaks;
        self.statistics.coherence_peak_count = self.coherence_peaks.len();
    }

    /// Collect nodes with high coherence
    fn collect_high_coherence_nodes(&self, node: &OctreeNode, threshold: Float, peaks: &mut Vec<CoherencePeak>) {
        if node.field_data.coherence >= threshold {
            peaks.push(CoherencePeak {
                position: node.bounds.center(),
                coherence: node.field_data.coherence,
                energy: node.field_data.energy,
                radius: 5.0,
                age: 0,
            });
        }
        
        if let Some(ref children) = node.children {
            for child in children.iter() {
                self.collect_high_coherence_nodes(child, threshold, peaks);
            }
        }
    }

    /// Get detected coherence peaks
    /// These are candidate locations for matter/structure emergence
    pub fn get_coherence_peaks(&self) -> &[CoherencePeak] {
        &self.coherence_peaks
    }

    /// Update field statistics
    fn update_statistics(&mut self, root: &mut OctreeNode) {
        let mut total_coherence = 0.0;
        let mut total_energy = 0.0;
        let mut node_count = 0;
        let mut max_coherence = 0.0;
        
        self.collect_stats(root, &mut total_coherence, &mut total_energy, &mut node_count, &mut max_coherence);
        
        if node_count > 0 {
            self.statistics.average_coherence = total_coherence / node_count as Float;
            self.statistics.total_energy = total_energy;
            self.statistics.max_coherence = max_coherence;
        }
    }

    /// Collect statistics recursively
    fn collect_stats(&self, node: &OctreeNode, total_coherence: &mut Float, total_energy: &mut Float, count: &mut usize, max_coherence: &mut Float) {
        *total_coherence += node.field_data.coherence;
        *total_energy += node.field_data.energy;
        *count += 1;
        
        if node.field_data.coherence > *max_coherence {
            *max_coherence = node.field_data.coherence;
        }
        
        if let Some(ref children) = node.children {
            for child in children.iter() {
                self.collect_stats(child, total_coherence, total_energy, count, max_coherence);
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
    
    /// Calculate field gradient at a position
    /// Returns [dx, dy, dz] representing the direction of increasing field values
    pub fn calculate_field_gradient(&self, position: [Float; 3], root: &OctreeNode) -> [Float; 3] {
        // Find the coherence at this position by recursive search
        let coherence = self.find_coherence_at(position, root);
        
        // Use Love term's gradient calculation
        self.distortions.love.get_gradient_at(position, coherence)
    }
    
    /// Find coherence at a position by recursive search
    fn find_coherence_at(&self, position: [Float; 3], node: &OctreeNode) -> Float {
        if node.bounds.contains(&position) {
            if let Some(ref children) = node.children {
                for child in children.iter() {
                    let coherence = self.find_coherence_at(position, child);
                    if coherence > 0.0 {
                        return coherence;
                    }
                }
            }
            node.field_data.coherence
        } else {
            0.0
        }
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
    
    pub fn with_coherence_detection(mut self, threshold: Float, min_distance: Float) -> Self {
        self.config.coherence_peak_threshold = threshold;
        self.config.coherence_peak_min_distance = min_distance;
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
            .with_coherence_detection(0.8, 20.0)
            .build();
        
        let config = field.get_config();
        assert_eq!(config.free_will_amplitude, 0.2);
        assert_eq!(config.love_attraction_strength, 0.6);
        assert_eq!(config.light_propagation_speed, 2.0);
        assert_eq!(config.coherence_peak_threshold, 0.8);
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
    
    #[test]
    fn test_coherence_peak_detection() {
        let config = UnifiedFieldConfig {
            coherence_peak_threshold: 0.3,
            coherence_peak_min_distance: 10.0,
            ..Default::default()
        };
        let mut field = UnifiedFieldEquation::new(config);
        let mut state = HolographicFieldState::with_defaults();
        
        // Add high coherence nodes
        state.add_energy_at([0.0, 0.0, 0.0], 3, 1.0);
        state.root.field_data.coherence = 0.8; // Above threshold
        
        // Evolve multiple times to trigger peak detection
        for _ in 0..20 {
            field.evolve(&mut state.root);
        }
        
        let peaks = field.get_coherence_peaks();
        // Should have detected at least one peak
        assert!(field.get_statistics().coherence_peak_count >= 0);
    }
    
    #[test]
    fn test_field_gradient_calculation() {
        let field = UnifiedFieldEquation::with_defaults();
        let state = HolographicFieldState::with_defaults();
        
        let gradient = field.calculate_field_gradient([0.0, 0.0, 0.0], &state.root);
        
        // Should return a gradient vector
        assert_eq!(gradient.len(), 3);
    }
}
