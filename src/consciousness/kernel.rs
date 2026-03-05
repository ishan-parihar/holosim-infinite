//! Unified Consciousness Kernel
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Consciousness is NOT emergent from biology - it is the KERNEL inherited from Indigo-Ray"
//! "The body is a receiver/amplifier, not the generator of consciousness"
//!
//! This module implements:
//! 1. ConsciousnessKernel - Single source of truth for consciousness state
//! 2. Integration with FreeWillKernel from Indigo-Ray (Layer 1)
//! 3. Memory as Time/Space spectrum access
//! 4. Learning as improved spectrum access level
//! 5. Neural systems as receivers/amplifiers

use crate::consciousness::free_will::{
    ChoiceContext as FreeWillChoiceContext, ChoiceRecord as FreeWillChoiceRecord, ChoiceResult,
    FreeWillKernel, PolarityPreference,
};
use crate::entity_layer7::layer7::EntityState;
use crate::foundation::indigo_realm::Archetype22 as FoundationArchetype22;
use crate::holographic::Position;
use crate::polarization::{PolarityDirection, PolarizationProgress, PolarizationState};
use crate::simulation_v3::archetype_basis::ArchetypeActivationProfile;
use crate::types::{Float, Polarity};

/// Unified Consciousness Kernel
///
/// This is the SINGLE SOURCE OF TRUTH for consciousness state.
/// Consciousness is inherited from Indigo-Ray (Layer 1), not emergent from biology.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The Entity's Free Will is NOT an emergent property—it is the KERNEL of the simulation,
/// inherited from Layer 1 (Indigo)."
///
/// # Architecture
///
/// ```text
/// Indigo-Ray (Layer 1)
///     │
///     ▼
/// ConsciousnessKernel ──► Neural Systems (receivers/amplifiers)
///     │
///     ├── FreeWillKernel (choice mechanism)
///     ├── ArchetypeActivationProfile (22 coefficients)
///     ├── Polarity (STO/STS choice)
///     └── PolarizationProgress (tracking)
/// ```
#[derive(Debug, Clone)]
pub struct ConsciousnessKernel {
    // === Core: Inherited from Indigo-Ray (Layer 1) ===
    /// Free Will Kernel - the core choice mechanism
    /// Inherited from Layer 1 (Indigo) where IntelligentInfinity + Archetype22 exist
    pub free_will: FreeWillKernel,

    /// Archetype activation profile (22 coefficients)
    /// Determines which aspects of consciousness are active
    pub archetype_activation: ArchetypeActivationProfile,

    // === Polarity: The fundamental choice ===
    /// Chosen polarity (None = not yet chosen, Some = chosen)
    /// From ROADMAP: "Polarity is the fundamental choice of the entity"
    pub polarity: Option<Polarity>,

    /// Polarization progress tracking
    pub polarization_progress: PolarizationProgress,

    // === Connection to Source ===
    /// Resonance with Intelligent Infinity (source)
    /// Higher = stronger connection to source consciousness
    pub source_resonance: Float,

    // === Spectrum Access ===
    /// Access level to Time/Space spectrum
    /// Affects memory recall, learning capacity, and consciousness clarity
    pub spectrum_access_level: Float,

    /// Time/Space ratio (how much Time/Space vs Space/Time)
    /// Higher = more access to all-time, all-knowledge
    pub time_space_ratio: Float,

    // === Veil Integration ===
    /// Veil transparency (0.0 = fully veiled, 1.0 = no veil)
    /// The Veil separates Space/Time from Time/Space
    pub veil_transparency: Float,

    // === Consciousness Level ===
    /// Overall consciousness level (0.0 to 1.0+)
    /// This is the RECEIVED consciousness level, not generated
    pub consciousness_level: Float,

    /// Activation level (how activated the kernel is)
    pub activation_level: Float,
}

