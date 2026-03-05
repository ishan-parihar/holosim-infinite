// Entity State Module
//
// This module implements the EntityState structure that tracks the mutable state
// that changes during Evolution.
//
// EntityState is the RAM (Random Access Memory) of the entity - it contains
// the mutable state that tracks activation levels, balance, and vibrational progress.
// This is what changes during Evolution.
//
// Knowledge Base Reference:
// - COSMOLOGICAL-ARCHITECTURE.md Section 5.1
// - Archetypes/1. Mind/A3. Catalyst of the Mind.json

use crate::archetypes::cycles::{
    ArchetypeChoice, ChoiceContext, ComplexType, GreaterCycle, GreaterCycleResult, LesserCycle,
};
use crate::energy_ray_centers::EnergyFlow;
use crate::entity_layer7::holographic_blueprint::Archetype22;
use crate::holographic_complex::HolographicComplex;
use crate::spectrum::ArchetypicalMind;
use crate::spirit_channel::SpiritChannel;
use crate::types::Float;
use std::fmt;

/// EntityState tracks the mutable state that changes during Evolution
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.1
/// "Entity State is RAM: The mutable state tracks activation levels,
/// balance, and vibrational progress. This is what changes during Evolution."
///
/// PHASE 1.1 REFACTOR:
/// EntityState now uses HolographicComplex which implements Mind/Body/Spirit
/// as ASPECTS of a unified structure, not separate components.
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.1
/// "Mind, Body, Spirit are not separate axes but 'inextricably intertwined'
/// developmental trajectories."
#[derive(Debug, Clone)]
pub struct EntityState {
    // 22-Archetype activation states (BACKWARD COMPATIBILITY)
    /// Each archetype can be: latent, activating, active, blocked, distorted
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The Entity's journey is about ACTIVATING the pre-existing holographic
    /// architecture that was ingrained during Involution."
    ///
    /// NOTE: This field is kept for backward compatibility.
    /// It is synchronized with holographic_complex.unified_structure.
    pub archetype_states: [ArchetypeState; 22],

    // Holographic Complex (PHASE 1.1 - ASPECT-BASED ARCHITECTURE)
    /// Mind/Body/Spirit as aspects of a unified consciousness structure
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.2
    /// "Mind, Body, Spirit are not separate axes but 'inextricably intertwined'
    /// developmental trajectories."
    pub holographic_complex: HolographicComplex,

    // Cross-coupling dynamics (Mind/Body/Spirit interaction)
    /// How Mind, Body, Spirit are interacting
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Mind, Body, and Spirit are not separate axes but 'inextricably intertwined'
    /// developmental trajectories."
    ///
    /// NOTE: This is now synced with holographic_complex.cross_coupling
    pub coupling_dynamics: CouplingDynamics,

    // Vibrational state (evolutionary progress)
    /// Determines which energy centers are accessible
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Entities differ not in architecture (which is universal) but in
    /// vibrational state (evolutionary progress)."
    pub vibrational_state: VibrationalState,

    // Spirit Channel (PHASE 2.4 - SPIRIT AS CHANNEL)
    /// Spirit as an active channel that funnels inpourings from Intelligent Infinity
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.8
    /// "Spirit functions as a 'channel' that funnels inpourings into consciousness."
    ///
    /// PHASE 2.4: Spirit is not just a vibration level, but an active channel
    /// that funnels intelligent energy from source into consciousness.
    pub spirit_channel: SpiritChannel,

    // Unprocessed Catalyst (PHASE 2.2 - ENTITY STATE COMPLETION)
    /// Accumulated unprocessed catalyst (shadow work)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Catalyst accumulates (trauma) because processing is impaired"
    ///
    /// PHASE 2.2: This field tracks catalyst that has been received but not yet
    /// processed into wisdom. This represents the entity's shadow work.
    pub unprocessed_catalyst: Vec<Catalyst>,
}

/// Archetype State
///
/// Each archetype can be in one of these states during evolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArchetypeState {
    /// Not yet activated
    #[default]
    Latent,

    /// In process of activation
    Activating,

    /// Fully functional
    Active,

    /// Unable to activate due to blockage
    Blocked,

    /// Active but distorted
    Distorted,
}

impl ArchetypeState {
    /// Check if archetype is active
    pub fn is_active(&self) -> bool {
        matches!(self, ArchetypeState::Active)
    }

    /// Check if archetype is blocked
    pub fn is_blocked(&self) -> bool {
        matches!(self, ArchetypeState::Blocked)
    }

    /// Check if archetype is distorted
    pub fn is_distorted(&self) -> bool {
        matches!(self, ArchetypeState::Distorted)
    }

    /// Get activation level (0.0 to 1.0)
    pub fn activation_level(&self) -> Float {
        match self {
            ArchetypeState::Latent => 0.0,
            ArchetypeState::Activating => 0.5,
            ArchetypeState::Active => 1.0,
            ArchetypeState::Blocked => 0.0,
            ArchetypeState::Distorted => 0.5,
        }
    }
}

/// Blockage - represents a blockage in the flow between aspects
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
/// "Blockages can occur 'betwixt spirit and mind, or body and mind, upon many different levels'"
#[derive(Debug, Clone, PartialEq)]
pub enum Blockage {
    /// Spirit-Mind blockage - Spirit cannot flow through Mind
    SpiritMind {
        /// Description of the blockage
        description: String,
        /// Severity (0.0 to 1.0)
        severity: Float,
        /// Archetype involved
        archetype_index: Option<usize>,
    },
    /// Body-Mind blockage - Body cannot communicate with Mind
    BodyMind {
        /// Description of the blockage
        description: String,
        /// Severity (0.0 to 1.0)
        severity: Float,
        /// Archetype involved
        archetype_index: Option<usize>,
    },
}

impl Blockage {
    /// Create a new Spirit-Mind blockage
    pub fn spirit_mind(description: impl Into<String>, severity: Float) -> Self {
        Blockage::SpiritMind {
            description: description.into(),
            severity: severity.clamp(0.0, 1.0),
            archetype_index: None,
        }
    }

    /// Create a new Body-Mind blockage
    pub fn body_mind(description: impl Into<String>, severity: Float) -> Self {
        Blockage::BodyMind {
            description: description.into(),
            severity: severity.clamp(0.0, 1.0),
            archetype_index: None,
        }
    }

    /// Get the severity of this blockage
    pub fn severity(&self) -> Float {
        match self {
            Blockage::SpiritMind { severity, .. } => *severity,
            Blockage::BodyMind { severity, .. } => *severity,
        }
    }

    /// Check if this blockage is severe (severity > 0.7)
    pub fn is_severe(&self) -> bool {
        self.severity() > 0.7
    }
}

/// Blockage Set - collection of blockages between aspects
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
/// "Blockages can occur 'betwixt spirit and mind, or body and mind, upon many different levels'"
#[derive(Debug, Clone, Default)]
pub struct BlockageSet {
    /// Blockages between Spirit and Mind
    pub spirit_mind_blockages: Vec<Blockage>,

