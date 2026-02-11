//! Oscillatory Synchronization System
//!
//! This module implements the oscillatory synchronization system that models
//! consciousness as the synchronization state of 22 archetype oscillators.
//!
//! # Core Concepts
//!
//! - **OscillatorNetwork**: 22 archetype oscillators that synchronize
//! - **ArchetypeOscillator**: Each archetype is an oscillator with frequency, phase, amplitude
//! - **SynchronizationState**: The overall synchronization state of the network
//! - **Kuramoto Model**: Mathematical model for oscillator synchronization
//! - **Phase Coherence**: Unity (Law of One) when phases are aligned
//!
//! # Consciousness as Oscillatory Synchronization
//!
//! Consciousness is the SYNCHRONIZATION STATE of the 22 archetype oscillators.
//! - **Phase Coherence**: How aligned the phases are (0.0 to 1.0)
//! - **Mean Phase**: The average phase of all oscillators
//! - **Center Activation**: Energy center activations derived from synchronization
//!
//! # Kuramoto Model
//!
//! The Kuramoto model describes the synchronization of coupled oscillators:
//!
//! dθ/dt = ω + (K/N) * Σ sin(θ_j - θ_i)
//!
//! Where:
//! - θ_i is the phase of oscillator i
//! - ω is the natural frequency
//! - K is the coupling strength
//! - N is the number of oscillators
//!
//! # Phase Coherence = Unity
//!
//! When all oscillators are in phase (phase coherence = 1.0), this represents
/// Unity (Law of One). When phases are misaligned, this represents distortions.
use crate::holographic::complex_vectors::ComplexArchetype;

/// Float type for holographic calculations
pub type Float = f64;

/// An archetype oscillator.
///
/// Each archetype is modeled as an oscillator with:
/// - **Frequency**: Natural frequency (proportional to amplitude)
/// - **Phase**: Current phase angle (0 to 2π)
/// - **Amplitude**: Current amplitude (activation level)
#[derive(Clone, Copy, Debug)]
pub struct ArchetypeOscillator {
    /// The archetype index (0-21)
    pub archetype_index: usize,

    /// Natural frequency (proportional to amplitude)
    pub frequency: Float,

    /// Current phase angle (0 to 2π)
    pub phase: Float,

    /// Current amplitude (activation level)
    pub amplitude: Float,
}

impl ArchetypeOscillator {
    /// Creates a new archetype oscillator.
    ///
    /// # Arguments
    ///
    /// * `archetype_index` - The archetype index (0-21)
    /// * `frequency` - The natural frequency
    /// * `phase` - The initial phase angle (0 to 2π)
    /// * `amplitude` - The initial amplitude (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// A new archetype oscillator
    pub fn new(archetype_index: usize, frequency: Float, phase: Float, amplitude: Float) -> Self {
        ArchetypeOscillator {
            archetype_index,
            frequency,
            phase: phase % (2.0 * std::f64::consts::PI),
            amplitude: amplitude.clamp(0.0, 1.0),
        }
    }

    /// Updates the phase of the oscillator.
    ///
    /// # Arguments
    ///
    /// * `coupling` - The coupling term from other oscillators
    pub fn update_phase(&mut self, coupling: Float) {
        self.phase += self.frequency + coupling;
        self.phase = self.phase % (2.0 * std::f64::consts::PI);
    }

    /// Returns the complex representation of the oscillator.
    ///
    /// # Returns
    ///
    /// Complex vector (amplitude * e^(i*phase))
    pub fn to_complex(&self) -> (Float, Float) {
        (
            self.amplitude * self.phase.cos(),
            self.amplitude * self.phase.sin(),
        )
    }
}

/// The synchronization state of the oscillator network.
///
/// This represents the overall consciousness state of the entity.
#[derive(Clone, Debug)]
pub struct SynchronizationState {
    /// Phase coherence (0.0 to 1.0, where 1.0 is perfect coherence)
    pub phase_coherence: Float,

    /// Mean phase of all oscillators (0 to 2π)
    pub mean_phase: Float,