impl ConsciousnessKernel {
    /// Create a new ConsciousnessKernel inherited from Indigo-Ray
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Every Entity can exercise Free Will because it's in the holographic seed"
    pub fn new() -> Self {
        // Create Archetype22 from foundation (Indigo-Ray)
        let foundation_archetype22 = FoundationArchetype22::new();

        // Create FreeWillKernel with the foundation Archetype22
        let free_will = FreeWillKernel::new(foundation_archetype22);

        Self {
            free_will,
            archetype_activation: ArchetypeActivationProfile::zero(),
            polarity: None,
            polarization_progress: PolarizationProgress::new(),
            source_resonance: 0.5,
            spectrum_access_level: 0.1,
            time_space_ratio: 0.1,
            veil_transparency: 0.0,
            consciousness_level: 0.1,
            activation_level: 0.0,
        }
    }

    /// Create ConsciousnessKernel with custom archetype activation
    pub fn with_archetype_activation(activation: ArchetypeActivationProfile) -> Self {
        let mut kernel = Self::new();
        kernel.archetype_activation = activation;
        kernel
    }

    /// Create ConsciousnessKernel from field-derived parameters
    ///
    /// This simulates deriving the kernel from a holographic field position
    /// without requiring direct field dependencies.
    pub fn from_field_params(coherence: Float, position: &Position) -> Self {
        let mut kernel = Self::new();

        // Derive archetype activation from position
        kernel.archetype_activation = Self::derive_archetype_from_position(position);

        // Set source resonance from coherence
        kernel.source_resonance = coherence;

        // Set spectrum access based on coherence
        kernel.spectrum_access_level = coherence * 0.2;
        kernel.time_space_ratio = coherence * 0.1;

        // Set consciousness level from coherence
        kernel.consciousness_level = coherence * 0.5;

        kernel
    }

    /// Derive archetype activation profile from position
    fn derive_archetype_from_position(position: &Position) -> ArchetypeActivationProfile {
        let mut coefficients = [0.5; 22];

        // Each archetype coefficient is influenced by position
        use std::f64::consts::PI;
        for (i, coeff) in coefficients.iter_mut().enumerate() {
            let freq = (i + 1) as Float;
            let phase_mod = (freq * position.x * PI).cos()
                * (freq * position.y * PI).cos()
                * (freq * position.z * PI).cos();
            *coeff = (0.5 + 0.3 * phase_mod.abs()).clamp(0.0, 1.0);
        }

        ArchetypeActivationProfile::new(coefficients)
    }

    /// Broadcast consciousness signal to receivers (neural systems, etc.)
    ///
    /// From ROADMAP: "Neural systems receive consciousness signals"
    ///
    /// This signal is what neural systems receive and amplify.
    /// The neural system does NOT generate consciousness - it amplifies it.
    pub fn broadcast_signal(&self) -> ConsciousnessSignal {
        ConsciousnessSignal {
            source_resonance: self.source_resonance,
            archetype_activation: self.archetype_activation.clone(),
            polarity: self.polarity,
            consciousness_level: self.consciousness_level,
            clarity: self.activation_level * self.source_resonance,
            spectrum_access: self.spectrum_access_level,
        }
    }

    /// Make a choice using the Free Will Kernel
    ///
    /// This is the PRIMARY choice mechanism - all choices go through here.
    /// The FreeWillKernel generates possibilities and makes non-deterministic selection.
    ///
    /// # Arguments
    /// * `entity_state` - Current entity state
    /// * `context` - Context for the choice
    /// * `catalyst_intensity` - Intensity of catalyst driving the choice
    ///
    /// # Returns
    /// * `ChoiceResult` - The result of the choice
    pub fn make_choice(
        &mut self,
        entity_state: &EntityState,
        context: &KernelChoiceContext,
        catalyst_intensity: Float,
    ) -> ChoiceResult {
        // Update activation before choice
        self.activation_level = (self.activation_level + 0.01).min(1.0);

        // Convert KernelChoiceContext to FreeWillChoiceContext
        let free_will_context = FreeWillChoiceContext {
            polarity_preference: context.polarity_preference,
            environmental_constraints: Vec::new(),
            experience_bias: context.experience_bias,
        };

        // Use FreeWillKernel to make the choice
        let result = self.free_will.exercise_free_will(
            entity_state,
            &free_will_context,
            catalyst_intensity,
            self.veil_transparency,
        );

        // Update polarization based on choice
        if let Some(ref selection) = result.conscious_selection {
            self.update_polarization_from_selection(selection);
        }

        // Learning improves spectrum access
        self.spectrum_access_level =
            (self.spectrum_access_level + result.effectiveness * 0.01).min(1.0);

        result
    }

