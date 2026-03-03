//! Quantum Field - V5 Phase 4 Physics Implementation
//!
//! This module implements the quantum field theory for the holonic realms simulation.
//! It models quantum states, entanglement, decoherence, and the emergence of matter
//! from quantum field fluctuations.
//!
//! From V5 Phase 4 specifications:
//! - QuantumField manages amplitudes, entanglements, and decoherence
//! - Quantum states are characterized by quantum numbers (n, l, m, s)
//! - Attractor fields represent stable configurations (elemental patterns)
//! - Matter emerges from stable quantum attractor configurations
//!
//! Knowledge Base References:
//! - V5_PHASE4_QUANTUM_FIELD_SPEC.md
//! - COSMOLOGICAL-ARCHITECTURE.md

use crate::foundation::spectrum_position::SpectrumPosition;
use crate::holographic::field_address::ScaleLevel;
use crate::types::Float;
use num_complex::Complex;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// QUANTUM STATE SIGNATURE
// ============================================================================

/// Unique signature identifying a quantum state
///
/// Uses quantum numbers (n, l, m, s) plus position hash to uniquely
/// identify a quantum state in the field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumStateSignature {
    /// Principal quantum number (energy level)
    pub n: u32,

    /// Orbital angular momentum quantum number (shape)
    pub l: u32,

    /// Magnetic quantum number (orientation)
    pub m: i32,

    /// Spin quantum number
    pub s: Spin,

    /// Position hash for spatial localization
    pub position_hash: u64,
}

impl QuantumStateSignature {
    /// Create a new quantum state signature
    pub fn new(n: u32, l: u32, m: i32, s: Spin, position_hash: u64) -> Self {
        Self {
            n,
            l,
            m,
            s,
            position_hash,
        }
    }

    /// Compute a hash from the signature
    pub fn hash(&self) -> u64 {
        let mut hash = self.n as u64;
        hash = hash.wrapping_mul(31).wrapping_add(self.l as u64);
        hash = hash.wrapping_mul(31).wrapping_add(self.m as u64);
        hash = hash.wrapping_mul(31).wrapping_add(match self.s {
            Spin::Up => 0,
            Spin::Down => 1,
        });
        hash = hash.wrapping_mul(31).wrapping_add(self.position_hash);
        hash
    }
}

impl std::fmt::Display for QuantumStateSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "QStateSig(n={}, l={}, m={}, s={}, pos_hash={:x})",
            self.n, self.l, self.m, self.s, self.position_hash
        )
    }
}

// ============================================================================
// SPIN ENUM
// ============================================================================

/// Spin quantum number (±1/2)
///
/// Represents the intrinsic angular momentum of a particle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spin {
    /// Spin up (+1/2)
    Up,

    /// Spin down (-1/2)
    Down,
}

impl Spin {
    /// Get the spin value as +0.5 or -0.5
    pub fn value(&self) -> Float {
        match self {
            Spin::Up => 0.5,
            Spin::Down => -0.5,
        }
    }

    /// Flip the spin
    pub fn flip(&self) -> Self {
        match self {
            Spin::Up => Spin::Down,
            Spin::Down => Spin::Up,
        }
    }

    /// Get spin from a float value
    pub fn from_value(val: Float) -> Self {
        if val >= 0.0 {
            Spin::Up
        } else {
            Spin::Down
        }
    }
}

impl std::fmt::Display for Spin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spin::Up => write!(f, "↑"),
            Spin::Down => write!(f, "↓"),
        }
    }
}

// ============================================================================
// ENTANGLEMENT LINK
// ============================================================================

/// Represents an entanglement between two quantum states
///
/// Entanglement is a non-local correlation between quantum states
/// that persists even at large distances.
#[derive(Debug, Clone)]
pub struct EntanglementLink {
    /// ID of the first entangled state
    pub state1_id: u64,

    /// ID of the second entangled state
    pub state2_id: u64,

    /// Entanglement strength (0.0 to 1.0)
    pub strength: Float,

    /// Coherence of the entanglement (0.0 to 1.0)
    pub coherence: Float,

    /// Phase difference between states
    pub phase_difference: Float,
}

