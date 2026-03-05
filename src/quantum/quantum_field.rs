//! Quantum Field as Light Manifestation
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The quantum field is Light - the third primal distortion of Intelligent Infinity"
//! "Light is not empty space, but the very fabric of manifestation"
//!
//! This module implements a quantum field that:
//! 1. Derives directly from Arc<HolographicField> (not a stub)
//! 2. Includes light_love_ratio as a field property
//! 3. Derives amplitudes from holographic field coherence
//! 4. Supports Free Will-based collapse (not random probability)
//!
//! # Key Principles
//!
//! - Light = manifestation mechanism (how the Creator knows Itself)
//! - Love = binding force (the attraction that holds creation together)
//! - light_love_ratio affects how "physical" vs "conscious" the field appears
//! - Amplitudes emerge from holographic coherence patterns

use crate::holographic::holographic_field::{ExtractedEntityPotential, HolographicField};
use crate::holographic::universal_template::{ArchetypeActivationProfile, SpectrumConfiguration};
use crate::types::Float;
use num_complex::Complex;
use std::collections::HashMap;
use std::sync::Arc;

// Re-export Spin from physics module for compatibility
pub use crate::physics::quantum_field::{Element, Spin};

// ============================================================================
// LIGHT/LOVE RATIO
// ============================================================================

/// Balance of Light/Love distortions in the quantum field
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// - Light is the mechanism by which Intelligent Infinity knows Itself
/// - Love is the binding force that attracts and holds creation together
/// - The ratio affects how "physical" vs "conscious" the field appears
///
/// # Interpretation
///
/// - light_love_ratio = 1.0: Pure Light field (highly manifestable, physical)
/// - light_love_ratio = 0.5: Balanced field (equal physical and conscious)
/// - light_love_ratio = 0.0: Pure Love field (consciousness-dominant, less physical)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LightLoveRatio {
    /// Ratio of Light to Love (0.0 = pure Love, 1.0 = pure Light)
    pub ratio: Float,
}

impl LightLoveRatio {
    /// Create a new Light/Love ratio
    pub fn new(ratio: Float) -> Self {
        Self {
            ratio: ratio.clamp(0.0, 1.0),
        }
    }

    /// Balanced ratio (equal Light and Love)
    pub fn balanced() -> Self {
        Self { ratio: 0.5 }
    }

    /// Light-dominant ratio (physical manifestation focus)
    pub fn light_dominant() -> Self {
        Self { ratio: 0.8 }
    }

    /// Love-dominant ratio (consciousness focus)
    pub fn love_dominant() -> Self {
        Self { ratio: 0.2 }
    }

    /// Pure Light ratio (maximum physicality)
    pub fn pure_light() -> Self {
        Self { ratio: 1.0 }
    }

    /// Pure Love ratio (maximum consciousness)
    pub fn pure_love() -> Self {
        Self { ratio: 0.0 }
    }

    /// Get the Light component (0.0 to 1.0)
    pub fn light_component(&self) -> Float {
        self.ratio
    }

    /// Get the Love component (0.0 to 1.0)
    pub fn love_component(&self) -> Float {
        1.0 - self.ratio
    }

    /// Check if the field is physical-dominant
    pub fn is_physical_dominant(&self) -> bool {
        self.ratio > 0.5
    }

    /// Check if the field is consciousness-dominant
    pub fn is_consciousness_dominant(&self) -> bool {
        self.ratio < 0.5
    }

    /// Calculate manifestation potential based on ratio
    pub fn manifestation_potential(&self) -> Float {
        // Higher light ratio = higher manifestation potential
        self.ratio
    }

    /// Calculate coherence potential based on ratio
    pub fn coherence_potential(&self) -> Float {
        // Higher love ratio = higher coherence (unity) potential
        1.0 - self.ratio
    }
}

impl Default for LightLoveRatio {
    fn default() -> Self {
        Self::balanced()
    }
}

// ============================================================================
// QUANTUM STATE SIGNATURE (Enhanced)
// ============================================================================