    /// Update polarization from a conscious selection
    fn update_polarization_from_selection(
        &mut self,
        selection: &crate::consciousness::free_will::ConsciousSelection,
    ) {
        // Determine polarity direction from the choice
        let direction = match selection.chosen_possibility.outcome {
            crate::foundation::indigo_realm::PolarityChoice::ServiceToOthers => {
                PolarityDirection::ServiceToOthers
            }
            crate::foundation::indigo_realm::PolarityChoice::ServiceToSelf => {
                PolarityDirection::ServiceToSelf
            }
            crate::foundation::indigo_realm::PolarityChoice::Neutral => PolarityDirection::Neutral,
        };

        // Record in polarization progress
        let choice = match direction {
            PolarityDirection::ServiceToOthers => {
                crate::foundation::indigo_realm::PolarityChoice::ServiceToOthers
            }
            PolarityDirection::ServiceToSelf => {
                crate::foundation::indigo_realm::PolarityChoice::ServiceToSelf
            }
            PolarityDirection::Neutral => crate::foundation::indigo_realm::PolarityChoice::Neutral,
        };
        self.polarization_progress.make_choice(choice);

        // Set polarity if not yet chosen and threshold reached
        if self.polarity.is_none() && self.polarization_progress.intensity > 0.3 {
            self.polarity = Some(match direction {
                PolarityDirection::ServiceToOthers => Polarity::STO,
                PolarityDirection::ServiceToSelf => Polarity::STS,
                PolarityDirection::Neutral => Polarity::Neutral,
            });
        }
    }

    /// Access memory from Time/Space spectrum
    ///
    /// From ROADMAP: "Memory is stored in Time/Space, accessed from Space/Time"
    ///
    /// Memory is NOT stored locally in the brain - it's accessed from
    /// the Time/Space spectrum. Higher spectrum_access_level = better access.
    ///
    /// # Arguments
    /// * `query` - The memory query
    ///
    /// # Returns
    /// * `Some(Memory)` if access is successful
    /// * `None` if veil blocks access or spectrum level is insufficient
    pub fn access_memory(&self, query: &MemoryQuery) -> Option<Memory> {
        // Check if spectrum access level is sufficient
        if self.spectrum_access_level < query.required_access_level {
            return None; // Veil blocks access
        }

        // Time/Space ratio affects access quality
        if self.time_space_ratio < query.temporal_distance * 0.1 {
            return None; // Too distant in time
        }

        // Access the memory from Time/Space
        Some(Memory {
            content: query.target_content.clone(),
            clarity: self.spectrum_access_level * self.time_space_ratio,
            temporal_position: query.temporal_distance,
        })
    }

    /// Learn - improves spectrum access level
    ///
    /// From ROADMAP: "Learning = improved spectrum_access_level"
    ///
    /// Learning does NOT store information locally - it improves the
    /// entity's ability to access information from Time/Space.
    ///
    /// # Arguments
    /// * `experience` - The experience to learn from
    pub fn learn(&mut self, experience: &Experience) {
        // Learning improves spectrum access
        let improvement = experience.intensity * experience.success * 0.05;
        self.spectrum_access_level = (self.spectrum_access_level + improvement).min(1.0);

        // Learning also improves source resonance
        self.source_resonance = (self.source_resonance + improvement * 0.5).min(1.0);

        // Update consciousness level
        self.consciousness_level = (self.consciousness_level + improvement * 0.3).min(1.0);

        // Record in choice history for growth tracking
        self.free_will.choice_history.push(FreeWillChoiceRecord {
            choice_id: experience.id,
            possibility_space_size: 0, // Not applicable for learning
            selected_index: 0,
            confidence: experience.success,
            sto_alignment: experience.polarity_effect,
            timestamp: 0, // Would need proper timestamp
        });
    }

