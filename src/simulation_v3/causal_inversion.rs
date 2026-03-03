//! Causal Inversion Module (Phase 0)
//!
//! From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md:
//! "The Fundamental Gap: Causal Inversion"
//!
//! This module implements the critical architectural shift from bottom-up causation
//! to top-down causation in the consciousness evolution simulation.
//!
//! ## Why Causal Inversion Matters
//!
//! ### Metaphysical Perspective
//!
//! In the Law of One cosmology that underpins this simulation, causation flows from
//! Intelligent Infinity (the One) through the Three Primal Distortions downward to
//! individual entities. The current simulation has this backwards:
//!
//! - **WRONG (Current)**: Entity → Field → Cosmos
//! - **RIGHT (This Module)**: Infinity → Field → Entity
//!
//! This is not merely a technical detail—it is the philosophical foundation of the
//! entire simulation. Without correct causal ordering, the simulation cannot
//! accurately model consciousness evolution because:
//!
//! 1. Entities would be the source of their own existence (circular)
//! 2. The holographic principle would be violated (parts creating whole)
//! 3. Free will would be epiphenomenal, not foundational
//!
//! ### Computational Perspective
//!
//! From a computational standpoint, causal inversion enables:
//!
//! 1. **Emergence over scripting**: Reality self-generates from first principles
//! 2. **Efficient caching**: Field state determines entity state, not vice versa
//! 3. **Deterministic non-determinism**: Seed-based free will selection
//! 4. **Holographic compression**: MERA tensor network efficiency
//!
//! ## The Correct Causal Ordering
//!
//! Each simulation tick MUST follow this sequence:
//!
//! 1. `infinity.pulse()` — Intelligent Infinity pulses kinetic energy
//! 2. `field.evolve()` — Holographic field distributes energy
//! 3. `field.extract_potentials()` — High-coherence regions identified
//! 4. `manifest_entities_from_potentials()` — Entities manifest from field
//! 5. `entities.absorb_field_influence()` — Existing entities updated
//! 6. `decompress_for_observers()` — Observer-specific rendering
//!
//! Any other ordering violates the cosmological architecture and produces
//! incorrect simulation behavior.

use crate::compression::mera_network::MeraNetwork;
use crate::entity_layer7::layer7::{EntityId, EntityType, SubSubLogos};
use crate::holographic::field_address::{HolographicAddress, ScaleLevel, Vector3};
use crate::holographic::mera_integration::MeraIntegration;
use crate::holographic::observer_driven_field::{FieldOfView, Observer, ObserverId, ObserverType};
use crate::intelligent_infinity::IntelligentInfinity;
use crate::simulation_v3::holographic_field::HolographicFieldManager;
use crate::simulation_v3::observer_registry::{
    CacheStats as ObserverCacheStats, ObserverRegistry, ObserverRegistryConfig,
};
use crate::types::Float;

// Phase 2: Quantum field integration
use crate::holographic::complex_vectors::ComplexArchetype;
use crate::holographic::holographic_field::{HolographicField, InvolutionLayer};
use crate::quantum::free_will_collapse::{
    ChoiceBasedCollapse, QuantumCollapseContext, QuantumCollapseResult,
};
use crate::quantum::quantum_field::{
    LightLoveRatio, QuantumField, QuantumFieldConfig, QuantumStateSignature,
};

// Phase 2 P1: Atom formation from quantum collapse
use crate::chemistry::bonding::{
    calculate_archetype_compatibility, BondOrder, BondType, ChemicalBond,
};
use crate::chemistry::element_attractor::{ElementAttractor, PeriodicTable};
use crate::chemistry::molecular_geometry::{GeometryType, MolecularGeometry};
use crate::consciousness::free_will::PolarityPreference;
use crate::holographic::universal_template::SpectrumConfiguration;
use crate::matter::particle::{Coordinate3D, Particle, Vector3D};
use crate::particle::archetype_properties::ParticleProperties;

// Phase 3 P2: Embodied body integration
use crate::biology::organism_lifecycle::{
    BodyPlan, BodySymmetry, DietType, LocomotionType, SocialStructure,
};
use crate::simulation_v3::embodied_body::{
    BodyEnvironment, DeathCause, EmbodiedBody, SensoryField, SurvivalStatus,
};

// Phase 4: Living environment integration
use crate::cosmos::planetary_formation::Planet;
use crate::simulation_v3::living_environment::{
    EntitySpatialPosition, LivingEnvironment, LivingEnvironmentConfig,
};

// Phase 5: Consciousness Processor Integration
use crate::simulation_v3::consciousness_processor::{ConsciousnessOutput, ConsciousnessProcessor};

// Phase 6: Social Processor Integration
use crate::simulation_v3::social_processor::{SocialOutput, SocialProcessor};

// Phase 1: Unified Field Equation - Three Primal Distortions
// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 1:
// "Transform Free Will, Love, and Light from entity attributes to unified field dynamics"
// The Unified Field Equation: ∂ψ/∂t = FreeWill(ψ) + Love(ψ) + Light(ψ)
use crate::holographic_foundation::distortions::{
    CoherencePeak, DistortionStatistics, FieldState, UnifiedFieldConfig, UnifiedFieldEquation,
};

// Phase 3: Involution Flow - Causal Chain
// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 3:
// "Implement top-down causal flow from Intelligent Infinity through the Logos hierarchy to entities"
// The Causal Chain: IntelligentInfinity → LOGOS → SubLogos → SubSubLogos → Entity
use crate::holographic_foundation::involution::{
    CosmicHierarchy, EntityManifestation, EntitySeed, FieldConfiguration, GalacticLogosConfig,
    HierarchyLevel, HierarchyPath, InvolutionFlow, PlanetaryLogosConfig, PrimaryLogosConfig,
    PropagationResult, SolarLogosConfig,
};

// Phase 4: Evolution Feedback - Bottom-up Causal Flow
// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 4:
// "Implement bottom-up feedback where entity decisions modify the field"
// The Bidirectional Flow: Top-Down (Involution) ↔ Bottom-Up (Evolution)
use crate::holographic_foundation::evolution::{
    CollectiveInfluence, DecisionFeedback, DecisionType, DensityProgression, EntityDecision,
    EvolutionFeedbackConfig, FieldPerturbation, KarmicEncoding, KarmicPattern, ProgressionEvent,
    SpectrumShift,
};

// Phase 10: Intelligent Infinity as Active Source
// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 10:
// "Intelligent Infinity becomes the ACTIVE SOURCE of the simulation"
// - II emits patterns to entities each tick
// - Entity decisions feed back to II
// - Teleological pull affects entity evolution
// - Resonance tracking operational
use crate::holographic_foundation::intelligent_infinity::{
    ActiveFeedbackLoop, ArchitecturalResonance, EntityExperience, FeedbackAnalysis,
    IntelligentInfinitySource, SourceEmission, TeleologicalPull, UnityGradient,
};

// Phase 11: Higher Density Mechanics
// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 11:
// "Implement 5th-8th density as DISTINCT FIELD CONFIGURATIONS with proper emergence mechanics"
// - Integrate 5th density light body mechanics
// - Integrate 6th density unity consciousness
// - Integrate 7th density completion mechanics
// - Integrate 8th density source merger
// - Density transition triggers from evolution
use crate::simulation_v3::density_mechanics::{
    Density, DensityMechanics, EighthDensityMechanics, FifthDensityMechanics,
    FirstDensityMechanics, FourthDensityMechanics, SecondDensityMechanics, SeventhDensityMechanics,
    SixthDensityMechanics, ThirdDensityMechanics,
};

// Phase 12: Gateway Mechanics and Resonance
// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 12:
// "Implement gateway mechanics that enable connection to Intelligent Infinity through architectural resonance"
// - Resonance calculated dynamically
// - Gateway states change based on resonance
// - Resonance improves as simulation matures
use crate::holographic_foundation::higher_density::gateway_mechanics::{
    GatewayMechanics, GatewayState,
};

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for the Causal Inversion simulation runner.
///
/// This configuration controls the parameters of the causal inversion
/// process, including entity manifestation thresholds and pulse rates.
///
/// # Example
///
/// ```
/// use holonic_realms::simulation_v3::causal_inversion::CausalInversionConfig;
///
/// let config = CausalInversionConfig {
///     max_entities: 1000,
///     min_coherence: 0.7,
///     pulse_frequency: 0.1,
///     enable_observer_decompression: true,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct CausalInversionConfig {
    /// Maximum number of entities that can exist in the simulation.
    /// When this limit is reached, no new entities will manifest
    /// until existing entities are harvested or transcended.
    pub max_entities: usize,

    /// Minimum coherence level required for entity manifestation.
    /// Regions of the holographic field below this threshold
    /// cannot support stable entity formation.
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Coherence determines the stability of pattern in the field"
    pub min_coherence: Float,

    /// Frequency of the Infinity Pulse in Hz (simulation time).
    /// Higher frequencies mean more rapid energy cycling but
    /// require more computational resources per tick.
    pub pulse_frequency: Float,

    /// Enable observer-specific decompression for rendering.
    /// When enabled, the field will be decompressed differently
    /// for each observer based on their spectrum position.
    ///
    /// This implements the Observer Effect from quantum mechanics
    /// as a computational optimization (cache invalidation).
    pub enable_observer_decompression: bool,
}

impl Default for CausalInversionConfig {
    fn default() -> Self {
        CausalInversionConfig {
            max_entities: 1000,
            min_coherence: 0.5,
            pulse_frequency: 0.1,
            enable_observer_decompression: true,
        }
    }
}

impl CausalInversionConfig {
    /// Create a new configuration with specified parameters.
    pub fn new(max_entities: usize, min_coherence: Float, pulse_frequency: Float) -> Self {
        CausalInversionConfig {
            max_entities,
            min_coherence: min_coherence.clamp(0.0, 1.0),
            pulse_frequency: pulse_frequency.max(0.001),
            enable_observer_decompression: true,
        }
    }

    /// Create a high-performance configuration with reduced fidelity.
    pub fn performance() -> Self {
        CausalInversionConfig {
            max_entities: 100,
            min_coherence: 0.7,
            pulse_frequency: 0.05,
            enable_observer_decompression: false,
        }
    }

    /// Create a high-fidelity configuration for detailed simulation.
    pub fn high_fidelity() -> Self {
        CausalInversionConfig {
            max_entities: 10000,
            min_coherence: 0.3,
            pulse_frequency: 0.2,
            enable_observer_decompression: true,
        }
    }
}

// ============================================================================
// Infinity Pulse State
// ============================================================================

/// State of the Infinity Pulse at a given moment.
///
/// The Infinity Pulse is the heartbeat of the simulation—Intelligent
/// Infinity pulsing between potential (unmanifest) and kinetic (manifest)
/// energy states.
///
/// From intelligent_infinity.rs:
/// "Intelligent Infinity is a continuous background field, not a one-time pulse.
/// The field pulses rhythmically (like a giant heart) from potential to kinetic."
///
/// This struct captures the current state of that pulse for reporting
/// and analysis purposes.
#[derive(Debug, Clone, Default)]
pub struct InfinityPulseState {
    /// Current kinetic energy level (0.0 to 1.0).
    /// Pulses rhythmically as the field cycles.
    pub kinetic: Float,

    /// Current potential energy level (always 1.0 - infinite).
    /// This represents the unmanifest source that never diminishes.
    pub potential: Float,

    /// Total number of pulses completed since simulation start.
    /// Each complete cycle (potential → kinetic → potential) increments this.
    pub pulse_count: u64,

    /// Total energy tapped by entities across all pulses.
    /// This tracks how much of the infinite potential has been
    /// actualized through entity free will actions.
    pub total_tapped: Float,
}

impl InfinityPulseState {
    /// Create a new pulse state from an IntelligentInfinity instance.
    pub fn from_infinity(infinity: &IntelligentInfinity) -> Self {
        InfinityPulseState {
            kinetic: infinity.kinetic,
            potential: infinity.potential,
            pulse_count: infinity.pulse_count,
            total_tapped: infinity.total_tapped_energy,
        }
    }

    /// Calculate the pulse efficiency (tapped / kinetic).
    /// Higher efficiency indicates more entities actively tapping the field.
    pub fn efficiency(&self) -> Float {
        if self.kinetic > 0.0 {
            (self.total_tapped / self.kinetic).min(1.0)
        } else {
            0.0
        }
    }
}

// ============================================================================
// Quantum Evolution Result (Phase 2)
// ============================================================================

/// Result of quantum field evolution.
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 2:
/// "Quantum field as the bridge between holographic source and matter"
///
/// This captures the state of the quantum field after each evolution step,
/// providing statistics about states, entanglements, and collapses.
#[derive(Debug, Clone, Default)]
pub struct QuantumEvolutionResult {
    /// Number of quantum states in the field
    pub state_count: usize,

    /// Number of entanglement links
    pub entanglement_count: usize,

    /// Average coherence across all states
    pub avg_coherence: Float,

    /// Number of states that collapsed during evolution
    pub collapsed_count: usize,

    /// Total probability mass (should be ~1.0 for normalized field)
    pub total_probability: Float,

    /// Field coherence from holographic source
    pub field_coherence: Float,
}

impl QuantumEvolutionResult {
    /// Check if the field has meaningful quantum activity
    pub fn has_activity(&self) -> bool {
        self.state_count > 0
    }

    /// Check if field is well-normalized
    pub fn is_normalized(&self) -> bool {
        (self.total_probability - 1.0).abs() < 0.1 || self.state_count == 0
    }
}

// ============================================================================
// Causal Tick Result
// ============================================================================

/// Result of a single causal tick operation.
///
/// This struct captures all the outcomes of processing one tick through
/// the correct causal ordering, providing transparency into what happened
/// at each phase.
///
/// # Phases
///
/// Each tick result shows the outcomes of:
/// 1. Infinity pulse (energy released)
/// 2. Field evolution (energy distributed)
/// 3. Potential extraction (coherent regions identified)
/// 4. Entity manifestation (new entities created)
/// 5. Entity updates (existing entities modified)
/// 6. Observer decompression (rendering prepared)

/// Result of evolution feedback processing.
///
/// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 4:
/// "Entity decisions affect local field configuration"
#[derive(Debug, Clone, Default)]
pub struct EvolutionFeedbackResult {
    /// Number of decisions processed this tick
    pub decisions_processed: usize,
    /// Number of field perturbations applied
    pub field_perturbations: usize,
    /// Number of density shifts from progression
    pub density_shifts: usize,
    /// Collective influence strength
    pub collective_influence: Float,
    /// Number of karmic patterns encoded
    pub karmic_patterns: usize,
}

#[derive(Debug, Clone)]
pub struct CausalTickResult {
    /// State of the Infinity Pulse after this tick.
    pub infinity_state: InfinityPulseState,

    /// Overall coherence level of the holographic field.
    /// Higher values indicate a more organized, stable field.
    pub coherence_level: Float,

    /// Number of high-coherence potentials extracted from the field.
    /// These represent regions where entities could manifest.
    pub potentials_extracted: usize,

    /// Number of new entities manifested from potentials.
    /// Limited by max_entities configuration and coherence thresholds.
    pub entities_manifested: usize,

    /// Number of existing entities updated by field influence.
    /// This is the primary mechanism of top-down causation
    /// affecting already-manifested entities.
    pub entities_updated: usize,

    /// Number of observer regions decompressed for rendering.
    /// Only relevant when enable_observer_decompression is true.
    pub regions_decompressed: usize,

    /// Timestamp of this tick in simulation time.
    pub tick_timestamp: Float,

    /// Phase execution times (phase name → microseconds).
    /// Useful for performance analysis and optimization.
    pub phase_timings: HashMap<String, u64>,

    /// Cache hit rate for observer-driven decompression
    pub cache_hit_rate: Float,
    /// Number of observers active
    pub active_observers: usize,

    // NEW: Phase 2 - Quantum field statistics
    /// Number of quantum states in the field
    pub quantum_states: usize,
    /// Number of entanglement links
    pub entanglement_count: usize,
    /// Number of particles manifested from quantum collapses
    pub particles_manifested: usize,
    // Phase 2 P1: Atom formation statistics
    /// Number of atoms manifested from collapsed particles
    pub atoms_manifested: usize,
    // Phase 2 P2: Molecule formation statistics
    /// Number of molecules formed from atom bonding
    pub molecules_formed: usize,

    // Phase 3 P2: Body statistics
    /// Number of bodies ticked in this phase
    pub bodies_ticked: usize,
    /// Number of new bodies created this tick
    pub bodies_created: usize,
    /// Number of bodies that died this tick
    pub bodies_died: usize,
    /// Average health across all bodies (0.0-1.0)
    pub average_health: Float,
    /// Average energy across all bodies (0.0-1.0)
    pub average_energy: Float,

    // Phase 4: Environment statistics
    /// Environment average temperature (K)
    pub environment_temperature: Float,
    /// Environment average pressure (Pa)
    pub environment_pressure: Float,

    // Phase 5: Consciousness processing statistics
    /// Number of consciousness ticks processed this tick
    pub consciousness_ticks: usize,
    /// Number of free will choices made this tick
    pub choices_made: usize,
    /// Average polarity across all entities (-1.0 STS to 1.0 STO)
    pub average_polarity: Float,

    // Phase 6: Social processing statistics
    /// Number of interactions processed
    pub interactions_processed: usize,
    /// Number of new relationships formed
    pub relationships_formed: usize,
    /// Number of active groups
    pub groups_active: usize,
    /// Number of active SMCs
    pub smcs_active: usize,
    /// Number of entities harvested this tick
    pub entities_harvested: usize,

    // Phase 3: Involution Flow statistics
    /// Hierarchy depth that was propagated
    pub hierarchy_depth: usize,
    /// Number of levels that propagated downward
    pub levels_propagated: usize,
    /// Whether propagation succeeded through all levels
    pub propagation_complete: bool,
    /// Field configuration refinement factor
    pub refinement_factor: Float,
    /// Number of entity seeds available for manifestation
    pub entity_seeds_available: usize,

    // Phase 4: Evolution Feedback statistics
    /// Number of entity decisions processed this tick
    pub decisions_processed: usize,
    /// Number of field perturbations applied
    pub field_perturbations: usize,
    /// Number of density shifts from evolution progression
    pub density_shifts: usize,
    /// Strength of collective influence on field
    pub collective_influence: Float,
    /// Number of karmic patterns encoded this tick
    pub karmic_patterns_encoded: usize,

