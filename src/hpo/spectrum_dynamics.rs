//! Spectrum Dynamics and the Veil (Phase 2)
//!
//! From REFACTOR_ROADMAP_COMPREHENSIVE_2026.md:
//! "The third phase implements the eight density bands as coupled oscillator fields 
//! and the Veil crossing dynamics at v=1."
//!
//! This module implements:
//! - 8 density bands as coupled oscillators
//! - Continuous spectrum position (0.0 - 1.0)
//! - Veil crossing at v=1 (Space/Time ↔ Time/Space)
//! - "Transcend and Include" dynamics
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Space/Time ↔ Time/Space Spectrum: Continuum with Veil at v=1"
//! "Density Octave: 8 densities (1st → 8th) representing consciousness stages"

use super::field_state::{Complex, DensityBand, FieldNodeData, Float, OctreeNode};

/// Configuration for spectrum dynamics
#[derive(Debug, Clone)]
pub struct SpectrumDynamicsConfig {
    /// Number of density bands (typically 8)
    pub density_band_count: usize,
    
    /// Base frequencies for each density band
    pub base_frequencies: [Float; 8],
    
    /// Coupling strength between adjacent densities
    pub coupling_strength: Float,
    
    /// Veil position (v = 1.0 is the crossing point)
    pub veil_position: Float,
    
    /// Veil thickness (region where crossing occurs)
    pub veil_thickness: Float,
    
    /// Rate of energy transfer between densities
    pub energy_transfer_rate: Float,
    
    /// Time step
    pub time_step: Float,
}

impl Default for SpectrumDynamicsConfig {
    fn default() -> Self {
        // Base frequencies follow the Law of One octave
        // Each density has a characteristic frequency
        SpectrumDynamicsConfig {
            density_band_count: 8,
            base_frequencies: [
                1.0,   // Violet (1st density) - slowest
                2.0,   // Indigo (2nd density)
                4.0,   // Blue (3rd density)
                8.0,   // Green (4th density)
                16.0,  // Yellow (5th density)
                32.0,  // Orange (6th density)
                64.0,  // Red (7th density)
                128.0, // Unknown (8th density) - fastest
            ],
            coupling_strength: 0.1,
            veil_position: 1.0,
            veil_thickness: 0.1,
            energy_transfer_rate: 0.05,
            time_step: 0.01,
        }
    }
}

/// Density band as a quantum oscillator
/// Each density band oscillates at its characteristic frequency
#[derive(Debug, Clone)]
pub struct DensityOscillator {
    /// Density band index (0-7)
    pub density_index: usize,
    
    /// Current amplitude
    pub amplitude: Float,
    
    /// Current phase
    pub phase: Float,
    
    /// Angular frequency
    pub frequency: Float,
    
    /// Damping coefficient
    pub damping: Float,
}

impl DensityOscillator {
    pub fn new(density_index: usize, frequency: Float) -> Self {
        DensityOscillator {
            density_index,
            amplitude: 0.0,
            phase: 0.0,
            frequency,
            damping: 0.01,
        }
    }

    /// Evolve the oscillator by one time step
    /// d²x/dt² = ω²x - γ dx/dt (damped harmonic oscillator)
    pub fn evolve(&mut self, dt: Float, driving_force: Float) {
        // Damped harmonic oscillator equation
        let acceleration = self.frequency * self.frequency * driving_force 
            - self.damping * self.phase;
        
        // Update phase (phase is like position in oscillator)
        self.phase += acceleration * dt;
        
        // Update amplitude (simplified)
        self.amplitude = self.amplitude * (1.0 - self.damping * dt) + driving_force * dt;
        
        // Keep amplitude bounded
        self.amplitude = self.amplitude.clamp(0.0, 1.0);
    }

    /// Get the current oscillation value
    pub fn get_value(&self) -> Float {
        self.amplitude * (self.phase * self.frequency).sin()
    }
}

/// Coupling between density oscillators
/// Implements "Transcend and Include" - higher densities include lower densities
#[derive(Debug, Clone)]
pub struct DensityCoupling {
    /// Coupling matrix (strength of coupling between densities i and j)
    coupling_matrix: [[Float; 8]; 8],
    
    /// Coupling strength
    pub strength: Float,
}

impl DensityCoupling {
    pub fn new(strength: Float) -> Self {
        // Initialize coupling matrix
        let mut coupling_matrix = [[0.0; 8]; 8];
        
        // Each density couples to adjacent densities
        // Higher densities include lower densities (transcend and include)
        for i in 0..8 {
            for j in 0..8 {
                if i == j {
                    // Self-coupling
                    coupling_matrix[i][j] = 1.0;
                } else if (i as i32 - j as i32).abs() == 1 {
                    // Adjacent densities have strong coupling
                    coupling_matrix[i][j] = strength;
                } else if j < i {
                    // Lower densities are "included" in higher
                    coupling_matrix[i][j] = strength * 0.5 / (i - j) as Float;
                } else {
                    // Higher densities can influence lower (top-down)
                    coupling_matrix[i][j] = strength * 0.2 / (j - i) as Float;
                }
            }
        }
        
        DensityCoupling {
            coupling_matrix,
            strength,
        }
    }