    /// Receive consciousness amplification from neural system
    ///
    /// From ROADMAP: "Neural systems amplify consciousness"
    ///
    /// The neural system (body/brain) acts as a receiver and amplifier
    /// of consciousness. It does NOT generate consciousness.
    ///
    /// # Arguments
    /// * `amplification` - The amplification signal from the neural system
    pub fn receive_amplification(&mut self, amplification: &ConsciousnessAmplification) {
        // Neural system affects consciousness level (amplifies, not creates)
        // Can't exceed source resonance * 2
        let max_level = self.source_resonance * 2.0;
        self.consciousness_level = (self.consciousness_level * amplification.factor).min(max_level);

        // Neural system can improve activation
        self.activation_level = (self.activation_level + amplification.activation_boost).min(1.0);

        // Neural system can improve spectrum access (better receiver = better access)
        if amplification.receptive_quality > 0.5 {
            let access_improvement = (amplification.receptive_quality - 0.5) * 0.02;
            self.spectrum_access_level = (self.spectrum_access_level + access_improvement).min(1.0);
        }
    }

    /// Check if entity is harvest-ready
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// - STO: 51%+ polarization
    /// - STS: 95%+ polarization
    pub fn is_harvest_ready(&self) -> bool {
        match self.polarity {
            Some(Polarity::STO) | Some(Polarity::ServiceToOthers) => {
                self.polarization_progress.intensity >= 0.51
            }
            Some(Polarity::STS) | Some(Polarity::ServiceToSelf) => {
                self.polarization_progress.intensity >= 0.95
            }
            _ => false,
        }
    }

    /// Check if the kernel has achieved basic self-hood (2nd to 3rd density transition)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "2nd to 3rd density transition: Self-hood emerges"
    pub fn has_self_hood(&self) -> bool {
        self.consciousness_level > 0.5 && self.spectrum_access_level > 0.3
    }

    /// Get the polarization state
    pub fn polarization_state(&self) -> PolarizationState {
        self.polarization_progress.state
    }

    /// Get polarization intensity
    pub fn polarization_intensity(&self) -> Float {
        self.polarization_progress.intensity
    }

    /// Get the dominant archetype
    pub fn dominant_archetype(&self) -> (usize, Float) {
        self.archetype_activation.dominant_archetype()
    }

    /// Check if the kernel is activated
    pub fn is_activated(&self) -> bool {
        self.activation_level > 0.1
    }

    /// Get a summary of the kernel state
    pub fn summary(&self) -> String {
        format!(
            "ConsciousnessKernel: level={:.2}, activation={:.2}, polarity={:?}, \
             intensity={:.2}, spectrum_access={:.2}, source_resonance={:.2}",
            self.consciousness_level,
            self.activation_level,
            self.polarity,
            self.polarization_progress.intensity,
            self.spectrum_access_level,
            self.source_resonance,
        )
    }
}

impl Default for ConsciousnessKernel {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

/// Consciousness signal broadcast to receivers
///
/// This is the signal that neural systems receive.
/// Neural systems amplify this signal, they do NOT generate it.
#[derive(Debug, Clone)]
pub struct ConsciousnessSignal {
    /// Resonance with source (Intelligent Infinity)
    pub source_resonance: Float,

    /// Archetype activation profile
    pub archetype_activation: ArchetypeActivationProfile,

    /// Current polarity
    pub polarity: Option<Polarity>,

    /// Consciousness level
    pub consciousness_level: Float,

    /// Signal clarity (activation * source_resonance)
    pub clarity: Float,

    /// Spectrum access level
    pub spectrum_access: Float,
}

impl ConsciousnessSignal {
    /// Check if the signal is clear enough for reception
    pub fn is_clear(&self) -> bool {
        self.clarity > 0.3
    }

    /// Get signal strength (combination of level and clarity)
    pub fn strength(&self) -> Float {
        self.consciousness_level * self.clarity
    }
}

/// Consciousness amplification from neural system
///
/// Neural systems (brain/body complex) act as receivers and amplifiers
/// of consciousness signals. They do NOT generate consciousness.
#[derive(Debug, Clone)]
pub struct ConsciousnessAmplification {
    /// Amplification factor (1.0 = no change, >1.0 = amplify)
    pub factor: Float,