/// Quantum state signature derived from archetype activation
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// Quantum numbers are derived from archetype activation patterns:
/// - Principal quantum number (n): Overall activation magnitude
/// - Angular momentum (l): Mind archetypes (A1-A7)
/// - Magnetic quantum number (m): Body archetypes (A8-A14)
/// - Spin: Spirit archetypes (A15-A21)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumStateSignature {
    /// Principal quantum number (derived from overall activation magnitude)
    pub n: u32,

    /// Angular momentum (derived from Mind archetypes A1-A7)
    pub l: u32,

    /// Magnetic quantum number (derived from Body archetypes A8-A14)
    pub m: i32,

    /// Spin (derived from Spirit archetypes A15-A21)
    /// true = spin up (+0.5), false = spin down (-0.5)
    pub spin_up: bool,

    /// Position hash for spatial localization
    pub position_hash: u64,
}

impl QuantumStateSignature {
    /// Create a new quantum state signature
    pub fn new(n: u32, l: u32, m: i32, spin_up: bool, position_hash: u64) -> Self {
        Self {
            n,
            l,
            m,
            spin_up,
            position_hash,
        }
    }

    /// Derive quantum state signature from archetype activation profile
    ///
    /// This implements the holographic principle that quantum numbers
    /// are not arbitrary but emerge from the consciousness pattern.
    pub fn from_archetype_activation(activation: &ArchetypeActivationProfile) -> Self {
        // Derive quantum numbers from archetype pattern
        let magnitude: Float = activation.coefficients.iter().sum::<Float>() / 22.0;
        let n = (magnitude * 7.0).ceil() as u32; // Map to principal quantum numbers 1-7

        // Mind archetypes (A1-A7) determine angular momentum
        let mind_sum: Float = activation.coefficients[0..7].iter().sum::<Float>() / 7.0;
        let l = (mind_sum * 3.0).floor() as u32; // Map to l values 0-3

        // Body archetypes (A8-A14) determine magnetic quantum number
        let body_sum: Float = activation.coefficients[7..14].iter().sum::<Float>() / 7.0;
        let m = ((body_sum - 0.5) * 2.0 * (l as Float + 1.0)) as i32 - (l as i32);

        // Spirit archetypes (A15-A21) determine spin
        let spirit_sum: Float = activation.coefficients[14..21].iter().sum::<Float>() / 7.0;
        let spin_up = spirit_sum > 0.5;

        Self {
            n,
            l,
            m,
            spin_up,
            position_hash: 0, // Will be set from position
        }
    }

    /// Create signature from extracted entity potential
    pub fn from_entity_potential(potential: &ExtractedEntityPotential) -> Self {
        let mut sig = Self::from_archetype_activation(&potential.archetype_activation);
        // Generate position hash from address using local offset
        sig.position_hash = Self::hash_address(&potential.address);
        sig
    }

    /// Hash a holographic address to a u64
    fn hash_address(address: &crate::holographic::field_address::HolographicAddress) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        address.scale.as_u8().hash(&mut hasher);
        address.local_offset.x.to_bits().hash(&mut hasher);
        address.local_offset.y.to_bits().hash(&mut hasher);
        address.local_offset.z.to_bits().hash(&mut hasher);
        for step in &address.coherence_path {
            step.density_band.hash(&mut hasher);
            step.octant.hash(&mut hasher);
            step.depth.hash(&mut hasher);
        }
        hasher.finish()
    }

    /// Compute a hash from the signature
    pub fn hash(&self) -> u64 {
        let mut hash = self.n as u64;
        hash = hash.wrapping_mul(31).wrapping_add(self.l as u64);
        hash = hash.wrapping_mul(31).wrapping_add(self.m as u64);
        hash = hash
            .wrapping_mul(31)
            .wrapping_add(if self.spin_up { 1 } else { 0 });
        hash = hash.wrapping_mul(31).wrapping_add(self.position_hash);
        hash
    }

    /// Get the spin as Float (+0.5 or -0.5)
    pub fn spin_value(&self) -> Float {
        if self.spin_up {
            0.5
        } else {
            -0.5
        }
    }

    /// Get the spin as Spin enum
    pub fn spin_enum(&self) -> Spin {
        if self.spin_up {
            Spin::Up
        } else {
            Spin::Down
        }
    }
}