    /// Calculate coupled evolution
    /// Each oscillator is influenced by its neighbors
    pub fn calculate_coupling(&self, amplitudes: &[Float; 8]) -> [Float; 8] {
        let mut result = [0.0; 8];
        
        for i in 0..8 {
            let mut coupling_effect = 0.0;
            for j in 0..8 {
                coupling_effect += self.coupling_matrix[i][j] * amplitudes[j];
            }
            result[i] = coupling_effect;
        }
        
        result
    }
}

/// The Veil - boundary between Space/Time and Time/Space
/// From COSMOLOGICAL-ARCHITECTURE.md: "Veil at v=1 creates separation between Time/Space and Space/Time"
#[derive(Debug, Clone)]
pub struct VeilDynamics {
    /// Position of the veil (v = 1.0 is the crossing point)
    pub veil_position: Float,
    
    /// Thickness of the veil region
    pub veil_thickness: Float,
    
    /// Current veil transparency (0.0 = opaque, 1.0 = transparent)
    pub transparency: Float,
}

impl VeilDynamics {
    pub fn new(veil_position: Float, veil_thickness: Float) -> Self {
        VeilDynamics {
            veil_position,
            veil_thickness,
            transparency: 0.5, // Start semi-transparent
        }
    }

    /// Calculate the veil factor at a given spectrum position
    /// Returns 0.0 outside veil, transitions through veil region
    pub fn get_veil_factor(&self, spectrum_position: Float) -> Float {
        let distance = (spectrum_position - self.veil_position).abs();
        
        if distance > self.veil_thickness {
            // Outside veil region
            if spectrum_position < self.veil_position {
                // Below veil - Space/Time dominant
                0.0
            } else {
                // Above veil - Time/Space dominant  
                1.0
            }
        } else {
            // Within veil region - interpolate
            distance / self.veil_thickness
        }
    }

    /// Check if a position is at the veil crossing
    pub fn is_at_veil(&self, spectrum_position: Float) -> bool {
        let distance = (spectrum_position - self.veil_position).abs();
        distance < self.veil_thickness * 0.5
    }

    /// Update veil transparency based on field coherence
    pub fn update_transparency(&mut self, coherence: Float) {
        // Higher coherence = more transparent veil
        // This represents clearer perception between dimensions
        self.transparency = (self.transparency * 0.95 + coherence * 0.05).clamp(0.0, 1.0);
    }

    /// Transform coordinates across the veil
    /// Space/Time ↔ Time/Space transformation
    pub fn transform_across_veil(&self, position: &[Float; 3], spectrum_position: Float, _dt: Float) -> [Float; 3] {
        let veil_factor = self.get_veil_factor(spectrum_position);
        
        // Below veil (space/time dominant): time flows forward, space is fixed
        // Above veil (time/space dominant): time becomes flexible, space flows
        // At veil: transformation occurs
        
        let time_dilation = 1.0 + veil_factor; // Time slows down approaching veil
        let space_flexibility = veil_factor;    // Space becomes more flexible above veil
        
        [
            position[0] * (1.0 + space_flexibility * 0.1),
            position[1] * (1.0 + space_flexibility * 0.1),
            position[2] * time_dilation, // Time affects depth dimension
        ]
    }
}

/// Spectrum position - continuous variable determining density distribution
/// From COSMOLOGICAL-ARCHITECTURE.md: "Spectrum position becomes continuous [0,1]"
#[derive(Debug, Clone)]
pub struct SpectrumPosition {
    /// Continuous position in spectrum (0.0 - 1.0+)
    /// 0.0 = 1st density (Violet)
    /// 0.875 = 8th density (Red)
    /// > 1.0 = Beyond the physical octave
    pub position: Float,
    
    /// Rate of change
    pub velocity: Float,
    
    /// Rate of change of velocity (acceleration)
    pub acceleration: Float,
}

impl SpectrumPosition {
    pub fn new(initial_position: Float) -> Self {
        SpectrumPosition {
            position: initial_position.clamp(0.0, 1.0),
            velocity: 0.0,
            acceleration: 0.0,
        }
    }

