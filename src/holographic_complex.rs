// Holographic Complex Module
//
// This module implements the HolographicComplex structure that represents
// Mind/Body/Spirit as aspects of a unified holographic consciousness structure.
//
// This is a CRITICAL architectural refactoring to align with COSMOLOGICAL-ARCHITECTURE.md:
// "Mind, Body, Spirit are not separate axes but 'inextricably intertwined'
// developmental trajectories. Their cross-coupling is not constructed—it is
// the DIRECT RESULT of the Involution process."
//
// Knowledge Base Reference:
// - COSMOLOGICAL-ARCHITECTURE.md Section 4.2
// - ARCHITECTURE_AUDIT_REPORT.md Section 2.1 (Critical Gap #1)

use crate::entity_state::ArchetypeState;
use crate::types::Float;
use std::fmt;
use std::sync::Arc;

/// HolographicComplex represents Mind/Body/Spirit as aspects of a unified structure
///
/// CRITICAL ARCHITECTURAL PRINCIPLE:
/// Mind/Body/Spirit are NOT separate components - they are different ASPECTS
/// of the same underlying holographic consciousness structure.
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.2
/// "Mind, Body, and Spirit are not separate fields—they are different ASPECTS
/// of the same underlying seed manifesting through different interfaces."
#[derive(Debug, Clone)]
pub struct HolographicComplex {
    /// The unified consciousness structure containing all 22 archetypes
    ///
    /// This is the SINGLE source of truth. All aspects (Mind, Body, Spirit)
    /// are views/interpretations of this unified structure.
    unified_structure: Arc<UnifiedConsciousnessStructure>,

    /// Cross-coupling state showing inextricable intertwining
    ///
    /// This demonstrates that Mind/Body/Spirit are not separate but
    /// inextricably intertwined developmental trajectories.
    cross_coupling: CrossCouplingState,
}

/// Unified Consciousness Structure
///
/// The single, unified structure that contains all 22 archetypes.
/// This is the "whole" that Mind/Body/Spirit are aspects of.
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.2
/// "The Entity's internal structure is not a separate construct but the
/// DIRECT emergence of the Light/Love architecture compressing into a
/// specific point in Space/Time."
///
/// MIGRATED TO: src/entity_layer7/layer7.rs (Phase 4.5 Migration 4)
/// Use crate::entity_layer7::layer7::UnifiedConsciousnessStructure instead
pub use crate::entity_layer7::layer7::UnifiedConsciousnessStructure;

// LEGACY CODE: The original implementation has been migrated to entity_layer7::layer7
// Keeping this import for backward compatibility during migration period
// TODO: Remove this re-export after all imports are updated (Phase 4.6)

impl Default for HolographicComplex {
    fn default() -> Self {
        Self::new()
    }
}

impl HolographicComplex {
    /// Create a new holographic complex
    ///
    /// This creates the unified structure with Mind/Body/Spirit as aspects.
    pub fn new() -> Self {
        Self {
            unified_structure: Arc::new(UnifiedConsciousnessStructure::new()),
            cross_coupling: CrossCouplingState::new(),
        }
    }

    /// View the complex as Mind aspect
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.2
    /// "Mind, Body, Spirit are different ASPECTS of the same underlying seed
    /// manifesting through different interfaces."
    pub fn as_mind(&self) -> MindAspect {
        MindAspect {
            unified_structure: Arc::clone(&self.unified_structure),
        }
    }

    /// View the complex as Body aspect
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.2
    /// "Mind, Body, Spirit are different ASPECTS of the same underlying seed
    /// manifesting through different interfaces."
    pub fn as_body(&self) -> BodyAspect {
        BodyAspect {
            unified_structure: Arc::clone(&self.unified_structure),
        }
    }

    /// View the complex as Spirit aspect
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.2
    /// "Mind, Body, Spirit are different ASPECTS of the same underlying seed
    /// manifesting through different interfaces."
    pub fn as_spirit(&self) -> SpiritAspect {
        SpiritAspect {
            unified_structure: Arc::clone(&self.unified_structure),
        }
    }

    /// Get the cross-coupling state
    ///
    /// This demonstrates that Mind/Body/Spirit are inextricably intertwined.
    pub fn get_cross_coupling(&self) -> &CrossCouplingState {
        &self.cross_coupling
    }