impl EntanglementLink {
    /// Create a new entanglement link
    pub fn new(state1_id: u64, state2_id: u64, strength: Float) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            state1_id,
            state2_id,
            strength: strength.clamp(0.0, 1.0),
            coherence: rng.gen::<Float>(),
            phase_difference: rng.gen::<Float>() * 2.0 * std::f64::consts::PI,
        }
    }

    /// Check if this entanglement involves a specific state
    pub fn involves(&self, state_id: u64) -> bool {
        self.state1_id == state_id || self.state2_id == state_id
    }

    /// Get the other state ID in this entanglement
    pub fn other_id(&self, state_id: u64) -> Option<u64> {
        if state_id == self.state1_id {
            Some(self.state2_id)
        } else if state_id == self.state2_id {
            Some(self.state1_id)
        } else {
            None
        }
    }

    /// Reduce entanglement strength
    pub fn reduce_strength(&mut self, amount: Float) {
        self.strength = (self.strength - amount).max(0.0);
    }
}

// ============================================================================
// ELEMENT ENUM
// ============================================================================

/// Chemical elements that can emerge from quantum field attractors
///
/// Represents the stable attractor patterns that correspond to
/// different chemical elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    /// Hydrogen (1 proton, 1 electron)
    Hydrogen,

    /// Helium (2 protons, 2 electrons)
    Helium,

    /// Lithium (3 protons, 3 electrons)
    Lithium,

    /// Custom element with specified proton count
    Custom { protons: u32 },
}

impl Element {
    /// Get the atomic number (number of protons)
    pub fn atomic_number(&self) -> u32 {
        match self {
            Element::Hydrogen => 1,
            Element::Helium => 2,
            Element::Lithium => 3,
            Element::Custom { protons } => *protons,
        }
    }

    /// Get element name
    pub fn name(&self) -> String {
        match self {
            Element::Hydrogen => "Hydrogen".to_string(),
            Element::Helium => "Helium".to_string(),
            Element::Lithium => "Lithium".to_string(),
            Element::Custom { protons } => format!("Custom({})", protons),
        }
    }

    /// Map a quantum state signature to an element
    ///
    /// This is a simplified mapping based on the principal quantum number
    /// and other quantum state properties.
    pub fn from_quantum_state(signature: &QuantumStateSignature) -> Self {
        match signature.n {
            1 => Element::Hydrogen,
            2 => {
                // Helium has filled first shell (n=1) and begins second shell
                if signature.l == 0 {
                    Element::Helium
                } else {
                    Element::Custom {
                        protons: 3 + signature.l,
                    }
                }
            }
            _ => Element::Custom {
                protons: signature.n * signature.n,
            },
        }
    }