    /// Blockages between Body and Mind
    pub body_mind_blockages: Vec<Blockage>,
}

impl BlockageSet {
    /// Add a Spirit-Mind blockage
    pub fn add_spirit_mind(&mut self, blockage: Blockage) {
        if matches!(blockage, Blockage::SpiritMind { .. }) {
            self.spirit_mind_blockages.push(blockage);
        }
    }

    /// Add a Body-Mind blockage
    pub fn add_body_mind(&mut self, blockage: Blockage) {
        if matches!(blockage, Blockage::BodyMind { .. }) {
            self.body_mind_blockages.push(blockage);
        }
    }

    /// Calculate total Spirit-Mind blockage severity
    pub fn spirit_mind_severity(&self) -> Float {
        self.spirit_mind_blockages
            .iter()
            .map(|b| b.severity())
            .sum::<Float>()
            .min(1.0)
    }

    /// Calculate total Body-Mind blockage severity
    pub fn body_mind_severity(&self) -> Float {
        self.body_mind_blockages
            .iter()
            .map(|b| b.severity())
            .sum::<Float>()
            .min(1.0)
    }

    /// Check if there are any severe blockages
    pub fn has_severe_blockages(&self) -> bool {
        self.spirit_mind_blockages.iter().any(|b| b.is_severe())
            || self.body_mind_blockages.iter().any(|b| b.is_severe())
    }

    /// Clear all blockages
    pub fn clear(&mut self) {
        self.spirit_mind_blockages.clear();
        self.body_mind_blockages.clear();
    }
}

/// Coupling Dynamics
///
/// How Mind, Body, Spirit are interacting
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
/// "Cross-coupling as emergent from involutionary structure, not constructed as explicit connections"
#[derive(Debug, Clone)]
pub struct CouplingDynamics {
    /// Body generates up-pouring (vital energy) from survival/experience
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
    /// "Body generates 'up-pouring' (vital energy)"
    pub up_pouring: Float,

    /// Spirit generates in-pouring (intelligent energy) from source
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
    /// "Spirit generates 'in-pouring' (intelligent energy)"
    pub in_pouring: Float,

    /// Mind is the valve - regulates the flow between Body and Spirit
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
    /// "Mind is the VALVE that regulates the flow"
    pub valve_state: crate::energy_ray_centers::ValveState,

    /// Blockages can occur at different levels
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
    /// "Blockages can occur 'betwixt spirit and mind, or body and mind, upon many different levels'"
    pub blockages: BlockageSet,

    /// Mind-Body coupling (0.0 to 1.0) - BACKWARD COMPATIBILITY
    pub mind_body: Float,

    /// Body-Spirit coupling (0.0 to 1.0) - BACKWARD COMPATIBILITY
    pub body_spirit: Float,

    /// Mind-Spirit coupling (0.0 to 1.0) - BACKWARD COMPATIBILITY
    pub mind_spirit: Float,

    /// Overall coupling balance (0.0 to 1.0) - BACKWARD COMPATIBILITY
    pub overall_balance: Float,
}

impl Default for CouplingDynamics {
    fn default() -> Self {
        Self {
            up_pouring: 0.0,
            in_pouring: 0.0,
            valve_state: crate::energy_ray_centers::ValveState::Closed,
            blockages: BlockageSet::default(),
            mind_body: 0.5,
            body_spirit: 0.5,
            mind_spirit: 0.5,
            overall_balance: 0.5,
        }
    }
}

impl CouplingDynamics {
    /// Calculate overall coupling balance
    pub fn calculate_balance(&mut self) {
        self.overall_balance = (self.mind_body + self.body_spirit + self.mind_spirit) / 3.0;
    }

    /// Get coupling for a specific complex pair
    pub fn get_coupling(&self, complex1: ComplexType, complex2: ComplexType) -> Float {
        match (complex1, complex2) {
            (ComplexType::Mind, ComplexType::Body) | (ComplexType::Body, ComplexType::Mind) => {
                self.mind_body
            }
            (ComplexType::Body, ComplexType::Spirit) | (ComplexType::Spirit, ComplexType::Body) => {
                self.body_spirit
            }
            (ComplexType::Mind, ComplexType::Spirit) | (ComplexType::Spirit, ComplexType::Mind) => {
                self.mind_spirit
            }
            _ => 0.0,
        }
    }

    /// Update up-pouring level (Body generates vital energy)
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
    /// "Body generates 'up-pouring' (vital energy)"
    pub fn update_up_pouring(&mut self, level: Float) {
        self.up_pouring = level.clamp(0.0, 1.0);
    }

    /// Update in-pouring level (Spirit generates intelligent energy)
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
    /// "Spirit generates 'in-pouring' (intelligent energy)"
    pub fn update_in_pouring(&mut self, level: Float) {
        self.in_pouring = level.clamp(0.0, 1.0);
    }

    /// Update valve state (Mind regulates the flow)
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.4
    /// "Mind is the VALVE that regulates the flow"
    pub fn update_valve_state(&mut self, state: crate::energy_ray_centers::ValveState) {
        self.valve_state = state;
    }

    /// Add a blockage
    pub fn add_blockage(&mut self, blockage: Blockage) {
        match &blockage {
            Blockage::SpiritMind { .. } => self.blockages.add_spirit_mind(blockage),
            Blockage::BodyMind { .. } => self.blockages.add_body_mind(blockage),
        }
    }

    /// Check if the valve is open (Mind is balanced)
    pub fn is_valve_open(&self) -> bool {
        matches!(
            self.valve_state,
            crate::energy_ray_centers::ValveState::Open
        )
    }

    /// Check if the valve is restricted (Mind is partially balanced)
    pub fn is_valve_restricted(&self) -> bool {
        matches!(
            self.valve_state,
            crate::energy_ray_centers::ValveState::Restricted
        )
    }

    /// Check if the valve is closed (Mind is not balanced)
    pub fn is_valve_closed(&self) -> bool {
        matches!(
            self.valve_state,
            crate::energy_ray_centers::ValveState::Closed
        )
    }
}

/// Vibrational State
///
/// Evolutionary progress of the entity
#[derive(Debug, Clone)]
pub struct VibrationalState {
    /// Overall vibrational level (0.0 to 1.0)
    pub overall_level: Float,

    /// Mind complex vibration (0.0 to 1.0)
    pub mind_vibration: Float,

    /// Body complex vibration (0.0 to 1.0)
    pub body_vibration: Float,

    /// Spirit complex vibration (0.0 to 1.0)
    pub spirit_vibration: Float,

    /// Current density level (1-7)
    pub density_level: u8,

    /// Unity awareness (for Violet ray)
    pub unity_awareness: Float,
}

impl Default for VibrationalState {
    fn default() -> Self {
        Self {
            overall_level: 0.1,
            mind_vibration: 0.1,
            body_vibration: 0.1,
            spirit_vibration: 0.1,
            density_level: 1,
            unity_awareness: 0.0,
        }
    }
}