    /// Get mutable reference to cross-coupling state
    pub fn cross_coupling_mut(&mut self) -> &mut CrossCouplingState {
        &mut self.cross_coupling
    }

    /// Update cross-coupling based on current aspect activations
    ///
    /// This demonstrates that coupling is not constructed but emerges
    /// from the aspect activations.
    pub fn update_cross_coupling(&mut self) {
        let mind_aspect = self.as_mind();
        let body_aspect = self.as_body();
        let spirit_aspect = self.as_spirit();

        // Calculate coupling based on aspect activations
        let mind_activation = mind_aspect.calculate_overall_activation();
        let body_activation = body_aspect.calculate_overall_activation();
        let spirit_activation = spirit_aspect.calculate_overall_activation();

        self.cross_coupling.mind_body_coupling = 1.0 - (mind_activation - body_activation).abs();
        self.cross_coupling.body_spirit_coupling =
            1.0 - (body_activation - spirit_activation).abs();
        self.cross_coupling.mind_spirit_coupling =
            1.0 - (mind_activation - spirit_activation).abs();
        self.cross_coupling.overall_intertwining = (self.cross_coupling.mind_body_coupling
            + self.cross_coupling.body_spirit_coupling
            + self.cross_coupling.mind_spirit_coupling)
            / 3.0;
    }

    /// Check if aspects reference the same underlying structure
    ///
    /// This is a key test for the aspect-based architecture.
    pub fn aspects_reference_same_structure(&self) -> bool {
        let mind = self.as_mind();
        let body = self.as_body();
        let spirit = self.as_spirit();

        // Check memory addresses - they should all point to the same structure
        let mind_ptr = Arc::as_ptr(&mind.unified_structure) as usize;
        let body_ptr = Arc::as_ptr(&body.unified_structure) as usize;
        let spirit_ptr = Arc::as_ptr(&spirit.unified_structure) as usize;

        mind_ptr == body_ptr && body_ptr == spirit_ptr
    }

    /// Get the unified consciousness structure (for internal use)
    pub fn unified_structure(&self) -> &UnifiedConsciousnessStructure {
        &self.unified_structure
    }

    /// Get mutable reference to unified structure (for internal use)
    ///
    /// This is wrapped in Arc for thread safety, so we need to use Arc::make_mut
    pub fn unified_structure_mut(&mut self) -> &mut UnifiedConsciousnessStructure {
        Arc::make_mut(&mut self.unified_structure)
    }
}

/// Mind Aspect
///
/// A view of the unified consciousness structure as the Mind complex.
///
/// Knowledge Base Reference: Archetypes/1. Mind/0. Mind Complex.json
/// "The Mind complex consists of archetypes A1-A7 (Matrix, Potentiator, Catalyst,
/// Experience, Significator, Transformation, Great Way)"
#[derive(Debug, Clone)]
pub struct MindAspect {
    /// Reference to the unified structure
    ///
    /// CRITICAL: This is a shared reference (Arc), NOT a copy.
    /// All aspects share the same underlying structure.
    unified_structure: Arc<UnifiedConsciousnessStructure>,
}

impl MindAspect {
    /// Get archetype state for Mind archetypes (A1-A7, indices 0-6)
    pub fn get_state(&self, archetype_index: usize) -> Option<ArchetypeState> {
        if archetype_index < 7 {
            self.unified_structure.get_state(archetype_index)
        } else {
            None
        }
    }

    /// Set archetype state for Mind archetypes
    pub fn set_state(&self, archetype_index: usize, _state: ArchetypeState) -> bool {
        if archetype_index < 7 {
            // Note: This requires mutable access through Arc::make_mut
            // In practice, this would be done through HolographicComplex
            false
        } else {
            false
        }
    }

    /// Get activation level for Mind archetypes
    pub fn get_activation(&self, archetype_index: usize) -> Option<Float> {
        if archetype_index < 7 {
            self.unified_structure.get_activation(archetype_index)
        } else {
            None
        }
    }

    /// Get wisdom level for Mind archetypes
    pub fn get_wisdom(&self, archetype_index: usize) -> Option<Float> {
        if archetype_index < 7 {
            self.unified_structure.get_wisdom(archetype_index)
        } else {
            None
        }
    }