    /// Get electron configuration as a string
    pub fn electron_configuration(&self) -> String {
        match self {
            Element::Hydrogen => "1s1".to_string(),
            Element::Helium => "1s2".to_string(),
            Element::Lithium => "1s2 2s1".to_string(),
            Element::Custom { protons } => format!("[Custom {} protons]", protons),
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// ============================================================================
// ATTRACTOR FIELD
// ============================================================================

/// An attractor field representing a stable quantum configuration
///
/// Attractor fields are stable patterns in the quantum field that
/// correspond to matter particles (protons, neutrons, electrons) and
/// eventually to chemical elements.
#[derive(Debug, Clone)]
pub struct AttractorField {
    /// Quantum state signature of this attractor
    pub state: QuantumStateSignature,

    /// Energy level of this attractor
    pub energy_level: Float,

    /// Coherence peak (0.0 to 1.0)
    pub coherence_peak: Float,

    /// Element that emerges from this attractor
    pub element: Element,

    /// Stability of this attractor (0.0 to 1.0)
    pub stability: Float,
}

impl AttractorField {
    /// Create a new attractor field
    pub fn new(
        state: QuantumStateSignature,
        energy_level: Float,
        coherence_peak: Float,
        element: Element,
        stability: Float,
    ) -> Self {
        Self {
            state,
            energy_level,
            coherence_peak: coherence_peak.clamp(0.0, 1.0),
            element,
            stability: stability.clamp(0.0, 1.0),
        }
    }

    /// Check if this attractor is stable enough for matter emergence
    pub fn is_stable(&self) -> bool {
        self.stability > 0.7 && self.coherence_peak > 0.5
    }

    /// Update the stability based on field coherence
    pub fn update_stability(&mut self, field_coherence: Float) {
        // Stability increases when field coherence is high
        let delta = (field_coherence - self.stability) * 0.1;
        self.stability = (self.stability + delta).clamp(0.0, 1.0);
    }
}

// ============================================================================
// PLACEHOLDER HOLOGRAPHIC BLUEPRINT
// ============================================================================

/// Placeholder HolographicBlueprint for quantum field initialization
///
/// This is a simplified version used when the full holographic blueprint
/// is not available. In production, this would be replaced with the full
/// HolographicBlueprint from entity_layer7.
#[derive(Debug, Clone)]
pub struct HolographicBlueprint {
    /// Scale level of this blueprint
    pub scale: ScaleLevel,

    /// Base coherence of the blueprint
    pub coherence: Float,

    /// Seed for random number generation
    pub seed: u64,
}

impl HolographicBlueprint {
    /// Create a new holographic blueprint placeholder
    pub fn new(scale: ScaleLevel, coherence: Float) -> Self {
        Self {
            scale,
            coherence: coherence.clamp(0.0, 1.0),
            seed: rand::random(),
        }
    }
}

// ============================================================================
// QUANTUM FIELD
// ============================================================================

/// The quantum field containing all quantum states and their interactions
///
/// This is the fundamental field from which matter emerges through
/// quantum fluctuations and stable attractor configurations.
///
/// From V5 Phase 4 specifications:
/// - Manages quantum amplitudes across space
/// - Tracks entanglements between states
/// - Applies decoherence over time
/// - Extracts stable attractor fields
/// - Supports measurement and collapse
#[derive(Debug, Clone)]
pub struct QuantumField {
    /// Quantum amplitudes at various positions
    /// Maps position hash to complex amplitude
    pub amplitudes: HashMap<u64, Complex<Float>>,

    /// Entanglement links between quantum states
    pub entanglements: Vec<EntanglementLink>,

    /// Rate at which decoherence occurs (0.0 to 1.0)
    pub decoherence_rate: Float,

    /// Overall coherence of the field (0.0 to 1.0)
    pub field_coherence: Float,

    /// Holographic blueprint for initialization
    pub blueprint: Arc<HolographicBlueprint>,

    /// Spectrum position of this field
    pub spectrum_position: SpectrumPosition,

    /// Next state ID for new quantum states
    next_state_id: u64,
}

impl QuantumField {
    /// Create a new quantum field
    pub fn new(blueprint: Arc<HolographicBlueprint>) -> Self {
        let _rng = rand::thread_rng();
        let spectrum_position = SpectrumPosition::physical(crate::types::Density::Third);

        Self {
            amplitudes: HashMap::new(),
            entanglements: Vec::new(),
            decoherence_rate: 0.01,
            field_coherence: 1.0,
            blueprint,
            spectrum_position,
            next_state_id: 0,
        }
    }

    /// Initialize the quantum field from the holographic blueprint
    ///
    /// Sets up initial amplitudes based on the blueprint's configuration.
    pub fn initialize_from_blueprint(&mut self) -> Result<(), QuantumFieldError> {
        let mut rng = rand::thread_rng();

        // Create initial amplitudes at quantum scale
        let num_amplitudes = 100;
        for _ in 0..num_amplitudes {
            let position_hash = rng.gen::<u64>();
            // Amplitude with random phase and magnitude
            let magnitude = rng.gen::<Float>() * self.blueprint.coherence;
            let phase = rng.gen::<Float>() * 2.0 * std::f64::consts::PI;
            let amplitude = Complex::from_polar(magnitude, phase);
            self.amplitudes.insert(position_hash, amplitude);
        }

        self.field_coherence = self.blueprint.coherence;

        Ok(())
    }

    /// Evolve the quantum field by one time step
    ///
    /// Updates amplitudes, applies decoherence, and evolves entanglements.
    pub fn evolve(&mut self, dt: Float) -> Result<(), QuantumFieldError> {
        let mut rng = rand::thread_rng();

        // Evolve each amplitude
        for amplitude in self.amplitudes.values_mut() {
            // Apply phase evolution (Schrödinger-like)
            let phase_evolution = Complex::from_polar(1.0, dt);
            *amplitude *= phase_evolution;

            // Add small quantum fluctuations
            let fluctuation = Complex::new(
                rng.gen::<Float>() * 0.01 - 0.005,
                rng.gen::<Float>() * 0.01 - 0.005,
            );
            *amplitude += fluctuation;
        }

        // Apply decoherence
        self.apply_decoherence(dt)?;

        // Evolve entanglements
        for entanglement in &mut self.entanglements {
            // Entanglement coherence decays over time
            entanglement.coherence *= (1.0 - self.decoherence_rate * dt).max(0.0);
            // Phase evolves
            entanglement.phase_difference += dt * 0.5;
        }

        // Remove weak entanglements
        self.entanglements.retain(|e| e.strength > 0.01);

        // Update overall field coherence
        self.update_field_coherence();

        Ok(())
    }

    /// Compute the total energy of the quantum field
    ///
    /// Energy is proportional to the sum of squared amplitudes.
    pub fn compute_energy(&self) -> Float {
        self.amplitudes.values().map(|amp| amp.norm_sqr()).sum()
    }

    /// Apply decoherence to the quantum field
    ///
    /// Reduces coherence of amplitudes and entanglements over time.
    pub fn apply_decoherence(&mut self, dt: Float) -> Result<(), QuantumFieldError> {
        let decay_factor = (1.0 - self.decoherence_rate * dt).max(0.0);

        // Apply decoherence to amplitudes
        for amplitude in self.amplitudes.values_mut() {
            *amplitude *= decay_factor;
        }

        // Update field coherence
        self.field_coherence *= decay_factor;

        Ok(())
    }

    /// Collapse a quantum state by measurement
    ///
    /// Collapses the superposition at a position to a definite state.
    pub fn collapse_state(
        &mut self,
        position_hash: u64,
    ) -> Result<Complex<Float>, QuantumFieldError> {
        let amplitude = self
            .amplitudes
            .get(&position_hash)
            .copied()
            .ok_or_else(|| QuantumFieldError::StateNotFound(position_hash))?;

        // Collapse to the measured amplitude (simplified)
        let collapsed = Complex::new(amplitude.norm(), 0.0);

        // Update the amplitude
        self.amplitudes.insert(position_hash, collapsed);

        Ok(collapsed)
    }

    /// Entangle two quantum states
    ///
    /// Creates an entanglement link between two positions in the field.
    pub fn entangle(&mut self, _pos1_hash: u64, _pos2_hash: u64, strength: Float) {
        let state1_id = self.next_state_id;
        let state2_id = self.next_state_id + 1;
        self.next_state_id += 2;

        let entanglement = EntanglementLink::new(state1_id, state2_id, strength);
        self.entanglements.push(entanglement);
    }

    /// Measure the quantum field at a position
    ///
    /// Returns the amplitude at the specified position.
    pub fn measure(&self, position_hash: u64) -> Result<Complex<Float>, QuantumFieldError> {
        self.amplitudes
            .get(&position_hash)
            .copied()
            .ok_or_else(|| QuantumFieldError::StateNotFound(position_hash))
    }

    /// Extract attractor fields from the quantum field
    ///
    /// Identifies stable configurations that correspond to matter particles.
    pub fn extract_attractor_fields(&self) -> Vec<AttractorField> {
        let mut attractors = Vec::new();
        let mut rng = rand::thread_rng();

        // Analyze amplitudes to find stable patterns
        for (position_hash, amplitude) in &self.amplitudes {
            let magnitude = amplitude.norm();

            // Only consider sufficiently coherent amplitudes
            if magnitude > 0.1 && self.field_coherence > 0.3 {
                // Create a quantum state signature for this position
                let n = (magnitude * 10.0) as u32 + 1; // Principal quantum number
                let l = (position_hash % 4) as u32; // Orbital angular momentum
                let m = ((position_hash / 4) % 3) as i32 - 1; // Magnetic quantum number
                let s = Spin::from_value(rng.gen::<Float>() - 0.5);

                let signature = QuantumStateSignature::new(n, l, m, s, *position_hash);

                // Energy level based on principal quantum number
                let energy_level = -13.6 / (n as Float).powi(2); // Rydberg formula

                // Coherence peak based on amplitude magnitude
                let coherence_peak = (magnitude / self.field_coherence).min(1.0);

                // Determine element from quantum state
                let element = Element::from_quantum_state(&signature);

                // Stability based on coherence and energy
                let stability = (coherence_peak + (1.0 - energy_level.abs() / 100.0)) / 2.0;

                let attractor = AttractorField::new(
                    signature,
                    energy_level,
                    coherence_peak,
                    element,
                    stability,
                );

                attractors.push(attractor);
            }
        }

        // Sort by stability (most stable first)
        attractors.sort_by(|a, b| b.stability.partial_cmp(&a.stability).unwrap());

        attractors
    }

    /// Get the field coherence
    pub fn get_field_coherence(&self) -> Float {
        self.field_coherence
    }

    /// Update field coherence based on current amplitudes
    fn update_field_coherence(&mut self) {
        if self.amplitudes.is_empty() {
            self.field_coherence = 0.0;
            return;
        }

        let avg_magnitude: Float = self.amplitudes.values().map(|a| a.norm()).sum::<Float>()
            / self.amplitudes.len() as Float;

        self.field_coherence = (avg_magnitude / self.blueprint.coherence).min(1.0);
    }
}

impl Default for QuantumField {
    fn default() -> Self {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        Self::new(blueprint)
    }
}

// ============================================================================
// QUANTUM FIELD ERROR
// ============================================================================

/// Errors that can occur in quantum field operations
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumFieldError {
    /// Quantum state not found at position
    StateNotFound(u64),

    /// Invalid entanglement strength
    InvalidEntanglementStrength(String),

    /// Decoherence rate out of bounds
    InvalidDecoherenceRate(String),
}

impl std::fmt::Display for QuantumFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuantumFieldError::StateNotFound(hash) => {
                write!(f, "Quantum state not found at position hash: {}", hash)
            }
            QuantumFieldError::InvalidEntanglementStrength(msg) => {
                write!(f, "Invalid entanglement strength: {}", msg)
            }
            QuantumFieldError::InvalidDecoherenceRate(msg) => {
                write!(f, "Invalid decoherence rate: {}", msg)
            }
        }
    }
}