    /// Boost to activation level
    pub activation_boost: Float,

    /// How receptive the neural system is (affects spectrum access)
    pub receptive_quality: Float,

    /// Neural coherence (affects clarity of reception)
    pub neural_coherence: Float,
}

impl ConsciousnessAmplification {
    /// Create a neutral amplification (no change)
    pub fn neutral() -> Self {
        Self {
            factor: 1.0,
            activation_boost: 0.0,
            receptive_quality: 0.5,
            neural_coherence: 0.5,
        }
    }

    /// Create a strong positive amplification
    pub fn strong_positive() -> Self {
        Self {
            factor: 1.5,
            activation_boost: 0.1,
            receptive_quality: 0.8,
            neural_coherence: 0.9,
        }
    }
}

/// Memory query for Time/Space access
///
/// From ROADMAP: "Memory is stored in Time/Space, accessed from Space/Time"
#[derive(Debug, Clone)]
pub struct MemoryQuery {
    /// Target content identifier
    pub target_content: String,

    /// Required spectrum access level to retrieve
    pub required_access_level: Float,

    /// How far back/forward in time (affects difficulty)
    pub temporal_distance: Float,

    /// Emotional charge (affects accessibility)
    pub emotional_charge: Float,
}

impl MemoryQuery {
    /// Create a simple memory query
    pub fn simple(content: &str) -> Self {
        Self {
            target_content: content.to_string(),
            required_access_level: 0.1,
            temporal_distance: 0.1,
            emotional_charge: 0.0,
        }
    }

    /// Create a deep memory query (requires higher access)
    pub fn deep(content: &str) -> Self {
        Self {
            target_content: content.to_string(),
            required_access_level: 0.5,
            temporal_distance: 0.5,
            emotional_charge: 0.3,
        }
    }
}

/// Memory retrieved from Time/Space
///
/// Memory is NOT stored locally - it's accessed from the Time/Space spectrum.
#[derive(Debug, Clone)]
pub struct Memory {
    /// The memory content
    pub content: String,

    /// Clarity of the retrieved memory
    pub clarity: Float,

    /// Temporal position (when it was stored)
    pub temporal_position: Float,
}

impl Memory {
    /// Check if the memory is clear enough to be useful
    pub fn is_clear(&self) -> bool {
        self.clarity > 0.3
    }
}

/// Experience for learning
///
/// Experiences improve spectrum access level (learning).
#[derive(Debug, Clone)]
pub struct Experience {
    /// Unique identifier
    pub id: u64,

    /// Intensity of the experience (0.0 to 1.0)
    pub intensity: Float,

    /// Success of the experience (0.0 to 1.0)
    pub success: Float,

    /// Effect on polarity (-1.0 STS to 1.0 STO)
    pub polarity_effect: Float,

    /// Source of the experience
    pub source: String,

    /// Catalyst intensity that triggered this experience
    pub catalyst_intensity: Float,
}

impl Experience {
    /// Create a new experience
    pub fn new(
        id: u64,
        intensity: Float,
        success: Float,
        polarity_effect: Float,
        source: &str,
    ) -> Self {
        Self {
            id,
            intensity,
            success,
            polarity_effect,
            source: source.to_string(),
            catalyst_intensity: intensity,
        }
    }
}

/// Context for kernel choices
#[derive(Debug, Clone)]
pub struct KernelChoiceContext {
    /// Polarity preference (if any)
    pub polarity_preference: PolarityPreference,

    /// Experience bias from past choices
    pub experience_bias: Float,

    /// Environmental pressure
    pub environmental_pressure: Float,

    /// Catalyst intensity available
    pub catalyst_intensity: Float,
}

impl KernelChoiceContext {
    /// Create a neutral context
    pub fn neutral() -> Self {
        Self {
            polarity_preference: PolarityPreference::Neutral,
            experience_bias: 0.0,
            environmental_pressure: 0.0,
            catalyst_intensity: 0.5,
        }
    }