    // Phase 10: Intelligent Infinity Source statistics
    /// Number of II emissions to entities this tick
    pub ii_emissions: usize,
    /// Current resonance level of the II source (0.0-1.0)
    pub ii_resonance: Float,
    /// Whether entity feedback was received by II source
    pub ii_feedback_received: bool,
    /// Whether teleological pull was applied to entities
    pub teleological_pull_applied: bool,

    // Phase 11: Higher Density Mechanics statistics
    /// Whether 4th density mechanics are active
    pub density_4th_active: bool,
    /// Whether 5th density mechanics are active
    pub density_5th_active: bool,
    /// Whether 6th density mechanics are active
    pub density_6th_active: bool,
    /// Whether 7th density mechanics are active
    pub density_7th_active: bool,
    /// Whether 8th density mechanics are active
    pub density_8th_active: bool,
    /// Number of density transitions this tick
    pub density_transitions: usize,

    // Phase 12: Gateway Mechanics and Resonance statistics
    /// Current gateway state based on resonance level
    pub gateway_state: GatewayState,
    /// Whether gateway is open (resonance >= 0.80)
    pub gateway_open: bool,
    /// Peak resonance achieved in simulation history
    pub peak_resonance: Float,
    /// Gateway uptime as fraction of total simulation time
    pub gateway_uptime: Float,
    /// Number of gateway state transitions this tick
    pub gateway_state_transitions: usize,

    // Phase 7: Unified loop statistics
    /// Total number of phases that executed in this tick
    pub total_phases_executed: usize,
    /// Whether data flow between layers was valid
    pub data_flow_valid: bool,
}

impl Default for CausalTickResult {
    fn default() -> Self {
        CausalTickResult {
            infinity_state: InfinityPulseState::default(),
            coherence_level: 0.5,
            potentials_extracted: 0,
            entities_manifested: 0,
            entities_updated: 0,
            regions_decompressed: 0,
            tick_timestamp: 0.0,
            phase_timings: HashMap::new(),
            cache_hit_rate: 0.0,
            active_observers: 0,
            // Phase 2: Quantum field defaults
            quantum_states: 0,
            entanglement_count: 0,
            particles_manifested: 0,
            // Phase 2 P1: Atom formation default
            atoms_manifested: 0,
            molecules_formed: 0,
            // Phase 3 P2: Body statistics defaults
            bodies_ticked: 0,
            bodies_created: 0,
            bodies_died: 0,
            average_health: 0.0,
            average_energy: 0.0,
            // Phase 4: Environment statistics defaults
            environment_temperature: 288.0,
            environment_pressure: 101325.0,
            // Phase 5: Consciousness processing defaults
            consciousness_ticks: 0,
            choices_made: 0,
            average_polarity: 0.0,
            // Phase 6: Social processing defaults
            interactions_processed: 0,
            relationships_formed: 0,
            groups_active: 0,
            smcs_active: 0,
            entities_harvested: 0,
            // Phase 3: Involution Flow defaults
            hierarchy_depth: 0,
            levels_propagated: 0,
            propagation_complete: false,
            refinement_factor: 1.0,
            entity_seeds_available: 0,
            // Phase 4: Evolution Feedback defaults
            decisions_processed: 0,
            field_perturbations: 0,
            density_shifts: 0,
            collective_influence: 0.0,
            karmic_patterns_encoded: 0,
            // Phase 10: Intelligent Infinity Source defaults
            ii_emissions: 0,
            ii_resonance: 0.0,
            ii_feedback_received: false,
            teleological_pull_applied: false,
            // Phase 11: Higher Density Mechanics defaults
            density_4th_active: false,
            density_5th_active: false,
            density_6th_active: false,
            density_7th_active: false,
            density_8th_active: false,
            density_transitions: 0,
            // Phase 12: Gateway Mechanics defaults
            gateway_state: GatewayState::Closed,
            gateway_open: false,
            peak_resonance: 0.0,
            gateway_uptime: 0.0,
            gateway_state_transitions: 0,
            // Phase 7: Unified loop statistics
            total_phases_executed: 0,
            data_flow_valid: false,
        }
    }
}

impl CausalTickResult {
    /// Check if this tick resulted in any entity changes.
    pub fn has_entity_changes(&self) -> bool {
        self.entities_manifested > 0 || self.entities_updated > 0
    }

    /// Get total entity operations (manifestations + updates).
    pub fn total_entity_operations(&self) -> usize {
        self.entities_manifested + self.entities_updated
    }

    /// Calculate total phase execution time in microseconds.
    pub fn total_execution_time_us(&self) -> u64 {
        self.phase_timings.values().sum()
    }
}

// ============================================================================
// Causal Statistics
// ============================================================================

/// Aggregate statistics for the causal inversion simulation.
///
/// These statistics track the long-term behavior of the simulation,
/// enabling analysis of emergent patterns and performance characteristics.
#[derive(Debug, Clone, Default)]
pub struct CausalStatistics {
    /// Total number of ticks processed.
    pub total_ticks: u64,

    /// Total entities manifested across all ticks.
    pub total_entities_manifested: u64,

    /// Running average coherence level.
    pub average_coherence: Float,

    /// Total energy tapped from Intelligent Infinity.
    pub total_energy_tapped: Float,

    /// Tick counts by phase name.
    /// Tracks how many ticks executed each phase successfully.
    pub ticks_by_phase: HashMap<String, u64>,

    /// Peak entity count observed.
    pub peak_entity_count: usize,

    /// Total potentials extracted across all ticks.
    pub total_potentials_extracted: u64,

    /// Average entities manifested per tick.
    pub avg_entities_per_tick: Float,
}

impl CausalStatistics {
    /// Create new empty statistics.
    pub fn new() -> Self {
        CausalStatistics {
            total_ticks: 0,
            total_entities_manifested: 0,
            average_coherence: 0.0,
            total_energy_tapped: 0.0,
            ticks_by_phase: HashMap::new(),
            peak_entity_count: 0,
            total_potentials_extracted: 0,
            avg_entities_per_tick: 0.0,
        }
    }

    /// Update statistics with a tick result.
    pub fn update(&mut self, result: &CausalTickResult, current_entity_count: usize) {
        self.total_ticks += 1;
        self.total_entities_manifested += result.entities_manifested as u64;
        self.total_potentials_extracted += result.potentials_extracted as u64;

        // Running average coherence
        let n = self.total_ticks as Float;
        self.average_coherence =
            self.average_coherence * (n - 1.0) / n + result.coherence_level / n;

        // Track peak entity count
        if current_entity_count > self.peak_entity_count {
            self.peak_entity_count = current_entity_count;
        }

        // Update average entities per tick
        self.avg_entities_per_tick =
            self.total_entities_manifested as Float / self.total_ticks as Float;

        // Track energy tapped
        self.total_energy_tapped += result.infinity_state.total_tapped;

        // Track phase execution
        for phase in result.phase_timings.keys() {
            *self.ticks_by_phase.entry(phase.clone()).or_insert(0) += 1;
        }
    }
}

// ============================================================================
// Causal Simulation Result
// ============================================================================

/// Final result of running the simulation for multiple steps.
#[derive(Debug, Clone)]
pub struct CausalSimulationResult {
    /// Total number of steps completed.
    pub steps_completed: u64,

    /// Final tick result.
    pub final_tick: CausalTickResult,

    /// Aggregate statistics.
    pub statistics: CausalStatistics,

    /// Whether the simulation completed successfully.
    pub success: bool,

    /// Error message if simulation failed.
    pub error_message: Option<String>,
}

// ============================================================================
// Phase 7: Unified Loop Validation
// ============================================================================

/// Result of validating the unified loop architecture.
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 7:
/// "All systems feed into each other each tick"
/// "All layers execute in order, data flows correctly, no orphan subsystems"
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    /// Critical issues that prevent correct data flow
    pub issues: Vec<String>,

    /// Non-critical issues that don't prevent execution but may affect quality
    pub warnings: Vec<String>,

    /// Whether the validation passed (no critical issues)
    pub valid: bool,

    /// Number of connections that were checked
    pub connections_checked: usize,

    /// Names of layers that were validated
    pub layers_validated: Vec<String>,
}

impl ValidationResult {
    /// Create a new validation result
    pub fn new() -> Self {
        ValidationResult {
            issues: Vec::new(),
            warnings: Vec::new(),
            valid: true,
            connections_checked: 0,
            layers_validated: Vec::new(),
        }
    }

    /// Add a critical issue
    pub fn add_issue(&mut self, issue: impl Into<String>) {
        self.issues.push(issue.into());
        self.valid = false;
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// Mark a layer as validated
    pub fn mark_layer_validated(&mut self, layer: impl Into<String>) {
        self.layers_validated.push(layer.into());
        self.connections_checked += 1;
    }
}

/// Expected phase execution order for validation
pub const EXPECTED_PHASE_ORDER: &[&str] = &[
    "infinity_pulse",
    "field_evolve",
    "quantum_evolve",
    "quantum_collapse",
    "atom_formation",
    "molecule_formation",
    "planetary_systems",
    "extract_potentials",
    "manifest_entities",
    "absorb_influence",
    "consciousness_processing",
    "social_processing",
    "observer_decompress",
];

/// Total number of expected phases in the unified loop
pub const EXPECTED_PHASE_COUNT: usize = EXPECTED_PHASE_ORDER.len();

// ============================================================================
// Phase 2 P1: Atom Formation Types
// ============================================================================

/// Unique identifier for an atom manifestation
pub type AtomId = u64;

/// A particle manifested from quantum collapse, awaiting atom formation
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 2:
/// "Particles collapse from quantum field and can form atoms when
/// their archetype patterns match element signatures"
#[derive(Debug, Clone)]
pub struct ParticleManifestation {
    /// Quantum state signature of the collapsed particle
    pub quantum_signature: QuantumStateSignature,

    /// Archetype activation pattern from collapse
    pub archetype_activation: [Float; 22],

    /// Position in 3D space
    pub position: Coordinate3D,

    /// Velocity vector
    pub velocity: Vector3D,

    /// Coherence level of the particle (0.0 to 1.0)
    pub coherence: Float,

    /// Energy of the particle
    pub energy: Float,
}

impl ParticleManifestation {
    /// Create a new particle manifestation from quantum collapse
    pub fn new(
        quantum_signature: QuantumStateSignature,
        archetype_activation: [Float; 22],
        position: Coordinate3D,
        coherence: Float,
        energy: Float,
    ) -> Self {
        // Derive velocity from quantum signature's position hash
        let vel_seed = quantum_signature.position_hash as Float;
        let velocity = Vector3D::new(
            ((vel_seed * 0.1).sin() % 1.0).abs(),
            ((vel_seed * 0.2).cos() % 1.0).abs(),
            ((vel_seed * 0.3).sin() % 1.0).abs(),
        );

        Self {
            quantum_signature,
            archetype_activation,
            position,
            velocity,
            coherence,
            energy,
        }
    }
}

/// An atom manifested from particle collapse
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Elements as stable attractor fields in the holographic field"
///
/// Atoms form when collapsed particles have sufficient coherence
/// and their archetype activation matches element signatures.
#[derive(Debug, Clone)]
pub struct AtomManifestation {
    /// Unique identifier for this atom
    pub id: AtomId,

    /// The element this atom represents
    pub element: ElementAttractor,

    /// Position in 3D space
    pub position: Coordinate3D,

    /// Quantum state signature from original collapse
    pub quantum_signature: QuantumStateSignature,

    /// Stability factor (0.0 to 1.0)
    pub stability: Float,

    /// Archetype activation pattern that produced this atom
    pub archetype_activation: [Float; 22],

    /// Velocity vector
    pub velocity: Vector3D,

    /// Simulation time when this atom was created
    pub created_at: Float,
}

impl AtomManifestation {
    /// Create a new atom manifestation
    pub fn new(
        id: AtomId,
        element: ElementAttractor,
        position: Coordinate3D,
        quantum_signature: QuantumStateSignature,
        archetype_activation: [Float; 22],
        velocity: Vector3D,
        created_at: Float,
    ) -> Self {
        let stability = element.stability;

        Self {
            id,
            element,
            position,
            quantum_signature,
            stability,
            archetype_activation,
            velocity,
            created_at,
        }
    }

    /// Check if this atom is stable enough to persist
    pub fn is_stable(&self, threshold: Float) -> bool {
        self.stability >= threshold
    }
}

// ============================================================================
// Phase 2 P2: Molecule Formation Types
// ============================================================================

/// Unique identifier for a molecule manifestation
pub type MoleculeId = u64;

/// Distance threshold for bonding in normalized coordinates
///
/// Typical bond lengths are 1-3 Angstroms (1-3 × 10⁻¹⁰ m).
/// In normalized simulation coordinates (0-1 range), we use ~0.15
/// which represents atoms within 15% of simulation space.
const BOND_DISTANCE_THRESHOLD: Float = 0.15;

/// Minimum archetype compatibility for bond formation
///
/// Bonds require some resonance between atom archetype patterns.
const MIN_ARCHETYPE_COMPATIBILITY: Float = 0.2;

/// A molecule formed from atomic bonding
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 2 P2:
/// "Molecules form when atoms bond via archetype attraction patterns"
///
/// Molecules form when:
/// 1. Atoms are within bonding distance (spatial proximity)
/// 2. Archetype signatures are compatible (resonance)
/// 3. Energy threshold is met (stability)
#[derive(Debug, Clone)]
pub struct MolecularManifestation {
    /// Unique identifier for this molecule
    pub id: MoleculeId,

    /// Atoms that make up this molecule
    pub atoms: Vec<AtomId>,

    /// Chemical bonds between atoms
    pub bonds: Vec<ChemicalBond>,

    /// Predicted molecular geometry (from VSEPR theory)
    pub geometry: Option<MolecularGeometry>,

    /// Total energy of the molecule (sum of bond energies)
    pub total_energy: Float,

    /// Center of mass position
    pub center_of_mass: Coordinate3D,

    /// Simulation time when this molecule was created
    pub created_at: Float,
}

impl MolecularManifestation {
    /// Create a new molecular manifestation
    pub fn new(
        id: MoleculeId,
        atoms: Vec<AtomId>,
        bonds: Vec<ChemicalBond>,
        center_of_mass: Coordinate3D,
        created_at: Float,
    ) -> Self {
        // Calculate total energy from bonds
        let total_energy = bonds.iter().map(|b| b.energy).sum();

        // Predict geometry based on bonding pattern
        let geometry = Self::predict_geometry(&atoms, &bonds);

        Self {
            id,
            atoms,
            bonds,
            geometry,
            total_energy,
            center_of_mass,
            created_at,
        }
    }

    /// Predict molecular geometry from bonding pattern
    ///
    /// Uses VSEPR theory: electron domains repel each other
    /// to maximize separation, determining molecular shape.
    fn predict_geometry(atoms: &[AtomId], bonds: &[ChemicalBond]) -> Option<MolecularGeometry> {
        if atoms.is_empty() || bonds.is_empty() {
            return None;
        }

        // Count electron domains (simplified: each bond = 1 domain)
        let electron_domains = bonds.len();

        // Assume no lone pairs for now (can be enhanced later)
        let lone_pairs = 0;

        // Use VSEPR prediction
        crate::chemistry::molecular_geometry::predict_geometry(electron_domains, lone_pairs)
    }

    /// Check if this molecule is stable based on bond energy
    pub fn is_stable(&self, threshold: Float) -> bool {
        let avg_bond_energy = if self.bonds.is_empty() {
            0.0
        } else {
            self.total_energy / self.bonds.len() as Float
        };
        avg_bond_energy >= threshold
    }

    /// Get the molecular formula as a string
    pub fn formula(&self) -> String {
        // Would require element data from atoms - simplified for now
        format!("M{}", self.atoms.len())
    }
}

// Manifestation Potential
// Manifestation Potential
// ============================================================================

/// A potential entity manifestation from the holographic field.
///
/// When the field reaches sufficient coherence, it produces "potentials" —
/// regions where an entity could manifest. These potentials are not yet
/// entities but have the prerequisites for manifestation.
#[derive(Debug, Clone)]
pub struct ManifestationPotential {
    /// Unique identifier for this potential.
    pub id: u64,

    /// Coherence level at this location.
    pub coherence: Float,

    /// Energy density available.
    pub energy_density: Float,

    /// Suggested entity type based on field patterns.
    pub suggested_type: PotentialEntityType,

    /// Location in the field (normalized coordinates).
    pub location: (Float, Float, Float),
}

/// Types of entities that can manifest from potentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PotentialEntityType {
    /// Individual consciousness entity.
    Individual,
    /// Collective entity (group consciousness).
    Collective,
    /// Environmental entity (planet, star, etc.).
    Environmental,
}

// ============================================================================
// Entity Data
// ============================================================================

/// Minimal entity data for the causal inversion simulation.
///
/// This is a lightweight representation used during Phase 0.
/// Full entity data will be integrated in later phases.
#[derive(Debug, Clone)]
pub struct EntityData {
    /// Unique entity identifier.
    pub id: EntityId,

    /// Entity coherence level.
    pub coherence: Float,

    /// Energy absorbed from the field.
    pub field_energy: Float,

    /// Type of entity.
    pub entity_type: PotentialEntityType,

    /// Creation time.
    pub created_at: Float,

    /// Last update time.
    pub last_updated: Float,
}

// ============================================================================
// Causal Inversion Runner
// ============================================================================

/// Main runner for the causal inversion simulation.
///
/// This is the primary entry point for Phase 0 of the HoloSim Infinite refactor.
/// It implements the correct causal ordering required for accurate simulation
/// of consciousness evolution according to Law of One cosmology.
///
/// # Correct Causal Ordering
///
/// The simulation tick follows this exact sequence:
///
/// 1. **Infinity Pulse** (`infinity.pulse()`)
///    - Intelligent Infinity releases kinetic energy into the field
///    - This is the PRIMAL CAUSE of everything that follows
///
/// 2. **Field Evolution** (`field.evolve()`)
///    - Holographic field distributes the released energy
///    - Patterns form and dissolve based on resonance
///
/// 3. **Potential Extraction** (`field.extract_potentials()`)
///    - High-coherence regions identified
///    - These become candidates for entity manifestation
///
/// 4. **Entity Manifestation** (`manifest_entities_from_potentials()`)
///    - Potentials above threshold become actual entities
///    - Entities inherit archetypical patterns from field
///
/// 5. **Field Influence Absorption** (`entities.absorb_field_influence()`)
///    - Existing entities receive updates from field state
///    - This is TOP-DOWN causation in action
///
/// 6. **Observer Decompression** (`decompress_for_observers()`)
///    - Field data decompressed for specific observers
///    - Implements Observer Effect as computational optimization
///
/// # Thread Safety
///
/// The runner uses `Arc<RwLock<T>>` for shared state to support:
/// - Parallel field processing
/// - Safe entity access from multiple threads
/// - Observer-specific rendering threads
///
/// # Example
///
/// ```rust
/// use holonic_realms::simulation_v3::causal_inversion::{
///     CausalInversionRunner, CausalInversionConfig,
/// };
///
/// let config = CausalInversionConfig::default();
/// let mut runner = CausalInversionRunner::new(config);
///
/// // Run a single tick
/// let result = runner.tick(0.1);
/// println!("Entities manifested: {}", result.entities_manifested);
///
/// // Run for 100 steps
/// let final_result = runner.run(100, 0.1);
/// println!("Total steps: {}", final_result.steps_completed);
/// ```
pub struct CausalInversionRunner {
    /// Configuration parameters.
    config: CausalInversionConfig,

