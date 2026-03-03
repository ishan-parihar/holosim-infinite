//! Holographic Entanglement via Shared Spectrum
//!
//! From ROADMAP:
//! "Entanglement = shared spectrum configuration"
//! "'Spooky action at a distance' is actually holographic non-locality"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Space emerges from field coherence - distance is a holographic illusion"
//! "Entangled particles share the same holographic address in different positions"
//!
//! This module implements:
//! 1. Entanglement as shared spectrum configuration
//! 2. Instant correlation via shared holographic reference
//! 3. Non-locality as holographic principle
//! 4. Measurement effects propagate through shared spectrum

use crate::holographic::universal_template::SpectrumConfiguration;
use crate::types::Float;

// Import quantum state signature
use super::quantum_field::QuantumStateSignature;

// ============================================================================
// SHARED SPECTRUM
// ============================================================================

/// Shared spectrum configuration for entangled particles
///
/// From ROADMAP:
/// "Entanglement = shared spectrum configuration"
///
/// When two particles are entangled, they share the same spectrum
/// configuration, which means they have correlated space/time
/// and time/space access. This creates the "spooky action at a distance"
/// effect - measuring one particle affects the other through their
/// shared holographic connection.
#[derive(Debug, Clone)]
pub struct SharedSpectrum {
    /// The shared spectrum configuration
    pub spectrum: SpectrumConfiguration,

    /// Entanglement coherence (0.0 to 1.0)
    pub coherence: Float,

    /// Phase relationship between entangled states
    pub phase: Float,

    /// Entanglement depth (how many levels of entanglement)
    pub depth: u32,
}

impl SharedSpectrum {
    /// Create a new shared spectrum
    pub fn new(spectrum: SpectrumConfiguration, coherence: Float, phase: Float) -> Self {
        Self {
            spectrum,
            coherence: coherence.clamp(0.0, 1.0),
            phase,
            depth: 1,
        }
    }

    /// Create shared spectrum from field coherence
    ///
    /// Higher field coherence = more stable entanglement
    pub fn from_field_coherence(field_coherence: Float) -> Self {
        let spectrum = SpectrumConfiguration::new(
            crate::spectrum::larson_framework::SpectrumRatio::space_time(1.0, 1.0),
            field_coherence,
            1.0 - field_coherence,
            field_coherence,
        );

        Self {
            spectrum,
            coherence: field_coherence,
            phase: 0.0,
            depth: 1,
        }
    }

    /// Create balanced shared spectrum
    pub fn balanced() -> Self {
        Self::new(SpectrumConfiguration::balanced(), 0.5, 0.0)
    }

    /// Create high-coherence shared spectrum (for stable entanglement)
    pub fn high_coherence() -> Self {
        Self::new(SpectrumConfiguration::time_space_dominant(), 0.9, 0.0)
    }

    /// Calculate correlation strength
    ///
    /// Higher coherence and phase alignment = stronger correlation
    pub fn correlation_strength(&self) -> Float {
        self.coherence * (1.0 - self.phase.abs() / std::f64::consts::PI)
    }

    /// Apply measurement effect
    ///
    /// When one particle is measured, the spectrum updates to reflect
    /// the collapse, affecting the other particle
    pub fn apply_measurement(&mut self, measurement_result: &Measurement) {
        // Measurement affects phase
        self.phase = (self.phase + measurement_result.phase_shift) % (2.0 * std::f64::consts::PI);

        // Coherence may increase due to measurement (wavefunction collapse)
        self.coherence = (self.coherence + 0.1).min(1.0);

        // Update spectrum based on measurement
        self.spectrum.evolve(measurement_result.coherence_delta);
    }

    /// Evolve the shared spectrum over time
    pub fn evolve(&mut self, dt: Float) {
        // Phase evolves
        self.phase = (self.phase + dt * 0.1) % (2.0 * std::f64::consts::PI);

        // Coherence slowly decays (decoherence)
        self.coherence *= 1.0 - dt * 0.01;
    }
}

impl Default for SharedSpectrum {
    fn default() -> Self {
        Self::balanced()
    }
}

// ============================================================================
// ENTANGLEMENT LINK
// ============================================================================

/// Entanglement link between two quantum states
///
/// From ROADMAP:
/// "Spooky action at a distance" is actually holographic non-locality
///
/// Entangled particles are connected through their shared spectrum
/// configuration, not through any spatial mechanism. This explains
/// why entanglement works instantaneously at any distance - distance
/// itself is a holographic illusion.
#[derive(Debug, Clone)]
pub struct EntanglementLink {
    /// First entangled quantum state
    pub state1: QuantumStateSignature,