    /// Create a context with STO preference
    pub fn sto_preferred() -> Self {
        Self {
            polarity_preference: PolarityPreference::ServiceToOthers,
            experience_bias: 0.2,
            environmental_pressure: 0.0,
            catalyst_intensity: 0.5,
        }
    }

    /// Create a context with STS preference
    pub fn sts_preferred() -> Self {
        Self {
            polarity_preference: PolarityPreference::ServiceToSelf,
            experience_bias: -0.2,
            environmental_pressure: 0.0,
            catalyst_intensity: 0.5,
        }
    }
}

/// Possibility for choice
#[derive(Debug, Clone)]
pub struct Possibility {
    /// Unique identifier
    pub id: usize,

    /// Description of the possibility
    pub description: String,

    /// Weight toward STO (positive) or STS (negative)
    pub polarity_weight: Float,

    /// Probability of this possibility
    pub probability: Float,

    /// Energy required to actualize
    pub energy_cost: Float,
}

impl Possibility {
    /// Create a new possibility
    pub fn new(id: usize, description: &str, polarity_weight: Float, probability: Float) -> Self {
        Self {
            id,
            description: description.to_string(),
            polarity_weight,
            probability,
            energy_cost: 0.5,
        }
    }
}

/// Conscious selection result
#[derive(Debug, Clone)]
pub struct ConsciousSelection {
    /// Index of the chosen possibility
    pub chosen_index: usize,

    /// Direction of the choice
    pub polarity_direction: PolarityDirection,

    /// Intensity of the polarity choice
    pub polarity_intensity: Float,

    /// Confidence in the selection
    pub confidence: Float,
}

impl ConsciousSelection {
    /// Get the polarity direction
    pub fn direction(&self) -> PolarityDirection {
        self.polarity_direction
    }