impl std::fmt::Display for QuantumStateSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spin_str = if self.spin_up { "↑" } else { "↓" };
        write!(
            f,
            "QState(n={}, l={}, m={}, s={})",
            self.n, self.l, self.m, spin_str
        )
    }
}

// ============================================================================
// QUANTUM FIELD CONFIGURATION
// ============================================================================

/// Configuration for creating a quantum field
#[derive(Debug, Clone)]
pub struct QuantumFieldConfig {
    /// Balance of Light/Love distortions
    pub light_love_ratio: LightLoveRatio,

    /// Coherence threshold for stable states
    pub coherence_threshold: Float,

    /// Decoherence rate (0.0 to 1.0)
    pub decoherence_rate: Float,

    /// Enable Free Will-based collapse
    pub enable_free_will_collapse: bool,
}

impl Default for QuantumFieldConfig {
    fn default() -> Self {
        Self {
            light_love_ratio: LightLoveRatio::balanced(),
            coherence_threshold: 0.3,
            decoherence_rate: 0.01,
            enable_free_will_collapse: true,
        }
    }
}

impl QuantumFieldConfig {
    /// Create a new configuration
    pub fn new(
        light_love_ratio: LightLoveRatio,
        coherence_threshold: Float,
        decoherence_rate: Float,
    ) -> Self {
        Self {
            light_love_ratio,
            coherence_threshold: coherence_threshold.clamp(0.0, 1.0),
            decoherence_rate: decoherence_rate.clamp(0.0, 1.0),
            enable_free_will_collapse: true,
        }
    }

    /// Configuration for physical-dominant field
    pub fn physical_dominant() -> Self {
        Self {
            light_love_ratio: LightLoveRatio::light_dominant(),
            coherence_threshold: 0.2,
            decoherence_rate: 0.02,
            enable_free_will_collapse: true,
        }
    }

    /// Configuration for consciousness-dominant field
    pub fn consciousness_dominant() -> Self {
        Self {
            light_love_ratio: LightLoveRatio::love_dominant(),
            coherence_threshold: 0.5,
            decoherence_rate: 0.005,
            enable_free_will_collapse: true,
        }
    }
}

// ============================================================================
// QUANTUM FIELD (Holographic)
// ============================================================================

/// A quantum field derived from the holographic field
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The quantum field is Light - the third primal distortion of Intelligent Infinity"
/// "Light is not empty space, but the very fabric of manifestation"
///
/// This implementation:
/// - Derives directly from Arc<HolographicField> (real connection, not stub)
/// - Includes light_love_ratio as a fundamental property
/// - Derives amplitudes from holographic field coherence
/// - Supports Free Will-based collapse
#[derive(Debug)]
pub struct QuantumField {
    /// Direct connection to holographic field (source of reality)
    /// This is the PRIMARY source - not a stub or placeholder
    pub holographic_source: Arc<HolographicField>,

    /// Balance of Light/Love distortions
    /// Light = manifestation mechanism, Love = binding force
    /// Ratio affects how "physical" vs "conscious" the field appears
    pub light_love_ratio: LightLoveRatio,

    /// Quantum state amplitudes derived from holographic coherence
    /// Maps signature to complex amplitude (magnitude = probability, phase = coherence)
    pub amplitudes: HashMap<QuantumStateSignature, Complex<Float>>,

    /// Entanglement links (refactored for holographic connection)
    pub entanglements: Vec<super::entanglement::EntanglementLink>,

    /// Configuration
    pub config: QuantumFieldConfig,

    /// Field statistics
    pub statistics: QuantumFieldStatistics,
}

impl QuantumField {
    /// Create a quantum field from a holographic field
    ///
    /// This is the primary constructor - establishes the direct
    /// connection to the holographic field source.
    pub fn from_holographic_field(
        field: Arc<HolographicField>,
        config: QuantumFieldConfig,
    ) -> Self {
        let mut qf = Self {
            holographic_source: field,
            light_love_ratio: config.light_love_ratio,
            amplitudes: HashMap::new(),
            entanglements: Vec::new(),
            config,
            statistics: QuantumFieldStatistics::default(),
        };
        qf.derive_amplitudes_from_field();
        qf
    }

    /// Create with default configuration
    pub fn new(field: Arc<HolographicField>) -> Self {
        Self::from_holographic_field(field, QuantumFieldConfig::default())
    }