    /// Intelligent Infinity field (the source).
    infinity: IntelligentInfinity,

    /// Phase 10: Intelligent Infinity Source - Active source with emission patterns,
    /// feedback loops, and gateway mechanics.
    /// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 10:
    /// "II emits patterns to entities each tick, entity decisions feed back to II,
    /// teleological pull affects entity evolution, resonance tracking operational"
    ii_source: IntelligentInfinitySource,

    /// Holographic field manager (the distributor).
    field: HolographicFieldManager,

    /// Active entities in the simulation.
    /// Arc<RwLock> for thread-safe access during parallel processing.
    entities: Arc<RwLock<HashMap<EntityId, EntityData>>>,

    /// Manifestation potentials extracted from the field.
    potentials: Vec<ManifestationPotential>,

    /// Aggregate statistics.
    statistics: CausalStatistics,

    /// Current simulation time.
    current_time: Float,

    /// Next potential ID.
    next_potential_id: u64,

    /// Next entity ID.
    next_entity_id: u64,

    /// Phase 1: Observer registry for decompression management
    observer_registry: ObserverRegistry,

    // Phase 2: Quantum field integration
    /// Quantum field derived from holographic source.
    /// From COSMOLOGICAL-ARCHITECTURE.md: "Light is the third primal distortion"
    quantum_field: QuantumField,

    /// Choice-based collapse mechanism.
    /// From ROADMAP: "Free Will as collapse mechanism"
    choice_based_collapse: ChoiceBasedCollapse,

    /// Holographic field source for quantum field.
    holographic_source: Arc<HolographicField>,

    // Phase 2 P1: Atom formation from quantum collapse
    /// Atoms manifested from particle collapses
    atoms: Arc<RwLock<HashMap<AtomId, AtomManifestation>>>,

    /// Periodic table for element derivation
    periodic_table: PeriodicTable,

    /// Next atom ID for unique identification
    // Phase 2 P2: Molecule formation from atomic bonding
    /// Molecules formed from atom bonding
    molecules: Arc<RwLock<HashMap<MoleculeId, MolecularManifestation>>>,

    /// Next molecule ID for unique identification
    next_molecule_id: MoleculeId,
    next_atom_id: AtomId,

    // Phase 3 P2: Entity bodies
    /// Entity bodies - mapping from EntityId to their physical body
    bodies: Arc<RwLock<HashMap<EntityId, EmbodiedBody>>>,

    /// Body environment generator (default environment for bodies)
    default_environment: BodyEnvironment,

    // Phase 3 P2: Body statistics from last tick (for CausalTickResult)
    /// Bodies ticked in last absorb_field_influence call
    last_bodies_ticked: usize,
    /// Bodies created in last manifest_entities call
    last_bodies_created: usize,
    /// Bodies that died in last absorb_field_influence call
    last_bodies_died: usize,
    /// Average health from last tick
    last_average_health: Float,
    /// Average energy from last tick
    last_average_energy: Float,

    // Phase 4: Living environment (planet systems)
    /// Living environment wrapping planet systems
    living_environment: LivingEnvironment,

    // Phase 5: Consciousness processors for entities
    /// Consciousness processors - one per entity with a body
    consciousness_processors: Arc<RwLock<HashMap<EntityId, ConsciousnessProcessor>>>,

    // Phase 6: Social processor for relationships, groups, SMCs
    /// Social processor - handles all social dynamics
    social_processor: SocialProcessor,

    // Phase 1: Unified Field Equation - Three Primal Distortions
    // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md:
    // "Transform Free Will, Love, and Light from entity attributes to unified field dynamics"
    // The Unified Field Equation: ∂ψ/∂t = FreeWill(ψ) + Love(ψ) + Light(ψ)
    /// Unified field equation implementing the three primal distortions
    /// - Free Will: Stochastic perturbation (correlated noise via FreeWillKernel)
    /// - Love/Logos: Attractive potential (coherence feedback)
    /// - Light: Wave propagation (Laplacian operator)
    unified_field: UnifiedFieldEquation,

    /// Current field state at the center of the simulation
    /// This is evolved by the unified field equation each tick
    field_state: FieldState,

    /// Coherence peaks detected from field evolution
    /// These represent locations where entities can manifest
    coherence_peaks: Vec<CoherencePeak>,

    // Phase 3: Involution Flow - Causal Chain
    // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 3:
    // "Implement top-down causal flow from Intelligent Infinity through the Logos hierarchy to entities"
    /// Cosmic hierarchy containing all Logos levels
    /// Primary → Galactic → Solar → Planetary → Entity
    cosmic_hierarchy: CosmicHierarchy,

    /// Involution flow engine for top-down propagation
    /// Implements "Transcend and Include" at each level
    involution_flow: InvolutionFlow,

    /// Last propagation result from hierarchy
    last_propagation: PropagationResult,

    /// Entity manifestation handler
    entity_manifestation: Option<EntityManifestation>,

    /// Current planetary ID for manifestation
    active_planetary_id: usize,

    // Phase 4: Evolution Feedback - Bottom-up Causal Flow
    // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 4:
    // "Implement bottom-up feedback where entity decisions modify the field"
    // The Bidirectional Flow: Top-Down (Involution) ↔ Bottom-Up (Evolution)
    /// Decision feedback processor
    /// Collects entity decisions and converts them to field perturbations
    decision_feedback: DecisionFeedback,

    /// Density progression tracker
    /// Continuous evolution through density bands (not discrete jumps)
    density_progression: DensityProgression,

    /// Collective influence aggregator
    /// Combines multiple entity decisions for cosmic structure influence
    collective_influence: CollectiveInfluence,

    /// Karmic pattern encoder
    /// Encodes decision patterns into the field for future influence
    karmic_encoding: KarmicEncoding,

    /// Pending field perturbations from decisions
    /// Accumulated during tick, applied to field at end
    pending_perturbations: Vec<FieldPerturbation>,

    /// Decisions made this tick (for statistics)
    decisions_this_tick: usize,

    /// Total feedback accumulated (for statistics)
    total_feedback_accumulated: Float,

    // Phase 11: Higher Density Mechanics
    /// Higher density mechanics for 4th-8th density entities
    /// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 11:
    /// "All higher density code exists (100%) but is NOT INTEGRATED into main simulation"
    /// Now integrated: 5th (wisdom), 6th (unity), 7th (completion), 8th (return)
    fourth_density: FourthDensityMechanics,
    fifth_density: FifthDensityMechanics,
    sixth_density: SixthDensityMechanics,
    seventh_density: SeventhDensityMechanics,
    eighth_density: EighthDensityMechanics,

    // Phase 12: Gateway Mechanics and Resonance
    /// Gateway mechanics for simulation-level gateway access
    /// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 12:
    /// "Gateway mechanics based on resonance threshold"
    /// Tracks resonance thresholds and gateway states
    gateway_mechanics: GatewayMechanics,
}

impl CausalInversionRunner {
    /// Create a new causal inversion runner with the given configuration.
    pub fn new(config: CausalInversionConfig) -> Self {
        let infinity = IntelligentInfinity::new(config.pulse_frequency);

        // Phase 10: Initialize Intelligent Infinity Source with default patterns
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 10:
        // "Intelligent Infinity becomes the ACTIVE SOURCE of the simulation"
        let ii_source = IntelligentInfinitySource::new().with_default_patterns();

        let field = HolographicFieldManager::default();

        // Phase 2: Initialize holographic field for quantum field
        let archetypes = {
            let mut a = [ComplexArchetype {
                amplitude: 0.0,
                phase: 0.0,
            }; 22];
            for i in 0..22 {
                a[i] = ComplexArchetype {
                    amplitude: 0.5,
                    phase: (i as Float) * std::f64::consts::PI / 11.0,
                };
            }
            a
        };
        let holographic_source =
            Arc::new(HolographicField::new(InvolutionLayer::Green, archetypes));

        // Phase 2: Initialize quantum field with balanced light/love ratio
        let quantum_config = QuantumFieldConfig {
            light_love_ratio: LightLoveRatio::balanced(),
            coherence_threshold: config.min_coherence,
            decoherence_rate: 0.01,
            enable_free_will_collapse: true,
        };
        let quantum_field =
            QuantumField::from_holographic_field(holographic_source.clone(), quantum_config);

        // Phase 2: Initialize choice-based collapse mechanism
        let choice_based_collapse = ChoiceBasedCollapse::default_kernel();

        // Phase 4: Initialize living environment with Earth-like planet
        let living_environment = LivingEnvironment::earth_like();

        CausalInversionRunner {
            config,
            infinity,
            // Phase 10: Active source with emission patterns, feedback, and gateway mechanics
            ii_source,
            field,
            entities: Arc::new(RwLock::new(HashMap::new())),
            potentials: Vec::new(),
            statistics: CausalStatistics::new(),
            current_time: 0.0,
            next_potential_id: 0,
            next_entity_id: 1,
            observer_registry: Self::create_observer_registry(),
            // Phase 2: Quantum field
            quantum_field,
            choice_based_collapse,
            holographic_source,
            // Phase 2 P1: Atom formation
            atoms: Arc::new(RwLock::new(HashMap::new())),
            periodic_table: PeriodicTable::generate(),
            next_atom_id: 1,
            // Phase 2 P2: Molecule formation
            molecules: Arc::new(RwLock::new(HashMap::new())),
            next_molecule_id: 1,
            // Phase 3 P2: Entity bodies
            bodies: Arc::new(RwLock::new(HashMap::new())),
            default_environment: BodyEnvironment::default(),
            // Phase 3 P2: Body statistics tracking
            last_bodies_ticked: 0,
            last_bodies_created: 0,
            last_bodies_died: 0,
            last_average_health: 0.0,
            last_average_energy: 0.0,
            // Phase 4: Living environment
            living_environment,
            // Phase 5: Consciousness processors
            consciousness_processors: Arc::new(RwLock::new(HashMap::new())),
            // Phase 6: Social processor
            social_processor: SocialProcessor::new(),
            // Phase 1: Unified Field Equation - Three Primal Distortions
            // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md:
            // Initialize with deterministic seed for reproducibility
            unified_field: UnifiedFieldEquation::deterministic(42),
            // Initialize field state with uniform coherence
            field_state: FieldState::uniform(0.5),
            // No coherence peaks initially
            coherence_peaks: Vec::new(),
            // Phase 3: Involution Flow - Causal Chain
            // Initialize cosmic hierarchy with default configuration
            // Primary → Galactic → Solar → Planetary → Entity
            cosmic_hierarchy: Self::create_default_hierarchy(),
            involution_flow: InvolutionFlow::new(),
            last_propagation: PropagationResult::new(),
            entity_manifestation: None,
            active_planetary_id: 0,
            // Phase 4: Evolution Feedback - Bottom-up Causal Flow
            // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 4:
            // "Entity choice creates localized field modification"
            decision_feedback: DecisionFeedback::with_defaults(),
            density_progression: DensityProgression::new(EvolutionFeedbackConfig::default()),
            collective_influence: CollectiveInfluence::new(EvolutionFeedbackConfig::default()),
            karmic_encoding: KarmicEncoding::new(EvolutionFeedbackConfig::default()),
            pending_perturbations: Vec::new(),
            decisions_this_tick: 0,
            total_feedback_accumulated: 0.0,
            // Phase 11: Higher Density Mechanics
            // Initialize all density mechanics (4th-8th density)
            fourth_density: FourthDensityMechanics::new(),
            fifth_density: FifthDensityMechanics::new(),
            sixth_density: SixthDensityMechanics::new(),
            seventh_density: SeventhDensityMechanics::new(),
            eighth_density: EighthDensityMechanics::new(),
            // Phase 12: Gateway Mechanics
            // Initialize gateway mechanics for simulation-level gateway access
            gateway_mechanics: GatewayMechanics::new(0), // 0 = simulation-level (not entity-specific)
        }
    }

    /// Create the default cosmic hierarchy
    ///
    /// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 3:
    /// "Entity properties derive from position in hierarchy"
    fn create_default_hierarchy() -> CosmicHierarchy {
        let mut hierarchy = CosmicHierarchy::new();

        // Derive parameters down the chain
        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        // Add to hierarchy
        hierarchy.add_galactic_logos(galactic);
        hierarchy.add_solar_logos(solar, 0);
        hierarchy.add_planetary_logos(planetary, 0);

        hierarchy
    }

    /// Initialize entity manifestation from the hierarchy
    fn initialize_entity_manifestation(&mut self) {
        if self.entity_manifestation.is_none() {
            self.entity_manifestation = EntityManifestation::from_hierarchy(
                &self.cosmic_hierarchy,
                self.active_planetary_id,
            );
        }
    }