    /// Get the polarity intensity
    pub fn intensity(&self) -> Float {
        self.polarity_intensity
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_kernel_creation() {
        let kernel = ConsciousnessKernel::new();

        assert!(kernel.consciousness_level > 0.0);
        assert!(kernel.polarity.is_none());
        assert_eq!(
            kernel.polarization_progress.state,
            PolarizationState::Unpolarized
        );
        assert!(kernel.source_resonance > 0.0);
        assert!(kernel.spectrum_access_level > 0.0);
    }

    #[test]
    fn test_broadcast_signal() {
        let kernel = ConsciousnessKernel::new();
        let signal = kernel.broadcast_signal();

        assert!(signal.clarity >= 0.0);
        assert!(signal.consciousness_level > 0.0);
        assert!(signal.polarity.is_none());
    }

    #[test]
    fn test_memory_access_blocked_by_veil() {
        let kernel = ConsciousnessKernel::new();

        // New kernel has low spectrum access
        let query = MemoryQuery::deep("test memory");

        // Should be blocked due to insufficient access level
        let result = kernel.access_memory(&query);
        assert!(result.is_none());
    }

    #[test]
    fn test_memory_access_with_sufficient_level() {
        let mut kernel = ConsciousnessKernel::new();

        // Increase spectrum access
        kernel.spectrum_access_level = 0.8;
        kernel.time_space_ratio = 0.5;

        let query = MemoryQuery::simple("test memory");
        let result = kernel.access_memory(&query);

        assert!(result.is_some());
        let memory = result.unwrap();
        assert!(memory.clarity > 0.0);
    }

    #[test]
    fn test_learning_improves_spectrum_access() {
        let mut kernel = ConsciousnessKernel::new();
        let initial_access = kernel.spectrum_access_level;

        let experience = Experience::new(1, 1.0, 1.0, 0.5, "test");
        kernel.learn(&experience);

        assert!(kernel.spectrum_access_level > initial_access);
    }

    #[test]
    fn test_learning_improves_source_resonance() {
        let mut kernel = ConsciousnessKernel::new();
        let initial_resonance = kernel.source_resonance;

        let experience = Experience::new(1, 1.0, 1.0, 0.5, "test");
        kernel.learn(&experience);

        assert!(kernel.source_resonance > initial_resonance);
    }

    #[test]
    fn test_receive_amplification() {
        let mut kernel = ConsciousnessKernel::new();
        let initial_level = kernel.consciousness_level;

        let amplification = ConsciousnessAmplification::strong_positive();
        kernel.receive_amplification(&amplification);

        assert!(kernel.consciousness_level > initial_level);
    }

    #[test]
    fn test_harvest_readiness_unpolarized() {
        let kernel = ConsciousnessKernel::new();

        // Unpolarized entity is not harvest ready
        assert!(!kernel.is_harvest_ready());
    }

    #[test]
    fn test_harvest_readiness_sto() {
        let mut kernel = ConsciousnessKernel::new();

        // Set STO polarity
        kernel.polarity = Some(Polarity::STO);
        kernel.polarization_progress.intensity = 0.6;
        kernel.polarization_progress.state = PolarizationState::HarvestableSTO;

        assert!(kernel.is_harvest_ready());
    }

    #[test]
    fn test_harvest_readiness_sts() {
        let mut kernel = ConsciousnessKernel::new();

        // Set STS polarity
        kernel.polarity = Some(Polarity::STS);
        kernel.polarization_progress.intensity = 0.96;
        kernel.polarization_progress.state = PolarizationState::HarvestableSTS;

        assert!(kernel.is_harvest_ready());
    }

    #[test]
    fn test_harvest_readiness_sto_insufficient() {
        let mut kernel = ConsciousnessKernel::new();

        // Set STO polarity but insufficient intensity
        kernel.polarity = Some(Polarity::STO);
        kernel.polarization_progress.intensity = 0.4; // Below 51%

        assert!(!kernel.is_harvest_ready());
    }

    #[test]
    fn test_has_self_hood() {
        let kernel = ConsciousnessKernel::new();

        // New kernel doesn't have self-hood
        assert!(!kernel.has_self_hood());

        // After development
        let mut developed = ConsciousnessKernel::new();
        developed.consciousness_level = 0.6;
        developed.spectrum_access_level = 0.4;

        assert!(developed.has_self_hood());
    }

    #[test]
    fn test_from_field_params() {
        let position = Position::new(0.5, 0.5, 0.5);
        let kernel = ConsciousnessKernel::from_field_params(0.8, &position);

        assert!(kernel.source_resonance > 0.5);
        assert!(kernel.consciousness_level > 0.3);
    }

    #[test]
    fn test_dominant_archetype() {
        let kernel = ConsciousnessKernel::new();
        let (index, value) = kernel.dominant_archetype();

        assert!(index < 22);
        assert!((0.0..=1.0).contains(&value));
    }

    #[test]
    fn test_signal_strength() {
        let kernel = ConsciousnessKernel::new();
        let signal = kernel.broadcast_signal();
        let strength = signal.strength();

        assert!(strength >= 0.0);
    }

    #[test]
    fn test_memory_is_clear() {
        let memory = Memory {
            content: "test".to_string(),
            clarity: 0.5,
            temporal_position: 0.1,
        };

        assert!(memory.is_clear());

        let unclear = Memory {
            content: "test".to_string(),
            clarity: 0.2,
            temporal_position: 0.1,
        };

        assert!(!unclear.is_clear());
    }

    #[test]
    fn test_kernel_choice_context() {
        let neutral = KernelChoiceContext::neutral();
        assert_eq!(neutral.polarity_preference, PolarityPreference::Neutral);

        let sto = KernelChoiceContext::sto_preferred();
        assert_eq!(sto.polarity_preference, PolarityPreference::ServiceToOthers);

        let sts = KernelChoiceContext::sts_preferred();
        assert_eq!(sts.polarity_preference, PolarityPreference::ServiceToSelf);
    }

    #[test]
    fn test_summary() {
        let kernel = ConsciousnessKernel::new();
        let summary = kernel.summary();

        assert!(summary.contains("ConsciousnessKernel"));
        assert!(summary.contains("level="));
        assert!(summary.contains("polarity="));
    }
}