impl VibrationalState {
    /// Calculate overall vibrational level
    pub fn calculate_overall(&mut self) {
        self.overall_level =
            (self.mind_vibration + self.body_vibration + self.spirit_vibration) / 3.0;
    }

    /// Update density level based on vibration
    pub fn update_density(&mut self) {
        self.density_level = match self.overall_level {
            x if x < 0.15 => 1,
            x if x < 0.30 => 2,
            x if x < 0.45 => 3,
            x if x < 0.60 => 4,
            x if x < 0.75 => 5,
            x if x < 0.90 => 6,
            _ => 7,
        };
    }

    /// Check if entity can access a specific density
    pub fn can_access_density(&self, density: u8) -> bool {
        self.density_level >= density
    }
}

/// Catalyst
///
/// External stimulus that triggers archetype activation
#[derive(Debug, Clone)]
pub struct Catalyst {
    /// Catalyst intensity (0.0 to 1.0)
    pub intensity: Float,

    /// Catalyst type
    pub catalyst_type: CatalystType,

    /// Target archetype (if any)
    pub target_archetype: Option<usize>,

    /// Whether catalyst is distorting
    pub is_distorting: bool,

    /// Whether catalyst is blocking
    pub is_blocking: bool,

    /// Whether catalyst is healing
    pub is_healing: bool,
}

/// Catalyst Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalystType {
    /// Mind catalyst
    Mind,

    /// Body catalyst
    Body,

    /// Spirit catalyst
    Spirit,

    /// General catalyst
    General,
}

impl Catalyst {
    /// Create a new catalyst
    pub fn new(intensity: Float, catalyst_type: CatalystType) -> Self {
        Self {
            intensity,
            catalyst_type,
            target_archetype: None,
            is_distorting: false,
            is_blocking: false,
            is_healing: false,
        }
    }

    /// Check if catalyst is distorting
    pub fn is_distorting(&self) -> bool {
        self.is_distorting
    }

    /// Check if catalyst is blocking
    pub fn is_blocking(&self) -> bool {
        self.is_blocking
    }

    /// Check if catalyst is healing
    pub fn is_healing(&self) -> bool {
        self.is_healing
    }
}

/// Transition Result
///
/// Result of archetype state transition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionResult {
    /// No change occurred
    NoChange,

    /// Activation started
    ActivationStarted,

    /// Still activating
    StillActivating,

    /// Activation complete
    ActivationComplete,

    /// Became blocked
    Blocked,

    /// Still blocked
    StillBlocked,

    /// Became distorted
    Distorted,

    /// Still distorted
    StillDistorted,

    /// Unblocking started
    UnblockingStarted,

    /// Distortion healed
    DistortionHealed,
}

/// Experience - processed catalyst
///
/// Knowledge Base Reference: Archetypes/1. Mind/A4. Experience of the Mind.json
/// "Processed output - integrated wisdom from catalyst"
#[derive(Debug, Clone)]
pub struct Experience {
    /// The catalyst that generated this experience
    pub catalyst_source: Catalyst,

    /// Wisdom extracted from the catalyst
    pub wisdom_extracted: Float,

    /// Integration level (0.0 to 1.0)
    pub integration_level: Float,

    /// Which complex this experience belongs to
    pub complex_type: ComplexType,
}

impl Experience {
    /// Create a new experience from catalyst
    pub fn new(catalyst: Catalyst, complex_type: ComplexType) -> Self {
        Self {
            catalyst_source: catalyst,
            wisdom_extracted: 0.0,
            integration_level: 0.0,
            complex_type,
        }
    }

    /// Update wisdom extraction
    pub fn extract_wisdom(&mut self, amount: Float) {
        self.wisdom_extracted = (self.wisdom_extracted + amount).min(1.0);
    }

    /// Update integration level
    pub fn integrate(&mut self, amount: Float) {
        self.integration_level = (self.integration_level + amount).min(1.0);
    }
}

/// Intelligent Energy - Spirit in-pouring
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.4
/// "The Spirit generates intelligent energy from source"
#[derive(Debug, Clone)]
pub struct IntelligentEnergy {
    /// Energy intensity (0.0 to infinity)
    pub intensity: Float,

    /// Source of the energy (from Intelligent Infinity)
    pub source: EnergySource,

    /// Purity level (0.0 to 1.0)
    pub purity: Float,
}

/// Energy Source
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnergySource {
    /// From Intelligent Infinity (purest)
    IntelligentInfinity,
    /// From the entity's spirit complex
    SpiritComplex,
    /// From the veil (filtered)
    VeilFiltered,
}

impl IntelligentEnergy {
    /// Create new intelligent energy from source
    pub fn new(intensity: Float, source: EnergySource) -> Self {
        Self {
            intensity,
            source,
            purity: match source {
                EnergySource::IntelligentInfinity => 1.0,
                EnergySource::SpiritComplex => 0.8,
                EnergySource::VeilFiltered => 0.5,
            },
        }
    }
}

// Thresholds for state transitions
const ACTIVATION_THRESHOLD: Float = 0.3;
const COMPLETION_THRESHOLD: Float = 0.7;

impl Default for EntityState {
    fn default() -> Self {
        let holographic_complex = HolographicComplex::new();
        Self {
            archetype_states: [ArchetypeState::Latent; 22],
            holographic_complex,
            coupling_dynamics: CouplingDynamics::default(),
            vibrational_state: VibrationalState::default(),
            spirit_channel: SpiritChannel::new(),
            unprocessed_catalyst: Vec::new(),
        }
    }
}

impl EntityState {
    /// Create a new entity state
    pub fn new() -> Self {
        Self::default()
    }

    // ============================================================================
    // PHASE 1.1: ASPECT-BASED ARCHITECTURE METHODS
    // ============================================================================

    /// Get the holographic complex (aspect-based structure)
    ///
    /// This provides access to Mind/Body/Spirit as aspects of a unified structure.
    pub fn holographic_complex(&self) -> &HolographicComplex {
        &self.holographic_complex
    }

    /// Get mutable reference to holographic complex
    pub fn holographic_complex_mut(&mut self) -> &mut HolographicComplex {
        &mut self.holographic_complex
    }

    /// Sync archetype_states with holographic_complex
    ///
    /// This ensures backward compatibility by keeping the old archetype_states array
    /// in sync with the new holographic_complex structure.
    pub fn sync_archetype_states(&mut self) {
        let structure = self.holographic_complex.unified_structure();
        for i in 0..22 {
            if let Some(state) = structure.get_state(i) {
                self.archetype_states[i] = state;
            }
        }
    }

    /// Sync holographic_complex with archetype_states
    ///
    /// This updates the holographic_complex when archetype_states is modified directly.
    pub fn sync_holographic_complex(&mut self) {
        let structure = self.holographic_complex.unified_structure_mut();
        for i in 0..22 {
            structure.set_state(i, self.archetype_states[i]);
        }
    }