    /// Second entangled quantum state
    pub state2: QuantumStateSignature,

    /// Shared spectrum configuration (the holographic connection)
    pub shared_spectrum: SharedSpectrum,

    /// Coherence of the entanglement (0.0 to 1.0)
    pub coherence: Float,

    /// Entanglement type
    pub entanglement_type: EntanglementType,

    /// Creation timestamp
    pub created_at: Float,
}

impl EntanglementLink {
    /// Create a new entanglement link
    pub fn new(
        state1: QuantumStateSignature,
        state2: QuantumStateSignature,
        shared_spectrum: SharedSpectrum,
        coherence: Float,
    ) -> Self {
        Self {
            state1,
            state2,
            shared_spectrum,
            coherence: coherence.clamp(0.0, 1.0),
            entanglement_type: EntanglementType::determine(&state1, &state2),
            created_at: 0.0,
        }
    }

    /// Create entanglement from shared potential
    ///
    /// When a holographic potential splits into two entangled states,
    /// they share the same spectrum configuration
    pub fn from_shared_potential(
        potential_spectrum: &SpectrumConfiguration,
        split_factor: Float,
    ) -> Self {
        // Create two states with opposite spins (like particle-antiparticle pairs)
        let state1 = QuantumStateSignature::new(1, 0, 0, true, 1);
        let state2 = QuantumStateSignature::new(1, 0, 0, false, 2);

        // Shared spectrum is modified by split factor
        let shared = SharedSpectrum::new(
            potential_spectrum.clone(),
            split_factor,
            std::f64::consts::PI, // Anti-phase for entanglement
        );

        Self::new(state1, state2, shared, split_factor)
    }

    /// Apply instant correlation via shared spectrum
    ///
    /// When a measurement is made on one particle, it instantly
    /// affects the other through their shared spectrum configuration
    pub fn apply_correlation(&mut self, measurement: &Measurement) -> InstantEffect {
        // Apply measurement to shared spectrum
        self.shared_spectrum.apply_measurement(measurement);

        // Calculate instant effect on the other particle
        let effect = InstantEffect {
            phase_change: self.shared_spectrum.phase,
            coherence_change: measurement.coherence_delta,
            spin_flip: self.should_flip_spin(measurement),
            correlation_preserved: self.shared_spectrum.correlation_strength(),
        };

        effect
    }

    /// Determine if the other particle's spin should flip
    fn should_flip_spin(&self, measurement: &Measurement) -> bool {
        // For spin-entangled pairs, measuring one determines the other
        // This is the EPR correlation
        measurement.measured_spin && self.entanglement_type == EntanglementType::SpinCorrelation
    }

    /// Check if this entanglement involves a specific state
    pub fn involves(&self, signature: &QuantumStateSignature) -> bool {
        self.state1 == *signature || self.state2 == *signature
    }

    /// Get the other state in this entanglement
    pub fn other_state(&self, signature: &QuantumStateSignature) -> Option<&QuantumStateSignature> {
        if &self.state1 == signature {
            Some(&self.state2)
        } else if &self.state2 == signature {
            Some(&self.state1)
        } else {
            None
        }
    }

    /// Evolve the entanglement over time
    pub fn evolve(&mut self, dt: Float) {
        // Shared spectrum evolves
        self.shared_spectrum.evolve(dt);

        // Coherence decays (decoherence)
        self.coherence *= 1.0 - dt * 0.005;

        // Update entanglement coherence from shared spectrum
        self.coherence = (self.coherence + self.shared_spectrum.coherence) / 2.0;
    }

    /// Check if entanglement is stable
    pub fn is_stable(&self) -> bool {
        self.coherence > 0.1 && self.shared_spectrum.coherence > 0.1
    }

    /// Calculate Bell inequality correlation
    ///
    /// For Bell tests, entangled particles should show correlation
    /// that violates Bell's inequality
    pub fn bell_correlation(&self) -> Float {
        // Simplified Bell correlation calculation
        // Real implementation would use actual Bell state mathematics
        self.shared_spectrum.correlation_strength() * self.coherence
    }
}

// ============================================================================
// ENTANGLEMENT TYPE
// ============================================================================