    /// Calculate overall activation of Mind complex
    pub fn calculate_overall_activation(&self) -> Float {
        let mut total = 0.0;
        for i in 0..7 {
            if let Some(activation) = self.unified_structure.get_activation(i) {
                total += activation;
            }
        }
        total / 7.0
    }

    /// Calculate overall wisdom of Mind complex
    pub fn calculate_overall_wisdom(&self) -> Float {
        let mut total = 0.0;
        for i in 0..7 {
            if let Some(wisdom) = self.unified_structure.get_wisdom(i) {
                total += wisdom;
            }
        }
        total / 7.0
    }

    /// Get all Mind archetype states
    pub fn get_all_states(&self) -> [ArchetypeState; 7] {
        let states: [ArchetypeState; 7] = (0..7)
            .map(|i| self.unified_structure.get_state(i).unwrap_or(ArchetypeState::Latent))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or([ArchetypeState::Latent; 7]);
        states
    }

    /// Check if this aspect references the same structure as another aspect
    pub fn references_same_structure(&self, other: &AspectView) -> bool {
        let other_ptr = match other {
            AspectView::Mind(m) => Arc::as_ptr(&m.unified_structure) as usize,
            AspectView::Body(b) => Arc::as_ptr(&b.unified_structure) as usize,
            AspectView::Spirit(s) => Arc::as_ptr(&s.unified_structure) as usize,
        };
        Arc::as_ptr(&self.unified_structure) as usize == other_ptr
    }

    /// Check if Mind is balanced and transparent
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
    /// "Mind is balanced when distortions are cleared"
    ///
    /// Mind is balanced (transparent) when distortions are cleared, allowing Spirit to shine through.
    /// This is the "transparency" that enables the valve to open.
    ///
    /// # Returns
    /// true if Mind is balanced, false otherwise
    pub fn is_balanced(&self) -> bool {
        // Mind is balanced when distortions are cleared
        // Check for blocked or distorted states in Mind archetypes
        let states = self.get_all_states();

        // Mind is balanced if there are no blocked or distorted states
        !states
            .iter()
            .any(|state| matches!(state, ArchetypeState::Blocked | ArchetypeState::Distorted))
    }

    /// Check if Mind is partially balanced
    ///
    /// Mind is partially balanced when some distortions are cleared but not all.
    /// This allows partial flow (Restricted valve state).
    ///
    /// # Returns
    /// true if Mind is partially balanced, false otherwise
    pub fn is_partially_balanced(&self) -> bool {
        // Mind is partially balanced if it's not fully balanced but has some activation
        let states = self.get_all_states();
        let blocked_count = states
            .iter()
            .filter(|state| matches!(state, ArchetypeState::Blocked | ArchetypeState::Distorted))
            .count();

        // Partially balanced if some (but not all) archetypes are blocked/distorted
        blocked_count > 0 && blocked_count < 7
    }

    /// Get the valve state of Mind
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
    /// "Mind is the VALVE that regulates the flow"
    ///
    /// The Mind acts as a valve regulating the flow between Body (up-pouring) and Spirit (in-pouring).
    /// The valve state depends on Mind's balance and transparency.
    ///
    /// # Returns
    /// The current valve state (Open, Restricted, or Closed)
    pub fn valve_state(&self) -> crate::energy_ray_centers::ValveState {
        use crate::energy_ray_centers::ValveState;

        // Check if Mind is balanced and transparent
        if self.is_balanced() {
            // Mind is fully transparent - valve is open
            // Up-pouring meets in-pouring - Green Ray activation
            ValveState::Open
        } else if self.is_partially_balanced() {
            // Mind is partially transparent - valve is restricted
            // Partial flow - some Spirit access, but limited
            ValveState::Restricted
        } else {
            // Mind is not transparent - valve is closed
            // No flow - Spirit remains as potential
            ValveState::Closed
        }
    }
}

/// Body Aspect
///
/// A view of the unified consciousness structure as the Body complex.
///
/// Knowledge Base Reference: Archetypes/2. Body/0. Body Complex.json
/// "The Body complex consists of archetypes A8-A14 (Matrix, Potentiator, Catalyst,
/// Experience, Significator, Transformation, Great Way)"
#[derive(Debug, Clone)]
pub struct BodyAspect {
    /// Reference to the unified structure
    ///
    /// CRITICAL: This is a shared reference (Arc), NOT a copy.
    /// All aspects share the same underlying structure.
    unified_structure: Arc<UnifiedConsciousnessStructure>,
}