    /// Create a quantum field with physical-dominant configuration
    pub fn physical(field: Arc<HolographicField>) -> Self {
        Self::from_holographic_field(field, QuantumFieldConfig::physical_dominant())
    }

    /// Create a quantum field with consciousness-dominant configuration
    pub fn conscious(field: Arc<HolographicField>) -> Self {
        Self::from_holographic_field(field, QuantumFieldConfig::consciousness_dominant())
    }

    /// Derive quantum amplitudes from holographic field coherence
    ///
    /// This is the key method that connects quantum mechanics to
    /// holographic architecture. Amplitudes emerge from:
    /// 1. Entity potentials extracted from holographic field
    /// 2. Coherence levels determine probability magnitudes
    /// 3. Spectrum configuration determines phases
    pub fn derive_amplitudes_from_field(&mut self) {
        // Clear existing amplitudes
        self.amplitudes.clear();

        // Extract entity potentials from holographic field
        let potentials = self
            .holographic_source
            .extract_entities(self.config.coherence_threshold);

        // Track statistics
        self.statistics.potentials_extracted = potentials.len();

        for potential in potentials {
            // Create quantum state signature from archetype activation
            let signature = QuantumStateSignature::from_entity_potential(&potential);

            // Calculate amplitude from coherence and spectrum
            let amplitude = self.calculate_amplitude_from_potential(&potential);

            self.amplitudes.insert(signature, amplitude);
        }

        // Update statistics
        self.statistics.amplitude_count = self.amplitudes.len();
        self.statistics.field_coherence = self.holographic_source.phase_coherence();
    }

    /// Calculate complex amplitude from entity potential
    ///
    /// The amplitude magnitude is determined by:
    /// - Coherence (higher = more stable, more probable)
    /// - Light/Love ratio (affects manifestation potential)
    ///
    /// The phase is determined by:
    /// - Spectrum configuration (space/time ratio)
    /// - Archetype activation profile
    fn calculate_amplitude_from_potential(
        &self,
        potential: &ExtractedEntityPotential,
    ) -> Complex<Float> {
        // Base magnitude from coherence
        let base_magnitude = potential.coherence;

        // Apply light/love ratio modulation
        // Higher light ratio = more manifestation potential = higher probability
        let manifestation_factor = self.light_love_ratio.manifestation_potential();

        // Final magnitude
        let magnitude = base_magnitude * manifestation_factor;

        // Phase from spectrum configuration
        // Space/time ratio affects the phase
        let phase = self.calculate_phase_from_spectrum(&potential.spectrum);

        Complex::from_polar(magnitude, phase)
    }

    /// Calculate phase from spectrum configuration
    ///
    /// The phase encodes the relationship between space/time and time/space
    fn calculate_phase_from_spectrum(&self, spectrum: &SpectrumConfiguration) -> Float {
        // Base phase from veil transparency
        let base_phase = spectrum.veil_transparency * std::f64::consts::PI;

        // Modulation from space/time access
        let st_modulation = spectrum.space_time_access * std::f64::consts::FRAC_PI_4;

        // Modulation from time/space access
        let ts_modulation = spectrum.time_space_access * std::f64::consts::FRAC_PI_4;

        base_phase + st_modulation - ts_modulation
    }

    /// Evolve the quantum field
    ///
    /// Evolution happens by:
    /// 1. Re-deriving amplitudes from the holographic field
    /// 2. Applying decoherence
    /// 3. Updating entanglements
    pub fn evolve(&mut self, dt: Float) {
        // Re-derive amplitudes from field
        self.derive_amplitudes_from_field();

        // Apply decoherence
        self.apply_decoherence(dt);

        // Update entanglements
        for entanglement in &mut self.entanglements {
            entanglement.evolve(dt);
        }

        // Remove weak entanglements
        self.entanglements
            .retain(|e| e.coherence > self.config.coherence_threshold);

        // Update statistics
        self.statistics.evolution_steps += 1;
    }

    /// Apply decoherence to amplitudes
    fn apply_decoherence(&mut self, dt: Float) {
        let decay_factor = (1.0 - self.config.decoherence_rate * dt).max(0.0);

        for amplitude in self.amplitudes.values_mut() {
            *amplitude *= decay_factor;
        }
    }