    /// Evolve spectrum position based on experiences and learning
    pub fn evolve(&mut self, dt: Float, experience_accumulation: Float, coherence: Float) {
        // Higher experience and coherence = progression toward higher densities
        // But progression is gradual (continuous), not discrete jumps
        
        // Acceleration based on experience
        self.acceleration = experience_accumulation * 0.001 * coherence;
        
        // Update velocity
        self.velocity += self.acceleration * dt;
        
        // Apply damping to prevent runaway progression
        self.velocity *= 0.99;
        
        // Update position
        self.position += self.velocity * dt;
        
        // Clamp to valid range (can exceed 1.0 for beings beyond physical octave)
        if self.position < 0.0 {
            self.position = 0.0;
            self.velocity = 0.0;
        }
    }

    /// Get the density distribution based on spectrum position
    /// Returns amplitudes for each of the 8 density bands
    pub fn get_density_amplitudes(&self) -> [Float; 8] {
        let mut result = [0.0; 8];
        
        // Map continuous position to density bands
        // Each density has a Gaussian distribution centered at its position
        for i in 0..8 {
            let density_center = i as Float / 8.0;
            let distance = (self.position - density_center).abs();
            
            // Gaussian falloff
            let amplitude = (-distance * distance * 50.0).exp();
            result[i] = amplitude;
        }
        
        // Normalize
        let sum: Float = result.iter().sum();
        if sum > 0.0 {
            for r in result.iter_mut() {
                *r /= sum;
            }
        }
        
        result
    }
}

/// Complete spectrum dynamics system
pub struct SpectrumDynamics {
    /// Configuration
    config: SpectrumDynamicsConfig,
    
    /// Oscillators for each density band
    oscillators: [DensityOscillator; 8],
    
    /// Coupling between densities
    coupling: DensityCoupling,
    
    /// Veil dynamics
    veil: VeilDynamics,
    
    /// Current spectrum positions (one per entity/region)
    spectrum_positions: Vec<SpectrumPosition>,
    
    /// Statistics
    pub statistics: SpectrumStatistics,
}

/// Statistics for spectrum dynamics
#[derive(Debug, Clone, Default)]
pub struct SpectrumStatistics {
    /// Average spectrum position
    pub average_position: Float,
    
    /// Number of veil crossings
    pub veil_crossings: usize,
    
    /// Average coherence
    pub average_coherence: Float,
    
    /// Density distribution
    pub density_distribution: [Float; 8],
}