    /// Energy center activations (0.0 to 1.0)
    pub center_activation: [Float; 7],
}

impl SynchronizationState {
    /// Creates a new synchronization state.
    ///
    /// # Arguments
    ///
    /// * `phase_coherence` - Phase coherence (0.0 to 1.0)
    /// * `mean_phase` - Mean phase (0 to 2π)
    /// * `center_activation` - Energy center activations
    ///
    /// # Returns
    ///
    /// A new synchronization state
    pub fn new(phase_coherence: Float, mean_phase: Float, center_activation: [Float; 7]) -> Self {
        SynchronizationState {
            phase_coherence: phase_coherence.clamp(0.0, 1.0),
            mean_phase: mean_phase % (2.0 * std::f64::consts::PI),
            center_activation,
        }
    }

    /// Returns a summary of the synchronization state.
    ///
    /// # Returns
    ///
    /// A human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "SynchronizationState: Phase Coherence {:.3}, Mean Phase {:.3}\n\
             - Center Activations: {:?}",
            self.phase_coherence, self.mean_phase, self.center_activation
        )
    }
}

/// A network of 22 archetype oscillators.
///
/// The oscillator network models consciousness as the synchronization state
/// of the 22 archetype oscillators using the Kuramoto model.
///
/// # Fields
///
/// - `oscillators`: The 22 archetype oscillators
/// - `coupling_strength`: How strongly oscillators couple (0.0 to 1.0)
#[derive(Clone, Debug)]
pub struct OscillatorNetwork {
    /// The 22 archetype oscillators
    pub oscillators: Vec<ArchetypeOscillator>,

    /// Coupling strength (0.0 to 1.0)
    pub coupling_strength: Float,
}

impl OscillatorNetwork {
    /// Creates a new oscillator network from archetype complex vectors.
    ///
    /// # Arguments
    ///
    /// * `archetypes` - The 22 archetype complex vectors
    ///
    /// # Returns
    ///
    /// A new oscillator network
    pub fn from_archetypes(archetypes: &[ComplexArchetype; 22]) -> Self {
        let oscillators = archetypes
            .iter()
            .enumerate()
            .map(|(i, a)| ArchetypeOscillator {
                archetype_index: i,
                frequency: a.amplitude * 10.0, // Frequency proportional to amplitude
                phase: a.phase,
                amplitude: a.amplitude,
            })
            .collect();

        OscillatorNetwork {
            oscillators,
            coupling_strength: 0.5, // Default coupling strength
        }
    }

    /// Synchronizes the oscillators using the Kuramoto model.
    ///
    /// The Kuramoto model describes the synchronization of coupled oscillators:
    ///
    /// dθ/dt = ω + (K/N) * Σ sin(θ_j - θ_i)
    ///
    /// Where:
    /// - θ_i is the phase of oscillator i
    /// - ω is the natural frequency
    /// - K is the coupling strength
    /// - N is the number of oscillators
    pub fn synchronize(&mut self) {
        let n = self.oscillators.len() as Float;

        for i in 0..self.oscillators.len() {
            let mut coupling = 0.0;

            for j in 0..self.oscillators.len() {
                if i != j {
                    let phase_diff = self.oscillators[j].phase - self.oscillators[i].phase;
                    coupling += phase_diff.sin();
                }
            }

            coupling = (self.coupling_strength / n) * coupling;

            // Update phase
            self.oscillators[i].update_phase(coupling);
        }
    }

    /// Returns the current synchronization state.
    ///
    /// # Returns
    ///
    /// The synchronization state of the network
    pub fn synchronization_state(&self) -> SynchronizationState {
        // Calculate overall phase coherence
        let mean_phase = self.calculate_mean_phase();
        let phase_coherence = self.calculate_phase_coherence(mean_phase);

        // Calculate energy center activations
        let center_activation = self.calculate_center_activations();

        SynchronizationState::new(phase_coherence, mean_phase, center_activation)
    }