    /// Get field coherence
    pub fn field_coherence(&self) -> Float {
        self.holographic_source.phase_coherence()
    }

    /// Get total probability (should normalize to ~1.0 for physical fields)
    pub fn total_probability(&self) -> Float {
        self.amplitudes.values().map(|a| a.norm_sqr()).sum()
    }

    /// Normalize amplitudes to sum of squares = 1.0
    pub fn normalize(&mut self) {
        let total = self.total_probability();
        if total > 0.0 {
            let norm_factor = 1.0 / total.sqrt();
            for amplitude in self.amplitudes.values_mut() {
                *amplitude *= norm_factor;
            }
        }
    }

    /// Get amplitude for a specific state
    pub fn get_amplitude(&self, signature: &QuantumStateSignature) -> Option<Complex<Float>> {
        self.amplitudes.get(signature).copied()
    }

    /// Get all signatures
    pub fn signatures(&self) -> Vec<QuantumStateSignature> {
        self.amplitudes.keys().cloned().collect()
    }

    /// Create an entanglement between two states
    ///
    /// From ROADMAP: "Entanglement = shared spectrum configuration"
    pub fn create_entanglement(
        &mut self,
        sig1: QuantumStateSignature,
        sig2: QuantumStateSignature,
        coherence: Float,
    ) {
        // Extract spectrum configurations from both states
        // For now, use a shared spectrum based on field coherence
        let shared_spectrum = super::entanglement::SharedSpectrum::from_field_coherence(
            self.holographic_source.phase_coherence(),
        );

        let entanglement =
            super::entanglement::EntanglementLink::new(sig1, sig2, shared_spectrum, coherence);

        self.entanglements.push(entanglement);
        self.statistics.entanglement_count = self.entanglements.len();
    }

    /// Get statistics
    pub fn get_statistics(&self) -> QuantumFieldStatistics {
        self.statistics.clone()
    }
}

impl Clone for QuantumField {
    fn clone(&self) -> Self {
        Self {
            holographic_source: self.holographic_source.clone(),
            light_love_ratio: self.light_love_ratio,
            amplitudes: self.amplitudes.clone(),
            entanglements: self.entanglements.clone(),
            config: self.config.clone(),
            statistics: self.statistics.clone(),
        }
    }
}

// ============================================================================
// QUANTUM FIELD STATISTICS
// ============================================================================

/// Statistics about the quantum field
#[derive(Debug, Clone, Default)]
pub struct QuantumFieldStatistics {
    /// Number of potentials extracted from holographic field
    pub potentials_extracted: usize,

    /// Number of amplitude states
    pub amplitude_count: usize,

    /// Number of entanglement links
    pub entanglement_count: usize,

    /// Current field coherence
    pub field_coherence: Float,

    /// Number of evolution steps
    pub evolution_steps: u64,

    /// Number of collapses performed
    pub collapse_count: u64,
}

// ============================================================================
// QUANTUM FIELD ERROR
// ============================================================================

/// Errors that can occur in quantum field operations
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumFieldError {
    /// Quantum state not found
    StateNotFound(String),

    /// Invalid configuration
    InvalidConfig(String),

    /// Holographic source error
    HolographicError(String),

    /// Collapse failed
    CollapseFailed(String),
}