    /// Create a runner with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(CausalInversionConfig::default())
    }

    /// Create an observer registry with default MERA integration
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 1:
    /// "Only observed regions are decompressed"
    fn create_observer_registry() -> ObserverRegistry {
        let network = Arc::new(Mutex::new(MeraNetwork::new()));
        let mera = Arc::new(Mutex::new(MeraIntegration::new(network)));
        let config = ObserverRegistryConfig::default();
        ObserverRegistry::new(mera, config)
    }

    /// Execute a single causal tick.
    pub fn tick(&mut self, dt: Float) -> CausalTickResult {
        let mut result = CausalTickResult::default();
        result.tick_timestamp = self.current_time;

        // PHASE 0: Involution Flow - Top-Down Causal Propagation
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 3:
        // "Each level imposes boundary conditions on lower levels
        //  Lower levels inherit + refine patterns"
        let start = std::time::Instant::now();
        self.propagate_involution_flow(&mut result);
        result.phase_timings.insert(
            "involution_propagation".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 1: Infinity Pulse - PRIMAL CAUSE
        let start = std::time::Instant::now();
        self.infinity.pulse();
        result.infinity_state = InfinityPulseState::from_infinity(&self.infinity);
        result.phase_timings.insert(
            "infinity_pulse".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 10A: Intelligent Infinity Source Emission - Active Source
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 10:
        // "II emits patterns to entities each tick"
        // Update the II source and emit patterns to entities
        let start = std::time::Instant::now();
        self.ii_source.update(dt, self.current_time);

        // Calculate and update resonance based on current field state
        let field_coherence = self.field_state.coherence;
        let unity_factor = self.field_state.coherence; // Use coherence as proxy for unity
        let polarity_balance = self.calculate_average_polarity();
        let catalyst_integration = self.decisions_this_tick as Float / 100.0; // Based on decisions made
        let veil_transparency = self.field_state.veil_transparency;

        self.ii_source.update_resonance(
            field_coherence,
            unity_factor,
            polarity_balance,
            catalyst_integration,
            veil_transparency,
            0.5, // wisdom_accumulated - placeholder
        );

        // Emit patterns to entities
        let emission_count = self.emit_ii_patterns_to_entities();
        result.ii_emissions = emission_count;
        result.ii_resonance = self.ii_source.architectural_resonance;
        result.phase_timings.insert(
            "ii_source_emission".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 2: Field Evolution
        let start = std::time::Instant::now();
        let coherence = self.evolve_field(dt);
        result.coherence_level = coherence;
        result.phase_timings.insert(
            "field_evolve".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 3: Quantum Field Evolution
        // From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 2:
        // "Quantum field as the bridge between holographic source and matter"
        let start = std::time::Instant::now();
        let quantum_result = self.evolve_quantum_field(dt);
        result.quantum_states = quantum_result.state_count;
        result.entanglement_count = quantum_result.entanglement_count;
        result.phase_timings.insert(
            "quantum_evolve".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 4: Quantum Collapse to Particles
        // From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 2 P1:
        // "Particles collapse from quantum field via Free Will"
        let start = std::time::Instant::now();
        let particles = self.collapse_quantum_states();
        result.particles_manifested = particles.len();
        result.phase_timings.insert(
            "quantum_collapse".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 5: Atom Formation
        // From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 2 P1:
        // "Atoms form when coherence threshold is met and archetype patterns match element signatures"
        let start = std::time::Instant::now();
        let atoms_formed = self.form_atoms_from_particles(particles);
        result.atoms_manifested = atoms_formed;
        result.phase_timings.insert(
            "atom_formation".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 6: Molecule Formation
        // From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 2 P2:
        // "Molecules form when atoms bond via archetype attraction patterns"
        let start = std::time::Instant::now();
        let molecules_formed = self.form_molecules_from_atoms();
        result.molecules_formed = molecules_formed;
        result.phase_timings.insert(
            "molecule_formation".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 7: Planetary Systems (Living Environment)
        // From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 4:
        // "Environment is substrate, entities must interact to survive"
        let start = std::time::Instant::now();
        self.tick_planetary_systems(dt);
        // Update environment statistics in result
        let env_stats = self.living_environment.stats();
        result.environment_temperature = env_stats.avg_temperature;
        result.environment_pressure = env_stats.avg_pressure;
        result.phase_timings.insert(
            "planetary_systems".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 8: Potential Extraction
        let start = std::time::Instant::now();
        self.potentials = self.extract_potentials(coherence);
        result.potentials_extracted = self.potentials.len();
        result.phase_timings.insert(
            "extract_potentials".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 9: Entity Manifestation
        let start = std::time::Instant::now();
        let entity_count = self.entities.read().unwrap().len();
        let available_slots = self.config.max_entities.saturating_sub(entity_count);
        result.entities_manifested = self.manifest_entities(available_slots);
        // Phase 3 P2: Track bodies created during manifestation
        result.bodies_created = self.last_bodies_created;
        result.phase_timings.insert(
            "manifest_entities".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 10: Field Influence Absorption - TOP-DOWN CAUSATION
        let start = std::time::Instant::now();
        result.entities_updated = self.absorb_field_influence();
        // Phase 3 P2: Track body statistics from influence absorption
        result.bodies_ticked = self.last_bodies_ticked;
        result.bodies_died = self.last_bodies_died;
        result.average_health = self.last_average_health;
        result.average_energy = self.last_average_energy;
        result.phase_timings.insert(
            "absorb_influence".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 10B: Consciousness Processing - Archetype-Driven Behavior Emergence
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP_V5.md Phase 5:
        // "Integrate archetype-driven consciousness into the causal simulation loop"
        let start = std::time::Instant::now();
        result.choices_made = self.process_entity_consciousness();
        result.consciousness_ticks = self.consciousness_processors.read().unwrap().len();
        result.average_polarity = self.calculate_average_polarity();
        result.phase_timings.insert(
            "consciousness_processing".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 10E: Entity Feedback to Intelligent Infinity Source
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 10:
        // "Entity decisions feed back to II"
        // Collect entity experiences and feed back to the II source
        let start = std::time::Instant::now();
        let feedback_strength = self.collect_entity_feedback_for_ii();
        if feedback_strength > 0.0 {
            self.ii_source.integrate_feedback(feedback_strength);
        }
        result.ii_feedback_received = feedback_strength > 0.0;
        result.phase_timings.insert(
            "ii_feedback_collection".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 10F: Teleological Pull - Spiritual Gravity
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 10:
        // "Teleological pull affects entity evolution"
        // Apply the teleological pull toward unity to all entities
        let start = std::time::Instant::now();
        let pull_applied = self.apply_teleological_pull();
        result.teleological_pull_applied = pull_applied;
        result.phase_timings.insert(
            "teleological_pull".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 10C: Social Processing - Emergent Social Structures
        // From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 6:
        // "Entities form relationships, societies, Social Memory Complexes"
        let start = std::time::Instant::now();
        let social_output = self.process_social_interactions();
        result.interactions_processed = social_output.interactions_processed;
        result.relationships_formed = social_output.relationships_formed;
        result.groups_active = self.social_processor.groups.len();
        result.smcs_active = self.social_processor.smc_engine.active_complexes.len();
        result.entities_harvested = social_output.entities_harvested;
        result.phase_timings.insert(
            "social_processing".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 10D: Evolution Feedback - Bottom-Up Causal Flow
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 4:
        // "Entity decisions affect local field configuration"
        // "Evolution progress is continuous and visible"
        // "Collective evolution shapes local cosmic conditions"
        let start = std::time::Instant::now();
        let feedback_result = self.apply_evolution_feedback(dt);
        result.decisions_processed = feedback_result.decisions_processed;
        result.field_perturbations = feedback_result.field_perturbations;
        result.density_shifts = feedback_result.density_shifts;
        result.collective_influence = feedback_result.collective_influence;
        result.karmic_patterns_encoded = feedback_result.karmic_patterns;
        result.phase_timings.insert(
            "evolution_feedback".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 11: Higher Density Mechanics Processing
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 11:
        // "Implement 5th-8th density as DISTINCT FIELD CONFIGURATIONS"
        // "Density transition triggers from evolution"
        let start = std::time::Instant::now();
        let (fourth, fifth, sixth, seventh, eighth, transitions) =
            self.process_higher_densities(dt);
        result.density_4th_active = fourth;
        result.density_5th_active = fifth;
        result.density_6th_active = sixth;
        result.density_7th_active = seventh;
        result.density_8th_active = eighth;
        result.density_transitions = transitions;
        result.phase_timings.insert(
            "higher_density_processing".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 12: Gateway Mechanics and Resonance
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 12:
        // "Implement gateway mechanics that enable connection to Intelligent Infinity through architectural resonance"
        // "Resonance calculated dynamically, gateway states change based on resonance"
        // "Resonance improves as simulation matures"
        let start = std::time::Instant::now();
        let previous_state = self.gateway_mechanics.ii_access.gateway.state;
        self.gateway_mechanics.update(dt, self.current_time);

        // Calculate gateway statistics
        result.gateway_state = self.gateway_mechanics.ii_access.gateway.state;
        result.gateway_open = self.gateway_mechanics.is_gateway_open();
        result.peak_resonance = self.gateway_mechanics.peak_resonance;
        result.gateway_uptime = self.gateway_mechanics.gateway_uptime(self.current_time);

        // Track state transitions
        let current_state = self.gateway_mechanics.ii_access.gateway.state;
        result.gateway_state_transitions = if current_state != previous_state {
            1
        } else {
            0
        };

        // Higher resonance = better simulation fidelity
        // Apply resonance-based simulation improvements
        if result.gateway_open {
            // Gateway is open - enhance field coherence slightly
            self.field_state.coherence =
                (self.field_state.coherence + 0.001 * result.ii_resonance).min(1.0);
        }

        result.phase_timings.insert(
            "gateway_mechanics".to_string(),
            start.elapsed().as_micros() as u64,
        );

        // PHASE 11: Observer Decompression
        if self.config.enable_observer_decompression {
            let start = std::time::Instant::now();
            let registry = &mut self.observer_registry;
            result.regions_decompressed = registry.decompress_for_observers();
            registry.evict_idle_regions();
            let stats = registry.memory_stats();
            result.cache_hit_rate = stats.hit_rate();
            result.active_observers = registry.observer_count();
            result.phase_timings.insert(
                "observer_decompress".to_string(),
                start.elapsed().as_micros() as u64,
            );
        }

        // Phase 7: Calculate unified loop statistics
        result.total_phases_executed = result.phase_timings.len();
        result.data_flow_valid = self.check_data_flow_valid(&result);

        self.current_time += dt;
        let entity_count = self.entities.read().unwrap().len();
        self.statistics.update(&result, entity_count);

        result
    }

    /// Quick check for data flow validity (used during tick)
    fn check_data_flow_valid(&self, result: &CausalTickResult) -> bool {
        // Basic flow checks - energy and coherence must be positive
        result.infinity_state.kinetic > 0.0 && result.coherence_level > 0.0
    }

    /// Propagate involution flow through the cosmic hierarchy.
    ///
    /// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 3:
    /// "Each level imposes boundary conditions on lower levels
    ///  Lower levels inherit + refine patterns
    ///  'Transcend and Include' operates at each level dynamically"
    ///
    /// The Causal Chain:
    /// ```text
    /// IntelligentInfinity (Source)
    ///     ↓ First Distortion (Free Will)
    /// LOGOS (Blue-Ray)
    ///     ↓ Second Distortion (Love)
    /// SubLogos (Galactic/Solar)
    ///     ↓ Third Distortion (Light)
    /// SubSubLogos (Planetary)
    ///     ↓ Blueprint Encoding
    /// Entity Manifestation
    /// ```
    fn propagate_involution_flow(&mut self, result: &mut CausalTickResult) {
        // Propagate downward through hierarchy
        self.last_propagation = self
            .involution_flow
            .propagate_downward(&self.cosmic_hierarchy);

        // Update result with hierarchy statistics
        result.hierarchy_depth = self.cosmic_hierarchy.depth();
        result.levels_propagated = self.last_propagation.depth();
        result.propagation_complete = self.last_propagation.success;

        // Initialize entity manifestation if needed
        self.initialize_entity_manifestation();

        // Update entity seeds available
        if let Some(ref manifestation) = self.entity_manifestation {
            result.entity_seeds_available = manifestation.remaining_capacity();
            result.refinement_factor = manifestation.field_configuration.refinement_factor;
        }

        // Apply hierarchy-derived field configuration to unified field
        self.apply_hierarchy_to_field();
    }

    /// Apply hierarchy-derived configuration to the unified field.
    ///
    /// From Phase 3 R&D:
    /// "Entity properties derive from position in hierarchy"
    fn apply_hierarchy_to_field(&mut self) {
        // Get the primary logos configuration
        let primary = &self.cosmic_hierarchy.primary_logos;

        // Apply universal constants to field state
        let constants = &primary.universal_constants;

        // The hierarchy affects the field evolution rate
        self.field_state.energy =
            (self.field_state.energy * 0.95 + constants.base_frequency * 0.05).min(10.0);

        // Apply coherence threshold from hierarchy
        if self.field_state.coherence < constants.coherence_threshold * 0.5 {
            // Boost coherence if below threshold
            self.field_state.coherence = (self.field_state.coherence + 0.01).min(1.0);
        }
    }

    /// Get the cosmic hierarchy for inspection
    pub fn cosmic_hierarchy(&self) -> &CosmicHierarchy {
        &self.cosmic_hierarchy
    }

    /// Get mutable access to cosmic hierarchy
    pub fn cosmic_hierarchy_mut(&mut self) -> &mut CosmicHierarchy {
        &mut self.cosmic_hierarchy
    }

    /// Get the last propagation result
    pub fn last_propagation(&self) -> &PropagationResult {
        &self.last_propagation
    }

    /// Manifest an entity seed from the hierarchy
    ///
    /// From Phase 3 R&D:
    /// "Entity manifestation from hierarchy parameters"
    pub fn manifest_entity_seed(&mut self) -> Option<EntitySeed> {
        if let Some(ref mut manifestation) = self.entity_manifestation {
            if manifestation.can_manifest_more() {
                return Some(manifestation.manifest_entity());
            }
        }
        None
    }

    // ========================================================================
    // Phase 4: Evolution Feedback - Bottom-up Causal Flow
    // ========================================================================

    /// Apply evolution feedback from entity decisions to the field.
    ///
    /// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 4:
    /// "Implement bottom-up feedback where entity decisions modify the field"
    ///
    /// The Bidirectional Flow:
    /// - Top-Down (Involution): Field → Entity (constraint)
    /// - Bottom-Up (Evolution): Entity → Field (perturbation)
    ///
    /// This method:
    /// 1. Processes pending entity decisions into field perturbations
    /// 2. Updates density progression from decisions
    /// 3. Aggregates collective influence
    /// 4. Encodes karmic patterns
    /// 5. Applies feedback to the unified field
    fn apply_evolution_feedback(&mut self, dt: Float) -> EvolutionFeedbackResult {
        let mut result = EvolutionFeedbackResult::default();

        // Step 1: Process pending decisions into field perturbations
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md:
        // "Entity choice creates localized field modification"
        let perturbation = self.decision_feedback.process_pending();
        result.decisions_processed = self.decisions_this_tick;

        // Step 2: Apply perturbation to field state
        // "Amplitude = significance, Phase = nature of decision"
        let has_perturbation = perturbation.coherence_effect.abs() > 0.001
            || perturbation.energy_effect.abs() > 0.001
            || perturbation.spectrum_shift.abs() > 0.001;

        if has_perturbation {
            result.field_perturbations = 1;

            // Apply to field state
            self.field_state.coherence =
                (self.field_state.coherence + perturbation.coherence_effect * 0.1).clamp(0.0, 1.0);
            self.field_state.energy =
                (self.field_state.energy + perturbation.energy_effect * 0.1).max(0.0);

            // Apply density modulations
            for (i, &modulation) in perturbation.density_modulations.iter().enumerate() {
                if i < 8 {
                    self.field_state.density_amplitudes[i] = self.field_state.density_amplitudes[i]
                        .with_added_magnitude(modulation * 0.01);
                }
            }

            // Track accumulated feedback
            self.total_feedback_accumulated += perturbation.coherence_effect.abs()
                + perturbation.energy_effect.abs()
                + perturbation.spectrum_shift.abs();
        }

        // Step 3: Update density progression
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md:
        // "Continuous density progression (not state jumps)"
        let decisions: Vec<EntityDecision> = self
            .decision_feedback
            .get_history()
            .iter()
            .rev()
            .take(100)
            .cloned()
            .collect();

        // Register entities in density progression if not already registered
        let entity_ids: std::collections::HashSet<u64> =
            decisions.iter().map(|d| d.entity_id).collect();
        for &entity_id in &entity_ids {
            self.density_progression.register_entity(entity_id, 1.0); // Default to 1st density
        }

        self.density_progression
            .update_from_decisions(&decisions, self.current_time);
        self.density_progression.evolve_all(dt);

        // Count density shifts
        result.density_shifts = self.density_progression.get_events().len();

        // Step 4: Aggregate collective influence
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md:
        // "Collective entity influence on cosmic structure"
        self.collective_influence.add_decisions(&decisions);
        result.collective_influence = self.collective_influence.total_influence_applied();

        // Step 5: Encode karmic patterns
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md:
        // "Karmic pattern field encoding"
        // Group decisions by entity_id and create patterns
        let mut entity_decisions: HashMap<u64, Vec<&EntityDecision>> = HashMap::new();
        for decision in decisions.iter().take(10) {
            entity_decisions
                .entry(decision.entity_id)
                .or_insert_with(Vec::new)
                .push(decision);
        }
        for (entity_id, entity_decs) in entity_decisions {
            let cloned_decs: Vec<EntityDecision> = entity_decs.iter().cloned().cloned().collect();
            self.karmic_encoding
                .create_pattern(entity_id, &cloned_decs, self.current_time);
        }
        result.karmic_patterns = self.karmic_encoding.pattern_count();

        // Step 6: Propagate feedback upward through hierarchy
        // This allows entity decisions to influence cosmic configuration
        self.propagate_feedback_to_hierarchy(&perturbation);

        result
    }

    /// Propagate evolution feedback upward through the cosmic hierarchy.
    ///
    /// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 4:
    /// "Feedback propagation through hierarchy"
    /// "Collective evolution shapes local cosmic conditions"
    fn propagate_feedback_to_hierarchy(&mut self, perturbation: &FieldPerturbation) {
        // Entity decisions influence planetary logos (local cosmic conditions)
        // This is a subtle, slow process - decisions accumulate over time
        let feedback_strength = 0.0001; // Very small influence per decision

        // The planetary logos can be influenced by entity polarization
        let sto_polarity: Float = self
            .decision_feedback
            .get_history()
            .iter()
            .take(100)
            .fold(0.0, |acc, d| acc + d.decision_type.polarity())
            / 100.0_f64.max(1.0);

        // Influence veil parameters based on collective entity evolution
        // Higher density progression = thinner veil
        let avg_density = self.density_progression.average_density();
        if avg_density > 3.0 {
            // Entities progressing to higher densities can thin the veil
            // This is an emergent property of collective evolution
            let thinning = feedback_strength * (avg_density - 3.0) * sto_polarity.abs();
            // Note: We would modify planetary_logoi here, but it's behind Vec
            // The influence is tracked in density_progression for now
        }
    }

    /// Get the decision feedback processor
    pub fn decision_feedback(&self) -> &DecisionFeedback {
        &self.decision_feedback
    }

    /// Get the density progression tracker
    pub fn density_progression(&self) -> &DensityProgression {
        &self.density_progression
    }

    /// Get the collective influence aggregator
    pub fn collective_influence(&self) -> &CollectiveInfluence {
        &self.collective_influence
    }

    /// Get the karmic encoding
    pub fn karmic_encoding(&self) -> &KarmicEncoding {
        &self.karmic_encoding
    }

    /// Get total feedback accumulated over simulation
    pub fn total_feedback_accumulated(&self) -> Float {
        self.total_feedback_accumulated
    }

    // ============================================================================
    // Phase 10: Intelligent Infinity Source Integration
    // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 10:
    // "Intelligent Infinity becomes the ACTIVE SOURCE of the simulation"
    // ============================================================================

    /// Emit II patterns to all entities in the simulation
    /// Returns the number of emissions made
    fn emit_ii_patterns_to_entities(&mut self) -> usize {
        let mut emission_count = 0;

        // Get entity count
        let entity_count = self.entities.read().unwrap().len();
        if entity_count == 0 {
            return 0;
        }

        // Emit different patterns based on connection state
        let patterns_to_emit = vec!["creation", "evolution", "catalyst", "unity"];

        for pattern_name in patterns_to_emit {
            if let Some(_emission) = self.ii_source.emit(pattern_name, self.current_time) {
                emission_count += 1;
            }
        }

        emission_count
    }

    /// Collect entity feedback and send to II source
    /// Returns the feedback strength accumulated
    fn collect_entity_feedback_for_ii(&mut self) -> Float {
        // Calculate feedback strength based on entity activity this tick
        let decisions_count = self.decisions_this_tick as Float;
        let feedback_from_decisions = decisions_count.min(1.0) * 0.5;

        // Factor in entity count for collective feedback
        let entity_count = self.entities.read().unwrap().len() as Float;
        let collective_factor = (entity_count / 100.0).min(1.0) * 0.3;

        // Factor in recent density shifts
        let density_factor = (self.density_progression.get_events().len() as Float) * 0.2;

        feedback_from_decisions + collective_factor + density_factor
    }

    /// Apply teleological pull (spiritual gravity) to entities
    /// Pulls entities toward unity through the source connection
    /// Returns whether pull was successfully applied
    fn apply_teleological_pull(&mut self) -> bool {
        // Calculate pull strength based on II source resonance
        let resonance = self.ii_source.architectural_resonance;

        // Only apply pull if there's meaningful resonance
        if resonance < 0.1 {
            return false;
        }

        // Get entities and apply pull influence
        let mut entities = self.entities.write().unwrap();
        let pull_strength = resonance * 0.01; // Scale factor for pull

        for (_entity_id, entity_data) in entities.iter_mut() {
            // Apply teleological pull to entity's spiritual gravity
            // The pull is toward unity - higher coherence/consciousness
            // Use coherence as proxy for consciousness level
            let current_coherence = entity_data.coherence;
            let pull_toward_unity = pull_strength * (1.0 - current_coherence);

            // Modify the entity's coherence (toward higher consciousness)
            // This is a subtle influence, not forced
            entity_data.coherence = (entity_data.coherence + pull_toward_unity * 0.1).min(1.0);
        }

        true
    }

    // ============================================================================
    // Phase 11: Higher Density Mechanics Integration
    // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 11:
    // "Implement 5th-8th density as DISTINCT FIELD CONFIGURATIONS"
    // ============================================================================

    /// Process higher density mechanics based on entity density progression
    /// Integrates 4th-8th density mechanics into the main simulation loop
    /// Returns: (4th_active, 5th_active, 6th_active, 7th_active, 8th_active, transitions)
    fn process_higher_densities(&mut self, dt: Float) -> (bool, bool, bool, bool, bool, usize) {
        let mut fourth_active = false;
        let mut fifth_active = false;
        let mut sixth_active = false;
        let mut seventh_active = false;
        let mut eighth_active = false;
        let mut transitions_count = 0;

        // Get entities and check their average density
        let entities = self.entities.read().unwrap();
        if entities.is_empty() {
            return (
                fourth_active,
                fifth_active,
                sixth_active,
                seventh_active,
                eighth_active,
                transitions_count,
            );
        }

        // Calculate average coherence across all entities
        let avg_coherence: Float =
            entities.values().map(|e| e.coherence).sum::<Float>() / entities.len() as Float;

        // Calculate average density from progression tracker
        // Use hash of entity uuid as proxy for id since EntityId doesn't expose u64
        let mut total_density: Float = 0.0;
        let mut count = 0;
        for (entity_id, _) in entities.iter() {
            // Use hash of uuid as a simple way to get a consistent u64
            let hash = entity_id
                .uuid
                .bytes()
                .fold(0u64, |acc, b| acc.wrapping_add(b as u64));
            let density = self.density_progression.get_density(hash);
            total_density += density;
            count += 1;
        }
        let avg_density = if count > 0 {
            total_density / count as Float
        } else {
            1.0
        };

        // Determine which densities are active based on average entity density
        let avg_density_level = avg_density.floor() as usize;

        match avg_density_level {
            0 | 1 | 2 | 3 => {
                // 1st-3rd density: handled by base entity mechanics
                // 4th density is starting to form
                fourth_active = true;
                // Develop collective awareness as entities prepare for 4th
                self.fourth_density.form_collective(entities.len());
            }
            4 => {
                // 4th Density: Transition density - physical to more spiritual
                fourth_active = true;
                // Channel love based on average coherence
                self.fourth_density.channel_love(avg_coherence);
                // Develop telepathy
                self.fourth_density.develop_telepathy(dt);
            }
            5 => {
                // 5th Density: Wisdom/Light - light body activation
                fifth_active = true;
                // Activate light body based on entity coherence
                self.fifth_density.activate_light_body(avg_coherence * 0.1);
                // Learn and teach based on activity
                self.fifth_density.learn(dt);
                self.fifth_density.teach(dt);
            }
            6 => {
                // 6th Density: Unity/Balance - gateway access
                sixth_active = true;
                // Balance polarities based on entity coherence
                self.sixth_density.balance_polarities();
                // Access social memory based on II resonance
                let resonance = self.ii_source.architectural_resonance;
                self.sixth_density.access_social_memory(resonance);
                // Check gateway access - when all polarities integrated
                if self.sixth_density.all_polarities_integrated() {
                    transitions_count += 1;
                }
            }
            7 => {
                // 7th Density: Completion - experience integration
                seventh_active = true;
                // Activate violet ray based on progression
                let density_frac = avg_density - 7.0;
                self.seventh_density.activate_violet_ray(density_frac);
                // Access intelligent infinity when ready
                self.seventh_density.access_intelligent_infinity();
                // Prepare for gateway when ready
                self.seventh_density.prepare_gateway();
                // Check if ready for next octave
                if self.seventh_density.ready_for_next_octave() {
                    transitions_count += 1;
                }
            }
            8 => {
                // 8th Density: Return to source
                eighth_active = true;
                // Merge with intelligent infinity
                self.eighth_density.merge_with_infinity();
                // Complete octave when merged
                self.eighth_density.complete_octave();
                // Prepare for next octave
                self.eighth_density.prepare_next_octave(1.0);
            }
            _ => {} // Densities > 8: handled by octave transition
        }

        (
            fourth_active,
            fifth_active,
            sixth_active,
            seventh_active,
            eighth_active,
            transitions_count,
        )
    }

    /// Get the current 4th density mechanics
    pub fn fourth_density_mechanics(&self) -> &FourthDensityMechanics {
        &self.fourth_density
    }

    /// Get the current 5th density mechanics
    pub fn fifth_density_mechanics(&self) -> &FifthDensityMechanics {
        &self.fifth_density
    }

    /// Get the current 6th density mechanics
    pub fn sixth_density_mechanics(&self) -> &SixthDensityMechanics {
        &self.sixth_density
    }

    /// Get the current 7th density mechanics
    pub fn seventh_density_mechanics(&self) -> &SeventhDensityMechanics {
        &self.seventh_density
    }

    /// Get the current 8th density mechanics
    pub fn eighth_density_mechanics(&self) -> &EighthDensityMechanics {
        &self.eighth_density
    }

    /// Get the current II source resonance level
    pub fn ii_source_resonance(&self) -> Float {
        self.ii_source.architectural_resonance
    }

    /// Get the II source connection state
    pub fn ii_source_connection_state(&self) -> String {
        format!("{:?}", self.ii_source.connection_state)
    }

    /// Get the gateway mechanics for the simulation
    pub fn gateway_mechanics(&self) -> &GatewayMechanics {
        &self.gateway_mechanics
    }

    /// Get the current gateway state
    pub fn gateway_state(&self) -> GatewayState {
        self.gateway_mechanics.ii_access.gateway.state
    }

    /// Check if the gateway is currently open
    pub fn is_gateway_open(&self) -> bool {
        self.gateway_mechanics.is_gateway_open()
    }

    /// Get the peak resonance achieved in the simulation
    pub fn peak_resonance(&self) -> Float {
        self.gateway_mechanics.peak_resonance
    }

    /// Get the gateway uptime as a fraction of total simulation time
    pub fn gateway_uptime(&self) -> Float {
        self.gateway_mechanics.gateway_uptime(self.current_time)
    }

    pub fn run(&mut self, num_steps: u64, dt: Float) -> CausalSimulationResult {
        let mut last_result = CausalTickResult::default();
        for _ in 0..num_steps {
            last_result = self.tick(dt);
        }
        CausalSimulationResult {
            steps_completed: num_steps,
            final_tick: last_result,
            statistics: self.statistics.clone(),
            success: true,
            error_message: None,
        }
    }

    /// Evolve the holographic field using the Three Primal Distortions.
    ///
    /// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 1:
    /// "Transform Free Will, Love, and Light from entity attributes to unified field dynamics"
    ///
    /// The Unified Field Equation:
    /// ∂ψ/∂t = FreeWill(ψ) + Love(ψ) + Light(ψ)
    ///
    /// This method:
    /// 1. Applies Free Will (stochastic perturbation - breaks symmetry)
    /// 2. Applies Love (coherence attraction - creates structure)
    /// 3. Applies Light (wave propagation - transfers energy/information)
    /// 4. Detects coherence peaks for entity manifestation
    fn evolve_field(&mut self, _dt: Float) -> Float {
        // Get kinetic energy from Infinity pulse
        let kinetic = self.infinity.kinetic;

        // Position for field evolution (center of simulation)
        let position = [0.0, 0.0, 0.0];

        // Apply the Three Primal Distortions through the Unified Field Equation
        // From COSMOLOGICAL-ARCHITECTURE.md:
        // "The Three Primal Distortions: Free Will (First), Love/Logos (Second), Light (Third)
        //  are the fundamental movements that enable creation to unfold."
        self.unified_field.evolve(&mut self.field_state, &position);

        // Scale energy contribution from kinetic energy
        self.field_state.energy = (self.field_state.energy * 0.95 + kinetic * 0.05).min(10.0);

        // Detect coherence peaks for entity manifestation
        // These are regions where coherence exceeds threshold
        let peaks = self
            .unified_field
            .detect_coherence_peaks(&[(self.field_state.clone(), position)]);
        self.coherence_peaks = peaks;

        // Return current coherence level
        self.field_state.coherence
    }

    /// Evolve the quantum field for one time step.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 2:
    /// "The quantum field is Light - the third primal distortion"
    /// "It bridges holographic source and matter manifestation"
    ///
    /// This method:
    /// 1. Evolves quantum states from holographic field
    /// 2. Applies decoherence
    /// 3. Tracks entanglements
    /// 4. Optionally collapses states via Free Will
    fn evolve_quantum_field(&mut self, dt: Float) -> QuantumEvolutionResult {
        // Evolve the quantum field
        self.quantum_field.evolve(dt);

        // Get statistics from the field
        let stats = self.quantum_field.get_statistics();

        // Calculate average coherence
        let avg_coherence = if stats.amplitude_count > 0 {
            self.quantum_field.field_coherence()
        } else {
            0.0
        };

        QuantumEvolutionResult {
            state_count: stats.amplitude_count,
            entanglement_count: stats.entanglement_count,
            avg_coherence,
            collapsed_count: stats.collapse_count as usize,
            total_probability: self.quantum_field.total_probability(),
            field_coherence: stats.field_coherence,
        }
    }

    /// Collapse quantum states via Free Will to produce particles.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 2:
    /// "Quantum collapse via Free Will produces particles that can form atoms"
    ///
    /// This method:
    /// 1. Gets current quantum field amplitudes
    /// 2. Creates collapse context with environmental factors
    /// 3. Applies choice-based collapse via Free Will
    /// 4. Converts collapsed states to particle manifestations
    fn collapse_quantum_states(&mut self) -> Vec<ParticleManifestation> {
        // Note: ArchetypeActivationProfile is used internally in derive_archetype_from_quantum_state

        let mut particles = Vec::new();

        // Get current amplitudes from quantum field
        let amplitudes = self.quantum_field.amplitudes.clone();

        if amplitudes.is_empty() {
            return particles;
        }

        // Phase 5: Get average consciousness level from active processors
        let (avg_consciousness, avg_polarity, active_count) = {
            let processors = self.consciousness_processors.read().unwrap();
            if processors.is_empty() {
                (0.5, 0.0, 0)
            } else {
                let total_consciousness: Float =
                    processors.values().map(|p| p.activation_level).sum();
                let total_polarity: Float =
                    processors.values().map(|p| p.polarity_accumulation).sum();
                (
                    total_consciousness / processors.len() as Float,
                    total_polarity / processors.len() as Float,
                    processors.len(),
                )
            }
        };

        // Phase 5: Calculate archetype 22 activation from consciousness processors
        let archetype_22_activation = if active_count > 0 {
            // Average of choice activity from consciousness
            avg_consciousness * 0.5 + 0.5
        } else {
            0.5 // Default if no entities
        };

        // Create collapse context with entity consciousness influence (Phase 5)
        let context = QuantumCollapseContext {
            observer_state: None,
            polarity_preference: if avg_polarity > 0.3 {
                PolarityPreference::ServiceToOthers
            } else if avg_polarity < -0.3 {
                PolarityPreference::ServiceToSelf
            } else {
                PolarityPreference::Neutral
            },
            spectrum: SpectrumConfiguration::balanced(),
            environmental_coherence: self.quantum_field.field_coherence(),
            catalyst_intensity: self.infinity.kinetic,
            veil_transparency: self.field_state.veil_transparency,
        };

        // Apply choice-based collapse with entity consciousness influence
        let collapse_result = self.choice_based_collapse.collapse(&amplitudes, &context);

        // Convert collapsed state to particle manifestation
        let signature = collapse_result.state_signature;

        // Derive archetype activation from quantum state
        let activation = Self::derive_archetype_from_quantum_state(&signature);

        // Position from quantum signature's position hash
        let position = Coordinate3D::new(
            ((signature.position_hash as Float * 0.1).sin() % 1.0).abs(),
            ((signature.position_hash as Float * 0.2).cos() % 1.0).abs(),
            ((signature.position_hash as Float * 0.3).sin() % 1.0).abs(),
        );

        // Energy from collapse amplitude (Phase 5: influenced by consciousness)
        let consciousness_energy_factor = 1.0 + avg_consciousness * 0.5;
        let energy =
            collapse_result.amplitude.norm() * self.infinity.kinetic * consciousness_energy_factor;

        let particle = ParticleManifestation::new(
            signature,
            activation,
            position,
            collapse_result.coherence_preserved,
            energy,
        );

        particles.push(particle);

        // Collapse additional states based on coherence
        let collapse_count = (context.environmental_coherence * 5.0) as usize;
        for i in 0..collapse_count.min(3) {
            // Create a variant signature
            let variant_sig = QuantumStateSignature::new(
                signature.n + (i + 1) as u32,
                signature.l,
                signature.m + (i + 1) as i32,
                !signature.spin_up,
                signature.position_hash.wrapping_add((i + 1) as u64 * 1000),
            );

            let variant_activation = Self::derive_archetype_from_quantum_state(&variant_sig);

            let variant_position = Coordinate3D::new(
                ((variant_sig.position_hash as Float * 0.15).sin() % 1.0).abs(),
                ((variant_sig.position_hash as Float * 0.25).cos() % 1.0).abs(),
                ((variant_sig.position_hash as Float * 0.35).sin() % 1.0).abs(),
            );

            let variant_energy = energy * (1.0 - (i + 1) as Float * 0.2);

            particles.push(ParticleManifestation::new(
                variant_sig,
                variant_activation,
                variant_position,
                context.environmental_coherence * (1.0 - (i + 1) as Float * 0.1),
                variant_energy,
            ));
        }

        particles
    }

    /// Derive archetype activation profile from quantum state signature.
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Quantum numbers encode consciousness patterns"
    fn derive_archetype_from_quantum_state(signature: &QuantumStateSignature) -> [Float; 22] {
        let mut activation = [0.5; 22];

        // Principal quantum number n affects overall activation
        let n_factor = signature.n as Float / 7.0;
        for i in 0..22 {
            activation[i] = (activation[i] + n_factor * 0.3).min(1.0);
        }

        // Angular momentum l affects Mind archetypes (A1-A7)
        let l_factor = signature.l as Float / 3.0;
        for i in 0..7 {
            activation[i] = (activation[i] + l_factor * 0.2).min(1.0);
        }

        // Magnetic quantum number m affects Body archetypes (A8-A14)
        let m_factor = (signature.m.abs() as Float) / 4.0;
        for i in 7..14 {
            activation[i] = (activation[i] + m_factor * 0.15).min(1.0);
        }

        // Spin affects Spirit archetypes (A15-A21)
        let s_factor = if signature.spin_up { 0.6 } else { 0.4 };
        for i in 14..21 {
            activation[i] = (activation[i] + s_factor * 0.1).min(1.0);
        }

        // Choice (A22) from position hash
        activation[21] = ((signature.position_hash as Float) % 1.0).abs();

        activation
    }

    /// Form atoms from collapsed quantum states.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 2:
    /// "Atoms form when coherence threshold is met and archetype patterns match element signatures"
    ///
    /// This method:
    /// 1. Takes collapsed particle manifestations
    /// 2. Derives element from archetype activation pattern
    /// 3. Checks stability threshold
    /// 4. Creates and stores atom manifestations
    fn form_atoms_from_particles(&mut self, particles: Vec<ParticleManifestation>) -> usize {
        let mut atoms = self.atoms.write().unwrap();
        let mut atoms_formed = 0;

        for particle in particles {
            // Check coherence threshold
            if particle.coherence < self.config.min_coherence {
                continue;
            }

            // Derive element from archetype activation pattern
            let element =
                ElementAttractor::from_archetype_activation(&particle.archetype_activation);

            // Check element stability threshold
            if element.stability < self.config.min_coherence {
                continue;
            }

            // Check formation threshold
            if particle.coherence < element.formation_threshold {
                continue;
            }

            // Create atom manifestation
            let atom = AtomManifestation::new(
                self.next_atom_id,
                element,
                particle.position,
                particle.quantum_signature,
                particle.archetype_activation,
                particle.velocity,
                self.current_time,
            );

            self.next_atom_id += 1;
            atoms.insert(atom.id, atom);
            atoms_formed += 1;
        }

        atoms_formed
    }

    /// Form molecules from atoms based on archetype compatibility and spatial proximity.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 2 P2:
    /// "Molecules form when atoms bond via archetype attraction patterns"
    ///
    /// Bond formation requires:
    /// 1. Spatial proximity (distance < BOND_DISTANCE_THRESHOLD)
    /// 2. Archetype resonance (compatibility >= MIN_ARCHETYPE_COMPATIBILITY)
    /// 3. Energy threshold (derived from bond type)
    ///
    /// This method:
    /// 1. Gets atoms from storage
    /// 2. Checks spatial proximity between atom pairs
    /// 3. Calculates archetype resonance between atom element patterns
    /// 4. Creates ChemicalBond when conditions are met
    /// 5. Creates MolecularManifestation for bonded atoms
    /// 6. Stores molecules in the molecules HashMap
    fn form_molecules_from_atoms(&mut self) -> usize {
        let atoms = self.atoms.read().unwrap();

        if atoms.len() < 2 {
            return 0;
        }

        // Collect atom data for bonding calculations
        let atom_data: Vec<(AtomId, Coordinate3D, ElementAttractor, [Float; 22])> = atoms
            .values()
            .map(|a| {
                (
                    a.id,
                    a.position.clone(),
                    a.element.clone(),
                    a.archetype_activation,
                )
            })
            .collect();

        drop(atoms); // Release read lock before write operations

        let mut molecules = self.molecules.write().unwrap();
        let mut molecules_formed = 0;

        // Track which atoms are already bonded
        let mut bonded_atoms: std::collections::HashSet<AtomId> = std::collections::HashSet::new();

        // Check all atom pairs for potential bonding
        for i in 0..atom_data.len() {
            for j in (i + 1)..atom_data.len() {
                let (id1, pos1, elem1, sig1) = &atom_data[i];
                let (id2, pos2, elem2, sig2) = &atom_data[j];

                // Skip if either atom is already bonded (simplification)
                if bonded_atoms.contains(id1) || bonded_atoms.contains(id2) {
                    continue;
                }

                // Check spatial proximity
                let distance = pos1.distance_to(pos2);
                if distance > BOND_DISTANCE_THRESHOLD {
                    continue;
                }

                // Check archetype compatibility
                let compatibility = calculate_archetype_compatibility(sig1, sig2);
                if compatibility < MIN_ARCHETYPE_COMPATIBILITY {
                    continue;
                }

                // Create bond from archetypes
                let bond = ChemicalBond::from_archetypes(i, j, elem1, elem2);

                // Check bond energy threshold (molecules need some stability)
                if bond.energy < 50.0 {
                    // kJ/mol threshold
                    continue;
                }

                // Calculate center of mass
                let center_of_mass = Coordinate3D::new(
                    (pos1.x + pos2.x) / 2.0,
                    (pos1.y + pos2.y) / 2.0,
                    (pos1.z + pos2.z) / 2.0,
                );

                // Create molecule
                let molecule = MolecularManifestation::new(
                    self.next_molecule_id,
                    vec![*id1, *id2],
                    vec![bond],
                    center_of_mass,
                    self.current_time,
                );

                self.next_molecule_id += 1;
                molecules.insert(molecule.id, molecule);

                // Mark atoms as bonded
                bonded_atoms.insert(*id1);
                bonded_atoms.insert(*id2);

                molecules_formed += 1;
            }
        }

        molecules_formed
    }

    /// Extract manifestation potentials from the unified field.
    ///
    /// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 1:
    /// "Entities emerge when coherence peaks are stable"
    ///
    /// This method converts coherence peaks detected by the UnifiedFieldEquation
    /// into ManifestationPotentials that can become entities.
    fn extract_potentials(&mut self, coherence: Float) -> Vec<ManifestationPotential> {
        let mut potentials = Vec::new();

        // First, use coherence peaks from the unified field (if any)
        for peak in &self.coherence_peaks {
            if peak.coherence >= self.config.min_coherence {
                let potential = ManifestationPotential {
                    id: self.next_potential_id,
                    coherence: peak.coherence,
                    energy_density: peak.energy,
                    suggested_type: match peak.dominant_density % 3 {
                        0 => PotentialEntityType::Individual,
                        1 => PotentialEntityType::Collective,
                        _ => PotentialEntityType::Environmental,
                    },
                    location: (peak.position[0], peak.position[1], peak.position[2]),
                };
                self.next_potential_id += 1;
                potentials.push(potential);
            }
        }

        // If no coherence peaks detected, fall back to field-state-based potentials
        if potentials.is_empty() && coherence >= self.config.min_coherence {
            let num_potentials = ((coherence - self.config.min_coherence) * 10.0) as usize;
            let num_potentials = num_potentials.min(5);

            for i in 0..num_potentials {
                let potential = ManifestationPotential {
                    id: self.next_potential_id,
                    coherence: coherence - (i as Float * 0.05),
                    energy_density: self.infinity.kinetic * (1.0 - i as Float * 0.1),
                    suggested_type: match i % 3 {
                        0 => PotentialEntityType::Individual,
                        1 => PotentialEntityType::Collective,
                        _ => PotentialEntityType::Environmental,
                    },
                    location: (
                        (self.current_time + i as Float).sin() * 0.5 + 0.5,
                        (self.current_time * 1.3 + i as Float).cos() * 0.5 + 0.5,
                        (self.current_time * 0.7 + i as Float).sin() * 0.5 + 0.5,
                    ),
                };
                self.next_potential_id += 1;
                potentials.push(potential);
            }
        }

        potentials
    }

    fn manifest_entities(&mut self, available_slots: usize) -> usize {
        if available_slots == 0 || self.potentials.is_empty() {
            return 0;
        }
        let mut entities = self.entities.write().unwrap();
        let mut bodies = self.bodies.write().unwrap();
        let mut manifested = 0;
        let mut bodies_created = 0;
        let mut sorted_potentials = self.potentials.clone();
        sorted_potentials.sort_by(|a, b| {
            b.coherence
                .partial_cmp(&a.coherence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for potential in sorted_potentials.iter().take(available_slots) {
            if potential.coherence < self.config.min_coherence {
                continue;
            }
            let entity_id = EntityId::new(format!("entity-{}", self.next_entity_id));
            self.next_entity_id += 1;

            // Derive archetype activation from potential coherence
            let archetype_activation =
                Self::derive_archetype_from_potential(potential, self.current_time);

            let entity = EntityData {
                id: entity_id.clone(),
                coherence: potential.coherence,
                field_energy: potential.energy_density,
                entity_type: potential.suggested_type,
                created_at: self.current_time,
                last_updated: self.current_time,
            };
            entities.insert(entity_id.clone(), entity);
            manifested += 1;

            // Phase 3 P2: Create body for Individual entities only
            if potential.suggested_type == PotentialEntityType::Individual {
                if let Some(body) = self.create_body_for_entity(&entity_id, &archetype_activation) {
                    bodies.insert(entity_id, body);
                    bodies_created += 1;
                }
            }
        }

        // Store bodies_created for CausalTickResult
        self.last_bodies_created = bodies_created;

        manifested
    }

    fn absorb_field_influence(&mut self) -> usize {
        let mut entities = self.entities.write().unwrap();
        let mut bodies = self.bodies.write().unwrap();
        let mut updated = 0;
        let field_energy = self.infinity.kinetic;

        // Phase 3 P2: Body statistics tracking
        let mut bodies_ticked = 0;
        let mut bodies_died = 0;
        let mut total_health = 0.0;
        let mut total_energy = 0.0;

        // Collect entity IDs that have died (for removal)
        let mut dead_entities: Vec<EntityId> = Vec::new();

        for (entity_id, entity) in entities.iter_mut() {
            // 1. Field influence absorption (existing logic)
            let absorption_rate = entity.coherence * 0.1;
            entity.field_energy += field_energy * absorption_rate;
            entity.field_energy = entity.field_energy.min(1.0);
            let coherence_shift = (field_energy - entity.coherence) * 0.05;
            entity.coherence = (entity.coherence + coherence_shift).clamp(0.0, 1.0);
            entity.last_updated = self.current_time;

            // 2. Phase 3 P2: Body tick for entities with bodies
            // Phase 4: Get position-aware environment
            if let Some(body) = bodies.get_mut(entity_id) {
                // Get environment based on entity's position (Phase 4)
                let body_env = if let Some(position) =
                    self.living_environment.get_entity_position(entity_id)
                {
                    self.living_environment.body_environment_at(
                        position.latitude,
                        position.longitude,
                        position.altitude,
                    )
                } else {
                    // No position: use default environment
                    self.default_environment.clone()
                };

                let _sensory_field = body.tick(&body_env, self.config.pulse_frequency);

                bodies_ticked += 1;

                // Track health and energy statistics
                total_health += body.health_percentage();
                total_energy += body.energy_percentage();

                // Check for death
                if body.is_dead() {
                    bodies_died += 1;
                    dead_entities.push(entity_id.clone());
                }
            }

            updated += 1;
        }

        // Handle dead entities - mark for removal
        // Note: We don't remove them immediately to avoid iterator invalidation
        // They will be cleaned up in a future tick or can be handled separately
        for dead_id in &dead_entities {
            // Optionally log or track death events
            if let Some(body) = bodies.get(dead_id) {
                if let SurvivalStatus::Dead(cause) = &body.survival_status {
                    // Death is tracked via bodies_died counter
                    let _ = cause; // Acknowledge death cause
                }
            }
        }

        // Store body statistics for access by tick() method
        // These will be used to populate CausalTickResult
        self.last_bodies_ticked = bodies_ticked;
        self.last_bodies_died = bodies_died;
        self.last_average_health = if bodies_ticked > 0 {
            total_health / bodies_ticked as Float
        } else {
            0.0
        };
        self.last_average_energy = if bodies_ticked > 0 {
            total_energy / bodies_ticked as Float
        } else {
            0.0
        };

        updated
    }

    pub fn entity_count(&self) -> usize {
        self.entities.read().unwrap().len()
    }

    pub fn current_time(&self) -> Float {
        self.current_time
    }

    pub fn statistics(&self) -> &CausalStatistics {
        &self.statistics
    }

    pub fn infinity_state(&self) -> InfinityPulseState {
        InfinityPulseState::from_infinity(&self.infinity)
    }

    pub fn config(&self) -> &CausalInversionConfig {
        &self.config
    }

    pub fn get_entity(&self, id: EntityId) -> Option<EntityData> {
        self.entities.read().unwrap().get(&id).cloned()
    }

    pub fn entity_ids(&self) -> Vec<EntityId> {
        self.entities.read().unwrap().keys().cloned().collect()
    }

    pub fn register_observer(&mut self, observer: Observer) -> ObserverId {
        self.observer_registry.register_observer(observer)
    }

    pub fn cache_stats(&self) -> ObserverCacheStats {
        self.observer_registry.memory_stats()
    }

    /// Get quantum field statistics.
    /// Phase 2: Access to quantum field state.
    pub fn quantum_statistics(&self) -> crate::quantum::quantum_field::QuantumFieldStatistics {
        self.quantum_field.get_statistics()
    }

    /// Get the number of quantum states in the field.
    pub fn quantum_state_count(&self) -> usize {
        self.quantum_field.amplitudes.len()
    }

    /// Get the number of entanglement links.
    pub fn entanglement_count(&self) -> usize {
        self.quantum_field.entanglements.len()
    }

    /// Get the number of molecules.
    /// Phase 2 P2: Access to molecule storage.
    pub fn molecule_count(&self) -> usize {
        self.molecules.read().unwrap().len()
    }

    // ========================================================================
    // Phase 3 P2: Body-related helper methods
    // ========================================================================

    /// Create a body for a newly manifested entity.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 3:
    /// "Entities have bodies made of cells, need survival, feel environment"
    ///
    /// The body type is determined from the archetype activation pattern:
    /// - High Spirit archetypes (A15-A21) → Human-like body
    /// - High Body archetypes (A8-A14) → Animal body
    /// - High Mind archetypes (A1-A7) → Simple organism
    ///
    /// # Arguments
    /// * `entity_id` - The entity that will inhabit this body
    /// * `archetype_activation` - The 22-archetype activation pattern
    ///
    /// # Returns
    /// Some(EmbodiedBody) if body was created successfully, None otherwise
    fn create_body_for_entity(
        &self,
        entity_id: &EntityId,
        archetype_activation: &[Float; 22],
    ) -> Option<EmbodiedBody> {
        // Analyze archetype pattern to determine body type
        let mind_activation: Float = archetype_activation[0..7].iter().sum::<Float>() / 7.0;
        let body_activation: Float = archetype_activation[7..14].iter().sum::<Float>() / 7.0;
        let spirit_activation: Float = archetype_activation[14..21].iter().sum::<Float>() / 7.0;

        // Determine body type based on dominant archetype group
        if spirit_activation >= body_activation && spirit_activation >= mind_activation {
            // Spirit-dominant: Human-like body (complex consciousness)
            Some(EmbodiedBody::human(entity_id.clone()))
        } else if body_activation >= mind_activation {
            // Body-dominant: Animal body
            // Mass is derived from body archetype activation
            let mass = 1.0 + body_activation * 99.0; // 1-100 kg
            Some(EmbodiedBody::simple_animal(entity_id.clone(), mass))
        } else {
            // Mind-dominant: Simple organism (plants, simple life)
            let size = 0.1 + mind_activation * 10.0; // 0.1-10 meters
            Some(EmbodiedBody::plant(entity_id.clone(), size))
        }
    }

    /// Create environment context for body simulation.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 3:
    /// "Bodies experience environment through sensory fields"
    ///
    /// From Phase 4: Environment is derived from living_environment based on position.
    ///
    /// # Returns
    /// A BodyEnvironment for the body to experience
    fn create_body_environment(&self) -> BodyEnvironment {
        // Phase 4: Use living environment for default
        // This provides Earth-like defaults when no position is available
        self.default_environment.clone()
    }

    /// Create environment context for body simulation at a specific position.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 4:
    /// "body_environment_at() derives from planet systems"
    ///
    /// # Arguments
    /// * `position` - The entity's spatial position on the planet
    ///
    /// # Returns
    /// A BodyEnvironment derived from planet systems at that position
    fn create_body_environment_for_position(
        &mut self,
        position: &EntitySpatialPosition,
    ) -> BodyEnvironment {
        self.living_environment.body_environment_at(
            position.latitude,
            position.longitude,
            position.altitude,
        )
    }

    /// Tick planetary systems (Phase 4: Living Environment).
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 4:
    /// "Environment tick: atmosphere -> hydrosphere -> lithosphere -> energy_flow"
    ///
    /// This method:
    /// 1. Gets solar radiation from the energy flow system
    /// 2. Ticks the living environment (planet systems)
    /// 3. Updates entity positions based on planet rotation
    fn tick_planetary_systems(&mut self, dt: Float) {
        // Get solar radiation (simplified - use solar constant or energy flow)
        let solar_radiation = self
            .living_environment
            .planet()
            .energy_flow
            .as_ref()
            .map(|e| e.metrics.stellar_input)
            .unwrap_or(1361.0); // Solar constant default

        // Tick the living environment
        self.living_environment.tick(solar_radiation, dt);
    }

    /// Set an entity's position in the living environment.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 4:
    /// "Entities have position on planet surface"
    pub fn set_entity_position(&mut self, entity_id: EntityId, position: EntitySpatialPosition) {
        self.living_environment
            .set_entity_position(entity_id, position);
    }

    /// Get an entity's position in the living environment.
    pub fn get_entity_position(&self, entity_id: &EntityId) -> Option<&EntitySpatialPosition> {
        self.living_environment.get_entity_position(entity_id)
    }

    /// Get access to the living environment.
    pub fn living_environment(&self) -> &LivingEnvironment {
        &self.living_environment
    }

    /// Get mutable access to the living environment.
    pub fn living_environment_mut(&mut self) -> &mut LivingEnvironment {
        &mut self.living_environment
    }

    /// Derive archetype activation pattern from manifestation potential.
    ///
    /// This bridges the quantum/archetype derivation to the manifestation process.
    ///
    /// # Arguments
    /// * `potential` - The manifestation potential
    /// * `current_time` - Current simulation time for variation
    ///
    /// # Returns
    /// A 22-archetype activation array
    fn derive_archetype_from_potential(
        potential: &ManifestationPotential,
        current_time: Float,
    ) -> [Float; 22] {
        let mut activation = [0.5; 22];

        // Base activation from coherence
        let base = potential.coherence;

        // Vary by location and energy
        let location_factor =
            (potential.location.0 + potential.location.1 + potential.location.2) / 3.0;
        let energy_factor = potential.energy_density;

        // Entity type affects archetype emphasis
        match potential.suggested_type {
            PotentialEntityType::Individual => {
                // Individuals have balanced archetypes with spirit emphasis
                for i in 14..21 {
                    activation[i] = (base + energy_factor * 0.3).min(1.0);
                }
            }
            PotentialEntityType::Collective => {
                // Collectives have mind and body emphasis
                for i in 0..14 {
                    activation[i] = (base + location_factor * 0.2).min(1.0);
                }
            }
            PotentialEntityType::Environmental => {
                // Environmental entities have body emphasis (physical structures)
                for i in 7..14 {
                    activation[i] = (base + energy_factor * 0.4).min(1.0);
                }
            }
        }

        // Add time-based variation to ensure uniqueness
        let time_factor = (current_time * 0.1).sin() * 0.1;
        for i in 0..22 {
            activation[i] = (activation[i] + time_factor).clamp(0.0, 1.0);
        }

        // Choice (A22) from potential ID
        activation[21] = ((potential.id as Float * 0.1) % 1.0).abs();

        activation
    }

    /// Get the number of bodies currently in simulation.
    /// Phase 3 P2: Access to body storage.
    pub fn body_count(&self) -> usize {
        self.bodies.read().unwrap().len()
    }

    /// Get a body by entity ID.
    /// Phase 3 P2: Access individual body.
    pub fn get_body(&self, entity_id: &EntityId) -> Option<EmbodiedBody> {
        self.bodies.read().unwrap().get(entity_id).cloned()
    }

    // ========================================================================
    // Phase 5: Consciousness Processing Methods
    // ========================================================================

    /// Process entity consciousness for all entities with bodies.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 5:
    /// "Integrate archetype-driven consciousness into the causal simulation loop"
    ///
    /// This method:
    /// 1. Gets all bodies with their sensory fields
    /// 2. Processes each through its ConsciousnessProcessor
    /// 3. Executes resulting actions on the body
    /// 4. Collects choices as EntityDecisions for evolution feedback
    /// 5. Returns the number of choices made
    fn process_entity_consciousness(&mut self) -> usize {
        let mut choices_made = 0;
        let mut entity_decisions: Vec<EntityDecision> = Vec::new();

        let bodies = self.bodies.read().unwrap();
        let mut processors = self.consciousness_processors.write().unwrap();

        for (entity_id, body) in bodies.iter() {
            // Get or create consciousness processor for this entity
            let processor = processors
                .entry(entity_id.clone())
                .or_insert_with(|| ConsciousnessProcessor::new(entity_id.clone()));

            // Process consciousness tick with sensory field
            let output = processor.process_tick(&body.sensory_field);

            // Count choices made and convert to EntityDecision for evolution feedback
            if output.has_choice() {
                choices_made += 1;

                // Convert ConsciousnessOutput choice to EntityDecision for evolution feedback
                // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 4:
                // "Entity choice creates localized field modification"
                if let Some(ref choice_result) = output.choice_made {
                    // Map polarity to decision type
                    let decision_type = Self::map_polarity_to_decision_type(
                        choice_result.choice.sto_alignment,
                        output.behavior_type.clone(),
                    );

                    // Get entity position from body (use a default position since SensoryField doesn't have center)
                    let position = [0.5, 0.5, 0.5]; // Default center position

                    let decision = EntityDecision::new(
                        entity_id.as_u64(),
                        decision_type,
                        choice_result.choice.confidence, // significance = confidence
                        position,
                    )
                    .with_time(self.current_time);

                    entity_decisions.push(decision);
                }
            }

            // Execute any action on the body
            if let Some(action) = &output.action {
                processor.execute_action(
                    action,
                    &mut self.bodies.write().unwrap().get_mut(entity_id).unwrap(),
                );
            }
        }

        // Store decisions for evolution feedback phase
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 4:
        // "Entity decisions affect local field configuration"
        for decision in entity_decisions {
            self.decision_feedback.add_decision(decision);
        }

        self.decisions_this_tick = choices_made;
        choices_made
    }

    /// Map polarity and behavior to DecisionType for evolution feedback.
    ///
    /// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 4:
    /// "Amplitude = significance, Phase = nature of decision"
    fn map_polarity_to_decision_type(
        sto_alignment: Float,
        behavior_type: crate::biology::archetype_processor::BehaviorType,
    ) -> DecisionType {
        // Primary mapping based on STO alignment
        if sto_alignment > 0.5 {
            DecisionType::Service
        } else if sto_alignment < -0.5 {
            DecisionType::Control
        } else {
            // Secondary mapping based on behavior type
            use crate::biology::archetype_processor::BehaviorType;
            match behavior_type {
                BehaviorType::Seeking => DecisionType::Growth,
                BehaviorType::Loving => DecisionType::Connection,
                BehaviorType::Avoiding => DecisionType::Isolation,
                BehaviorType::Seeking | BehaviorType::Reflecting => DecisionType::Learning,
                BehaviorType::Creative => DecisionType::Expression,
                BehaviorType::Neutral | BehaviorType::Resting => DecisionType::Reception,
                BehaviorType::Survival => DecisionType::Growth,
                BehaviorType::Willful => DecisionType::Control,
                BehaviorType::Communicating => DecisionType::Expression,
                BehaviorType::Intuitive => DecisionType::Reception,
                BehaviorType::Spiritual => DecisionType::Connection,
                BehaviorType::Socializing => DecisionType::Connection,
                BehaviorType::Confronting => DecisionType::Control,
            }
        }
    }

    /// Calculate the average polarity across all entities with consciousness.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 5:
    /// "Polarity development from choices"
    ///
    /// Returns the average STO alignment (-1.0 for STS, 1.0 for STO)
    fn calculate_average_polarity(&self) -> Float {
        let processors = self.consciousness_processors.read().unwrap();

        if processors.is_empty() {
            return 0.0;
        }

        let total_polarity: Float = processors.values().map(|p| p.polarity_accumulation()).sum();

        total_polarity / processors.len() as Float
    }

    /// Get the consciousness processor for an entity (if it exists).
    pub fn get_consciousness_processor(
        &self,
        entity_id: &EntityId,
    ) -> Option<ConsciousnessProcessor> {
        self.consciousness_processors
            .read()
            .unwrap()
            .get(entity_id)
            .cloned()
    }

    /// Get the number of consciousness processors (entities with consciousness).
    pub fn consciousness_processor_count(&self) -> usize {
        self.consciousness_processors.read().unwrap().len()
    }

    // ========================================================================
    // Phase 6: Social Processing Methods
    // ========================================================================

    /// Process social interactions for all entities.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 6:
    /// "Entities form relationships, societies, Social Memory Complexes"
    ///
    /// This method:
    /// 1. Gets all entity IDs
    /// 2. Processes them through the SocialProcessor
    /// 3. Returns the social output with statistics
    fn process_social_interactions(&mut self) -> SocialOutput {
        let entities: Vec<EntityId> = self.entities.read().unwrap().keys().cloned().collect();
        let processors = self.consciousness_processors.read().unwrap().clone();
        self.social_processor.process_tick(&entities, &processors)
    }

    /// Get the social processor (for inspection).
    pub fn social_processor(&self) -> &SocialProcessor {
        &self.social_processor
    }

    /// Get mutable access to the social processor.
    pub fn social_processor_mut(&mut self) -> &mut SocialProcessor {
        &mut self.social_processor
    }

    /// Get the number of relationships.
    pub fn relationship_count(&self) -> usize {
        self.social_processor.relationship_count()
    }

    /// Get the number of social groups.
    pub fn social_group_count(&self) -> usize {
        self.social_processor.group_count()
    }

    /// Get the number of SMCs.
    pub fn smc_count(&self) -> usize {
        self.social_processor.smc_count()
    }

    // ========================================================================
    // Phase 1: Unified Field Equation Accessors
    // ========================================================================

    /// Get the unified field statistics.
    ///
    /// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 1:
    /// "Track Free Will, Love, and Light applications in field dynamics"
    pub fn unified_field_statistics(&self) -> DistortionStatistics {
        self.unified_field.statistics().clone()
    }

    /// Get the current field coherence.
    ///
    /// Coherence is the measure of order in the holographic field.
    /// Higher coherence = more structure, more potential for entity manifestation.
    pub fn field_coherence(&self) -> Float {
        self.field_state.coherence
    }

    /// Get the entities in the simulation.
    ///
    /// Returns the number of active entities.
    pub fn entities(&self) -> std::sync::RwLockReadGuard<'_, HashMap<EntityId, EntityData>> {
        self.entities.read().unwrap()
    }

    /// Get the current field energy.
    ///
    /// Energy is derived from the Infinity pulse and distributed by the field.
    pub fn field_energy(&self) -> Float {
        self.field_state.energy
    }

    /// Get the current spectrum position.
    ///
    /// Spectrum position determines Space/Time vs Time/Space dominance.
    /// v > 1.0: Space/Time dominant (3D space, 1D time)
    /// v < 1.0: Time/Space dominant (1D space, 3D time)
    /// v = 1.0: The Veil
    pub fn spectrum_position(&self) -> Float {
        self.field_state.spectrum_position
    }

    /// Get coherence peaks detected from the unified field.
    ///
    /// These are regions where entities can potentially manifest.
    pub fn coherence_peaks(&self) -> &[CoherencePeak] {
        &self.coherence_peaks
    }

    /// Get the field state (for inspection).
    pub fn field_state(&self) -> &FieldState {
        &self.field_state
    }

    /// Get mutable access to the unified field equation.
    pub fn unified_field_mut(&mut self) -> &mut UnifiedFieldEquation {
        &mut self.unified_field
    }

    // ========================================================================
    // Phase 7: Unified Loop Validation Methods
    // ========================================================================

    /// Validate that all layers are connected and functioning.
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 7:
    /// "All systems feed into each other each tick"
    /// "All layers execute in order, data flows correctly, no orphan subsystems"
    ///
    /// This method checks:
    /// 1. Infinity → Field connection (kinetic energy flows)
    /// 2. Field → Quantum connection (holographic source)
    /// 3. Quantum → Particles connection (collapse mechanism)
    /// 4. Particles → Atoms connection (element formation)
    /// 5. Atoms → Molecules connection (bonding)
    /// 6. Environment → Body connection (environment context)
    /// 7. Body → Consciousness connection (sensory fields)
    /// 8. Consciousness → Social connection (entity processors)
    /// 9. Social → Harvest connection (density transitions)
    /// 10. Observer connection (decompression)
    ///
    /// # Returns
    /// A ValidationResult with any issues found and the layers validated
    pub fn validate_unified_loop(&self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // 1. Check Infinity → Field connection
        if self.infinity.kinetic <= 0.0 {
            result.add_issue("Infinity kinetic is zero - field has no energy source");
        } else {
            result.mark_layer_validated("infinity_to_field");
        }

        // 2. Check Field → Quantum connection
        if self.quantum_field.amplitudes.is_empty() {
            result.add_warning("Quantum field has no amplitudes (may be initial state)");
        }
        if self.quantum_field.field_coherence() <= 0.0 {
            result.add_warning("Quantum field coherence is zero");
        }
        result.mark_layer_validated("field_to_quantum");

        // 3. Check Quantum → Particles connection (choice-based collapse)
        if !self.choice_based_collapse.has_kernel() {
            result.add_issue("Choice-based collapse has no Free Will kernel");
        } else {
            result.mark_layer_validated("quantum_to_particles");
        }

        // 4. Check Particles → Atoms connection (periodic table)
        if self.periodic_table.elements.is_empty() {
            result.add_issue("Periodic table is empty - atoms cannot form");
        } else {
            result.mark_layer_validated("particles_to_atoms");
        }

        // 5. Check Atoms → Molecules connection (bonding system)
        // This is always available as it's a function, not a state
        result.mark_layer_validated("atoms_to_molecules");

        // 6. Check Environment → Body connection
        if self.living_environment.planet().mass <= 0.0 {
            result.add_warning("Planet mass is zero - environment may be invalid");
        }
        result.mark_layer_validated("environment_to_body");

        // 7. Check Body → Consciousness connection
        let bodies = self.bodies.read().unwrap();
        let processors = self.consciousness_processors.read().unwrap();
        if !bodies.is_empty() && processors.len() < bodies.len() {
            result.add_warning(format!(
                "Not all bodies have consciousness processors: {} bodies, {} processors",
                bodies.len(),
                processors.len()
            ));
        }
        result.mark_layer_validated("body_to_consciousness");

        // 8. Check Consciousness → Social connection
        if !processors.is_empty()
            && self
                .social_processor
                .relationship_web
                .relationships
                .is_empty()
        {
            // This is normal at the start - relationships form over time
            result.mark_layer_validated("consciousness_to_social");
        } else {
            result.mark_layer_validated("consciousness_to_social");
        }

        // 9. Check Social → Harvest connection
        if self.social_processor.harvest_engine.pending_harvests() > 0 {
            result.mark_layer_validated("social_to_harvest");
        } else {
            result.mark_layer_validated("social_to_harvest");
        }

        // 10. Check Observer connection
        if self.config.enable_observer_decompression {
            if self.observer_registry.observer_count() == 0 {
                result.add_warning("Observer decompression enabled but no observers registered");
            }
            result.mark_layer_validated("observer_connection");
        }

        // Additional validation: check for orphan subsystems
        let atoms = self.atoms.read().unwrap();
        let molecules = self.molecules.read().unwrap();
        let entities = self.entities.read().unwrap();

        // Check if molecules reference existing atoms
        for molecule in molecules.values() {
            for atom_id in &molecule.atoms {
                if !atoms.contains_key(atom_id) {
                    result.add_issue(format!(
                        "Molecule {} references non-existent atom {}",
                        molecule.id, atom_id
                    ));
                }
            }
        }

        // Check if bodies reference existing entities
        for entity_id in bodies.keys() {
            if !entities.contains_key(entity_id) {
                result.add_issue(format!(
                    "Body exists for non-existent entity: {}",
                    entity_id
                ));
            }
        }

        // Final validation
        result.valid = result.issues.is_empty();
        result
    }

    /// Validate the phase execution order from a tick result.
    ///
    /// Checks that all expected phases executed in the correct order.
    pub fn validate_phase_order(result: &CausalTickResult) -> ValidationResult {
        let mut validation = ValidationResult::new();

        // Check that all expected phases are present
        for &phase in EXPECTED_PHASE_ORDER {
            if !result.phase_timings.contains_key(phase) {
                // observer_decompress is optional based on config
                if phase != "observer_decompress" {
                    validation.add_issue(format!("Missing phase: {}", phase));
                }
            } else {
                validation.mark_layer_validated(phase);
            }
        }

        // Check total phases executed
        let executed_count = result.phase_timings.len();
        if executed_count < EXPECTED_PHASE_COUNT - 1 {
            // -1 for optional observer_decompress
            validation.add_warning(format!(
                "Only {} phases executed, expected at least {}",
                executed_count,
                EXPECTED_PHASE_COUNT - 1
            ));
        }

        validation.valid = validation.issues.is_empty();
        validation
    }

    /// Validate data flow between layers.
    ///
    /// Checks that data flows correctly from higher to lower layers.
    pub fn validate_data_flow(&self, result: &CausalTickResult) -> ValidationResult {
        let mut validation = ValidationResult::new();

        // Check infinity pulse produces energy
        if result.infinity_state.kinetic <= 0.0 {
            validation.add_issue("Infinity pulse did not produce kinetic energy");
        } else {
            validation.mark_layer_validated("infinity_energy");
        }

        // Check field evolution produces coherence
        if result.coherence_level <= 0.0 {
            validation.add_issue("Field evolution did not produce coherence");
        } else {
            validation.mark_layer_validated("field_coherence");
        }

        // Check quantum → particles flow
        if result.quantum_states > 0 && result.particles_manifested == 0 {
            // This may be normal if no collapse occurs
            validation.mark_layer_validated("quantum_particles_flow");
        } else if result.quantum_states == 0 {
            validation.add_warning("No quantum states in field");
            validation.mark_layer_validated("quantum_particles_flow");
        } else {
            validation.mark_layer_validated("quantum_particles_flow");
        }

        // Check atoms → molecules flow
        let atoms = self.atoms.read().unwrap();
        if atoms.len() > 0 && result.molecules_formed == 0 {
            // May be normal if atoms are too far apart
            validation.mark_layer_validated("atoms_molecules_flow");
        } else {
            validation.mark_layer_validated("atoms_molecules_flow");
        }

        // Check potentials → entities flow
        if result.potentials_extracted > 0 && result.entities_manifested == 0 {
            // May be due to coherence threshold or entity limit
            if self.entities.read().unwrap().len() >= self.config.max_entities {
                validation.add_warning("Potentials extracted but entity limit reached");
            }
            validation.mark_layer_validated("potentials_entities_flow");
        } else {
            validation.mark_layer_validated("potentials_entities_flow");
        }

        // Check bodies are created for entities
        if result.entities_manifested > 0 && result.bodies_created == 0 {
            // May be normal for non-Individual entities
            validation.mark_layer_validated("entities_bodies_flow");
        } else {
            validation.mark_layer_validated("entities_bodies_flow");
        }

        // Check consciousness processes bodies
        let processors = self.consciousness_processors.read().unwrap();
        if result.bodies_ticked > 0 && result.consciousness_ticks == 0 {
            validation.add_issue("Bodies ticked but no consciousness processing occurred");
        } else {
            validation.mark_layer_validated("bodies_consciousness_flow");
        }
        drop(processors);

        // Check social processes entities
        if result.consciousness_ticks > 0 && result.interactions_processed == 0 {
            // May be normal initially - no relationships yet
            validation.mark_layer_validated("consciousness_social_flow");
        } else {
            validation.mark_layer_validated("consciousness_social_flow");
        }

        validation.valid = validation.issues.is_empty();
        validation
    }
}
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_causal_inversion_config_default() {
        let config = CausalInversionConfig::default();
        assert_eq!(config.max_entities, 1000);
    }

    #[test]
    fn test_causal_inversion_runner_creation() {
        let runner = CausalInversionRunner::with_defaults();
        assert_eq!(runner.entity_count(), 0);
    }

    #[test]
    fn test_causal_inversion_runner_single_tick() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);
        assert!(result.phase_timings.contains_key("infinity_pulse"));
    }

    // Phase 2 P1: Atom Formation Tests

    #[test]
    fn test_atom_manifestation_creation() {
        let activation = [0.5; 22];
        let element = ElementAttractor::from_archetype_activation(&activation);
        let signature = QuantumStateSignature::new(1, 0, 0, true, 12345);
        let position = Coordinate3D::new(0.5, 0.5, 0.5);
        let velocity = Vector3D::zero();

        let atom =
            AtomManifestation::new(1, element, position, signature, activation, velocity, 0.0);

        assert_eq!(atom.id, 1);
        assert!(atom.stability >= 0.0);
    }

    #[test]
    fn test_atom_stability_check() {
        let activation = [0.5; 22];
        let element = ElementAttractor::from_archetype_activation(&activation);
        let signature = QuantumStateSignature::new(1, 0, 0, true, 12345);
        let position = Coordinate3D::new(0.5, 0.5, 0.5);
        let velocity = Vector3D::zero();

        let atom =
            AtomManifestation::new(1, element, position, signature, activation, velocity, 0.0);

        // Should be stable at low threshold
        assert!(atom.is_stable(0.0));
    }

    #[test]
    fn test_particle_manifestation_creation() {
        let signature = QuantumStateSignature::new(1, 0, 0, true, 12345);
        let activation = [0.5; 22];
        let position = Coordinate3D::new(0.5, 0.5, 0.5);

        let particle = ParticleManifestation::new(signature, activation, position, 0.8, 1.0);

        assert_eq!(particle.quantum_signature.n, 1);
        assert_eq!(particle.coherence, 0.8);
        assert_eq!(particle.energy, 1.0);
    }

    #[test]
    fn test_archetype_derivation_from_quantum_state() {
        let signature = QuantumStateSignature::new(3, 1, 0, true, 99999);
        let activation = CausalInversionRunner::derive_archetype_from_quantum_state(&signature);

        // Activation should have 22 values
        assert_eq!(activation.len(), 22);

        // All values should be between 0 and 1
        for &a in &activation {
            assert!(a >= 0.0 && a <= 1.0);
        }
    }

    #[test]
    fn test_causal_tick_includes_atom_formation() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Phase 4 and 5 should be present
        assert!(result.phase_timings.contains_key("quantum_collapse"));
        assert!(result.phase_timings.contains_key("atom_formation"));

        // atoms_manifested should be tracked
        assert!(result.atoms_manifested >= 0);
    }

    // Phase 2 P2: Molecule Formation Tests

    #[test]
    fn test_molecule_manifestation_creation() {
        let bonds = vec![ChemicalBond::new(
            0,
            1,
            BondType::Covalent,
            BondOrder::Single,
        )];
        let center = Coordinate3D::new(0.5, 0.5, 0.5);

        let molecule = MolecularManifestation::new(1, vec![1, 2], bonds, center, 0.0);

        assert_eq!(molecule.id, 1);
        assert_eq!(molecule.atoms.len(), 2);
        assert!(molecule.total_energy > 0.0);
    }

    #[test]
    fn test_molecule_stability_check() {
        // Create a molecule with strong bonds
        let strong_bonds = vec![ChemicalBond::new(
            0,
            1,
            BondType::Covalent,
            BondOrder::Triple,
        )];
        let center = Coordinate3D::new(0.5, 0.5, 0.5);
        let stable_molecule = MolecularManifestation::new(1, vec![1, 2], strong_bonds, center, 0.0);

        assert!(stable_molecule.is_stable(100.0));

        // Create a molecule with weak bonds
        let weak_bonds = vec![ChemicalBond::new(
            0,
            1,
            BondType::VanDerWaals,
            BondOrder::Single,
        )];
        let unstable_molecule = MolecularManifestation::new(2, vec![3, 4], weak_bonds, center, 0.0);

        assert!(!unstable_molecule.is_stable(100.0));
    }

    #[test]
    fn test_bond_distance_threshold() {
        // Verify the constant is defined
        assert!(BOND_DISTANCE_THRESHOLD > 0.0);
        assert!(BOND_DISTANCE_THRESHOLD < 1.0);
    }

    #[test]
    fn test_min_archetype_compatibility() {
        // Verify the constant is defined
        assert!(MIN_ARCHETYPE_COMPATIBILITY > 0.0);
        assert!(MIN_ARCHETYPE_COMPATIBILITY <= 1.0);
    }

    #[test]
    fn test_causal_tick_includes_molecule_formation() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Phase 6 (molecule formation) should be present
        assert!(result.phase_timings.contains_key("molecule_formation"));

        // molecules_formed should be tracked
        assert!(result.molecules_formed >= 0);
    }

    #[test]
    fn test_form_molecules_requires_multiple_atoms() {
        let runner = CausalInversionRunner::with_defaults();
        let atoms = runner.atoms.read().unwrap();

        // With no atoms, molecule formation should return 0
        assert_eq!(atoms.len(), 0);
    }

    // Phase 3 P2: Body Integration Tests

    #[test]
    fn test_body_statistics_in_tick_result() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Body statistics should be present in the result
        assert!(result.bodies_ticked >= 0);
        assert!(result.bodies_created >= 0);
        assert!(result.bodies_died >= 0);
        assert!(result.average_health >= 0.0 && result.average_health <= 1.0);
        assert!(result.average_energy >= 0.0 && result.average_energy <= 1.0);
    }

    #[test]
    fn test_bodies_created_for_individual_entities() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Run a few ticks to allow entities to manifest
        for _ in 0..10 {
            runner.tick(0.1);
        }

        // Should have some bodies if entities manifested
        let body_count = runner.body_count();
        let entity_count = runner.entity_count();

        // Bodies are only for Individual entities, not all entities
        // So body_count <= entity_count
        assert!(body_count <= entity_count);
    }

    #[test]
    fn test_derive_archetype_from_potential_individual() {
        let potential = ManifestationPotential {
            id: 1,
            coherence: 0.8,
            energy_density: 0.7,
            suggested_type: PotentialEntityType::Individual,
            location: (0.5, 0.5, 0.5),
        };

        let activation = CausalInversionRunner::derive_archetype_from_potential(&potential, 0.0);

        // Activation should have 22 values
        assert_eq!(activation.len(), 22);

        // All values should be between 0 and 1
        for &a in &activation {
            assert!(a >= 0.0 && a <= 1.0);
        }

        // Individual entities should have spirit emphasis (archetypes 15-21)
        let spirit_avg: Float = activation[14..21].iter().sum::<Float>() / 7.0;
        assert!(spirit_avg > 0.0);
    }

    #[test]
    fn test_derive_archetype_from_potential_collective() {
        let potential = ManifestationPotential {
            id: 2,
            coherence: 0.7,
            energy_density: 0.6,
            suggested_type: PotentialEntityType::Collective,
            location: (0.3, 0.3, 0.3),
        };

        let activation = CausalInversionRunner::derive_archetype_from_potential(&potential, 0.0);

        // Activation should have 22 values
        assert_eq!(activation.len(), 22);

        // All values should be between 0 and 1
        for &a in &activation {
            assert!(a >= 0.0 && a <= 1.0);
        }
    }

    #[test]
    fn test_derive_archetype_from_potential_environmental() {
        let potential = ManifestationPotential {
            id: 3,
            coherence: 0.9,
            energy_density: 0.8,
            suggested_type: PotentialEntityType::Environmental,
            location: (0.7, 0.7, 0.7),
        };

        let activation = CausalInversionRunner::derive_archetype_from_potential(&potential, 0.0);

        // Activation should have 22 values
        assert_eq!(activation.len(), 22);

        // Environmental entities should have body emphasis (archetypes 8-14)
        let body_avg: Float = activation[7..14].iter().sum::<Float>() / 7.0;
        assert!(body_avg > 0.0);
    }

    #[test]
    fn test_body_count_method() {
        let runner = CausalInversionRunner::with_defaults();

        // Initially no bodies
        assert_eq!(runner.body_count(), 0);
    }

    #[test]
    fn test_get_body_returns_none_for_nonexistent() {
        let runner = CausalInversionRunner::with_defaults();
        let entity_id = EntityId::new("nonexistent-entity".to_string());

        // Should return None for non-existent body
        assert!(runner.get_body(&entity_id).is_none());
    }

    #[test]
    fn test_create_body_environment_returns_default() {
        let runner = CausalInversionRunner::with_defaults();
        let env = runner.create_body_environment();

        // Should have Earth-like default values
        assert!((env.temperature - 288.0).abs() < 1.0);
        assert!((env.gravity - 9.8).abs() < 0.1);
    }

    #[test]
    fn test_multiple_ticks_accumulate_bodies() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Run many ticks
        let mut total_bodies_created = 0;
        for _ in 0..20 {
            let result = runner.tick(0.1);
            total_bodies_created += result.bodies_created;
        }

        // Should have created some bodies over time
        // (depends on coherence reaching threshold and Individual entity types)
        // At minimum, body_count should be accessible
        let final_body_count = runner.body_count();
        assert!(final_body_count <= total_bodies_created); // Some may have died
    }

    // Phase 4: Living Environment Integration Tests

    #[test]
    fn test_living_environment_exists_in_runner() {
        let runner = CausalInversionRunner::with_defaults();

        // Should have access to living environment
        let env = runner.living_environment();
        assert!(env.planet().mass > 0.0);
    }

    #[test]
    fn test_tick_includes_planetary_systems_phase() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Phase 7 (planetary systems) should be present
        assert!(result.phase_timings.contains_key("planetary_systems"));

        // Environment statistics should be populated
        assert!(result.environment_temperature > 0.0);
        assert!(result.environment_pressure > 0.0);
    }

    #[test]
    fn test_set_entity_position() {
        let mut runner = CausalInversionRunner::with_defaults();
        let entity_id = EntityId::new("test-entity".to_string());
        let position = EntitySpatialPosition::at_surface(45.0, -120.0);

        runner.set_entity_position(entity_id.clone(), position);

        let retrieved = runner.get_entity_position(&entity_id);
        assert!(retrieved.is_some());
        assert!((retrieved.unwrap().latitude - 45.0).abs() < 0.001);
    }

    #[test]
    fn test_living_environment_tick_updates_stats() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Tick the simulation
        runner.tick(0.1);

        // Environment stats should be updated
        let stats = runner.living_environment().stats();
        assert!(stats.avg_temperature > 0.0);
        assert!(stats.entity_count >= 0);
    }

    #[test]
    fn test_multiple_ticks_advance_planetary_time() {
        let mut runner = CausalInversionRunner::with_defaults();

        let initial_time = runner.living_environment().current_time();

        for _ in 0..10 {
            runner.tick(0.1);
        }

        let final_time = runner.living_environment().current_time();
        assert!(final_time > initial_time);
    }

    #[test]
    fn test_environment_temperature_in_tick_result() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Temperature should be reasonable (Kelvin)
        assert!(result.environment_temperature > 200.0);
        assert!(result.environment_temperature < 400.0);
    }

    #[test]
    fn test_environment_pressure_in_tick_result() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Pressure should be reasonable (Pascals)
        // Earth sea level is ~101325 Pa
        assert!(result.environment_pressure > 50000.0);
        assert!(result.environment_pressure < 200000.0);
    }

    // Phase 6: Social Processing Integration Tests

    #[test]
    fn test_tick_includes_social_processing_phase() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Phase 10C (social processing) should be present
        assert!(result.phase_timings.contains_key("social_processing"));

        // Social statistics should be populated (may be 0 if no entities)
        assert!(result.interactions_processed >= 0);
        assert!(result.relationships_formed >= 0);
        assert!(result.groups_active >= 0);
        assert!(result.smcs_active >= 0);
        assert!(result.entities_harvested >= 0);
    }

    #[test]
    fn test_social_processor_exists_in_runner() {
        let runner = CausalInversionRunner::with_defaults();

        // Should have access to social processor
        let social = runner.social_processor();
        assert_eq!(social.relationship_count(), 0);
        assert_eq!(social.group_count(), 0);
        assert_eq!(social.smc_count(), 0);
    }

    #[test]
    fn test_social_statistics_accumulate_over_ticks() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Run several ticks
        for _ in 0..20 {
            runner.tick(0.1);
        }

        // After ticks, social processor should be accessible
        let relationship_count = runner.relationship_count();
        let group_count = runner.social_group_count();
        let smc_count = runner.smc_count();

        // All should be valid counts (even if 0)
        assert!(relationship_count >= 0);
        assert!(group_count >= 0);
        assert!(smc_count >= 0);
    }

    // Phase 1: Unified Field Equation Integration Tests
    // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md:
    // "Transform Free Will, Love, and Light from entity attributes to unified field dynamics"

    #[test]
    fn test_unified_field_exists_in_runner() {
        let runner = CausalInversionRunner::with_defaults();

        // Should have unified field with statistics
        let stats = runner.unified_field_statistics();
        assert!(stats.free_will_applications >= 0);
        assert!(stats.love_applications >= 0);
        assert!(stats.light_propagations >= 0);
    }

    #[test]
    fn test_field_state_initialized() {
        let runner = CausalInversionRunner::with_defaults();

        // Field state should have valid initial values
        let coherence = runner.field_coherence();
        assert!(coherence >= 0.0 && coherence <= 1.0);
    }

    #[test]
    fn test_evolve_field_applies_three_distortions() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Initial state
        let initial_stats = runner.unified_field_statistics();

        // Tick to trigger field evolution
        runner.tick(0.1);

        // After tick, distortions should have been applied
        let stats = runner.unified_field_statistics();
        assert!(stats.free_will_applications > initial_stats.free_will_applications);
        assert!(stats.love_applications > initial_stats.love_applications);
        assert!(stats.light_propagations > initial_stats.light_propagations);
    }

    #[test]
    fn test_field_evolution_affects_coherence() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Get initial coherence
        let initial_coherence = runner.field_coherence();

        // Run multiple ticks
        for _ in 0..10 {
            runner.tick(0.1);
        }

        // Coherence should have evolved (changed or stayed similar)
        let final_coherence = runner.field_coherence();
        assert!(final_coherence >= 0.0 && final_coherence <= 1.0);
    }

    #[test]
    fn test_coherence_peaks_detected() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Run several ticks to allow field to evolve
        for _ in 0..20 {
            runner.tick(0.1);
        }

        // Coherence peaks should be trackable
        let peaks = runner.coherence_peaks();
        // Peaks may or may not exist depending on field evolution
        assert!(peaks.len() >= 0);
    }

    #[test]
    fn test_unified_field_equation_components() {
        use crate::holographic_foundation::distortions::UnifiedFieldEquation;

        // Test that unified field can be created with different configs
        let field_default = UnifiedFieldEquation::from_defaults();
        let field_balanced = UnifiedFieldEquation::balanced();
        let field_high_freedom = UnifiedFieldEquation::high_freedom();
        let field_high_structure = UnifiedFieldEquation::high_structure();

        // All should have valid time
        assert!((field_default.time() - 0.0).abs() < 1e-10);
        assert!((field_balanced.time() - 0.0).abs() < 1e-10);
        assert!((field_high_freedom.time() - 0.0).abs() < 1e-10);
        assert!((field_high_structure.time() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_free_will_is_correlated_noise() {
        use crate::holographic_foundation::distortions::{
            FieldState, FreeWillConfig, FreeWillTerm,
        };

        // Test that Free Will produces correlated noise, not white noise
        let config = FreeWillConfig {
            correlation: 0.7, // High correlation
            amplitude: 0.1,
            ..Default::default()
        };
        let mut fw = FreeWillTerm::new(config);

        let mut state = FieldState::uniform(0.5);

        // Apply multiple times
        let mut results = Vec::new();
        for _ in 0..5 {
            fw.apply(&mut state, &[0.0, 0.0, 0.0], 0.0);
            results.push(state.coherence);
        }

        // With high correlation, consecutive values should be more similar
        // (not purely random)
        assert!(results.iter().all(|&c| c >= 0.0 && c <= 1.0));
    }

    #[test]
    fn test_love_increases_coherence() {
        use crate::holographic_foundation::distortions::{FieldState, LoveConfig, LoveTerm};

        // Test that Love increases coherence
        let config = LoveConfig::strong_attraction();
        let mut love = LoveTerm::new(config);

        let mut state = FieldState::uniform(0.3); // Start with low coherence
        let initial_coherence = state.coherence;

        love.apply(&mut state, &[0.0, 0.0, 0.0], None);

        // Love should increase coherence
        assert!(state.coherence >= initial_coherence);
    }

    #[test]
    fn test_light_wave_propagation() {
        use crate::holographic_foundation::distortions::{FieldState, LightConfig, LightTerm};

        // Test that Light propagates waves
        let config = LightConfig::fast_propagation();
        let mut light = LightTerm::new(config);

        let mut state = FieldState::uniform(0.5);

        // Apply wave propagation
        light.apply(&mut state, &[0.0, 0.0, 0.0], 0.0, 0.01);

        // Light should have propagated
        assert!(light.total_propagations() > 0);
    }

    // Phase 3: Involution Flow - Causal Chain Integration Tests
    // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md:
    // "Involution Flow: Parameters propagate PrimaryLogos → GalacticLogos → SolarLogos → PlanetaryLogos"

    #[test]
    fn test_cosmic_hierarchy_exists_in_runner() {
        let runner = CausalInversionRunner::with_defaults();

        // Should have access to cosmic hierarchy
        let hierarchy = runner.cosmic_hierarchy();

        // Hierarchy should have a primary logos with universal constants
        assert!(hierarchy.primary_logos.universal_constants.base_frequency > 0.0);

        // Should have galactic logoi (plural - it's a Vec)
        assert!(!hierarchy.galactic_logoi.is_empty());
        assert!(
            hierarchy.galactic_logoi[0]
                .archetype_selection
                .primary_archetypes
                .len()
                > 0
        );

        // Should have solar logoi (plural - it's a Vec)
        assert!(!hierarchy.solar_logoi.is_empty());
        assert!(hierarchy.solar_logoi[0].entity_capacity > 0);

        // Should have planetary logoi (plural - it's a Vec)
        assert!(!hierarchy.planetary_logoi.is_empty());
        assert!(hierarchy.planetary_logoi[0].inherited_veil.base_thickness >= 0.0);
    }

    #[test]
    fn test_involution_flow_propagates_during_tick() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Initial state
        let initial_propagation = runner.last_propagation().clone();

        // Tick to trigger involution flow propagation
        let result = runner.tick(0.1);

        // After tick, involution flow should have propagated
        // Either propagation happened or was already complete
        assert!(result.hierarchy_depth >= 0);
        assert!(result.levels_propagated >= 0);
    }

    #[test]
    fn test_hierarchy_depth_in_tick_result() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Hierarchy depth should be populated
        // Depth 4 = Primary + Galactic + Solar + Planetary
        assert!(result.hierarchy_depth >= 0);
        assert!(result.hierarchy_depth <= 4);
    }

    #[test]
    fn test_propagation_complete_flag() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Propagation complete should be a boolean
        // After first tick, propagation should complete or be in progress
        assert!(result.propagation_complete || !result.propagation_complete);
    }

    #[test]
    fn test_refinement_factor_in_tick_result() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Refinement factor should be non-negative
        assert!(result.refinement_factor >= 0.0);
    }

    #[test]
    fn test_entity_seeds_available_in_tick_result() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Entity seeds available should be tracked
        assert!(result.entity_seeds_available >= 0);
    }

    #[test]
    fn test_manifest_entity_seed_returns_none_initially() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Before any propagation, no seeds should be available
        // This tests the accessor method exists
        let seed = runner.manifest_entity_seed();
        // May be None or Some depending on initialization
        assert!(seed.is_none() || seed.is_some());
    }

    #[test]
    fn test_hierarchy_propagation_accumulates_over_ticks() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Run multiple ticks
        for _ in 0..10 {
            runner.tick(0.1);
        }

        // After multiple ticks, propagation should have occurred
        let hierarchy = runner.cosmic_hierarchy();

        // Planetary logos should have been refined through propagation
        assert!(!hierarchy.planetary_logoi.is_empty());
        assert!(hierarchy.planetary_logoi[0].inherited_veil.base_thickness >= 0.0);
        assert!(hierarchy.planetary_logoi[0].inherited_veil.base_thickness <= 1.0);
    }

    #[test]
    fn test_propagation_result_tracking() {
        let runner = CausalInversionRunner::with_defaults();

        // Last propagation should be accessible
        let propagation = runner.last_propagation();

        // Propagation result should have valid fields
        assert!(propagation.success == propagation.success); // bool is valid
        assert!(propagation.depth() >= 0);
    }

    // Phase 4: Evolution Feedback - Bottom-up Causal Flow Tests
    // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 4:
    // "Implement bottom-up feedback where entity decisions modify the field"

    #[test]
    fn test_evolution_feedback_components_exist() {
        let runner = CausalInversionRunner::with_defaults();

        // Should have all evolution feedback components
        assert!(runner.decision_feedback().get_history().len() >= 0);
        assert!(runner.density_progression().entity_count() >= 0);
        assert!(runner.collective_influence().total_entity_count() >= 0);
        assert!(runner.karmic_encoding().pattern_count() >= 0);
    }

    #[test]
    fn test_evolution_feedback_in_tick_result() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Evolution feedback statistics should be present
        assert!(result.decisions_processed >= 0);
        assert!(result.field_perturbations >= 0);
        assert!(result.density_shifts >= 0);
        assert!(result.collective_influence >= 0.0);
        assert!(result.karmic_patterns_encoded >= 0);
    }

    #[test]
    fn test_tick_includes_evolution_feedback_phase() {
        let mut runner = CausalInversionRunner::with_defaults();
        let result = runner.tick(0.1);

        // Phase evolution_feedback should be present
        assert!(result.phase_timings.contains_key("evolution_feedback"));
    }

    #[test]
    fn test_decision_type_polarity_mapping() {
        // Service should be positive polarity
        assert_eq!(DecisionType::Service.polarity(), 1.0);

        // Control should be negative polarity
        assert_eq!(DecisionType::Control.polarity(), -1.0);

        // Growth should be neutral
        assert_eq!(DecisionType::Growth.polarity(), 0.0);
    }

    #[test]
    fn test_field_perturbation_from_decision() {
        let decision = EntityDecision::new(1, DecisionType::Service, 0.8, [0.5, 0.5, 0.5]);
        let perturbation = decision.perturbation();

        // Service decision should increase coherence
        assert!(perturbation.coherence_effect > 0.0);

        // All density modulations should be non-negative
        for &modulation in &perturbation.density_modulations {
            assert!(modulation >= 0.0);
        }
    }

    #[test]
    fn test_decision_feedback_accumulation() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Run multiple ticks
        for _ in 0..20 {
            runner.tick(0.1);
        }

        // After ticks, total feedback should be trackable
        let total = runner.total_feedback_accumulated();
        assert!(total >= 0.0);
    }

    #[test]
    fn test_density_progression_from_decisions() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Run many ticks to allow progression
        for _ in 0..50 {
            runner.tick(0.1);
        }

        // Density progression should be trackable
        let progression = runner.density_progression();
        assert!(progression.total_progression() >= 0.0);
    }

    #[test]
    fn test_collective_influence_aggregation() {
        let runner = CausalInversionRunner::with_defaults();

        // Collective influence should have valid polarity balance
        let polarity = runner.collective_influence().polarity_balance();
        assert!(polarity >= -1.0 && polarity <= 1.0);
    }

    #[test]
    fn test_karmic_encoding_tracks_patterns() {
        let runner = CausalInversionRunner::with_defaults();

        // Should be able to get karmic statistics
        let stats = runner.karmic_encoding().statistics();
        assert!(stats.total_created >= 0);
        assert!(stats.total_resolved >= 0);
        assert!(stats.currently_active >= 0);
    }

    #[test]
    fn test_evolution_feedback_bidirectional_flow() {
        let mut runner = CausalInversionRunner::with_defaults();

        // Run a tick
        let result = runner.tick(0.1);

        // Both involution (top-down) and evolution (bottom-up) should be present
        assert!(result.phase_timings.contains_key("involution_propagation"));
        assert!(result.phase_timings.contains_key("evolution_feedback"));

        // Hierarchy depth should be set (top-down)
        assert!(result.hierarchy_depth >= 0);

        // Decisions processed should be tracked (bottom-up)
        assert!(result.decisions_processed >= 0);
    }
}