    /// Get Mind aspect
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.2
    /// "Mind, Body, Spirit are different ASPECTS of the same underlying seed"
    pub fn as_mind(&self) -> crate::holographic_complex::MindAspect {
        self.holographic_complex.as_mind()
    }

    /// Get Body aspect
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.2
    /// "Mind, Body, Spirit are different ASPECTS of the same underlying seed"
    pub fn as_body(&self) -> crate::holographic_complex::BodyAspect {
        self.holographic_complex.as_body()
    }

    /// Get Spirit aspect
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.2
    /// "Mind, Body, Spirit are different ASPECTS of the same underlying seed"
    pub fn as_spirit(&self) -> crate::holographic_complex::SpiritAspect {
        self.holographic_complex.as_spirit()
    }

    /// Check if aspects reference the same structure
    ///
    /// This is a key test for the aspect-based architecture.
    pub fn aspects_reference_same_structure(&self) -> bool {
        self.holographic_complex.aspects_reference_same_structure()
    }

    /// Update cross-coupling using holographic complex
    ///
    /// This demonstrates that coupling emerges from aspect activations.
    pub fn update_cross_coupling_from_aspects(&mut self) {
        self.holographic_complex.update_cross_coupling();

        // Sync with legacy coupling_dynamics
        let coupling = self.holographic_complex.get_cross_coupling();
        self.coupling_dynamics.mind_body = coupling.mind_body_coupling;
        self.coupling_dynamics.body_spirit = coupling.body_spirit_coupling;
        self.coupling_dynamics.mind_spirit = coupling.mind_spirit_coupling;
        self.coupling_dynamics.overall_balance = coupling.overall_intertwining;
    }

    // ============================================================================
    // END PHASE 1.1 METHODS
    // ============================================================================

    /// Process archetype state transition based on catalyst
    ///
    /// Knowledge Base Reference: Archetypes/1. Mind/A3. Catalyst of the Mind.json
    pub fn process_archetype_transition(
        &mut self,
        archetype_index: usize,
        catalyst: Catalyst,
    ) -> TransitionResult {
        if archetype_index >= 22 {
            return TransitionResult::NoChange;
        }

        let current_state = self.archetype_states[archetype_index];

        match current_state {
            ArchetypeState::Latent => {
                // Catalyst can trigger activation
                if catalyst.intensity > ACTIVATION_THRESHOLD {
                    self.archetype_states[archetype_index] = ArchetypeState::Activating;
                    // Sync with holographic complex
                    self.sync_holographic_complex();
                    TransitionResult::ActivationStarted
                } else {
                    TransitionResult::NoChange
                }
            }
            ArchetypeState::Activating => {
                // Continued catalyst completes activation
                if catalyst.intensity > COMPLETION_THRESHOLD {
                    self.archetype_states[archetype_index] = ArchetypeState::Active;
                    // Sync with holographic complex
                    self.sync_holographic_complex();
                    TransitionResult::ActivationComplete
                } else {
                    TransitionResult::StillActivating
                }
            }
            ArchetypeState::Active => {
                // Catalyst can cause distortion or blocking
                if catalyst.is_distorting() {
                    self.archetype_states[archetype_index] = ArchetypeState::Distorted;
                    self.sync_holographic_complex();
                    TransitionResult::Distorted
                } else if catalyst.is_blocking() {
                    self.archetype_states[archetype_index] = ArchetypeState::Blocked;
                    self.sync_holographic_complex();
                    TransitionResult::Blocked
                } else {
                    TransitionResult::NoChange
                }
            }
            ArchetypeState::Blocked => {
                // Catalyst can unblock if processed correctly
                if catalyst.is_healing() {
                    self.archetype_states[archetype_index] = ArchetypeState::Activating;
                    self.sync_holographic_complex();
                    TransitionResult::UnblockingStarted
                } else {
                    TransitionResult::StillBlocked
                }
            }
            ArchetypeState::Distorted => {
                // Catalyst can heal distortion
                if catalyst.is_healing() {
                    self.archetype_states[archetype_index] = ArchetypeState::Active;
                    self.sync_holographic_complex();
                    TransitionResult::DistortionHealed
                } else {
                    TransitionResult::StillDistorted
                }
            }
        }
    }

    /// Get archetype state
    pub fn get_archetype_state(&self, archetype_index: usize) -> Option<ArchetypeState> {
        if archetype_index < 22 {
            Some(self.archetype_states[archetype_index])
        } else {
            None
        }
    }

    /// Set archetype state directly
    pub fn set_archetype_state(&mut self, archetype_index: usize, state: ArchetypeState) -> bool {
        if archetype_index < 22 {
            self.archetype_states[archetype_index] = state;
            // Sync with holographic complex
            self.sync_holographic_complex();
            true
        } else {
            false
        }
    }

    /// Calculate overall archetype activation
    pub fn calculate_overall_activation(&self) -> Float {
        let total: Float = self
            .archetype_states
            .iter()
            .map(|state| state.activation_level())
            .sum();
        total / 22.0
    }

    /// Update coupling dynamics based on archetype states
    pub fn update_coupling(&mut self) {
        // Calculate coupling based on archetype activations in each complex
        let mind_activation: Float = self.archetype_states[0..7]
            .iter()
            .map(|state| state.activation_level())
            .sum::<Float>()
            / 7.0;

        let body_activation: Float = self.archetype_states[7..14]
            .iter()
            .map(|state| state.activation_level())
            .sum::<Float>()
            / 7.0;

        let spirit_activation: Float = self.archetype_states[14..21]
            .iter()
            .map(|state| state.activation_level())
            .sum::<Float>()
            / 7.0;

        // Update coupling based on activation balance
        self.coupling_dynamics.mind_body = 1.0 - (mind_activation - body_activation).abs();
        self.coupling_dynamics.body_spirit = 1.0 - (body_activation - spirit_activation).abs();
        self.coupling_dynamics.mind_spirit = 1.0 - (mind_activation - spirit_activation).abs();
        self.coupling_dynamics.calculate_balance();
    }

    /// Update vibrational state based on archetype activations
    pub fn update_vibration(&mut self) {
        // Calculate vibration for each complex
        self.vibrational_state.mind_vibration = self.archetype_states[0..7]
            .iter()
            .map(|state| state.activation_level())
            .sum::<Float>()
            / 7.0;

        self.vibrational_state.body_vibration = self.archetype_states[7..14]
            .iter()
            .map(|state| state.activation_level())
            .sum::<Float>()
            / 7.0;

        self.vibrational_state.spirit_vibration = self.archetype_states[14..21]
            .iter()
            .map(|state| state.activation_level())
            .sum::<Float>()
            / 7.0;

        self.vibrational_state.calculate_overall();
        self.vibrational_state.update_density();
    }

    /// Apply archetypical biases from soul stream
    ///
    /// (This will be implemented when SoulStream is added)
    pub fn apply_archetypical_biases(&mut self, _biases: &[Float; 22]) {
        // Bias affects initial activation level
        // (Implementation to be added when SoulStream is available)
    }