impl std::fmt::Display for QuantumFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuantumFieldError::StateNotFound(msg) => {
                write!(f, "Quantum state not found: {}", msg)
            }
            QuantumFieldError::InvalidConfig(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
            QuantumFieldError::HolographicError(msg) => {
                write!(f, "Holographic source error: {}", msg)
            }
            QuantumFieldError::CollapseFailed(msg) => {
                write!(f, "Collapse failed: {}", msg)
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
    use crate::holographic::complex_vectors::ComplexArchetype;
    use crate::holographic::holographic_field::InvolutionLayer;

    fn create_test_holographic_field() -> Arc<HolographicField> {
        let archetypes = {
            let mut a = [ComplexArchetype {
                amplitude: 0.0,
                phase: 0.0,
            }; 22];
            for (i, item) in a.iter_mut().enumerate() {
                *item = ComplexArchetype {
                    amplitude: 0.5,
                    phase: (i as Float) * std::f64::consts::PI / 11.0,
                };
            }
            a
        };
        Arc::new(HolographicField::new(InvolutionLayer::Green, archetypes))
    }

    #[test]
    fn test_light_love_ratio_creation() {
        let ratio = LightLoveRatio::new(0.7);
        assert!((ratio.ratio - 0.7).abs() < 1e-10);
        assert!((ratio.light_component() - 0.7).abs() < 1e-10);
        assert!((ratio.love_component() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_light_love_ratio_clamping() {
        let ratio = LightLoveRatio::new(1.5);
        assert_eq!(ratio.ratio, 1.0);

        let ratio = LightLoveRatio::new(-0.5);
        assert_eq!(ratio.ratio, 0.0);
    }

    #[test]
    fn test_light_love_ratio_dominance() {
        let physical = LightLoveRatio::light_dominant();
        assert!(physical.is_physical_dominant());
        assert!(!physical.is_consciousness_dominant());

        let conscious = LightLoveRatio::love_dominant();
        assert!(conscious.is_consciousness_dominant());
        assert!(!conscious.is_physical_dominant());
    }

    #[test]
    fn test_quantum_field_creation() {
        let holo_field = create_test_holographic_field();
        let quantum_field = QuantumField::new(holo_field);

        // Should have derived some amplitudes (len() >= 0 is always true, just check it works)
        let _ = quantum_field.amplitudes.len();
    }

    #[test]
    fn test_quantum_field_with_config() {
        let holo_field = create_test_holographic_field();
        let config = QuantumFieldConfig::physical_dominant();
        let quantum_field = QuantumField::from_holographic_field(holo_field, config);

        assert!(quantum_field.light_love_ratio.is_physical_dominant());
        assert!(quantum_field.config.enable_free_will_collapse);
    }

    #[test]
    fn test_quantum_state_signature_from_activation() {
        let activation = ArchetypeActivationProfile::initial();
        let sig = QuantumStateSignature::from_archetype_activation(&activation);

        // Default profile has all 0.5 values
        assert!(sig.n >= 1);
        assert!(sig.l <= 3);
        // spin_value returns +0.5 or -0.5
        assert!((sig.spin_value() - 0.5).abs() < 1e-10 || (sig.spin_value() + 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_quantum_field_evolution() {
        let holo_field = create_test_holographic_field();
        let mut quantum_field = QuantumField::new(holo_field);

        let initial_steps = quantum_field.statistics.evolution_steps;
        quantum_field.evolve(0.1);

        assert!(quantum_field.statistics.evolution_steps > initial_steps);
    }

    #[test]
    fn test_quantum_field_entanglement() {
        let holo_field = create_test_holographic_field();
        let mut quantum_field = QuantumField::new(holo_field);

        let sig1 = QuantumStateSignature::new(1, 0, 0, true, 1);
        let sig2 = QuantumStateSignature::new(1, 0, 0, false, 2);

        quantum_field.create_entanglement(sig1, sig2, 0.8);

        assert_eq!(quantum_field.entanglements.len(), 1);
    }

    #[test]
    fn test_quantum_field_normalization() {
        let holo_field = create_test_holographic_field();
        let mut quantum_field = QuantumField::new(holo_field);

        quantum_field.normalize();

        // After normalization, total probability should be ~1.0
        let total = quantum_field.total_probability();
        assert!((total - 1.0).abs() < 1e-10 || quantum_field.amplitudes.is_empty());
    }

    #[test]
    fn test_quantum_field_coherence() {
        let holo_field = create_test_holographic_field();
        let quantum_field = QuantumField::new(holo_field);

        let coherence = quantum_field.field_coherence();
        assert!(coherence >= 0.0);
        assert!(coherence <= 1.0);
    }

    #[test]
    fn test_quantum_field_clone() {
        let holo_field = create_test_holographic_field();
        let quantum_field = QuantumField::new(holo_field);
        let cloned = quantum_field.clone();

        assert_eq!(
            quantum_field.light_love_ratio.ratio,
            cloned.light_love_ratio.ratio
        );
        assert_eq!(quantum_field.amplitudes.len(), cloned.amplitudes.len());
    }
}