/// Types of quantum entanglement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntanglementType {
    /// Spin correlation (spin-up ↔ spin-down)
    SpinCorrelation,

    /// Position-momentum correlation
    PositionMomentum,

    /// Energy-time correlation
    EnergyTime,

    /// Polarization correlation (photons)
    Polarization,

    /// Orbital angular momentum correlation
    OrbitalAngularMomentum,

    /// General quantum correlation
    General,
}

impl EntanglementType {
    /// Determine entanglement type from quantum states
    pub fn determine(state1: &QuantumStateSignature, state2: &QuantumStateSignature) -> Self {
        // Check for spin correlation (opposite spins)
        if state1.spin_up != state2.spin_up && state1.n == state2.n && state1.l == state2.l {
            return EntanglementType::SpinCorrelation;
        }

        // Check for position-momentum correlation
        if state1.n != state2.n && state1.l == state2.l {
            return EntanglementType::PositionMomentum;
        }

        // Default to general
        EntanglementType::General
    }
}

impl Default for EntanglementType {
    fn default() -> Self {
        EntanglementType::General
    }
}

// ============================================================================
// INSTANT EFFECT
// ============================================================================

/// Instant effect from measurement on entangled particle
///
/// This represents the "spooky action at a distance" - the instant
/// effect that measuring one entangled particle has on the other.
#[derive(Debug, Clone)]
pub struct InstantEffect {
    /// Phase change in the other particle
    pub phase_change: Float,

    /// Coherence change in the other particle
    pub coherence_change: Float,

    /// Whether the spin should flip
    pub spin_flip: bool,

    /// How much correlation was preserved
    pub correlation_preserved: Float,
}

impl InstantEffect {
    /// Check if this is a significant effect
    pub fn is_significant(&self) -> bool {
        self.phase_change.abs() > 0.01 || self.coherence_change.abs() > 0.01 || self.spin_flip
    }

    /// Apply to a quantum state
    pub fn apply_to_state(&self, state: &mut QuantumStateSignature) {
        if self.spin_flip {
            state.spin_up = !state.spin_up;
        }
    }
}

// ============================================================================
// MEASUREMENT
// ============================================================================

/// A measurement on an entangled quantum state
#[derive(Debug, Clone)]
pub struct Measurement {
    /// Which observable was measured
    pub observable: MeasuredObservable,

    /// Result of the measurement
    pub result: Float,

    /// Phase shift caused by measurement
    pub phase_shift: Float,

    /// Change in coherence from measurement
    pub coherence_delta: Float,

    /// Whether spin was measured
    pub measured_spin: bool,

    /// Measurement precision (0.0 to 1.0)
    pub precision: Float,
}

impl Measurement {
    /// Create a spin measurement
    pub fn spin_measurement(result: Float, precision: Float) -> Self {
        Self {
            observable: MeasuredObservable::Spin,
            result,
            phase_shift: if result > 0.0 {
                0.0
            } else {
                std::f64::consts::PI
            },
            coherence_delta: 0.1,
            measured_spin: true,
            precision,
        }
    }

    /// Create a position measurement
    pub fn position_measurement(result: Float, precision: Float) -> Self {
        Self {
            observable: MeasuredObservable::Position,
            result,
            phase_shift: 0.0,
            coherence_delta: 0.05,
            measured_spin: false,
            precision,
        }
    }

    /// Create an energy measurement
    pub fn energy_measurement(result: Float, precision: Float) -> Self {
        Self {
            observable: MeasuredObservable::Energy,
            result,
            phase_shift: std::f64::consts::FRAC_PI_2,
            coherence_delta: 0.08,
            measured_spin: false,
            precision,
        }
    }
}

impl Default for Measurement {
    fn default() -> Self {
        Self {
            observable: MeasuredObservable::General,
            result: 0.0,
            phase_shift: 0.0,
            coherence_delta: 0.0,
            measured_spin: false,
            precision: 1.0,
        }
    }
}

/// Observable that can be measured
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasuredObservable {
    /// Spin angular momentum
    Spin,

    /// Position
    Position,

    /// Momentum
    Momentum,

    /// Energy
    Energy,

    /// Polarization
    Polarization,

    /// General observable
    General,
}

// ============================================================================
// ENTANGLEMENT STATISTICS
// ============================================================================

/// Statistics about entanglement
#[derive(Debug, Clone, Default)]
pub struct EntanglementStatistics {
    /// Total entanglement links
    pub total_entanglements: usize,

    /// Active (stable) entanglements
    pub active_entanglements: usize,

    /// Average coherence
    pub avg_coherence: Float,