impl BodyAspect {
    /// Get archetype state for Body archetypes (A8-A14, indices 7-13)
    pub fn get_state(&self, archetype_index: usize) -> Option<ArchetypeState> {
        if archetype_index < 7 {
            self.unified_structure.get_state(7 + archetype_index)
        } else {
            None
        }
    }

    /// Set archetype state for Body archetypes
    pub fn set_state(&self, archetype_index: usize, _state: ArchetypeState) -> bool {
        if archetype_index < 7 {
            // Note: This requires mutable access through Arc::make_mut
            false
        } else {
            false
        }
    }

    /// Get activation level for Body archetypes
    pub fn get_activation(&self, archetype_index: usize) -> Option<Float> {
        if archetype_index < 7 {
            self.unified_structure.get_activation(7 + archetype_index)
        } else {
            None
        }
    }

    /// Get wisdom level for Body archetypes
    pub fn get_wisdom(&self, archetype_index: usize) -> Option<Float> {
        if archetype_index < 7 {
            self.unified_structure.get_wisdom(7 + archetype_index)
        } else {
            None
        }
    }

    /// Calculate overall activation of Body complex
    pub fn calculate_overall_activation(&self) -> Float {
        let mut total = 0.0;
        for i in 7..14 {
            if let Some(activation) = self.unified_structure.get_activation(i) {
                total += activation;
            }
        }
        total / 7.0
    }

    /// Calculate overall wisdom of Body complex
    pub fn calculate_overall_wisdom(&self) -> Float {
        let mut total = 0.0;
        for i in 7..14 {
            if let Some(wisdom) = self.unified_structure.get_wisdom(i) {
                total += wisdom;
            }
        }
        total / 7.0
    }

    /// Get all Body archetype states
    pub fn get_all_states(&self) -> [ArchetypeState; 7] {
        let states: [ArchetypeState; 7] = (0..7)
            .map(|i| self.unified_structure.get_state(7 + i).unwrap_or(ArchetypeState::Latent))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or([ArchetypeState::Latent; 7]);
        states
    }

    /// Check if this aspect references the same structure as another aspect
    pub fn references_same_structure(&self, other: &AspectView) -> bool {
        let other_ptr = match other {
            AspectView::Mind(m) => Arc::as_ptr(&m.unified_structure) as usize,
            AspectView::Body(b) => Arc::as_ptr(&b.unified_structure) as usize,
            AspectView::Spirit(s) => Arc::as_ptr(&s.unified_structure) as usize,
        };
        Arc::as_ptr(&self.unified_structure) as usize == other_ptr
    }

    /// Generate up-pouring (vital energy) from survival/experience
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
    /// "Body generates 'up-pouring' (vital energy)"
    ///
    /// The Body generates vital energy from survival instincts and physical experience.
    /// This energy flows upward toward the Mind for processing.
    ///
    /// # Returns
    /// The level of up-pouring (0.0 to 1.0)
    pub fn generate_up_pouring(&self) -> Float {
        // Calculate up-pouring based on Body activation and wisdom
        // Higher activation and wisdom = more vital energy generated
        let activation = self.calculate_overall_activation();
        let _wisdom = self.calculate_overall_wisdom();

        // Up-pouring is primarily driven by survival instincts (Red/Orange rays)
        // and physical experience
        let survival_drive = self.unified_structure.get_activation(20).unwrap_or(0.0);

        let self_awareness = self.unified_structure.get_activation(13).unwrap_or(0.0);

        // Up-pouring combines survival drive, self-awareness, and overall activation
        let up_pouring = (survival_drive * 0.5) + (self_awareness * 0.3) + (activation * 0.2);

        up_pouring.clamp(0.0, 1.0)
    }
}

/// Spirit Aspect
///
/// A view of the unified consciousness structure as the Spirit complex.
///
/// Knowledge Base Reference: Archetypes/3. Spirit/0. Spirit Complex.json
/// "The Spirit complex consists of archetypes A15-A21 (Matrix, Potentiator, Catalyst,
/// Experience, Significator, Transformation, Great Way)"
#[derive(Debug, Clone)]
pub struct SpiritAspect {
    /// Reference to the unified structure
    ///
    /// CRITICAL: This is a shared reference (Arc), NOT a copy.
    /// All aspects share the same underlying structure.
    unified_structure: Arc<UnifiedConsciousnessStructure>,
}