    /// Get coupling dynamics
    pub fn coupling_dynamics(&self) -> &CouplingDynamics {
        &self.coupling_dynamics
    }

    /// Get vibrational state
    pub fn vibrational_state(&self) -> &VibrationalState {
        &self.vibrational_state
    }

    /// Get the spirit channel
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.8
    /// "Spirit functions as a 'channel' that funnels inpourings into consciousness."
    pub fn spirit_channel(&self) -> &SpiritChannel {
        &self.spirit_channel
    }

    /// Get mutable reference to the spirit channel
    pub fn spirit_channel_mut(&mut self) -> &mut SpiritChannel {
        &mut self.spirit_channel
    }

    // ========== Phase 6: Evolution System Enhancement Methods ==========

    /// Process body catalyst (up-pouring)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "Body provides sensory input (Catalyst) to the Mind"
    pub fn process_body_catalyst(&mut self, catalyst: Catalyst) -> Catalyst {
        // Body receives raw catalyst and prepares it for mind processing
        // This is the "up-pouring" from the physical body

        // Filter catalyst through body complex
        let filtered_catalyst = if catalyst.catalyst_type == CatalystType::Body {
            // Body catalyst is processed directly
            catalyst
        } else {
            // Non-body catalyst is received through the body
            Catalyst {
                intensity: catalyst.intensity * 0.9, // Slight attenuation
                catalyst_type: CatalystType::Body,
                target_archetype: catalyst.target_archetype,
                is_distorting: catalyst.is_distorting,
                is_blocking: catalyst.is_blocking,
                is_healing: catalyst.is_healing,
            }
        };

        // Update body vibration based on catalyst intensity
        self.vibrational_state.body_vibration =
            (self.vibrational_state.body_vibration + filtered_catalyst.intensity * 0.01).min(1.0);

        filtered_catalyst
    }

    /// Process mind archetypes through Lesser Cycle
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "The Lesser Cycle: A1 → A2 → A3 → A4"
    pub fn process_mind_archetypes(
        &mut self,
        catalyst: Catalyst,
        _archetypes: &ArchetypicalMind,
    ) -> Experience {
        // Create Lesser Cycle for Mind complex
        let mut lesser_cycle = LesserCycle::new(ComplexType::Mind);

        // Process catalyst through the Lesser Cycle
        // Note: LesserCycle::process takes catalyst intensity as Float
        let result = lesser_cycle.process(catalyst.intensity);

        // Update archetype states based on cycle result
        if result.success {
            // Update mind vibration based on processing efficiency
            self.vibrational_state.mind_vibration =
                (self.vibrational_state.mind_vibration + result.efficiency * 0.05).min(1.0);

            // Update archetype states for archetypes involved in processing
            // (A1-A4 for Mind complex)
            for i in 0..4 {
                if catalyst.intensity > 0.3 {
                    let transition_catalyst = Catalyst {
                        intensity: catalyst.intensity * result.efficiency,
                        catalyst_type: CatalystType::Mind,
                        target_archetype: Some(i),
                        is_distorting: catalyst.is_distorting,
                        is_blocking: catalyst.is_blocking,
                        is_healing: catalyst.is_healing,
                    };
                    self.process_archetype_transition(i, transition_catalyst);
                }
            }

            Experience {
                catalyst_source: catalyst,
                wisdom_extracted: result.wisdom,
                integration_level: result.efficiency,
                complex_type: ComplexType::Mind,
            }
        } else {
            // Catalyst was blocked - accumulate as shadow
            // (Shadow handling to be implemented in Phase 6.3)
            Experience {
                catalyst_source: catalyst,
                wisdom_extracted: 0.0,
                integration_level: 0.0,
                complex_type: ComplexType::Mind,
            }
        }
    }

    /// Check if Mind is balanced (The Valve)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "Is the Mind transparent enough to let Spirit through?"
    pub fn is_mind_balanced(&self) -> bool {
        // Mind is balanced when:
        // 1. Coupling dynamics are balanced
        // 2. Mind vibration is high enough
        // 3. Archetype states are not distorted/blocked

        let coupling_balanced = self.coupling_dynamics.overall_balance >= 0.6;
        let mind_active = self.vibrational_state.mind_vibration >= 0.4;

        // Check if mind archetypes are not severely blocked
        let mind_archetypes_ok = self.archetype_states[0..7]
            .iter()
            .filter(|s| matches!(s, ArchetypeState::Blocked | ArchetypeState::Distorted))
            .count()
            < 3;

        coupling_balanced && mind_active && mind_archetypes_ok
    }

    /// Channel spirit in-pouring (down-pouring)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "The Spirit generates intelligent energy from source"
    ///
    /// PHASE 2.4: Now uses SpiritChannel to demonstrate Spirit as an active channel.
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.8
    /// "Spirit functions as a 'channel' that funnels inpourings into consciousness."
    pub fn channel_spirit_inpouring(&mut self) -> IntelligentEnergy {
        // Update spirit channel based on current spirit vibration
        // Only update if channel is not blocked (blocked state persists)
        if !self.spirit_channel.is_blocked() {
            self.spirit_channel.source_connection.connection_strength =
                self.vibrational_state.spirit_vibration;

            // Channel is open if spirit vibration is high enough
            if self.vibrational_state.spirit_vibration >= 0.3 {
                self.spirit_channel.open();
            } else {
                self.spirit_channel.close();
            }
        }

        // Funnel intelligent energy through the spirit channel
        let energy = self.spirit_channel.funnel_intelligent_energy();

        // Update spirit vibration
        self.vibrational_state.spirit_vibration =
            (self.vibrational_state.spirit_vibration + 0.01).min(1.0);

        energy
    }

    /// Check if entity can make a choice (Greater Cycle)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "Greater Cycle: Significator → Choice → Transformation"
    pub fn can_make_choice(&self) -> bool {
        // Entity can make choice when:
        // 1. Mind is balanced
        // 2. Spirit is accessible
        // 3. Free Will is active (Archetype 22)

        let mind_balanced = self.is_mind_balanced();
        let spirit_accessible = self.vibrational_state.spirit_vibration >= 0.3;

        // Check if Archetype 22 (The Choice) is active
        let free_will_active = matches!(
            self.archetype_states[21],
            ArchetypeState::Active | ArchetypeState::Activating
        );

        mind_balanced && spirit_accessible && free_will_active
    }

    /// Make a choice using Archetype 22 (The Choice)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "Greater Cycle: Significator → Choice → Transformation"
    pub fn make_choice(&mut self, free_will: &Archetype22) -> ArchetypeChoice {
        // Create choice context based on current state
        // Note: ChoiceContext is an enum in the archetypes module
        let context = ChoiceContext::CatalystProcessing;

        // Determine polarity based on current state
        let polarity = if self.coupling_dynamics.overall_balance > 0.5 {
            crate::types::Polarity::STO
        } else {
            crate::types::Polarity::STS
        };

        // Make choice based on free will capacity and polarization
        let choice = ArchetypeChoice {
            polarity,
            intensity: free_will.choice_capacity * 0.5,
            context,
        };

        // Update Archetype 22 state
        self.archetype_states[21] = ArchetypeState::Active;

        choice
    }