    /// Average Bell correlation
    pub avg_bell_correlation: Float,

    /// Entanglements by type
    pub by_type: std::collections::HashMap<EntanglementType, usize>,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_spectrum_creation() {
        let spectrum = SharedSpectrum::balanced();
        assert_eq!(spectrum.coherence, 0.5);
        assert_eq!(spectrum.depth, 1);
    }

    #[test]
    fn test_shared_spectrum_from_coherence() {
        let spectrum = SharedSpectrum::from_field_coherence(0.8);
        assert_eq!(spectrum.coherence, 0.8);
    }

    #[test]
    fn test_shared_spectrum_correlation_strength() {
        let spectrum = SharedSpectrum::new(SpectrumConfiguration::balanced(), 1.0, 0.0);
        assert!((spectrum.correlation_strength() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_entanglement_link_creation() {
        let state1 = QuantumStateSignature::new(1, 0, 0, true, 1);
        let state2 = QuantumStateSignature::new(1, 0, 0, false, 2);
        let shared = SharedSpectrum::balanced();

        let link = EntanglementLink::new(state1.clone(), state2.clone(), shared, 0.8);

        assert!(link.involves(&state1));
        assert!(link.involves(&state2));
        assert_eq!(link.coherence, 0.8);
    }

    #[test]
    fn test_entanglement_other_state() {
        let state1 = QuantumStateSignature::new(1, 0, 0, true, 1);
        let state2 = QuantumStateSignature::new(1, 0, 0, false, 2);
        let shared = SharedSpectrum::balanced();

        let link = EntanglementLink::new(state1.clone(), state2.clone(), shared, 0.8);

        assert_eq!(link.other_state(&state1), Some(&state2));
        assert_eq!(link.other_state(&state2), Some(&state1));
    }

    #[test]
    fn test_entanglement_evolution() {
        let state1 = QuantumStateSignature::new(1, 0, 0, true, 1);
        let state2 = QuantumStateSignature::new(1, 0, 0, false, 2);
        let shared = SharedSpectrum::balanced();

        let mut link = EntanglementLink::new(state1, state2, shared, 0.8);
        let initial_coherence = link.coherence;

        link.evolve(0.1);

        // Coherence should decrease due to decoherence
        assert!(link.coherence < initial_coherence);
    }

    #[test]
    fn test_entanglement_stability() {
        let state1 = QuantumStateSignature::new(1, 0, 0, true, 1);
        let state2 = QuantumStateSignature::new(1, 0, 0, false, 2);
        let shared = SharedSpectrum::high_coherence();

        let link = EntanglementLink::new(state1, state2, shared, 0.9);
        assert!(link.is_stable());
    }

    #[test]
    fn test_measurement_application() {
        let state1 = QuantumStateSignature::new(1, 0, 0, true, 1);
        let state2 = QuantumStateSignature::new(1, 0, 0, false, 2);
        let shared = SharedSpectrum::balanced();

        let mut link = EntanglementLink::new(state1, state2, shared, 0.8);
        let measurement = Measurement::spin_measurement(0.5, 1.0);

        let effect = link.apply_correlation(&measurement);

        assert!(effect.is_significant());
    }

    #[test]
    fn test_entanglement_type_determination() {
        let state1 = QuantumStateSignature::new(1, 0, 0, true, 1);
        let state2 = QuantumStateSignature::new(1, 0, 0, false, 2);

        let ent_type = EntanglementType::determine(&state1, &state2);
        assert_eq!(ent_type, EntanglementType::SpinCorrelation);
    }

    #[test]
    fn test_bell_correlation() {
        let state1 = QuantumStateSignature::new(1, 0, 0, true, 1);
        let state2 = QuantumStateSignature::new(1, 0, 0, false, 2);
        let shared = SharedSpectrum::high_coherence();

        let link = EntanglementLink::new(state1, state2, shared, 0.9);
        let correlation = link.bell_correlation();

        // High coherence should give high Bell correlation
        assert!(correlation > 0.5);
    }

    #[test]
    fn test_instant_effect_spin_flip() {
        let effect = InstantEffect {
            phase_change: 0.1,
            coherence_change: 0.05,
            spin_flip: true,
            correlation_preserved: 0.8,
        };

        let mut state = QuantumStateSignature::new(1, 0, 0, true, 1);
        assert!(state.spin_up);
        effect.apply_to_state(&mut state);

        assert!(!state.spin_up);
    }
}