impl SpiritAspect {
    /// Get archetype state for Spirit archetypes (A15-A21, indices 14-20)
    pub fn get_state(&self, archetype_index: usize) -> Option<ArchetypeState> {
        if archetype_index < 7 {
            self.unified_structure.get_state(14 + archetype_index)
        } else {
            None
        }
    }

    /// Set archetype state for Spirit archetypes
    pub fn set_state(&self, archetype_index: usize, _state: ArchetypeState) -> bool {
        if archetype_index < 7 {
            // Note: This requires mutable access through Arc::make_mut
            false
        } else {
            false
        }
    }

    /// Get activation level for Spirit archetypes
    pub fn get_activation(&self, archetype_index: usize) -> Option<Float> {
        if archetype_index < 7 {
            self.unified_structure.get_activation(14 + archetype_index)
        } else {
            None
        }
    }

    /// Get wisdom level for Spirit archetypes
    pub fn get_wisdom(&self, archetype_index: usize) -> Option<Float> {
        if archetype_index < 7 {
            self.unified_structure.get_wisdom(14 + archetype_index)
        } else {
            None
        }
    }

    /// Calculate overall activation of Spirit complex
    pub fn calculate_overall_activation(&self) -> Float {
        let mut total = 0.0;
        for i in 14..21 {
            if let Some(activation) = self.unified_structure.get_activation(i) {
                total += activation;
            }
        }
        total / 7.0
    }

    /// Calculate overall wisdom of Spirit complex
    pub fn calculate_overall_wisdom(&self) -> Float {
        let mut total = 0.0;
        for i in 14..21 {
            if let Some(wisdom) = self.unified_structure.get_wisdom(i) {
                total += wisdom;
            }
        }
        total / 7.0
    }

    /// Get all Spirit archetype states
    pub fn get_all_states(&self) -> [ArchetypeState; 7] {
        let states: [ArchetypeState; 7] = (0..7)
            .map(|i| self.unified_structure.get_state(14 + i).unwrap_or(ArchetypeState::Latent))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or([ArchetypeState::Latent; 7]);
        states
    }

    /// Check if this aspect references the same structure as another aspect
    pub fn references_same_structure(&self, other: &AspectView) -> bool {
        let other_ptr = match other {
            AspectView::Mind(m) => Arc::as_ptr(&m.unified_structure) as usize,
            AspectView::Body(b) => Arc::as_ptr(&b.unified_structure) as usize,
            AspectView::Spirit(s) => Arc::as_ptr(&s.unified_structure) as usize,
        };
        Arc::as_ptr(&self.unified_structure) as usize == other_ptr
    }

    /// Generate in-pouring (intelligent energy) from source
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
    /// "Spirit generates 'in-pouring' (intelligent energy)"
    ///
    /// The Spirit generates intelligent energy from Intelligent Infinity (Source).
    /// This energy flows inward toward the Mind, but requires a balanced Mind to access.
    ///
    /// # Returns
    /// The level of in-pouring (0.0 to 1.0)
    pub fn generate_in_pouring(&self) -> Float {
        // Calculate in-pouring based on Spirit activation and wisdom
        // Higher activation and wisdom = more intelligent energy generated
        let activation = self.calculate_overall_activation();
        let _wisdom = self.calculate_overall_wisdom();

        // In-pouring is primarily driven by connection to Source (Violet/Indigo rays)
        // and spiritual wisdom
        let unity_awareness = self.unified_structure.get_activation(0).unwrap_or(0.0);

        let divinity_connection = self.unified_structure.get_activation(6).unwrap_or(0.0);

        // In-pouring combines unity awareness, divinity connection, and overall activation
        let in_pouring = (unity_awareness * 0.5) + (divinity_connection * 0.3) + (activation * 0.2);

        in_pouring.clamp(0.0, 1.0)
    }
}

/// Aspect View Enum
///
/// Allows generic handling of different aspect types
#[derive(Debug, Clone)]
pub enum AspectView {
    Mind(MindAspect),
    Body(BodyAspect),
    Spirit(SpiritAspect),
}