    /// Apply transformation to Great Way (Archetype 7)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "Greater Cycle: Significator → Choice → Transformation"
    pub fn apply_transformation(&mut self, choice: ArchetypeChoice) -> Float {
        // Apply transformation to A7 (Great Way)
        // Transformation intensity is based on choice intensity
        let transformation_intensity = choice.intensity * 0.8;

        // Update Archetype 7 state
        if transformation_intensity > 0.5 {
            self.archetype_states[6] = ArchetypeState::Active;
        }

        transformation_intensity
    }

    /// Modify Great Way based on transformation
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "Greater Cycle: Significator → Choice → Transformation"
    pub fn modify_great_way(&mut self, transformation: Float) {
        // Modify the environment container based on transformation
        // This affects how future catalysts are processed

        // Update coupling dynamics based on transformation
        self.coupling_dynamics.mind_spirit =
            (self.coupling_dynamics.mind_spirit + transformation * 0.1).min(1.0);
        self.coupling_dynamics.calculate_balance();
    }

    /// Accumulate unprocessed catalyst (shadow material)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "Catalyst accumulates (trauma) because processing is impaired"
    pub fn accumulate_unprocessed_catalyst(&mut self, experience: Experience) {
        // Accumulate as shadow material
        // (Shadow handling to be implemented in Phase 6.3)

        // For now, just increase blockage in relevant archetypes
        let archetype_index = match experience.complex_type {
            ComplexType::Mind => 2,    // A3 Catalyst
            ComplexType::Body => 9,    // A3 Catalyst (Body)
            ComplexType::Spirit => 16, // A3 Catalyst (Spirit)
        };

        if archetype_index < 22 {
            self.archetype_states[archetype_index] = ArchetypeState::Blocked;
        }
    }

    /// Accumulate shadow material (up-pouring that couldn't integrate)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.4
    /// "Up-pouring accumulates as unprocessed energy"
    pub fn accumulate_shadow(&mut self, _up_pouring: EnergyFlow) {
        // Shadow accumulation handling
        // (To be implemented in Phase 6.3)
    }

    /// Get valve state for Mind complex
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.4
    /// "Mind is the VALVE that regulates the flow between Body and Spirit"
    pub fn valve_state(&self) -> crate::energy_ray_centers::ValveState {
        use crate::energy_ray_centers::ValveState;

        if self.coupling_dynamics.overall_balance >= 0.7 {
            ValveState::Open
        } else if self.coupling_dynamics.overall_balance >= 0.4 {
            ValveState::Restricted
        } else {
            ValveState::Closed
        }
    }

    /// Get VibrationalStateRef for energy center activation
    pub fn vibrational_state_ref(&self) -> crate::energy_ray_centers::VibrationalStateRef {
        crate::energy_ray_centers::VibrationalStateRef {
            overall_level: self.vibrational_state.overall_level,
            mind_vibration: self.vibrational_state.mind_vibration,
            body_vibration: self.vibrational_state.body_vibration,
            spirit_vibration: self.vibrational_state.spirit_vibration,
            density_level: self.vibrational_state.density_level,
        }
    }

    /// Add distortion to entity state
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.3
    /// "This creates distortions if the foundation is not solid"
    pub fn add_distortion(&mut self, _target_center: usize, distortion: Float) {
        // Add distortion to archetype states
        for i in 0..22 {
            if matches!(self.archetype_states[i], ArchetypeState::Active) {
                // Chance to distort based on distortion level
                if distortion > 0.5 && (i % 3 == 0) {
                    self.archetype_states[i] = ArchetypeState::Distorted;
                }
            }
        }
    }

    /// Process Greater Cycle for transformation
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "Greater Cycle: Significator → Choice → Transformation"
    pub fn process_greater_cycle(
        &mut self,
        choice: ArchetypeChoice,
        _archetypes: &ArchetypicalMind,
    ) -> GreaterCycleResult {
        // Create Greater Cycle for Mind complex
        let mut greater_cycle = GreaterCycle::new(ComplexType::Mind);

        // Process choice through the Greater Cycle
        // Note: GreaterCycle::process takes experience intensity as Float
        let result = greater_cycle.process(choice.intensity);

        // Update archetype states based on cycle result
        if result.success {
            // Update A5 (Significator), A6 (Transformation), A7 (Great Way)
            for i in 4..7 {
                if result.transformation > 0.3 {
                    let transition_catalyst = Catalyst {
                        intensity: result.transformation,
                        catalyst_type: CatalystType::Mind,
                        target_archetype: Some(i),
                        is_distorting: false,
                        is_blocking: false,
                        is_healing: true,
                    };
                    self.process_archetype_transition(i, transition_catalyst);
                }
            }
        }

        result
    }
}