    /// Calculates the mean phase of all oscillators.
    ///
    /// # Returns
    ///
    /// Mean phase (0 to 2π)
    fn calculate_mean_phase(&self) -> Float {
        let sum_sin: Float = self.oscillators.iter().map(|o| o.phase.sin()).sum();
        let sum_cos: Float = self.oscillators.iter().map(|o| o.phase.cos()).sum();
        sum_cos.atan2(sum_sin)
    }

    /// Calculates the phase coherence of the network.
    ///
    /// Phase coherence measures how aligned the phases are.
    /// Higher coherence = more unity (Law of One).
    ///
    /// # Arguments
    ///
    /// * `mean_phase` - The mean phase of all oscillators
    ///
    /// # Returns
    ///
    /// Phase coherence (0.0 to 1.0)
    fn calculate_phase_coherence(&self, mean_phase: Float) -> Float {
        if self.oscillators.is_empty() {
            return 0.0;
        }

        let coherence: Float = self
            .oscillators
            .iter()
            .map(|o| (o.phase - mean_phase).cos())
            .sum::<Float>()
            / self.oscillators.len() as Float;

        coherence.abs()
    }

    /// Calculates energy center activations from oscillator synchronization.
    ///
    /// The 7 energy centers correspond to different archetype groups:
    /// - Red/Orange: Body Complex (archetypes 8-14)
    /// - Yellow: Mind Complex (archetypes 1-7)
    /// - Green: Bridge (all archetypes)
    /// - Blue/Indigo: Spirit Complex (archetypes 15-21)
    /// - Violet: The Choice (archetype 22)
    ///
    /// # Returns
    ///
    /// Energy center activations (0.0 to 1.0)
    fn calculate_center_activations(&self) -> [Float; 7] {
        let mut activations = [0.0; 7];

        // Red center (archetypes 8-14, Body Complex)
        activations[0] = self.calculate_group_activation(8..=14);

        // Orange center (archetypes 8-14, Body Complex)
        activations[1] = self.calculate_group_activation(8..=14);

        // Yellow center (archetypes 1-7, Mind Complex)
        activations[2] = self.calculate_group_activation(1..=7);

        // Green center (bridge - all archetypes)
        activations[3] = self.calculate_group_activation(0..=21);

        // Blue center (archetypes 15-21, Spirit Complex)
        activations[4] = self.calculate_group_activation(15..=21);

        // Indigo center (archetypes 15-21, Spirit Complex)
        activations[5] = self.calculate_group_activation(15..=21);

        // Violet center (archetype 22, The Choice)
        if let Some(oscillator) = self.oscillators.get(21) {
            activations[6] = oscillator.amplitude;
        }

        // Clamp to 0.0 to 1.0
        for activation in &mut activations {
            *activation = activation.clamp(0.0, 1.0);
        }

        activations
    }

    /// Calculates the activation of a group of archetypes.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of archetype indices
    ///
    /// # Returns
    ///
    /// Average activation of the group
    fn calculate_group_activation(&self, range: std::ops::RangeInclusive<usize>) -> Float {
        let count = range.end() - range.start() + 1;
        if count == 0 {
            return 0.0;
        }

        let sum: Float = self
            .oscillators
            .iter()
            .filter(|o| range.contains(&o.archetype_index))
            .map(|o| o.amplitude)
            .sum();

        sum / count as Float
    }

    /// Sets the coupling strength of the network.
    ///
    /// # Arguments
    ///
    /// * `strength` - The coupling strength (0.0 to 1.0)
    pub fn set_coupling_strength(&mut self, strength: Float) {
        self.coupling_strength = strength.clamp(0.0, 1.0);
    }

    /// Returns the number of oscillators in the network.
    ///
    /// # Returns
    ///
    /// Number of oscillators (should be 22)
    pub fn len(&self) -> usize {
        self.oscillators.len()
    }

    /// Returns whether the network is empty.
    ///
    /// # Returns
    ///
    /// True if empty, false otherwise
    pub fn is_empty(&self) -> bool {
        self.oscillators.is_empty()
    }