impl SpectrumDynamics {
    pub fn new(config: SpectrumDynamicsConfig) -> Self {
        // Initialize oscillators with base frequencies
        let oscillators = [
            DensityOscillator::new(0, config.base_frequencies[0]),
            DensityOscillator::new(1, config.base_frequencies[1]),
            DensityOscillator::new(2, config.base_frequencies[2]),
            DensityOscillator::new(3, config.base_frequencies[3]),
            DensityOscillator::new(4, config.base_frequencies[4]),
            DensityOscillator::new(5, config.base_frequencies[5]),
            DensityOscillator::new(6, config.base_frequencies[6]),
            DensityOscillator::new(7, config.base_frequencies[7]),
        ];
        
        SpectrumDynamics {
            config: config.clone(),
            oscillators,
            coupling: DensityCoupling::new(config.coupling_strength),
            veil: VeilDynamics::new(config.veil_position, config.veil_thickness),
            spectrum_positions: Vec::new(),
            statistics: SpectrumStatistics::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(SpectrumDynamicsConfig::default())
    }

    /// Initialize spectrum positions for entities
    pub fn initialize_entities(&mut self, entity_count: usize) {
        self.spectrum_positions.clear();
        
        for i in 0..entity_count {
            // Entities start at different densities based on their nature
            let initial = if i < entity_count / 4 {
                0.125 // 1st density
            } else if i < entity_count / 2 {
                0.375 // 3rd density
            } else if i < 3 * entity_count / 4 {
                0.625 // 5th density
            } else {
                0.875 // 7th density
            };
            
            self.spectrum_positions.push(SpectrumPosition::new(initial));
        }
    }

    /// Evolve the spectrum by one time step
    pub fn evolve(&mut self, dt: Float, coherence: Float, experience_accumulation: Float) {
        // Get current amplitudes
        let mut amplitudes: [Float; 8] = [0.0; 8];
        for (i, osc) in self.oscillators.iter().enumerate() {
            amplitudes[i] = osc.amplitude;
        }
        
        // Calculate coupling effects
        let coupling_effects = self.coupling.calculate_coupling(&amplitudes);
        
        // Evolve each oscillator
        for (i, osc) in self.oscillators.iter_mut().enumerate() {
            // Driving force from coupling + experience
            let driving = coupling_effects[i] + experience_accumulation * 0.1;
            osc.evolve(dt, driving);
        }
        
        // Evolve spectrum positions
        for pos in self.spectrum_positions.iter_mut() {
            let was_at_veil = self.veil.is_at_veil(pos.position);
            pos.evolve(dt, experience_accumulation, coherence);
            let is_at_veil = self.veil.is_at_veil(pos.position);
            
            // Track veil crossings
            if was_at_veil != is_at_veil {
                self.statistics.veil_crossings += 1;
            }
        }
        
        // Update veil transparency
        self.veil.update_transparency(coherence);
        
        // Update statistics
        self.update_statistics();
    }

    /// Update statistics
    fn update_statistics(&mut self) {
        // Average spectrum position
        let sum: Float = self.spectrum_positions.iter().map(|p| p.position).sum();
        self.statistics.average_position = if !self.spectrum_positions.is_empty() {
            sum / self.spectrum_positions.len() as Float
        } else {
            0.5
        };
        
        // Density distribution
        for i in 0..8 {
            self.statistics.density_distribution[i] = self.oscillators[i].amplitude;
        }
    }

    /// Get density amplitudes for a specific position
    pub fn get_density_amplitudes_at(&self, position: Float) -> [Float; 8] {
        // Use spectrum position to determine distribution
        let temp_pos = SpectrumPosition::new(position);
        temp_pos.get_density_amplitudes()
    }

    /// Check if entity is at veil crossing
    pub fn is_entity_at_veil(&self, entity_idx: usize) -> bool {
        if entity_idx < self.spectrum_positions.len() {
            self.veil.is_at_veil(self.spectrum_positions[entity_idx].position)
        } else {
            false
        }
    }

    /// Transform entity position across veil
    pub fn transform_entity_across_veil(&self, entity_idx: usize, position: &[Float; 3], dt: Float) -> [Float; 3] {
        if entity_idx < self.spectrum_positions.len() {
            self.veil.transform_across_veil(position, self.spectrum_positions[entity_idx].position, dt)
        } else {
            *position
        }
    }

    /// Get veil transparency
    pub fn get_veil_transparency(&self) -> Float {
        self.veil.transparency
    }

    /// Get oscillator for density band
    pub fn get_oscillator(&self, density: usize) -> Option<&DensityOscillator> {
        if density < 8 {
            Some(&self.oscillators[density])
        } else {
            None
        }
    }

    /// Get spectrum position for entity
    pub fn get_spectrum_position(&self, entity_idx: usize) -> Option<Float> {
        if entity_idx < self.spectrum_positions.len() {
            Some(self.spectrum_positions[entity_idx].position)
        } else {
            None
        }
    }

    /// Modify coupling strength at runtime
    pub fn set_coupling_strength(&mut self, strength: Float) {
        self.config.coupling_strength = strength;
        self.coupling = DensityCoupling::new(strength);
    }

    /// Get configuration
    pub fn get_config(&self) -> &SpectrumDynamicsConfig {
        &self.config
    }
}

/// Apply spectrum dynamics to field node
pub fn apply_spectrum_to_node(
    node: &mut OctreeNode,
    spectrum: &SpectrumDynamics,
    entity_idx: usize,
    dt: Float,
) {
    // Get density amplitudes from spectrum position
    if let Some(pos) = spectrum.get_spectrum_position(entity_idx) {
        let amplitudes = spectrum.get_density_amplitudes_at(pos);
        
        // Apply to node's density amplitudes
        for (i, amp) in node.field_data.density_amplitudes.iter_mut().enumerate() {
            // Combine with existing field
            let new_amp = amp.magnitude() * (1.0 - dt) + amplitudes[i] * dt;
            let phase = amp.phase();
            *amp = Complex::from_polar(new_amp, phase);
        }
        
        // Update spectrum position in node
        node.field_data.spectrum_position = pos;
        
        // Update veil transparency
        node.field_data.veil_transparency = spectrum.get_veil_transparency();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_dynamics_creation() {
        let spectrum = SpectrumDynamics::with_defaults();
        assert_eq!(spectrum.statistics.average_position, 0.0);
    }

    #[test]
    fn test_oscillator_evolution() {
        let mut osc = DensityOscillator::new(0, 1.0);
        osc.evolve(0.1, 0.5);
        assert!(osc.amplitude >= 0.0);
    }

    #[test]
    fn test_veil_factor() {
        let veil = VeilDynamics::new(1.0, 0.1);
        assert_eq!(veil.get_veil_factor(0.5), 0.0);
        assert_eq!(veil.get_veil_factor(1.5), 1.0);
    }

    #[test]
    fn test_spectrum_position() {
        let mut pos = SpectrumPosition::new(0.5);
        pos.evolve(0.1, 1.0, 0.5);
        assert!(pos.position >= 0.0);
    }

    #[test]
    fn test_density_amplitudes() {
        let pos = SpectrumPosition::new(0.5);
        let amps = pos.get_density_amplitudes();
        assert_eq!(amps.len(), 8);
        let sum: Float = amps.iter().sum();
        assert!((sum - 1.0).abs() < 0.001);
    }
}