/// Cross-Coupling State
///
/// Demonstrates that Mind/Body/Spirit are inextricably intertwined.
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.2
/// "Mind, Body, and Spirit are not separate axes but 'inextricably intertwined'
/// developmental trajectories. Their cross-coupling is not constructed—it is
/// the DIRECT RESULT of the Involution process."
///
/// MIGRATED TO: src/entity_layer7/layer7.rs (Phase 4.5 Migration 4)
/// Use crate::entity_layer7::layer7::CrossCouplingState instead
pub use crate::entity_layer7::layer7::CrossCouplingState;

// LEGACY CODE: The original implementation has been migrated to entity_layer7::layer7
// Keeping this re-export for backward compatibility during migration period
// TODO: Remove this re-export after all imports are updated (Phase 4.6)

/// Complex Type
///
/// Represents the three complexes: Mind, Body, Spirit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexType {
    /// Mind Complex (A1-A7)
    Mind,
    /// Body Complex (A8-A14)
    Body,
    /// Spirit Complex (A15-A21)
    Spirit,
}

impl fmt::Display for HolographicComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mind = self.as_mind();
        let body = self.as_body();
        let spirit = self.as_spirit();

        write!(
            f,
            "HolographicComplex:\n\
             - Mind Activation: {:.2}\n\
             - Mind Wisdom: {:.2}\n\
             - Body Activation: {:.2}\n\
             - Body Wisdom: {:.2}\n\
             - Spirit Activation: {:.2}\n\
             - Spirit Wisdom: {:.2}\n\
             - Mind-Body Coupling: {:.2}\n\
             - Body-Spirit Coupling: {:.2}\n\
             - Mind-Spirit Coupling: {:.2}\n\
             - Overall Intertwining: {:.2}\n\
             - Aspects Reference Same Structure: {}",
            mind.calculate_overall_activation(),
            mind.calculate_overall_wisdom(),
            body.calculate_overall_activation(),
            body.calculate_overall_wisdom(),
            spirit.calculate_overall_activation(),
            spirit.calculate_overall_wisdom(),
            self.cross_coupling.mind_body_coupling,
            self.cross_coupling.body_spirit_coupling,
            self.cross_coupling.mind_spirit_coupling,
            self.cross_coupling.overall_intertwining,
            self.aspects_reference_same_structure()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holographic_complex_creation() {
        let complex = HolographicComplex::new();
        assert!(complex.aspects_reference_same_structure());
    }

    #[test]
    fn test_aspects_reference_same_structure() {
        let complex = HolographicComplex::new();
        let mind = complex.as_mind();
        let body = complex.as_body();
        let spirit = complex.as_spirit();

        // Verify that all aspects reference the same underlying structure
        assert!(mind.references_same_structure(&AspectView::Body(body.clone())));
        assert!(body.references_same_structure(&AspectView::Spirit(spirit.clone())));
        assert!(spirit.references_same_structure(&AspectView::Mind(mind.clone())));
    }

    #[test]
    fn test_mind_aspect() {
        let complex = HolographicComplex::new();
        let mind = complex.as_mind();

        // Test getting states
        assert_eq!(mind.get_state(0), Some(ArchetypeState::Latent));
        assert_eq!(mind.get_state(6), Some(ArchetypeState::Latent));
        assert_eq!(mind.get_state(7), None); // Out of range for Mind

        // Test activation calculation
        assert_eq!(mind.calculate_overall_activation(), 0.0);
    }

    #[test]
    fn test_body_aspect() {
        let complex = HolographicComplex::new();
        let body = complex.as_body();

        // Test getting states
        assert_eq!(body.get_state(0), Some(ArchetypeState::Latent)); // A8 (index 7)
        assert_eq!(body.get_state(6), Some(ArchetypeState::Latent)); // A14 (index 13)
        assert_eq!(body.get_state(7), None); // Out of range for Body

        // Test activation calculation
        assert_eq!(body.calculate_overall_activation(), 0.0);
    }

    #[test]
    fn test_spirit_aspect() {
        let complex = HolographicComplex::new();
        let spirit = complex.as_spirit();

        // Test getting states
        assert_eq!(spirit.get_state(0), Some(ArchetypeState::Latent)); // A15 (index 14)
        assert_eq!(spirit.get_state(6), Some(ArchetypeState::Latent)); // A21 (index 20)
        assert_eq!(spirit.get_state(7), None); // Out of range for Spirit

        // Test activation calculation
        assert_eq!(spirit.calculate_overall_activation(), 0.0);
    }

    #[test]
    fn test_cross_coupling_state() {
        let mut coupling = CrossCouplingState::new();
        assert_eq!(coupling.overall_intertwining, 0.5);
        assert!(!coupling.is_inextricably_intertwined());

        // Increase coupling
        coupling.mind_body_coupling = 0.8;
        coupling.body_spirit_coupling = 0.8;
        coupling.mind_spirit_coupling = 0.8;
        coupling.calculate_balance();

        // Use approximate comparison for floating point
        assert!((coupling.overall_intertwining - 0.8).abs() < 0.0001);
        assert!(coupling.is_inextricably_intertwined());
    }

    #[test]
    fn test_update_cross_coupling() {
        let mut complex = HolographicComplex::new();

        // Activate some archetypes in unified structure
        {
            let structure = Arc::make_mut(&mut complex.unified_structure);
            structure.set_activation(0, 1.0); // Mind A1
            structure.set_activation(7, 1.0); // Body A8
            structure.set_activation(14, 1.0); // Spirit A15
        }

        // Update cross-coupling
        complex.update_cross_coupling();

        // Verify coupling is high (balanced activations)
        assert!(complex.cross_coupling.mind_body_coupling > 0.9);
        assert!(complex.cross_coupling.body_spirit_coupling > 0.9);
        assert!(complex.cross_coupling.mind_spirit_coupling > 0.9);
        assert!(complex.cross_coupling.is_inextricably_intertwined());
    }

    #[test]
    fn test_unified_consciousness_structure() {
        let mut structure = UnifiedConsciousnessStructure::new();

        // Test getting and setting states
        assert_eq!(structure.get_state(0), Some(ArchetypeState::Latent));
        assert!(structure.set_state(0, ArchetypeState::Active));
        assert_eq!(structure.get_state(0), Some(ArchetypeState::Active));

        // Test getting and setting activations
        assert_eq!(structure.get_activation(0), Some(0.0));
        assert!(structure.set_activation(0, 0.5));
        assert_eq!(structure.get_activation(0), Some(0.5));

        // Test getting and setting wisdom
        assert_eq!(structure.get_wisdom(0), Some(0.0));
        assert!(structure.set_wisdom(0, 0.75));
        assert_eq!(structure.get_wisdom(0), Some(0.75));

        // Test out of range
        assert_eq!(structure.get_state(22), None);
        assert!(!structure.set_state(22, ArchetypeState::Active));
    }

    #[test]
    fn test_aspect_view_enum() {
        let complex = HolographicComplex::new();
        let mind = complex.as_mind();
        let body = complex.as_body();
        let spirit = complex.as_spirit();

        let mind_view = AspectView::Mind(mind.clone());
        let body_view = AspectView::Body(body.clone());
        let spirit_view = AspectView::Spirit(spirit.clone());

        // Test that all views reference the same structure
        assert!(mind.references_same_structure(&body_view));
        assert!(body.references_same_structure(&spirit_view));
        assert!(spirit.references_same_structure(&mind_view));
    }

    #[test]
    fn test_complex_type() {
        let mind = ComplexType::Mind;
        let body = ComplexType::Body;
        let spirit = ComplexType::Spirit;

        assert_eq!(mind, ComplexType::Mind);
        assert_eq!(body, ComplexType::Body);
        assert_eq!(spirit, ComplexType::Spirit);
        assert_ne!(mind, body);
        assert_ne!(body, spirit);
        assert_ne!(spirit, mind);
    }

    #[test]
    fn test_get_coupling() {
        let mut coupling = CrossCouplingState::new();
        coupling.mind_body_coupling = 0.8;
        coupling.body_spirit_coupling = 0.6;
        coupling.mind_spirit_coupling = 0.7;

        assert_eq!(coupling.mind_body_coupling, 0.8);
        assert_eq!(coupling.body_spirit_coupling, 0.6);
        assert_eq!(coupling.mind_spirit_coupling, 0.7);
    }
}