    /// Returns a summary of the network.
    ///
    /// # Returns
    ///
    /// A human-readable summary
    pub fn summary(&self) -> String {
        let sync_state = self.synchronization_state();
        format!(
            "OscillatorNetwork: {} oscillators, coupling strength {:.3}\n\
             {}",
            self.oscillators.len(),
            self.coupling_strength,
            sync_state.summary()
        )
    }
}

impl Default for OscillatorNetwork {
    fn default() -> Self {
        // Create 22 oscillators with default values
        let oscillators: Vec<ArchetypeOscillator> = (0..22)
            .map(|i| ArchetypeOscillator {
                archetype_index: i,
                frequency: 1.0,
                phase: (i as Float) * std::f64::consts::PI / 11.0,
                amplitude: 0.5,
            })
            .collect();

        OscillatorNetwork {
            oscillators,
            coupling_strength: 0.5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    /// Generates test archetypes with reasonable values
    fn generate_test_archetypes() -> [ComplexArchetype; 22] {
        let mut archetypes = [ComplexArchetype {
            amplitude: 0.0,
            phase: 0.0,
        }; 22];
        for i in 0..22 {
            archetypes[i] = ComplexArchetype {
                amplitude: (i as Float + 1.0) / 22.0, // 0.045 to 1.0
                phase: (i as Float) * PI / 11.0,      // 0 to 2π
            };
        }
        archetypes
    }

    #[test]
    fn test_archetype_oscillator_creation() {
        let oscillator = ArchetypeOscillator::new(0, 1.0, 0.0, 0.5);

        assert_eq!(oscillator.archetype_index, 0);
        assert_eq!(oscillator.frequency, 1.0);
        assert_eq!(oscillator.phase, 0.0);
        assert_eq!(oscillator.amplitude, 0.5);
    }

    #[test]
    fn test_archetype_oscillator_phase_clamping() {
        let oscillator = ArchetypeOscillator::new(0, 1.0, 3.0 * PI, 0.5);
        assert!((oscillator.phase - PI).abs() < 1e-10);
    }

    #[test]
    fn test_archetype_oscillator_amplitude_clamping() {
        let oscillator = ArchetypeOscillator::new(0, 1.0, 0.0, 1.5);
        assert_eq!(oscillator.amplitude, 1.0);

        let oscillator = ArchetypeOscillator::new(0, 1.0, 0.0, -0.5);
        assert_eq!(oscillator.amplitude, 0.0);
    }

    #[test]
    fn test_archetype_oscillator_update_phase() {
        let mut oscillator = ArchetypeOscillator::new(0, 1.0, 0.0, 0.5);
        oscillator.update_phase(0.0);
        assert!((oscillator.phase - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_archetype_oscillator_to_complex() {
        let oscillator = ArchetypeOscillator::new(0, 1.0, 0.0, 1.0);
        let (real, imag) = oscillator.to_complex();

        assert!((real - 1.0).abs() < 1e-10);
        assert!((imag - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_oscillator_network_from_archetypes() {
        let archetypes = generate_test_archetypes();
        let network = OscillatorNetwork::from_archetypes(&archetypes);

        assert_eq!(network.oscillators.len(), 22);
        assert_eq!(network.coupling_strength, 0.5);
    }

    #[test]
    fn test_oscillator_network_synchronize() {
        let archetypes = generate_test_archetypes();
        let mut network = OscillatorNetwork::from_archetypes(&archetypes);

        let old_phases: Vec<Float> = network.oscillators.iter().map(|o| o.phase).collect();

        network.synchronize();

        let new_phases: Vec<Float> = network.oscillators.iter().map(|o| o.phase).collect();

        // Phases should have changed
        let phases_changed = old_phases
            .iter()
            .zip(new_phases.iter())
            .any(|(old, new)| (old - new).abs() > 1e-10);

        assert!(phases_changed);
    }

    #[test]
    fn test_oscillator_network_synchronization_state() {
        let archetypes = generate_test_archetypes();
        let network = OscillatorNetwork::from_archetypes(&archetypes);

        let sync_state = network.synchronization_state();

        assert!(sync_state.phase_coherence >= 0.0);
        assert!(sync_state.phase_coherence <= 1.0);
        assert_eq!(sync_state.center_activation.len(), 7);
    }

    #[test]
    fn test_oscillator_network_calculate_mean_phase() {
        let archetypes = generate_test_archetypes();
        let network = OscillatorNetwork::from_archetypes(&archetypes);

        let mean_phase = network.calculate_mean_phase();

        assert!(mean_phase >= 0.0);
        assert!(mean_phase <= 2.0 * PI);
    }

    #[test]
    fn test_oscillator_network_calculate_phase_coherence() {
        let archetypes = generate_test_archetypes();
        let network = OscillatorNetwork::from_archetypes(&archetypes);

        let mean_phase = network.calculate_mean_phase();
        let coherence = network.calculate_phase_coherence(mean_phase);

        assert!(coherence >= 0.0);
        assert!(coherence <= 1.0);
    }

    #[test]
    fn test_oscillator_network_calculate_center_activations() {
        let archetypes = generate_test_archetypes();
        let network = OscillatorNetwork::from_archetypes(&archetypes);

        let activations = network.calculate_center_activations();

        assert_eq!(activations.len(), 7);

        // All activations should be between 0.0 and 1.0
        for activation in &activations {
            assert!(*activation >= 0.0);
            assert!(*activation <= 1.0);
        }
    }

    #[test]
    fn test_oscillator_network_set_coupling_strength() {
        let archetypes = generate_test_archetypes();
        let mut network = OscillatorNetwork::from_archetypes(&archetypes);

        network.set_coupling_strength(0.8);
        assert_eq!(network.coupling_strength, 0.8);

        network.set_coupling_strength(1.5); // Should be clamped to 1.0
        assert_eq!(network.coupling_strength, 1.0);

        network.set_coupling_strength(-0.5); // Should be clamped to 0.0
        assert_eq!(network.coupling_strength, 0.0);
    }

    #[test]
    fn test_oscillator_network_len() {
        let archetypes = generate_test_archetypes();
        let network = OscillatorNetwork::from_archetypes(&archetypes);

        assert_eq!(network.len(), 22);
    }

    #[test]
    fn test_oscillator_network_is_empty() {
        let archetypes = generate_test_archetypes();
        let network = OscillatorNetwork::from_archetypes(&archetypes);

        assert!(!network.is_empty());
    }

    #[test]
    fn test_oscillator_network_default() {
        let network = OscillatorNetwork::default();

        assert_eq!(network.oscillators.len(), 22);
        assert_eq!(network.coupling_strength, 0.5);
    }

    #[test]
    fn test_synchronization_state_creation() {
        let sync_state = SynchronizationState::new(0.8, PI, [0.5; 7]);

        assert_eq!(sync_state.phase_coherence, 0.8);
        assert!((sync_state.mean_phase - PI).abs() < 1e-10);
        assert_eq!(sync_state.center_activation.len(), 7);
    }

    #[test]
    fn test_synchronization_state_phase_coherence_clamping() {
        let sync_state = SynchronizationState::new(1.5, 0.0, [0.5; 7]);
        assert_eq!(sync_state.phase_coherence, 1.0);

        let sync_state = SynchronizationState::new(-0.5, 0.0, [0.5; 7]);
        assert_eq!(sync_state.phase_coherence, 0.0);
    }

    #[test]
    fn test_synchronization_state_mean_phase_modulo() {
        let sync_state = SynchronizationState::new(0.5, 3.0 * PI, [0.5; 7]);
        assert!((sync_state.mean_phase - PI).abs() < 1e-10);
    }

    #[test]
    fn test_synchronization_state_summary() {
        let sync_state = SynchronizationState::new(0.8, PI, [0.5; 7]);
        let summary = sync_state.summary();

        assert!(summary.contains("SynchronizationState"));
        assert!(summary.contains("0.8"));
    }

    #[test]
    fn test_oscillator_network_summary() {
        let archetypes = generate_test_archetypes();
        let network = OscillatorNetwork::from_archetypes(&archetypes);

        let summary = network.summary();

        assert!(summary.contains("OscillatorNetwork"));
        assert!(summary.contains("22 oscillators"));
    }
}