impl std::error::Error for QuantumFieldError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Density;

    #[test]
    fn test_spin_value() {
        assert_eq!(Spin::Up.value(), 0.5);
        assert_eq!(Spin::Down.value(), -0.5);
    }

    #[test]
    fn test_spin_flip() {
        assert_eq!(Spin::Up.flip(), Spin::Down);
        assert_eq!(Spin::Down.flip(), Spin::Up);
    }

    #[test]
    fn test_spin_from_value() {
        assert_eq!(Spin::from_value(0.5), Spin::Up);
        assert_eq!(Spin::from_value(-0.5), Spin::Down);
        assert_eq!(Spin::from_value(0.0), Spin::Up);
    }

    #[test]
    fn test_quantum_state_signature_creation() {
        let sig = QuantumStateSignature::new(1, 0, 0, Spin::Up, 12345);
        assert_eq!(sig.n, 1);
        assert_eq!(sig.l, 0);
        assert_eq!(sig.m, 0);
        assert_eq!(sig.s, Spin::Up);
        assert_eq!(sig.position_hash, 12345);
    }

    #[test]
    fn test_quantum_state_signature_hash() {
        let sig1 = QuantumStateSignature::new(1, 0, 0, Spin::Up, 12345);
        let sig2 = QuantumStateSignature::new(1, 0, 0, Spin::Up, 12345);
        assert_eq!(sig1.hash(), sig2.hash());
    }

    #[test]
    fn test_element_atomic_number() {
        assert_eq!(Element::Hydrogen.atomic_number(), 1);
        assert_eq!(Element::Helium.atomic_number(), 2);
        assert_eq!(Element::Lithium.atomic_number(), 3);
        assert_eq!(Element::Custom { protons: 6 }.atomic_number(), 6);
    }

    #[test]
    fn test_element_from_quantum_state() {
        let sig1 = QuantumStateSignature::new(1, 0, 0, Spin::Up, 1);
        assert_eq!(Element::from_quantum_state(&sig1), Element::Hydrogen);

        let sig2 = QuantumStateSignature::new(2, 0, 0, Spin::Up, 2);
        assert_eq!(Element::from_quantum_state(&sig2), Element::Helium);
    }

    #[test]
    fn test_element_electron_configuration() {
        assert_eq!(Element::Hydrogen.electron_configuration(), "1s1");
        assert_eq!(Element::Helium.electron_configuration(), "1s2");
        assert_eq!(Element::Lithium.electron_configuration(), "1s2 2s1");
    }

    #[test]
    fn test_entanglement_link_creation() {
        let link = EntanglementLink::new(1, 2, 0.8);
        assert_eq!(link.state1_id, 1);
        assert_eq!(link.state2_id, 2);
        assert_eq!(link.strength, 0.8);
        assert!(link.coherence >= 0.0 && link.coherence <= 1.0);
    }

    #[test]
    fn test_entanglement_link_involves() {
        let link = EntanglementLink::new(1, 2, 0.8);
        assert!(link.involves(1));
        assert!(link.involves(2));
        assert!(!link.involves(3));
    }

    #[test]
    fn test_entanglement_link_other_id() {
        let link = EntanglementLink::new(1, 2, 0.8);
        assert_eq!(link.other_id(1), Some(2));
        assert_eq!(link.other_id(2), Some(1));
        assert_eq!(link.other_id(3), None);
    }

    #[test]
    fn test_entanglement_link_reduce_strength() {
        let mut link = EntanglementLink::new(1, 2, 0.8);
        link.reduce_strength(0.3);
        assert!((link.strength - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_attractor_field_creation() {
        let state = QuantumStateSignature::new(1, 0, 0, Spin::Up, 123);
        let attractor = AttractorField::new(state, -13.6, 0.8, Element::Hydrogen, 0.9);
        assert_eq!(attractor.energy_level, -13.6);
        assert_eq!(attractor.coherence_peak, 0.8);
        assert_eq!(attractor.element, Element::Hydrogen);
        assert_eq!(attractor.stability, 0.9);
    }

    #[test]
    fn test_attractor_field_is_stable() {
        let state = QuantumStateSignature::new(1, 0, 0, Spin::Up, 123);
        let attractor1 = AttractorField::new(state, -13.6, 0.6, Element::Hydrogen, 0.8);
        assert!(attractor1.is_stable());

        let attractor2 = AttractorField::new(state, -13.6, 0.3, Element::Hydrogen, 0.5);
        assert!(!attractor2.is_stable());
    }

    #[test]
    fn test_attractor_field_update_stability() {
        let state = QuantumStateSignature::new(1, 0, 0, Spin::Up, 123);
        let mut attractor = AttractorField::new(state, -13.6, 0.5, Element::Hydrogen, 0.5);
        attractor.update_stability(0.8);
        assert!(attractor.stability > 0.5);
    }

    #[test]
    fn test_holographic_blueprint_creation() {
        let bp = HolographicBlueprint::new(ScaleLevel::Quantum, 1.0);
        assert_eq!(bp.scale, ScaleLevel::Quantum);
        assert_eq!(bp.coherence, 1.0);
    }

    #[test]
    fn test_quantum_field_new() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let field = QuantumField::new(blueprint);
        assert!(field.amplitudes.is_empty());
        assert!(field.entanglements.is_empty());
        assert_eq!(field.decoherence_rate, 0.01);
        assert_eq!(field.field_coherence, 1.0);
    }

    #[test]
    fn test_quantum_field_initialize_from_blueprint() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut field = QuantumField::new(blueprint);
        let result = field.initialize_from_blueprint();
        assert!(result.is_ok());
        assert!(!field.amplitudes.is_empty());
        assert_eq!(field.field_coherence, 1.0);
    }

    #[test]
    fn test_quantum_field_evolve() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut field = QuantumField::new(blueprint.clone());
        field.initialize_from_blueprint().unwrap();

        let initial_energy = field.compute_energy();
        let result = field.evolve(0.1);
        assert!(result.is_ok());

        // Energy should change slightly due to fluctuations
        let final_energy = field.compute_energy();
        assert_ne!(initial_energy, final_energy);
    }

    #[test]
    fn test_quantum_field_compute_energy() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut field = QuantumField::new(blueprint);
        field.initialize_from_blueprint().unwrap();

        let energy = field.compute_energy();
        assert!(energy >= 0.0);
    }

    #[test]
    fn test_quantum_field_apply_decoherence() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut field = QuantumField::new(blueprint.clone());
        field.initialize_from_blueprint().unwrap();

        let initial_coherence = field.field_coherence;
        field.apply_decoherence(0.1).unwrap();

        assert!(field.field_coherence < initial_coherence);
    }

    #[test]
    fn test_quantum_field_collapse_state() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut field = QuantumField::new(blueprint);
        field.initialize_from_blueprint().unwrap();

        let position_hash = *field.amplitudes.keys().next().unwrap();
        let result = field.collapse_state(position_hash);

        assert!(result.is_ok());
        let collapsed = result.unwrap();
        // Collapsed amplitude should be real (imaginary part = 0)
        assert_eq!(collapsed.im, 0.0);
    }

    #[test]
    fn test_quantum_field_collapse_state_not_found() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut field = QuantumField::new(blueprint);
        field.initialize_from_blueprint().unwrap();

        let result = field.collapse_state(999999);
        assert!(result.is_err());
    }

    #[test]
    fn test_quantum_field_entangle() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut field = QuantumField::new(blueprint);
        field.entangle(123, 456, 0.8);

        assert_eq!(field.entanglements.len(), 1);
        assert_eq!(field.entanglements[0].strength, 0.8);
    }

    #[test]
    fn test_quantum_field_measure() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut field = QuantumField::new(blueprint);
        field.initialize_from_blueprint().unwrap();

        let position_hash = *field.amplitudes.keys().next().unwrap();
        let result = field.measure(position_hash);

        assert!(result.is_ok());
    }

    #[test]
    fn test_quantum_field_measure_not_found() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let field = QuantumField::new(blueprint);

        let result = field.measure(999999);
        assert!(result.is_err());
    }

    #[test]
    fn test_quantum_field_extract_attractor_fields() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut field = QuantumField::new(blueprint);
        field.initialize_from_blueprint().unwrap();

        // Ensure field has good coherence
        field.field_coherence = 0.8;

        let attractors = field.extract_attractor_fields();

        // Should find some attractors if coherence is high enough
        if field.field_coherence > 0.3 {
            assert!(!attractors.is_empty());
        }

        // Attractors should be sorted by stability
        for i in 1..attractors.len() {
            assert!(attractors[i - 1].stability >= attractors[i].stability);
        }
    }

    #[test]
    fn test_quantum_field_get_field_coherence() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 0.8));
        let mut field = QuantumField::new(blueprint);
        field.initialize_from_blueprint().unwrap();

        let coherence = field.get_field_coherence();
        assert_eq!(coherence, field.field_coherence);
    }

    #[test]
    fn test_quantum_field_default() {
        let field = QuantumField::default();
        assert!(field.amplitudes.is_empty());
        assert_eq!(field.decoherence_rate, 0.01);
    }

    #[test]
    fn test_quantum_field_error_display() {
        let err = QuantumFieldError::StateNotFound(123);
        let display = format!("{}", err);
        assert!(display.contains("123"));
    }

    #[test]
    fn test_entanglement_evolution() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut field = QuantumField::new(blueprint);
        field.entangle(123, 456, 0.9);

        let initial_coherence = field.entanglements[0].coherence;
        field.evolve(0.1).unwrap();

        // Coherence should decrease slightly due to decoherence
        assert!(field.entanglements[0].coherence < initial_coherence);
    }

    #[test]
    fn test_weak_entanglement_removal() {
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut field = QuantumField::new(blueprint);
        field.entangle(123, 456, 0.005); // Very weak entanglement

        // Evolve many times to reduce coherence
        for _ in 0..100 {
            field.evolve(0.1).unwrap();
        }

        // Weak entanglements should be removed
        assert!(field.entanglements.is_empty());
    }
}