impl fmt::Display for EntityState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EntityState:\n\
             - Overall Activation: {:.2}\n\
             - Mind Activation: {:.2}\n\
             - Body Activation: {:.2}\n\
             - Spirit Activation: {:.2}\n\
             - Coupling Balance: {:.2}\n\
             - Vibrational Level: {:.2}\n\
             - Density Level: {}\n\
             - Unity Awareness: {:.2}\n\
             - Mind Balanced: {}\n\
             - Spirit Channel State: {:?}\n\
             - Spirit Channel Clear: {}\n\
             - Spirit Channel Throughput: {:.2}",
            self.calculate_overall_activation(),
            self.vibrational_state.mind_vibration,
            self.vibrational_state.body_vibration,
            self.vibrational_state.spirit_vibration,
            self.coupling_dynamics.overall_balance,
            self.vibrational_state.overall_level,
            self.vibrational_state.density_level,
            self.vibrational_state.unity_awareness,
            self.is_mind_balanced(),
            self.spirit_channel.channel_state(),
            self.spirit_channel.is_clear(),
            self.spirit_channel.effective_throughput()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_state_creation() {
        let state = EntityState::new();
        assert_eq!(state.calculate_overall_activation(), 0.0);
    }

    #[test]
    fn test_archetype_state_transitions() {
        let mut state = EntityState::new();

        // Test Latent -> Activating
        let catalyst = Catalyst::new(0.5, CatalystType::Mind);
        let result = state.process_archetype_transition(0, catalyst);
        assert_eq!(result, TransitionResult::ActivationStarted);
        assert_eq!(state.archetype_states[0], ArchetypeState::Activating);

        // Test Activating -> Active
        let catalyst = Catalyst::new(0.8, CatalystType::Mind);
        let result = state.process_archetype_transition(0, catalyst);
        assert_eq!(result, TransitionResult::ActivationComplete);
        assert_eq!(state.archetype_states[0], ArchetypeState::Active);
    }

    #[test]
    fn test_coupling_calculation() {
        let mut state = EntityState::new();

        // Activate some mind archetypes
        state.set_archetype_state(0, ArchetypeState::Active);
        state.set_archetype_state(1, ArchetypeState::Active);

        // Activate some body archetypes
        state.set_archetype_state(7, ArchetypeState::Active);
        state.set_archetype_state(8, ArchetypeState::Active);

        state.update_coupling();

        assert!(state.coupling_dynamics.mind_body > 0.5);
    }

    #[test]
    fn test_vibrational_state() {
        let mut state = EntityState::new();

        // Activate archetypes in all complexes
        state.set_archetype_state(0, ArchetypeState::Active);
        state.set_archetype_state(7, ArchetypeState::Active);
        state.set_archetype_state(14, ArchetypeState::Active);

        state.update_vibration();

        assert!(state.vibrational_state.mind_vibration > 0.0);
        assert!(state.vibrational_state.body_vibration > 0.0);
        assert!(state.vibrational_state.spirit_vibration > 0.0);
    }

    #[test]
    fn test_density_update() {
        let mut state = EntityState::new();

        // Activate enough archetypes to reach higher density
        for i in 0..21 {
            state.set_archetype_state(i, ArchetypeState::Active);
        }

        state.update_vibration();

        assert!(state.vibrational_state.density_level > 1);
    }

    #[test]
    fn test_distortion_and_blocking() {
        let mut state = EntityState::new();

        // Activate archetype
        state.set_archetype_state(0, ArchetypeState::Active);

        // Apply distorting catalyst
        let mut catalyst = Catalyst::new(0.5, CatalystType::Mind);
        catalyst.is_distorting = true;
        state.process_archetype_transition(0, catalyst);

        assert_eq!(state.archetype_states[0], ArchetypeState::Distorted);

        // Apply healing catalyst
        let mut catalyst = Catalyst::new(0.5, CatalystType::Mind);
        catalyst.is_healing = true;
        state.process_archetype_transition(0, catalyst);

        assert_eq!(state.archetype_states[0], ArchetypeState::Active);
    }

    // ============================================================================
    // PHASE 1.1: ASPECT-BASED ARCHITECTURE TESTS
    // ============================================================================

    #[test]
    fn test_aspects_reference_same_structure() {
        let state = EntityState::new();

        // Verify that all aspects reference the same underlying structure
        assert!(state.aspects_reference_same_structure());
    }

    #[test]
    fn test_mind_aspect() {
        let mut state = EntityState::new();

        // Activate a mind archetype (A1, index 0)
        state.set_archetype_state(0, ArchetypeState::Active);
        // Also set activation level
        state
            .holographic_complex_mut()
            .unified_structure_mut()
            .set_activation(0, 1.0);

        // Get mind aspect
        let mind = state.as_mind();

        // Verify that mind aspect sees the activation
        assert_eq!(mind.get_state(0), Some(ArchetypeState::Active));
        assert!((mind.calculate_overall_activation() - 1.0 / 7.0).abs() < 0.0001);
    }

    #[test]
    fn test_body_aspect() {
        let mut state = EntityState::new();

        // Activate a body archetype (A8, index 7)
        state.set_archetype_state(7, ArchetypeState::Active);
        // Also set activation level
        state
            .holographic_complex_mut()
            .unified_structure_mut()
            .set_activation(7, 1.0);

        // Get body aspect
        let body = state.as_body();

        // Verify that body aspect sees the activation
        assert_eq!(body.get_state(0), Some(ArchetypeState::Active));
        assert!((body.calculate_overall_activation() - 1.0 / 7.0).abs() < 0.0001);
    }

    #[test]
    fn test_spirit_aspect() {
        let mut state = EntityState::new();

        // Activate a spirit archetype (A15, index 14)
        state.set_archetype_state(14, ArchetypeState::Active);
        // Also set activation level
        state
            .holographic_complex_mut()
            .unified_structure_mut()
            .set_activation(14, 1.0);

        // Get spirit aspect
        let spirit = state.as_spirit();

        // Verify that spirit aspect sees the activation
        assert_eq!(spirit.get_state(0), Some(ArchetypeState::Active));
        assert!((spirit.calculate_overall_activation() - 1.0 / 7.0).abs() < 0.0001);
    }

    #[test]
    fn test_cross_coupling_from_aspects() {
        let mut state = EntityState::new();

        // Activate archetypes in all complexes
        state.set_archetype_state(0, ArchetypeState::Active); // Mind A1
        state.set_archetype_state(7, ArchetypeState::Active); // Body A8
        state.set_archetype_state(14, ArchetypeState::Active); // Spirit A15

        // Set activation levels
        state
            .holographic_complex_mut()
            .unified_structure_mut()
            .set_activation(0, 1.0);
        state
            .holographic_complex_mut()
            .unified_structure_mut()
            .set_activation(7, 1.0);
        state
            .holographic_complex_mut()
            .unified_structure_mut()
            .set_activation(14, 1.0);

        // Update cross-coupling from aspects
        state.update_cross_coupling_from_aspects();

        // Verify that coupling is high (balanced activations)
        assert!(state.coupling_dynamics.mind_body > 0.9);
        assert!(state.coupling_dynamics.body_spirit > 0.9);
        assert!(state.coupling_dynamics.mind_spirit > 0.9);
        assert!(state.coupling_dynamics.overall_balance > 0.9);
    }

    #[test]
    fn test_inextricable_intertwining() {
        let mut state = EntityState::new();

        // Activate archetypes in all complexes
        state.set_archetype_state(0, ArchetypeState::Active);
        state.set_archetype_state(7, ArchetypeState::Active);
        state.set_archetype_state(14, ArchetypeState::Active);

        // Set activation levels
        state
            .holographic_complex_mut()
            .unified_structure_mut()
            .set_activation(0, 1.0);
        state
            .holographic_complex_mut()
            .unified_structure_mut()
            .set_activation(7, 1.0);
        state
            .holographic_complex_mut()
            .unified_structure_mut()
            .set_activation(14, 1.0);

        state.update_cross_coupling_from_aspects();

        // Verify inextricable intertwining
        assert!(state
            .holographic_complex()
            .get_cross_coupling()
            .is_inextricably_intertwined());
    }

    #[test]
    fn test_backward_compatibility() {
        let mut state = EntityState::new();

        // Use old API
        state.set_archetype_state(0, ArchetypeState::Active);
        state.set_archetype_state(7, ArchetypeState::Active);
        state.set_archetype_state(14, ArchetypeState::Active);

        // Verify old API still works
        assert_eq!(state.archetype_states[0], ArchetypeState::Active);
        assert_eq!(state.archetype_states[7], ArchetypeState::Active);
        assert_eq!(state.archetype_states[14], ArchetypeState::Active);

        // Verify new API sees the same data
        assert_eq!(state.as_mind().get_state(0), Some(ArchetypeState::Active));
        assert_eq!(state.as_body().get_state(0), Some(ArchetypeState::Active));
        assert_eq!(state.as_spirit().get_state(0), Some(ArchetypeState::Active));
    }

    #[test]
    fn test_sync_archetype_states() {
        let mut state = EntityState::new();

        // Modify holographic complex directly
        {
            let structure = state.holographic_complex_mut().unified_structure_mut();
            structure.set_state(0, ArchetypeState::Active);
            structure.set_activation(0, 1.0);
        }

        // Sync with archetype_states
        state.sync_archetype_states();

        // Verify sync worked
        assert_eq!(state.archetype_states[0], ArchetypeState::Active);
    }

    #[test]
    fn test_sync_holographic_complex() {
        let mut state = EntityState::new();

        // Modify archetype_states directly
        state.archetype_states[0] = ArchetypeState::Active;
        state.archetype_states[7] = ArchetypeState::Active;
        state.archetype_states[14] = ArchetypeState::Active;

        // Sync with holographic complex
        state.sync_holographic_complex();

        // Verify sync worked
        assert_eq!(
            state.holographic_complex().unified_structure().get_state(0),
            Some(ArchetypeState::Active)
        );
        assert_eq!(
            state.holographic_complex().unified_structure().get_state(7),
            Some(ArchetypeState::Active)
        );
        assert_eq!(
            state
                .holographic_complex()
                .unified_structure()
                .get_state(14),
            Some(ArchetypeState::Active)
        );
    }

    #[test]
    fn test_aspect_activation_affects_coupling() {
        let mut state = EntityState::new();

        // Activate mind archetype
        state.set_archetype_state(0, ArchetypeState::Active);
        state
            .holographic_complex_mut()
            .unified_structure_mut()
            .set_activation(0, 1.0);
        state.update_cross_coupling_from_aspects();

        // Mind activation should affect coupling
        let coupling_before = state.coupling_dynamics.mind_body;

        // Activate body archetype
        state.set_archetype_state(7, ArchetypeState::Active);
        state
            .holographic_complex_mut()
            .unified_structure_mut()
            .set_activation(7, 1.0);
        state.update_cross_coupling_from_aspects();

        let coupling_after = state.coupling_dynamics.mind_body;

        // Coupling should increase (more balanced)
        assert!(coupling_after > coupling_before);
    }

    #[test]
    fn test_holographic_complex_creation() {
        let state = EntityState::new();

        // Verify holographic complex exists
        assert!(state.aspects_reference_same_structure());

        // Verify cross-coupling is initialized
        assert_eq!(
            state
                .holographic_complex()
                .get_cross_coupling()
                .overall_intertwining,
            0.5
        );
    }

    // ============================================================================
    // END PHASE 1.1 TESTS
    // ============================================================================

    // ============================================================================
    // PHASE 2.4: SPIRIT AS CHANNEL TESTS
    // ============================================================================

    #[test]
    fn test_spirit_channel_integration() {
        let state = EntityState::new();

        // Verify that spirit channel exists
        assert!(!state.spirit_channel.is_connected());
        assert_eq!(
            state.spirit_channel.channel_state(),
            crate::spirit_channel::ChannelState::Closed
        );
    }

    #[test]
    fn test_spirit_channel_getters() {
        let state = EntityState::new();

        // Get spirit channel
        let channel = state.spirit_channel();

        // Verify getters work
        assert!(!channel.is_connected());
        assert_eq!(
            channel.channel_state(),
            crate::spirit_channel::ChannelState::Closed
        );
    }

    #[test]
    fn test_spirit_channel_mutability() {
        let mut state = EntityState::new();

        // Modify spirit channel
        state.spirit_channel_mut().establish_connection();
        state.spirit_channel_mut().open();

        // Verify changes
        assert!(state.spirit_channel.is_connected());
        assert!(state.spirit_channel.is_open());
    }

    #[test]
    fn test_channel_spirit_inpouring_uses_spirit_channel() {
        let mut state = EntityState::new();

        // Set spirit vibration high enough to open channel
        state.vibrational_state.spirit_vibration = 0.5;

        // Establish connection to source
        state.spirit_channel_mut().establish_connection();

        // Channel spirit inpouring
        let energy = state.channel_spirit_inpouring();

        // Verify energy is funneled
        assert!(energy.intensity > 0.0);
        assert_eq!(energy.source, EnergySource::SpiritComplex);

        // Verify spirit channel is updated
        assert!(state.spirit_channel.is_open());
    }

    #[test]
    fn test_channel_spirit_inpouring_low_vibration() {
        let mut state = EntityState::new();

        // Set spirit vibration low (channel stays closed)
        state.vibrational_state.spirit_vibration = 0.2;

        // Channel spirit inpouring
        let energy = state.channel_spirit_inpouring();

        // Verify no energy is funneled (channel is closed)
        assert_eq!(energy.intensity, 0.0);

        // Verify spirit channel is closed
        assert!(!state.spirit_channel.is_clear());
    }

    #[test]
    fn test_channel_spirit_inpouring_high_vibration() {
        let mut state = EntityState::new();

        // Set spirit vibration high (channel opens)
        state.vibrational_state.spirit_vibration = 0.8;

        // Establish connection to source
        state.spirit_channel_mut().establish_connection();

        // Channel spirit inpouring
        let energy = state.channel_spirit_inpouring();

        // Verify energy is funneled
        assert!(energy.intensity > 0.0);

        // Verify spirit channel is open
        assert!(state.spirit_channel.is_open());
        assert!(state.spirit_channel.is_clear());
    }

    #[test]
    fn test_spirit_channel_affects_inpouring() {
        let mut state = EntityState::new();

        // Set spirit vibration high
        state.vibrational_state.spirit_vibration = 0.5;

        // Channel spirit inpouring with open channel
        state.spirit_channel_mut().establish_connection();
        state.spirit_channel_mut().open();
        state.spirit_channel_mut().strengthen_connection(0.9); // Strengthen connection

        // First inpouring (open channel)
        let energy_open = state.channel_spirit_inpouring();

        // Now block the channel
        state.spirit_channel_mut().block();
        let energy_blocked = state.channel_spirit_inpouring();

        // Verify blocked channel produces less energy
        assert!(energy_open.intensity > energy_blocked.intensity);
        assert_eq!(energy_blocked.intensity, 0.0); // Blocked produces no energy
    }

    #[test]
    fn test_spirit_channel_in_state_display() {
        let mut state = EntityState::new();

        // Set up spirit channel
        state.spirit_channel_mut().establish_connection();
        state.spirit_channel_mut().open();

        // Get display string
        let display = format!("{}", state);

        // Verify spirit channel info is in display
        assert!(display.contains("Spirit Channel State"));
        assert!(display.contains("Spirit Channel Clear"));
        assert!(display.contains("Spirit Channel Throughput"));
    }

    // ============================================================================
    // END PHASE 2.4 TESTS
    // ============================================================================
}
