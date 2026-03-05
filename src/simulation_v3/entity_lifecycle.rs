// Entity Lifecycle Management Module
//
// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 2:
// "Create EntityLifecycleManager that manages entity evolution through densities (1st → 8th Density)"
//
// This module implements:
// 1. EntityLifecycleManager that evolves entities through densities
// 2. Density transition mechanism
// 3. Spectrum access evolution
// 4. Integration with consciousness module (Free Will, Archetype 22)
// 5. Integration with memory module (holographic memory, soul stream)
// 6. Evolutionary trajectories
// 7. Entity state tracking over time
//
// Phase 3 Update (February 2026):
// - Integrated polarity requirements for density transitions
// - Free Will choices affect evolution at 3rd density and above

use crate::consciousness::free_will::{ChoiceContext, FreeWillKernel, PolarityPreference};
use crate::entity_layer7::layer7::{
    EntityExperience, EntityId, EntitySpectrumAccess, EntityState, EvolutionaryTrajectory,
    Layer7KarmicPattern, ResolutionStatus, SpectrumAccessLevel,
};
use crate::evolution::{
    AdaptiveAttractorField, EntityFeedback, IntelligentInfinity, TeleologicalProgress,
};
use crate::evolution_density_octave::density_octave::{Density, DensityOctave};
use crate::evolution_density_octave::spectrum_access::SpectrumAccessMechanism;
use crate::memory::holographic_memory::{ExperienceMetadata, ExperienceType, HolographicMemory};
use crate::types::{Density as TypesDensity, Float};
use rand::Rng;
use std::collections::HashMap;

/// Entity Lifecycle Manager
///
/// Manages entity evolution through densities from 1st to 8th Density.
///
/// Phase 3: Integrated intelligent evolution system with adaptive attractors
/// and teleological direction.
#[derive(Debug, Clone)]
pub struct EntityLifecycleManager {
    /// Entity storage (indexed by EntityId)
    pub entities: HashMap<EntityId, EntityLifecycleData>,

    /// Evolutionary trajectories for each entity
    pub evolution_trajectories: HashMap<EntityId, Vec<EvolutionaryState>>,

    /// Spectrum access mechanism for evolution
    pub spectrum_access: SpectrumAccessMechanism,

    /// Free Will kernels for each entity
    pub free_will_kernels: HashMap<EntityId, FreeWillKernel>,

    /// Holographic memory for experience storage
    pub holographic_memory: HolographicMemory,

    /// Lifecycle statistics
    pub statistics: LifecycleStatistics,

    /// Simulation time
    pub simulation_time: u64,

    // ========================================================================
    // PHASE 3: INTELLIGENT EVOLUTION SYSTEM
    // ========================================================================
    /// Adaptive attractor fields for each density
    ///
    /// These fields learn and adjust based on entity feedback,
    /// replacing static mechanical attractors with intelligent learning attractors.
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3:
    /// "Adaptive attractor fields (not static)"
    /// "Learning system: attractors adjust based on entity choices"
    ///
    /// Note: Uses TypesDensity from types module, not Density from density_octave module
    pub adaptive_attractors: HashMap<TypesDensity, AdaptiveAttractorField>,

    /// Teleological progress tracking for each entity
    ///
    /// Tracks how well each entity is aligned with its evolutionary purpose:
    /// "return to Intelligent-Infinity, having served"
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3:
    /// "Progress tracking toward purpose"
    pub teleological_tracker: HashMap<EntityId, TeleologicalProgress>,

    /// Active Intelligent-Infinity field
    ///
    /// Collects feedback from entities and analyzes patterns to guide evolution.
    /// Emits teleological pull toward source.
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3:
    /// "Active Intelligent-Infinity with learning"
    pub intelligent_infinity: IntelligentInfinity,
}

/// Entity Lifecycle Data
///
/// Complete lifecycle data for a single entity.
///
/// Phase 3: Includes teleological progress tracking.
#[derive(Debug, Clone)]
pub struct EntityLifecycleData {
    /// Entity ID
    pub entity_id: EntityId,

    /// Current state
    pub current_state: EntityState,

    /// Current spectrum access
    pub current_spectrum_access: EntitySpectrumAccess,

    /// Current density
    pub current_density: crate::evolution_density_octave::density_octave::Density,

    /// Density octave for tracking this entity's density progression
    pub density_octave: DensityOctave,

    /// Evolutionary trajectory
    pub evolutionary_trajectory: EvolutionaryTrajectory,

    /// Evolutionary rate (0.5x to 1.5x) - individual variation in evolutionary speed
    pub evolutionary_rate: f64,

    /// Phase 2: Karmic patterns for this entity
    /// Each entity has unique karmic patterns that influence its evolutionary path
    pub karmic_patterns: Vec<Layer7KarmicPattern>,

    /// Total experiences accumulated
    pub total_experiences: u64,

    /// Density transitions made
    pub density_transitions: Vec<DensityTransitionRecord>,

    /// Phase 1: Original spectrum configuration
    /// This preserves the unique spectrum configuration created during involution.
    /// Without this, all entities would have identical spectrum ratios (1.0),
    /// preventing true individualization.
    pub original_spectrum_configuration: EntitySpectrumAccess,

    /// Phase 1: Polarity progression
    /// Tracks entity's polarization journey through the new polarization system.
    pub polarization: crate::polarization::PolarizationProgress,

    /// Phase 3: Teleological progress toward purpose
    ///
    /// Tracks how well this entity is aligned with its evolutionary purpose:
    /// "return to Intelligent-Infinity, having served"
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3:
    /// "Progress tracking toward purpose"
    pub teleological_progress: TeleologicalProgress,

    /// Phase 2: Holographic field coherence at entity's location
    /// This is set by the simulation runner based on global and local field state
    pub field_coherence: Option<f64>,
}

/// Evolutionary State
///
/// Snapshot of entity state at a point in time.
#[derive(Debug, Clone)]
pub struct EvolutionaryState {
    /// Timestamp
    pub timestamp: u64,

    /// Density
    pub density: String,

    /// Density sublevel (if applicable)
    pub density_sublevel: Option<String>,

    /// Spectrum access level
    pub spectrum_access_level: SpectrumAccessLevel,

    /// Vibrational state
    pub vibrational_state: crate::entity_layer7::layer7::VibrationalState,

    /// Polarity state
    pub polarity_state: crate::entity_layer7::layer7::PolarityState,

    /// Developmental level (0.0 to 1.0)
    pub developmental_level: Float,

    /// Veil thickness (1.0 = fully opaque, 0.0 = fully transparent)
    pub veil_thickness: Float,

    /// Consciousness level
    pub consciousness_level: Float,
}

/// Density Transition Record
///
/// Record of a density transition.
#[derive(Debug, Clone)]
pub struct DensityTransitionRecord {
    /// Transition ID
    pub transition_id: u64,

    /// From density
    pub from_density: String,

    /// To density
    pub to_density: String,

    /// Timestamp
    pub timestamp: u64,

    /// Progress percentage at transition
    pub progress_percentage: f64,
}

/// Evolution Result
///
/// Result of evolving entities for a number of steps.
#[derive(Debug, Clone)]
pub struct EvolutionResult {
    /// Number of steps evolved
    pub steps: u64,

    /// Number of density transitions that occurred
    pub transitions: usize,

    /// Number of spectrum access upgrades
    pub spectrum_access_upgrades: usize,

    /// Final statistics
    pub final_statistics: LifecycleStatistics,

    /// Execution time
    pub execution_time: std::time::Duration,
}

/// Lifecycle Statistics
///
/// Statistics about entity evolution.
///
/// Phase 3: Includes teleological metrics.
#[derive(Debug, Clone)]
pub struct LifecycleStatistics {
    /// Total number of entities
    pub total_entities: usize,

    /// Entities by density
    pub entities_by_density: HashMap<String, usize>,

    /// Entities by spectrum access level
    pub entities_by_access_level: HashMap<String, usize>,

    /// Average consciousness level
    pub avg_consciousness_level: Float,

    /// Total experiences accumulated
    pub total_experiences: u64,

    /// Total density transitions
    pub total_transitions: usize,

    /// Average developmental level
    pub avg_developmental_level: Float,

    /// Polarity distribution
    pub polarization_distribution: PolarizationDistribution,

    // ========================================================================
    // PHASE 3: TELEOLOGICAL METRICS
    // ========================================================================
    /// Average purpose alignment across all entities
    ///
    /// Measures how aligned entities are with returning to source.
    pub average_purpose_alignment: f64,

    /// Average coherence with source
    ///
    /// Measures resonance with Intelligent-Infinity.
    pub average_coherence_with_source: f64,

    /// Average service orientation (STO/STS)
    ///
    /// Positive = STO bias, Negative = STS bias
    pub average_service_orientation: f64,

    /// Attractor effectiveness history
    ///
    /// Tracks how well attractors have been guiding entities over time.
    pub attractor_effectiveness_history: Vec<f64>,
}

/// Polarity Distribution
///
/// Distribution of entities by polarization.
#[derive(Debug, Clone)]
pub struct PolarizationDistribution {
    /// Service-to-Others entities
    pub sto_count: usize,

    /// Service-to-Self entities
    pub sts_count: usize,

    /// Unpolarized entities
    pub unpolarized_count: usize,

    /// Average STO score
    pub avg_sto_score: Float,

    /// Average STS score
    pub avg_sts_score: Float,
}

impl EntityLifecycleManager {
    /// Create a new EntityLifecycleManager
    ///
    /// Phase 3: Initializes intelligent evolution system with adaptive attractors
    /// and teleological tracking.
    pub fn new() -> Self {
        let mut manager = EntityLifecycleManager {
            entities: HashMap::new(),
            evolution_trajectories: HashMap::new(),
            spectrum_access: SpectrumAccessMechanism::new(),
            free_will_kernels: HashMap::new(),
            holographic_memory: HolographicMemory::new(1000),
            statistics: LifecycleStatistics::default(),
            simulation_time: 0,
            // Phase 3: Initialize intelligent evolution system
            adaptive_attractors: HashMap::new(),
            teleological_tracker: HashMap::new(),
            intelligent_infinity: IntelligentInfinity::with_default_frequency(),
        };

        // Initialize adaptive attractors for each density
        manager.initialize_adaptive_attractors();

        manager
    }

    /// Add an entity to the lifecycle manager
    ///
    /// IMPORTANT: All entities START at First density and EVOLVE through densities.
    /// Scale is derived from current density, not passed as a parameter.
    ///
    /// Phase 1: Now accepts spectrum_configuration parameter to preserve unique
    /// spectrum configurations created during involution. Without this, all entities
    /// would have identical spectrum ratios, preventing true individualization.
    pub fn add_entity(
        &mut self,
        entity_id: EntityId,
        initial_state: EntityState,
        evolutionary_trajectory: EvolutionaryTrajectory,
        free_will_kernel: FreeWillKernel,
        initial_density: crate::evolution_density_octave::density_octave::Density,
        spectrum_configuration: EntitySpectrumAccess,
    ) {
        // Phase 1: Preserve the original spectrum configuration from involution
        // This is CRITICAL for individualization - without this, all entities
        // would have identical spectrum ratios (1.0)
        let spectrum_access = spectrum_configuration;

        // All entities start at First density
        let current_density = initial_density;

        // Each entity gets its own density octave to track individual progression
        // Initialize with First density
        let density_octave = DensityOctave::with_initial_density(current_density);

        // Generate unique evolutionary rate for this entity
        // This affects how quickly the entity progresses through the density octave
        // Phase 1 Final: Adjusted range to 0.3x-1.7x for target funnel
        // Expected distribution after 50 steps: ~60 at 1st, ~30 at 2nd, ~10 at 3rd
        // Entities with 0.3x rate: ~0.35 per step → 17.5 in 50 steps (stay at 1st)
        // Entities with 1.0x rate: ~1.0 per step → 50 in 50 steps (reach 2nd)
        // Entities with 1.7x rate: ~1.6 per step → 80 in 50 steps (reach 3rd)
        let evolutionary_rate = if let Some(existing_data) = self.entities.get(&entity_id) {
            // If entity already exists, use its existing rate
            existing_data.evolutionary_rate
        } else {
            // Generate a new random rate between 0.3x and 1.7x
            use rand::Rng;
            let mut rng = rand::thread_rng();
            rng.gen_range(0.3..1.7)
        };

        // Phase 2: Generate unique karmic patterns for this entity
        // Each entity has unique karmic patterns that influence its evolutionary path
        let karmic_patterns = Self::generate_unique_karmic_patterns(&entity_id);

        // Phase 1: Initialize polarization progression
        // All entities start unpolarized
        let polarization = crate::polarization::PolarizationProgress::new();

        let lifecycle_data = EntityLifecycleData {
            entity_id: entity_id.clone(),
            current_state: initial_state.clone(),
            current_spectrum_access: spectrum_access.clone(),
            current_density,
            density_octave,
            evolutionary_trajectory,
            evolutionary_rate,
            karmic_patterns,
            total_experiences: 0,
            density_transitions: Vec::new(),
            original_spectrum_configuration: spectrum_access, // Phase 1: Store original
            polarization,                                     // Phase 1: Add polarization
            teleological_progress: TeleologicalProgress::new(0.1, 0.1, 0.0, 0.0), // Phase 3: Add teleological progress
            field_coherence: None, // Phase 2: Will be set by simulation runner
        };

        self.entities
            .insert(entity_id.clone(), lifecycle_data.clone());
        self.free_will_kernels
            .insert(entity_id.clone(), free_will_kernel);
        self.evolution_trajectories
            .insert(entity_id.clone(), Vec::new());

        // Phase 3: Initialize teleological tracking for this entity
        self.initialize_teleological_tracking(&entity_id, &lifecycle_data);

        self.record_evolutionary_state(&entity_id);
        self.update_statistics();
    }

    /// Phase 2: Generate unique karmic patterns for an entity
    ///
    /// Each entity has unique karmic patterns that influence its evolutionary path.
    /// Karmic patterns represent unresolved experiences from previous incarnations
    /// that the entity seeks to resolve.
    fn generate_unique_karmic_patterns(entity_id: &EntityId) -> Vec<Layer7KarmicPattern> {
        use rand::Rng;
        use rand::SeedableRng;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Use entity ID to seed the random number generator for consistent results
        let mut hasher = DefaultHasher::new();
        entity_id.uuid.hash(&mut hasher);
        let seed = hasher.finish();
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        // Generate 1-5 karmic patterns per entity
        let num_patterns = rng.gen_range(1..=5);
        let mut patterns = Vec::new();

        for i in 0..num_patterns {
            let pattern = Layer7KarmicPattern {
                pattern_id: format!("karma-{}-{}", entity_id.uuid, i),
                intensity: rng.gen_range(0.3..0.9),
                resolution_status: ResolutionStatus::Unresolved,
            };
            patterns.push(pattern);
        }

        patterns
    }

    // ========================================================================
    // PHASE 3: INTELLIGENT EVOLUTION SYSTEM METHODS
    // ========================================================================

    /// Initialize adaptive attractors for each density
    ///
    /// Creates adaptive attractor fields for all densities, replacing
    /// static mechanical attractors with intelligent learning attractors.
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3:
    /// "Adaptive attractor fields (not static)"
    fn initialize_adaptive_attractors(&mut self) {
        // Create adaptive attractors for each density
        // Different densities have different base strengths
        // Note: Use TypesDensity from types module, not Density from density_octave module
        let densities = vec![
            (TypesDensity::First, 0.2),
            (TypesDensity::Second, 0.25),
            (TypesDensity::Third, 0.3),
            (TypesDensity::Fourth, 0.35),
            (TypesDensity::Fifth, 0.4),
            (TypesDensity::Sixth, 0.45),
            (TypesDensity::Seventh, 0.5),
            (TypesDensity::Eighth, 0.55),
        ];

        for (density, base_strength) in densities {
            let attractor = AdaptiveAttractorField::new(density, base_strength);
            self.adaptive_attractors.insert(density, attractor);
        }
    }

    /// Initialize intelligent evolution system (Phase 3)
    ///
    /// Sets up the adaptive attractors and teleological tracking for all entities.
    /// Called during manager initialization.
    pub fn initialize_intelligent_evolution(&mut self) {
        // Initialize adaptive attractors for each density
        self.initialize_adaptive_attractors();

        // Initialize teleological tracking for all existing entities
        let entity_ids: Vec<(EntityId, EntityLifecycleData)> = self
            .entities
            .iter()
            .map(|(id, data)| (id.clone(), data.clone()))
            .collect();

        for (entity_id, entity_data) in entity_ids {
            self.initialize_teleological_tracking(&entity_id, &entity_data);
        }
    }

    /// Initialize teleological tracking for a single entity
    ///
    /// Sets up teleological progress tracking for an entity.
    fn initialize_teleological_tracking(
        &mut self,
        entity_id: &EntityId,
        entity_data: &EntityLifecycleData,
    ) {
        // Create initial teleological progress assessment
        let teleological = TeleologicalProgress::new(
            0.1,                                                    // Initial purpose alignment
            0.1,                                                    // Initial coherence with source
            entity_data.current_state.polarity_state.polarity_bias, // Service orientation
            0.0,                                                    // Initial wisdom
        );

        self.teleological_tracker
            .insert(entity_id.clone(), teleological);
    }

    /// Process entity feedback and update adaptive attractors
    ///
    /// This is the core feedback loop for intelligent evolution.
    /// Entities provide feedback on attractor effectiveness, which is used
    /// to adjust attractor strength for future entities.
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3:
    /// "Feedback loop: entities → Intelligent-Infinity"
    /// "Entities provide feedback on attractor effectiveness"
    ///
    /// # Arguments
    /// * `entity_id` - Entity providing feedback
    /// * `feedback` - Feedback about attractor effectiveness
    pub fn process_entity_feedback(&mut self, entity_id: &EntityId, feedback: EntityFeedback) {
        // Get current density to determine which attractor to update
        // Note: Need to convert density_octave::Density to types::Density
        let current_density_octave = if let Some(entity_data) = self.entities.get(entity_id) {
            entity_data.current_density
        } else {
            return; // Entity not found
        };

        // Convert density_octave::Density to types::Density
        let current_density_types = match current_density_octave {
            Density::First(_) => TypesDensity::First,
            Density::Second(_) => TypesDensity::Second,
            Density::Third => TypesDensity::Third,
            Density::Fourth => TypesDensity::Fourth,
            Density::Fifth => TypesDensity::Fifth,
            Density::Sixth => TypesDensity::Sixth,
            Density::Seventh => TypesDensity::Seventh,
            Density::Eighth => TypesDensity::Eighth,
        };

        // Send feedback to adaptive attractor
        if let Some(attractor) = self.adaptive_attractors.get_mut(&current_density_types) {
            attractor.receive_feedback(feedback.clone());

            // Adjust attractor strength based on feedback
            attractor.adjust_strength();
        }

        // Send feedback to Intelligent-Infinity
        self.intelligent_infinity.receive_entity_feedback(feedback);
    }

    /// Update teleological tracking for all entities
    ///
    /// Updates the teleological progress for all entities based on their
    /// current state and evolutionary progress.
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3:
    /// "Progress tracking toward purpose"
    pub fn update_teleological_tracking(&mut self) {
        let entity_data_map: Vec<(EntityId, EntityLifecycleData)> = self
            .entities
            .iter()
            .map(|(id, data)| (id.clone(), data.clone()))
            .collect();

        for (entity_id, entity_data) in entity_data_map {
            // Calculate updated teleological progress
            let purpose_alignment = calculate_entity_purpose_alignment(&entity_data);
            let coherence_with_source = calculate_entity_coherence_with_source(&entity_data);
            let service_orientation = entity_data.current_state.polarity_state.polarity_bias;
            let wisdom_accumulated = calculate_entity_wisdom(&entity_data);

            let teleological = TeleologicalProgress::new(
                purpose_alignment,
                coherence_with_source,
                service_orientation,
                wisdom_accumulated,
            );

            self.teleological_tracker
                .insert(entity_id.clone(), teleological.clone());

            // Update entity's stored teleological progress
            if let Some(data) = self.entities.get_mut(&entity_id) {
                data.teleological_progress = teleological;
            }
        }
    }

    /// Get teleological evolution modifier for an entity
    ///
    /// Returns a modifier that boosts or reduces evolution rate based on
    /// the entity's alignment with its teleological purpose.
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3:
    /// "Coherence with purpose affects evolution"
    ///
    /// # Arguments
    /// * `entity_id` - Entity to get modifier for
    ///
    /// # Returns
    /// Evolution rate modifier (0.5 to 2.0)
    /// - 1.0 = normal rate
    /// - > 1.0 = accelerated by teleological alignment
    /// - < 1.0 = slowed by teleological misalignment
    pub fn get_teleological_modifier(&self, entity_id: &EntityId) -> f64 {
        if let Some(teleological) = self.teleological_tracker.get(entity_id) {
            // Base modifier from overall progress
            let base_modifier = 0.5 + teleological.overall_progress; // 0.5 to 1.5

            // Boost from purpose alignment
            let purpose_boost = teleological.purpose_alignment * 0.5; // 0.0 to 0.5

            // Boost from coherence with source
            let coherence_boost = teleological.coherence_with_source * 0.5; // 0.0 to 0.5

            // Combine all factors
            let modifier = base_modifier + purpose_boost + coherence_boost;

            modifier.clamp(0.5, 2.0)
        } else {
            // Default modifier if no tracking data
            1.0
        }
    }

    /// Evolve entities for a number of steps
    ///
    /// Phase 1: Parallel Independent Evolution
    ///
    /// This method now uses parallel processing to evolve entities independently.
    /// Each entity advances its own evolution clock based on its unique spectrum
    /// configuration and makes evolutionary choices when ready.
    ///
    /// Key changes from Phase 1:
    /// - Entities advance their evolution clocks independently
    /// - Entities make evolutionary choices when their clock reaches threshold
    /// - Evolution is driven by entity choices, not global steps
    /// - Spectrum configuration influences evolution speed
    pub fn evolve_entities(&mut self, steps: u64) -> EvolutionResult {
        let start_time = std::time::Instant::now();
        let mut total_transitions = 0;
        let mut spectrum_access_upgrades = 0;

        for _step in 0..steps {
            self.simulation_time += 1;

            // Phase 1: Collect entity data for serial processing (to avoid borrow issues)
            // This is a simpler approach that avoids the parallel borrowing issues
            // while still implementing independent evolution clocks

            let entity_ids: Vec<(EntityId, EntityLifecycleData)> = self
                .entities
                .iter()
                .map(|(id, data)| (id.clone(), data.clone()))
                .collect();

            // Process entities sequentially but with independent evolution clocks
            for (entity_id, entity_data) in entity_ids {
                // Calculate clock advancement for this entity
                // Each entity advances based on its own spectrum configuration
                let clock_advancement = self.calculate_clock_advancement(&entity_data);

                // Check if entity is ready to evolve
                let is_ready = self.check_evolution_readiness(&entity_data, clock_advancement);

                if is_ready {
                    // Entity is ready - make evolutionary choice and evolve
                    let result = self.evolve_single_entity_with_choice(&entity_id);
                    total_transitions += result.transitions;
                    spectrum_access_upgrades += result.spectrum_access_upgrades;
                } else {
                    // Entity not ready - just accumulate experience
                    let result = self.evolve_single_entity(&entity_id);
                    total_transitions += result.transitions;
                    spectrum_access_upgrades += result.spectrum_access_upgrades;
                }
            }

            // Phase 3: Update teleological tracking for all entities
            self.update_teleological_tracking();

            // Phase 3: Pulse Intelligent-Infinity
            self.intelligent_infinity.pulse();
        }

        self.update_statistics();

        EvolutionResult {
            steps,
            transitions: total_transitions,
            spectrum_access_upgrades,
            final_statistics: self.statistics.clone(),
            execution_time: start_time.elapsed(),
        }
    }

    /// Calculate clock advancement for an entity
    ///
    /// Phase 1: Independent Evolution
    ///
    /// Each entity advances its evolution clock based on:
    /// - Evolutionary rate (faster entities = faster clock)
    /// - Spectrum configuration (unique spectrum affects clock speed)
    /// - Consciousness level (higher consciousness = faster clock)
    fn calculate_clock_advancement(&self, entity_data: &EntityLifecycleData) -> f64 {
        // Base advancement: 1.0 per step
        let base_advancement = 1.0;

        // Evolutionary rate multiplier (0.3x to 1.7x)
        let rate_multiplier = entity_data.evolutionary_rate;

        // Spectrum configuration multiplier
        // Use the original spectrum configuration to calculate unique advancement
        let spectrum_ratio = entity_data
            .original_spectrum_configuration
            .space_time_access
            / entity_data
                .original_spectrum_configuration
                .oneness_access
                .max(0.01);
        let spectrum_multiplier = if spectrum_ratio >= 1.0 {
            1.0 + (spectrum_ratio - 1.0) * 0.1
        } else {
            0.8 + spectrum_ratio * 0.2
        };

        // Consciousness level multiplier
        let consciousness_multiplier = 1.0 + entity_data.current_state.consciousness_level * 0.5;

        // Experience accumulation multiplier
        let experience_multiplier = 1.0 + entity_data.current_state.experience_accumulation * 0.01;

        // Calculate total advancement
        base_advancement
            * rate_multiplier
            * spectrum_multiplier
            * consciousness_multiplier
            * experience_multiplier
    }

    /// Check if entity is ready to evolve
    ///
    /// Phase 1: Independent Evolution
    ///
    /// An entity is ready to evolve when its internal clock reaches a threshold.
    /// This is independent of other entities' clocks.
    fn check_evolution_readiness(
        &self,
        entity_data: &EntityLifecycleData,
        _clock_advancement: f64,
    ) -> bool {
        // For now, use a simplified check based on density octave progress
        // In a full implementation, we would track the evolution clock explicitly
        let progress = entity_data.density_octave.get_progress();

        // Check if entity has made sufficient progress
        progress >= 0.1 // Ready when progress >= 10%
    }

    // ========================================================================
    // PHASE 5: ORGANIC EVOLUTION - PROBABILISTIC TRANSITIONS
    // ========================================================================

    /// Calculate transition probability for an entity
    ///
    /// Phase 5: Organic Evolution
    ///
    /// This method calculates the probability of a successful density transition
    /// based on multiple factors that influence organic evolution.
    ///
    /// Phase 3: Added teleological modifier for purposeful evolution.
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 5:
    /// "Replace deterministic transitions with probabilistic emergence"
    ///
    /// Probability Formula:
    /// transition_probability = base_probability
    ///                       * spectrum_factor
    ///                       * catalyst_factor
    ///                       * will_factor
    ///                       * karmic_factor
    ///                       * archetype_factor
    ///                       * attractor_factor
    ///                       * teleological_factor  [Phase 3]
    ///
    /// Returns a probability value between 0.0 and 1.0.
    fn calculate_transition_probability(&self, entity_data: &EntityLifecycleData) -> f64 {
        // Base probability: 10% chance of transition if ready
        let base_probability = 0.1;

        // Spectrum factor: Entity's spectrum access ratio influences probability
        // Higher spectrum access = higher transition probability
        let spectrum_ratio = entity_data
            .original_spectrum_configuration
            .space_time_access
            / entity_data
                .original_spectrum_configuration
                .oneness_access
                .max(0.01);
        let spectrum_factor = if spectrum_ratio >= 1.0 {
            1.0 + (spectrum_ratio - 1.0) * 0.2 // Up to 1.4x for space/time dominant
        } else {
            0.8 + spectrum_ratio * 0.4 // Up to 1.2x for balanced
        };

        // Catalyst factor: Catalyst events processed increase probability
        // More catalysts = higher transition probability
        let total_experiences = entity_data.total_experiences as f64;
        let catalyst_factor = 1.0 + (total_experiences / 1000.0).min(1.0); // Up to 2.0x

        // Will factor: Free will exercises increase probability
        // Higher consciousness = stronger will = higher probability
        let consciousness_level = entity_data.current_state.consciousness_level;
        let will_factor = 1.0 + consciousness_level * 1.0; // Up to 2.0x

        // Karmic factor: Karmic pattern intensity influences probability
        // Higher karmic intensity = more intense experiences = faster evolution
        let karmic_intensity: f64 = if !entity_data.karmic_patterns.is_empty() {
            entity_data
                .karmic_patterns
                .iter()
                .map(|p| p.intensity)
                .sum::<f64>()
                / entity_data.karmic_patterns.len() as f64
        } else {
            0.0
        };
        let karmic_factor = 1.0 + karmic_intensity * 0.5; // Up to 1.5x

        // Archetype factor: Balanced archetype activation increases probability
        // Imbalanced activation reduces probability
        use crate::entity_layer7::layer7::ResolutionStatus;
        let unresolved_patterns = entity_data
            .karmic_patterns
            .iter()
            .filter(|p| p.resolution_status == ResolutionStatus::Unresolved)
            .count();
        let archetype_factor = if unresolved_patterns == 0 {
            1.2 // No unresolved patterns = balanced = higher probability
        } else {
            1.0 - (unresolved_patterns as f64 * 0.05).max(0.0) // Each unresolved pattern reduces probability
        };

        // Attractor factor: Attractor fields guide evolution
        // Get the attractor field for the current density
        let current_attractor = entity_data.current_density.get_attractor_field();
        let entity_resonance = entity_data.current_state.vibrational_state.frequency;
        let entity_polarity_bias = entity_data.current_state.polarity_state.polarity_bias;
        let entity_progress = entity_data.density_octave.get_progress();
        let attractor_factor = current_attractor.calculate_attractor_influence(
            entity_progress,
            entity_resonance,
            entity_polarity_bias,
        );

        // Phase 3: Get teleological modifier
        // Coherence with purpose increases transition probability
        let teleological_modifier = self.get_teleological_modifier(&entity_data.entity_id);

        // Phase 2: Field coherence factor
        // Entities in high-coherence holographic fields evolve faster
        let field_coherence = entity_data.field_coherence.unwrap_or(0.5);
        let coherence_modifier = 1.0 + (field_coherence - 0.5) * 0.5;
        // High coherence (+50%) accelerates evolution
        // Low coherence (-50%) decelerates evolution

        // Calculate final probability
        let final_probability = base_probability
            * spectrum_factor
            * catalyst_factor
            * will_factor
            * karmic_factor
            * archetype_factor
            * attractor_factor
            * teleological_modifier // Phase 3: Add teleological boost
            * coherence_modifier; // Phase 2: Add field coherence factor

        // Clamp probability to valid range [0.0, 1.0]
        final_probability.clamp(0.0, 1.0)
    }

    /// Attempt transition with probability roll
    ///
    /// Phase 5: Organic Evolution
    ///
    /// Attempts a density transition based on calculated probability.
    /// Returns true if transition succeeded, false otherwise.
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 5:
    /// "Replace deterministic transitions with probabilistic emergence"
    fn attempt_transition_with_probability(
        &mut self,
        entity_id: &EntityId,
        density_octave: &mut DensityOctave,
    ) -> bool {
        let entity_data = self.entities.get(entity_id).cloned().unwrap();

        // Calculate transition probability
        let probability = self.calculate_transition_probability(&entity_data);

        // Roll for transition
        let roll: f64 = rand::random();

        if roll < probability {
            // Transition succeeded - attempt the transition
            self.attempt_density_transition(entity_id, density_octave)
        } else {
            // Transition failed - entity stays at current density
            false
        }
    }

    /// Attempt non-linear transition (skip sub-levels)
    ///
    /// Phase 5: Organic Evolution
    ///
    /// Instead of only attempting the next sub-level, entities can attempt:
    /// - Next sub-level: 70% probability if ready
    /// - Skip one sub-level: 20% probability if significantly ready
    /// - Skip two sub-levels: 10% probability if extremely ready
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 5:
    /// "Enable non-linear development and 'leaps'"
    fn attempt_non_linear_transition(
        &mut self,
        entity_id: &EntityId,
        density_octave: &mut DensityOctave,
    ) -> bool {
        let entity_data = self.entities.get(entity_id).cloned().unwrap();
        let progress = entity_data.density_octave.get_progress();

        // Calculate transition readiness threshold based on current density
        let transition_readiness = density_octave.check_collective_emergence_readiness();
        let base_threshold = transition_readiness.required_progress / 100.0;

        // Calculate random choice for non-linear attempt
        let roll: f64 = rand::random();

        // Determine transition type based on progress and random roll
        let transition_type = if progress > base_threshold * 2.0 && roll < 0.1 {
            // Skip two sub-levels (quantum leap)
            2
        } else if progress > base_threshold * 1.5 && roll < 0.3 {
            // Skip one sub-level
            1
        } else if progress >= base_threshold && roll < 1.0 {
            // Next sub-level
            0
        } else {
            // Not ready for any transition
            return false;
        };

        // Attempt transition based on type
        match transition_type {
            0 => {
                // Next sub-level
                self.attempt_transition_with_probability(entity_id, density_octave)
            }
            1 => {
                // Skip one sub-level
                self.advance_density_octave_by_steps(entity_id, density_octave, 2)
            }
            2 => {
                // Skip two sub-levels (quantum leap)
                self.quantum_leap_transition(entity_id, density_octave)
            }
            _ => false,
        }
    }

    /// Advance density octave by multiple steps (for non-linear transitions)
    ///
    /// Phase 5: Organic Evolution
    ///
    /// Attempts to advance the density octave by a specified number of steps.
    /// This enables entities to skip sub-levels when ready.
    fn advance_density_octave_by_steps(
        &mut self,
        entity_id: &EntityId,
        density_octave: &mut DensityOctave,
        steps: usize,
    ) -> bool {
        let mut transitions_succeeded = 0;

        for _ in 0..steps {
            if self.attempt_transition_with_probability(entity_id, density_octave) {
                transitions_succeeded += 1;
            } else {
                break; // Stop if a transition fails
            }
        }

        transitions_succeeded == steps
    }

    /// Quantum leap transition (rapid multi-level advancement)
    ///
    /// Phase 5: Organic Evolution
    ///
    /// A quantum leap is a rare event where an entity jumps 2-3 sub-levels at once.
    /// Requires high consciousness level and high polarization strength.
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 5:
    /// "Add 'leap' mechanism for quantum jumps in evolution"
    fn quantum_leap_transition(
        &mut self,
        entity_id: &EntityId,
        density_octave: &mut DensityOctave,
    ) -> bool {
        let entity_data = self.entities.get(entity_id).cloned().unwrap();

        // Check requirements for quantum leap
        let consciousness_level = entity_data.current_state.consciousness_level;
        let polarization_strength = entity_data
            .current_state
            .polarity_state
            .polarization_strength;

        // Quantum leap requires high consciousness and polarization
        if consciousness_level < 0.5 || polarization_strength < 0.3 {
            return false;
        }

        // Quantum leap is rare - only 1% chance when requirements are met
        let leap_roll: f64 = rand::random();
        if leap_roll > 0.01 {
            return false;
        }

        // Determine leap distance (2-3 sub-levels)
        let leap_steps = if rand::random::<f64>() < 0.7 { 2 } else { 3 };

        // Attempt the leap
        self.advance_density_octave_by_steps(entity_id, density_octave, leap_steps)
    }

    /// Evolve a single entity with evolutionary choice
    ///
    /// Phase 1: Independent Decision-Making
    ///
    /// This method makes an evolutionary choice for the entity using its
    /// Free Will kernel, then processes the evolution based on that choice.
    fn evolve_single_entity_with_choice(&mut self, entity_id: &EntityId) -> SingleEvolutionResult {
        // Clone entity data to avoid borrowing issues
        let entity_data_clone = self.entities.get(entity_id).cloned().unwrap();

        // Clone free will kernel to avoid borrowing issues
        let free_will_kernel = self.free_will_kernels.get(entity_id).cloned().unwrap();

        // Make evolutionary choice using the cloned kernel
        let _choice = self.make_evolutionary_choice(&entity_data_clone, &free_will_kernel);

        // Update the free will kernel with the evolved state
        if let Some(kernel) = self.free_will_kernels.get_mut(entity_id) {
            *kernel = free_will_kernel;
        }

        // Process evolution
        self.evolve_single_entity(entity_id)
    }

    /// Make evolutionary choice for an entity
    ///
    /// Phase 1: Independent Decision-Making
    ///
    /// This uses the Free Will kernel to make a non-deterministic choice
    /// about the entity's evolutionary path.
    fn make_evolutionary_choice(
        &mut self,
        entity_data: &EntityLifecycleData,
        free_will_kernel: &crate::consciousness::free_will::FreeWillKernel,
    ) -> crate::entity_layer7::layer7::EvolutionaryChoice {
        use crate::consciousness::free_will::{ChoiceContext, PolarityPreference};

        // Determine polarity preference based on current state
        let polarity_preference = if entity_data.current_state.polarity_state.polarity_bias > 0.1 {
            PolarityPreference::ServiceToOthers
        } else if entity_data.current_state.polarity_state.polarity_bias < -0.1 {
            PolarityPreference::ServiceToSelf
        } else {
            PolarityPreference::Neutral
        };

        // Create choice context
        let context = ChoiceContext {
            polarity_preference,
            environmental_constraints: Vec::new(),
            experience_bias: entity_data.total_experiences as f64 / 1000.0,
        };

        // Exercise Free Will to make the choice (using a clone to avoid borrow issues)
        // Phase 0: Use default catalyst intensity and veil transparency
        let catalyst_intensity = 0.5;
        let veil_transparency = entity_data.current_spectrum_access.oneness_access;
        let mut kernel_clone = free_will_kernel.clone();
        let choice_result = kernel_clone.exercise_free_will(
            &entity_data.current_state,
            &context,
            catalyst_intensity,
            veil_transparency,
        );

        // Determine the evolutionary choice
        let density_octave_progress = entity_data.density_octave.get_progress();

        if density_octave_progress >= 0.25 {
            // Ready for density transition
            match choice_result.choice.sto_alignment {
                align if align > 0.5 => crate::entity_layer7::layer7::EvolutionaryChoice::AttemptDensityTransitionSTO,
                align if align < -0.5 => crate::entity_layer7::layer7::EvolutionaryChoice::AttemptDensityTransitionSTS,
                _ => crate::entity_layer7::layer7::EvolutionaryChoice::AttemptDensityTransitionNeutral,
            }
        } else {
            // Seek catalyst instead
            match choice_result.choice.sto_alignment {
                align if align > 0.3 => {
                    crate::entity_layer7::layer7::EvolutionaryChoice::SeekCatalystSTO
                }
                align if align < -0.3 => {
                    crate::entity_layer7::layer7::EvolutionaryChoice::SeekCatalystSTS
                }
                _ => crate::entity_layer7::layer7::EvolutionaryChoice::SeekCatalystNeutral,
            }
        }
    }

    /// Evolve a single entity for one step
    fn evolve_single_entity(&mut self, entity_id: &EntityId) -> SingleEvolutionResult {
        // Clone the entity data to work with it
        let lifecycle_data = self.entities.get(entity_id).cloned().unwrap();

        // Recalculate spectrum access based on current state
        let spectrum_access = self
            .spectrum_access
            .calculate_access(&lifecycle_data.current_state);

        // Use the entity's own density octave for progression
        let mut density_octave = lifecycle_data.density_octave.clone();

        // Update the entity's density octave progress using current spectrum access
        // Phase 1: Update to use new method name (was update_progress)
        // Note: The new method only takes entity_state, not spectrum_access
        density_octave.update_collective_emergence(&lifecycle_data.current_state);

        // Check transition readiness using the entity's density octave
        // Phase 1: Update to use new method name (was check_transition_readiness)
        let transition_readiness = density_octave.check_collective_emergence_readiness();

        let mut transitions = 0;
        if transition_readiness.is_ready {
            // Phase 5: Use probabilistic transition instead of deterministic transition
            // Attempt non-linear transition (may skip sub-levels)
            if self.attempt_non_linear_transition(entity_id, &mut density_octave) {
                transitions += 1;
            }
        }

        // Store the updated density octave and spectrum access back to the entity
        if let Some(data) = self.entities.get_mut(entity_id) {
            data.density_octave = density_octave;
            data.current_spectrum_access = spectrum_access;
        }

        let mut spectrum_access_upgrades = 0;
        if self.evolve_spectrum_access(entity_id) {
            spectrum_access_upgrades += 1;
        }

        // Phase 2: Apply attractor-field pull to entity
        // From COSMOLOGICAL-ARCHITECTURE.md:
        // "Evolution is the journey of being pulled by 'spiritual gravity' toward the source"
        self.apply_attractor_field_pull(entity_id);

        // Phase 3: Collect feedback on attractor effectiveness
        // After applying attractor pull, collect feedback to send to adaptive attractors
        let current_density_octave = self
            .entities
            .get(entity_id)
            .map(|e| e.current_density)
            .unwrap();

        // Convert density_octave::Density to types::Density for attractor lookup
        let current_density_types = match current_density_octave {
            Density::First(_) => TypesDensity::First,
            Density::Second(_) => TypesDensity::Second,
            Density::Third => TypesDensity::Third,
            Density::Fourth => TypesDensity::Fourth,
            Density::Fifth => TypesDensity::Fifth,
            Density::Sixth => TypesDensity::Sixth,
            Density::Seventh => TypesDensity::Seventh,
            Density::Eighth => TypesDensity::Eighth,
        };

        let attractor_pull = self
            .adaptive_attractors
            .get(&current_density_types)
            .map(|a| a.get_pull_strength())
            .unwrap_or(0.0);

        let evolution_progress = self
            .entities
            .get(entity_id)
            .map(|e| e.density_octave.get_progress())
            .unwrap_or(0.0);

        let alignment_with_attractor = self
            .entities
            .get(entity_id)
            .map(|e| e.teleological_progress.purpose_alignment)
            .unwrap_or(0.0);

        let feedback = EntityFeedback {
            entity_id: entity_id.clone(),
            attractor_pull,
            evolution_progress,
            alignment_with_attractor,
            timestamp: self.simulation_time,
        };

        // Send feedback to attractor and Intelligent-Infinity
        self.process_entity_feedback(entity_id, feedback);

        self.exercise_free_will(entity_id);
        self.accumulate_experience(entity_id);
        self.record_evolutionary_state(entity_id);

        SingleEvolutionResult {
            transitions,
            spectrum_access_upgrades,
        }
    }

    /// Attempt density transition for an entity
    ///
    /// Phase 2 Update: Now resolves karmic patterns during density transitions.
    /// Some karmic patterns are resolved during each density transition, providing
    /// bonus learning value for future evolution.
    ///
    /// Phase 3 Update: Now checks polarity requirements for density transitions.
    /// STO/STS choices affect transition success at 4th density and above.
    ///
    /// Phase 1 Update: Now handles both major density transitions (DensityTransition)
    /// and sub-level transitions (Advanced) to allow proper progression through
    /// the density octave.
    fn attempt_density_transition(
        &mut self,
        entity_id: &EntityId,
        density_octave: &mut DensityOctave,
    ) -> bool {
        let _lifecycle_data = self.entities.get(entity_id).cloned().unwrap();

        // Use the entity's density octave to attempt transition
        // Phase 1: Update to use new method name (was advance)
        // Note: The new method takes no arguments
        let transition_result = density_octave.advance_collective_emergence();

        match transition_result {
            crate::evolution_density_octave::density_octave::DensityTransitionResult::DensityTransition {
                from_density,
                to_density,
                progress_percentage,
            } => {
                // Phase 3: Check polarity requirement for the target density
                // Only applies to 4th density and above
                let target_density = self.parse_density_string(&to_density);
                if !self.check_polarity_requirement(entity_id, &target_density) {
                    // Polarity requirement not met - transition fails
                    return false;
                }
                let from_density_clone = from_density.clone();
                let to_density_clone = to_density.clone();
                let new_density = self.parse_density_string(&to_density_clone);

                if let Some(data) = self.entities.get_mut(entity_id) {
                    data.current_density = new_density;

                    // Phase 2: Resolve karmic patterns during density transitions
                    // Each density transition resolves some karmic patterns
                    let num_patterns_to_resolve = (data.karmic_patterns.len() / 2).max(1); // Resolve ~50% of patterns
                    let mut resolved_count = 0;

                    for pattern in data.karmic_patterns.iter_mut() {
                        if pattern.resolution_status == ResolutionStatus::Unresolved && resolved_count < num_patterns_to_resolve {
                            // Mark pattern as resolved
                            pattern.resolution_status = ResolutionStatus::Resolved;
                            resolved_count += 1;
                        }
                    }

                    let transition_record = DensityTransitionRecord {
                        transition_id: data.density_transitions.len() as u64,
                        from_density: from_density_clone,
                        to_density: to_density_clone,
                        timestamp: self.simulation_time,
                        progress_percentage,
                    };
                    data.density_transitions.push(transition_record);
                }
                true
            }
            crate::evolution_density_octave::density_octave::DensityTransitionResult::Advanced {
                from_density,
                to_density,
                progress_percentage,
            } => {
                // Phase 1: Handle sub-level transitions (Quantum→Atomic→Molecular→Planetary, etc.)
                let new_density = self.parse_density_string(&to_density);

                if let Some(data) = self.entities.get_mut(entity_id) {
                    data.current_density = new_density;

                    let transition_record = DensityTransitionRecord {
                        transition_id: data.density_transitions.len() as u64,
                        from_density,
                        to_density,
                        timestamp: self.simulation_time,
                        progress_percentage,
                    };
                    data.density_transitions.push(transition_record);
                }
                true
            }
            crate::evolution_density_octave::density_octave::DensityTransitionResult::NotReady { .. } => {
                false
            }
            crate::evolution_density_octave::density_octave::DensityTransitionResult::AlreadyComplete => {
                false
            }
        }
    }

    // ========================================================================
    // PHASE 3: POLARITY CONSEQUENCES
    // ========================================================================

    /// Check polarity requirement for density transition
    ///
    /// Phase 1 Update: Now uses the new polarization system.
    /// STO requires 51% intensity for harvest, STS requires 95% intensity.
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "STO choices increase service-to-others alignment"
    /// "STS choices increase service-to-self alignment"
    /// "Polarity affects density transition thresholds"
    fn check_polarity_requirement(&self, entity_id: &EntityId, target_density: &Density) -> bool {
        let lifecycle_data = self.entities.get(entity_id).unwrap();

        // Phase 1: Use new polarization system
        let polarization = &lifecycle_data.polarization;
        let intensity = polarization.intensity;
        let state = polarization.state;

        match target_density {
            // 1st and 2nd density - no polarity requirement
            Density::First(_) | Density::Second(_) => true,

            // 3rd density - minimal polarity requirement (developing self-awareness)
            Density::Third => {
                // Require at least some polarization intensity (10%+)
                intensity >= 0.1
            }

            // 4th density - harvestable status required (51%+ for STO, 95%+ for STS)
            Density::Fourth => {
                // Require harvestable status
                polarization.is_harvestable()
            }

            // 5th density - higher polarization required (75%+)
            Density::Fifth => {
                // Require higher polarization intensity
                // STO: 75%+, STS: 95%+ (same as harvest threshold)
                match state {
                    crate::polarization::PolarizationState::PolarizedSTO
                    | crate::polarization::PolarizationState::HarvestableSTO => intensity >= 0.75,
                    crate::polarization::PolarizationState::PolarizedSTS
                    | crate::polarization::PolarizationState::HarvestableSTS => intensity >= 0.95,
                    _ => false,
                }
            }

            // 6th density - very high polarization required (90%+)
            Density::Sixth => {
                // Require very high polarization intensity
                match state {
                    crate::polarization::PolarizationState::PolarizedSTO
                    | crate::polarization::PolarizationState::HarvestableSTO => intensity >= 0.90,
                    crate::polarization::PolarizationState::PolarizedSTS
                    | crate::polarization::PolarizationState::HarvestableSTS => intensity >= 0.95,
                    _ => false,
                }
            }

            // 7th density - near-complete polarization required (95%+)
            Density::Seventh => {
                // Require near-complete polarization
                match state {
                    crate::polarization::PolarizationState::PolarizedSTO
                    | crate::polarization::PolarizationState::HarvestableSTO => intensity >= 0.95,
                    crate::polarization::PolarizationState::PolarizedSTS
                    | crate::polarization::PolarizationState::HarvestableSTS => intensity >= 0.95,
                    _ => false,
                }
            }

            // 8th density - return to source, no requirement
            Density::Eighth => true,
        }
    }

    /// Evolve spectrum access for an entity
    fn evolve_spectrum_access(&mut self, entity_id: &EntityId) -> bool {
        let lifecycle_data = self.entities.get(entity_id).cloned().unwrap();
        let evolution_result = self
            .spectrum_access
            .evolve_access(&lifecycle_data.current_state);

        match evolution_result {
            crate::evolution_density_octave::spectrum_access::AccessEvolutionResult::Advanced {
                ..
            } => {
                if let Some(data) = self.entities.get_mut(entity_id) {
                    let mut spectrum_access =
                        self.spectrum_access.calculate_access(&data.current_state);

                    // Phase 7: Update spectrum access as entity evolves (veil thins)
                    spectrum_access.update_spectrum_access();

                    data.current_spectrum_access = spectrum_access;
                }
                true
            }
            _ => false,
        }
    }

    /// Exercise Free Will for an entity
    fn exercise_free_will(&mut self, entity_id: &EntityId) {
        if !self.free_will_kernels.contains_key(entity_id) {
            return;
        }

        let lifecycle_data = self.entities.get(entity_id).cloned().unwrap();
        let free_will_kernel = self.free_will_kernels.get_mut(entity_id).unwrap();

        let polarity_preference = if lifecycle_data.current_state.polarity_state.polarity_bias > 0.0
        {
            PolarityPreference::ServiceToOthers
        } else if lifecycle_data.current_state.polarity_state.polarity_bias < 0.0 {
            PolarityPreference::ServiceToSelf
        } else {
            PolarityPreference::Neutral
        };

        let context = ChoiceContext {
            polarity_preference,
            environmental_constraints: Vec::new(),
            experience_bias: lifecycle_data.total_experiences as Float / 1000.0,
        };
        let _choice_result = free_will_kernel.exercise_free_will(
            &lifecycle_data.current_state,
            &context,
            0.5,                                                   // default catalyst intensity
            lifecycle_data.current_spectrum_access.oneness_access, // veil transparency
        );

        if let Some(data) = self.entities.get_mut(entity_id) {
            data.current_state.consciousness_level = free_will_kernel.consciousness_level;

            let stats = free_will_kernel.get_statistics();
            if stats.avg_sto_alignment > 0.0 {
                data.current_state.polarity_state.polarity_bias += 0.01;
            } else if stats.avg_sto_alignment < 0.0 {
                data.current_state.polarity_state.polarity_bias -= 0.01;
            }
        }
    }

    /// Accumulate experience for an entity
    ///
    /// Phase 2 Update: Now applies karmic intensity multiplier to experience accumulation.
    /// Higher karmic intensity leads to more intense experiences and faster evolution.
    fn accumulate_experience(&mut self, entity_id: &EntityId) {
        let lifecycle_data = self.entities.get(entity_id).cloned().unwrap();
        let sto_bias = lifecycle_data.current_state.polarity_state.polarity_bias;

        // Get evolutionary rate (0.5x to 1.5x) - individual variation in evolutionary speed
        // This affects how quickly entities progress through the density octave
        let evolutionary_rate = lifecycle_data.evolutionary_rate;

        // Phase 2: Calculate total karmic intensity for this entity
        // Higher intensity karmic patterns attract more intense experiences
        let karmic_intensity: f64 = if !lifecycle_data.karmic_patterns.is_empty() {
            lifecycle_data
                .karmic_patterns
                .iter()
                .map(|p| p.intensity)
                .sum::<f64>()
                / lifecycle_data.karmic_patterns.len() as f64
        } else {
            0.0
        };

        // Phase 2: Calculate karmic intensity multiplier
        // Entities with high karmic intensity accumulate experience faster
        // Phase 1: Reduced multiplier range from 1.0-1.5 to 1.0-1.2 to slow progression
        // Multiplier ranges from 1.0 (no karmic patterns) to 1.2 (max karmic intensity)
        let karmic_multiplier = 1.0 + karmic_intensity * 0.2;

        // Density-specific experience accumulation
        // From COSMOLOGICAL-ARCHITECTURE.md:
        // - 1st density: Accumulate experience through physical interaction
        // - 2nd density: Accumulate experience through growth and survival
        // - 3rd density: Accumulate experience through polarization and choice
        // - 4th+ density: Accumulate experience through wisdom and service
        //
        // Phase 1: Density Progression - Adjusted to create correct evolutionary funnel
        // Expected distribution after 50 steps: ~60 at 1st, ~30 at 2nd, ~10 at 3rd
        // With MAX_EXPERIENCE_FOR_COMPLETE_OCTAVE = 100.0:
        // - 1st → 2nd density: need ~25 experience (25% of 100)
        // - 2nd → 3rd density: need ~50 experience (50% of 100)
        //
        // To achieve ~60 at 1st, ~30 at 2nd, ~10 at 3rd after 50 steps:
        // - Need ~60 entities with <25 experience after 50 steps
        // - Need ~30 entities with 25-50 experience after 50 steps
        // - Need ~10 entities with >50 experience after 50 steps
        //
        // Phase 1 Final: Fine-tuned rates for target funnel

        let (base_intensity, base_learning, base_consciousness) =
            match &lifecycle_data.current_density {
                crate::evolution_density_octave::density_octave::Density::First(_) => {
                    // 1st Density: Physical interaction - slower accumulation
                    // Phase 5 Fix: Adjusted to 0.25 for balanced progression (~0.3 experience/step average)
                    // Expected: ~200 steps to reach 2nd density (62.5 experience)
                    (0.25, 0.05, 0.025)
                }
                crate::evolution_density_octave::density_octave::Density::Second(_) => {
                    // 2nd Density: Growth and survival - moderate accumulation
                    // Phase 5 Fix: Adjusted to 0.35 for balanced progression (~0.42 experience/step average)
                    // Expected: ~150 steps to reach 3rd density (62.5 more experience)
                    (0.35, 0.07, 0.037)
                }
                crate::evolution_density_octave::density_octave::Density::Third => {
                    // 3rd Density: Polarity and choice - faster accumulation
                    // Phase 5 Fix: Adjusted to 0.45 for balanced progression (~0.54 experience/step average)
                    // Expected: ~125 steps to reach 4th density (62.5 more experience)
                    (0.45, 0.09, 0.054)
                }
                crate::evolution_density_octave::density_octave::Density::Fourth => {
                    // 4th Density: Understanding and love - significant accumulation
                    // Phase 5 Fix: Adjusted to 0.5
                    (0.5, 0.1, 0.06)
                }
                crate::evolution_density_octave::density_octave::Density::Fifth => {
                    // 5th Density: Wisdom and teaching - rapid accumulation
                    // Phase 5 Fix: Adjusted to 0.55
                    (0.55, 0.11, 0.075)
                }
                crate::evolution_density_octave::density_octave::Density::Sixth => {
                    // 6th Density: Unity and harmony - very rapid accumulation
                    // Phase 5 Fix: Adjusted to 0.6
                    (0.6, 0.12, 0.09)
                }
                crate::evolution_density_octave::density_octave::Density::Seventh => {
                    // 7th Density: Completion - near-instantaneous accumulation
                    // Phase 5 Fix: Adjusted to 0.65
                    (0.65, 0.13, 0.1)
                }
                crate::evolution_density_octave::density_octave::Density::Eighth => {
                    // 8th Density: Return to source - complete
                    (0.0, 0.0, 0.0)
                }
            };

        // Phase 2: Apply evolutionary rate AND karmic intensity multiplier to experience accumulation
        // Entities with 1.5x rate accumulate experience 50% faster
        // Entities with 0.5x rate accumulate experience 50% slower
        // Entities with high karmic intensity accumulate experience up to 50% faster
        let combined_multiplier = evolutionary_rate * karmic_multiplier;

        let experience = EntityExperience {
            intensity: base_intensity * combined_multiplier,
            learning_value: base_learning * combined_multiplier,
            consciousness_expansion: base_consciousness * combined_multiplier,
            frequency_shift: 0.02 * combined_multiplier,
            amplitude_change: 0.01 * combined_multiplier,
            coherence_improvement: 0.03 * combined_multiplier,
            polarity_impact: sto_bias * 0.05 * combined_multiplier,
            polarization_strength_change: 0.02 * combined_multiplier,
        };

        // Phase 2: Calculate bonus learning value from resolved karmic patterns
        let resolved_bonus: f64 = lifecycle_data
            .karmic_patterns
            .iter()
            .filter(|p| p.resolution_status == ResolutionStatus::Resolved)
            .map(|p| p.intensity * 0.2) // 20% bonus learning per resolved pattern
            .sum();

        let hypervector_components = self.create_experience_hypervector(&experience);
        let metadata = ExperienceMetadata {
            experience_id: lifecycle_data.total_experiences,
            experience_type: ExperienceType::Physical,
            timestamp: self.simulation_time,
            emotional_intensity: experience.intensity,
            sto_alignment: sto_bias,
            associated_entities: vec![],
        };

        self.holographic_memory
            .store_experience(hypervector_components, metadata);

        if let Some(data) = self.entities.get_mut(entity_id) {
            data.current_state.vibrational_state.frequency += experience.frequency_shift;
            data.current_state.vibrational_state.amplitude += experience.amplitude_change;
            data.current_state.vibrational_state.coherence += experience.coherence_improvement;
            data.current_state.polarity_state.polarity_bias += experience.polarity_impact;
            data.current_state.polarity_state.polarization_strength +=
                experience.polarization_strength_change;
            data.current_state.consciousness_level += experience.consciousness_expansion;
            // Phase 2: Add resolved karmic pattern bonus to learning progress
            data.current_state.learning_progress += experience.learning_value + resolved_bonus;
            data.current_state.experience_accumulation += experience.intensity;
            data.total_experiences += 1;
        }
    }

    /// Create experience hypervector
    fn create_experience_hypervector(&self, experience: &EntityExperience) -> Vec<Float> {
        let mut rng = rand::thread_rng();
        let dimensionality = self.holographic_memory.dimensionality;

        (0..dimensionality)
            .map(|_| {
                let base = rng.gen::<f64>();
                base * (1.0 + experience.intensity * 0.5)
                    * (1.0 + experience.polarity_impact.abs() * 0.3)
                    * (1.0 + experience.learning_value * 0.2)
            })
            .collect()
    }

    /// Record evolutionary state for an entity
    fn record_evolutionary_state(&mut self, entity_id: &EntityId) {
        let lifecycle_data = self.entities.get(entity_id).cloned().unwrap();

        let developmental_level = (lifecycle_data.current_state.consciousness_level
            + lifecycle_data.current_state.learning_progress)
            / 2.0;

        let evolutionary_state = EvolutionaryState {
            timestamp: self.simulation_time,
            density: format!("{:?}", lifecycle_data.current_density),
            density_sublevel: None,
            spectrum_access_level: lifecycle_data.current_spectrum_access.evolutionary_level,
            vibrational_state: lifecycle_data.current_state.vibrational_state.clone(),
            polarity_state: lifecycle_data.current_state.polarity_state.clone(),
            developmental_level,
            veil_thickness: 1.0 - lifecycle_data.current_spectrum_access.veil_transparency,
            consciousness_level: lifecycle_data.current_state.consciousness_level,
        };

        if let Some(trajectory) = self.evolution_trajectories.get_mut(entity_id) {
            trajectory.push(evolutionary_state);
        }
    }

    /// Apply attractor-field pull to entity
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// - "Each stage creates attractor-fields that pull toward the next level"
    /// - "Attractor-fields are the 'architectural artifacts'"
    /// - "Attractor-fields are 'spiritual gravity' that pulls the previous stage toward the next"
    /// - "Evolution is the journey of being pulled by 'spiritual gravity' toward the source"
    ///
    /// Phase 2: This method applies the "spiritual gravity" pull to entities,
    /// helping them progress toward higher densities based on their readiness.
    fn apply_attractor_field_pull(&mut self, entity_id: &EntityId) {
        // Get lifecycle data
        let lifecycle_data = self.entities.get(entity_id).cloned().unwrap();

        // Calculate attractor-field pull strength based on entity readiness
        // From COSMOLOGICAL-ARCHITECTURE.md:
        // "Attractor-field strength varies based on entity readiness"

        let polarization_intensity = lifecycle_data.polarization.intensity;
        let consciousness_level = lifecycle_data.current_state.consciousness_level;
        let experience_factor =
            (lifecycle_data.current_state.experience_accumulation / 1000.0).min(1.0);
        let learning_factor = lifecycle_data.current_state.learning_progress;
        let veil_factor = lifecycle_data.current_spectrum_access.oneness_access; // veil transparency

        // Weighted average of all factors
        let pull_strength = polarization_intensity * 0.3
            + consciousness_level * 0.2
            + experience_factor * 0.2
            + learning_factor * 0.2
            + veil_factor * 0.1;

        // Apply the pull to entity's progression metrics
        if let Some(data) = self.entities.get_mut(entity_id) {
            // Small increment per step
            let pull_increment = pull_strength * 0.01;

            // Apply pull to polarization intensity
            data.polarization.intensity =
                (data.polarization.intensity + pull_increment * 0.3).min(1.0);

            // Apply pull to consciousness level (through current_state)
            data.current_state.consciousness_level =
                (data.current_state.consciousness_level + pull_increment * 0.2).min(1.0);

            // Apply pull to experience accumulation
            data.current_state.experience_accumulation += pull_increment * 0.2;

            // Apply pull to learning progress
            data.current_state.learning_progress =
                (data.current_state.learning_progress + pull_increment * 0.2).min(1.0);
        }
    }

    /// Update lifecycle statistics
    ///
    /// Phase 3: Added teleological metrics to statistics.
    fn update_statistics(&mut self) {
        let total_entities = self.entities.len();

        if total_entities == 0 {
            return;
        }

        let mut entities_by_density = HashMap::new();
        let mut entities_by_access_level = HashMap::new();
        let mut total_consciousness = 0.0;
        let mut total_experiences = 0;
        let mut total_transitions = 0;
        let mut total_developmental = 0.0;
        let mut sto_count = 0;
        let mut sts_count = 0;
        let mut unpolarized_count = 0;
        let mut total_sto_score = 0.0;
        let mut total_sts_score = 0.0;

        // Phase 3: Teleological metrics
        let mut total_purpose_alignment = 0.0;
        let mut total_coherence_with_source = 0.0;
        let mut total_service_orientation = 0.0;

        for lifecycle_data in self.entities.values() {
            let density_key = format!("{:?}", lifecycle_data.current_density);
            *entities_by_density.entry(density_key).or_insert(0) += 1;

            let access_key = format!(
                "{:?}",
                lifecycle_data.current_spectrum_access.evolutionary_level
            );
            *entities_by_access_level.entry(access_key).or_insert(0) += 1;

            total_consciousness += lifecycle_data.current_state.consciousness_level;
            total_experiences += lifecycle_data.total_experiences;
            total_transitions += lifecycle_data.density_transitions.len();

            let developmental_level = (lifecycle_data.current_state.consciousness_level
                + lifecycle_data.current_state.learning_progress)
                / 2.0;
            total_developmental += developmental_level;

            let polarity_bias = lifecycle_data.current_state.polarity_state.polarity_bias;

            if polarity_bias > 0.1 {
                sto_count += 1;
                total_sto_score += polarity_bias;
            } else if polarity_bias < -0.1 {
                sts_count += 1;
                total_sts_score += polarity_bias.abs();
            } else {
                unpolarized_count += 1;
            }

            // Phase 3: Accumulate teleological metrics
            total_purpose_alignment += lifecycle_data.teleological_progress.purpose_alignment;
            total_coherence_with_source +=
                lifecycle_data.teleological_progress.coherence_with_source;
            total_service_orientation += lifecycle_data.teleological_progress.service_orientation;
        }

        let avg_sto_score = if sto_count > 0 {
            total_sto_score / sto_count as Float
        } else {
            0.0
        };

        let avg_sts_score = if sts_count > 0 {
            total_sts_score / sts_count as Float
        } else {
            0.0
        };

        // Phase 3: Calculate average teleological metrics
        let average_purpose_alignment = total_purpose_alignment / total_entities as f64;
        let average_coherence_with_source = total_coherence_with_source / total_entities as f64;
        let average_service_orientation = total_service_orientation / total_entities as f64;

        // Phase 3: Track attractor effectiveness
        if let Some(analysis) = &self.intelligent_infinity.latest_analysis {
            self.statistics
                .attractor_effectiveness_history
                .push(analysis.average_effectiveness);
            // Limit history size
            if self.statistics.attractor_effectiveness_history.len() > 1000 {
                self.statistics.attractor_effectiveness_history.remove(0);
            }
        }

        self.statistics = LifecycleStatistics {
            total_entities,
            entities_by_density,
            entities_by_access_level,
            avg_consciousness_level: total_consciousness / total_entities as Float,
            total_experiences,
            total_transitions,
            avg_developmental_level: total_developmental / total_entities as Float,
            polarization_distribution: PolarizationDistribution {
                sto_count,
                sts_count,
                unpolarized_count,
                avg_sto_score,
                avg_sts_score,
            },
            // Phase 3: Add teleological metrics
            average_purpose_alignment,
            average_coherence_with_source,
            average_service_orientation,
            attractor_effectiveness_history: self
                .statistics
                .attractor_effectiveness_history
                .clone(),
        };
    }

    /// Parse density string to Density enum
    fn parse_density_string(
        &self,
        density_str: &str,
    ) -> crate::evolution_density_octave::density_octave::Density {
        // Phase 1: Handle sub-level transitions properly
        // Check for sub-levels first
        if density_str.contains("Quantum") {
            crate::evolution_density_octave::density_octave::Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            )
        } else if density_str.contains("Atomic") {
            crate::evolution_density_octave::density_octave::Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Atomic,
            )
        } else if density_str.contains("Molecular") {
            crate::evolution_density_octave::density_octave::Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Molecular,
            )
        } else if density_str.contains("Planetary") {
            crate::evolution_density_octave::density_octave::Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Planetary,
            )
        } else if density_str.contains("Cellular") {
            crate::evolution_density_octave::density_octave::Density::Second(
                crate::evolution_density_octave::density_octave::Density2SubLevel::Cellular,
            )
        } else if density_str.contains("Simple Life") {
            crate::evolution_density_octave::density_octave::Density::Second(
                crate::evolution_density_octave::density_octave::Density2SubLevel::SimpleLife,
            )
        } else if density_str.contains("Complex Life") {
            crate::evolution_density_octave::density_octave::Density::Second(
                crate::evolution_density_octave::density_octave::Density2SubLevel::ComplexLife,
            )
        } else if density_str.contains("1st") {
            crate::evolution_density_octave::density_octave::Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            )
        } else if density_str.contains("2nd") {
            crate::evolution_density_octave::density_octave::Density::Second(
                crate::evolution_density_octave::density_octave::Density2SubLevel::Cellular,
            )
        } else if density_str.contains("3rd") {
            crate::evolution_density_octave::density_octave::Density::Third
        } else if density_str.contains("4th") {
            crate::evolution_density_octave::density_octave::Density::Fourth
        } else if density_str.contains("5th") {
            crate::evolution_density_octave::density_octave::Density::Fifth
        } else if density_str.contains("6th") {
            crate::evolution_density_octave::density_octave::Density::Sixth
        } else if density_str.contains("7th") {
            crate::evolution_density_octave::density_octave::Density::Seventh
        } else if density_str.contains("8th") {
            crate::evolution_density_octave::density_octave::Density::Eighth
        } else {
            crate::evolution_density_octave::density_octave::Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            )
        }
    }

    /// Get lifecycle data for an entity
    pub fn get_entity_lifecycle_data(&self, entity_id: &EntityId) -> Option<&EntityLifecycleData> {
        self.entities.get(entity_id)
    }

    /// Get evolutionary trajectory for an entity
    pub fn get_evolutionary_trajectory(
        &self,
        entity_id: &EntityId,
    ) -> Option<&Vec<EvolutionaryState>> {
        self.evolution_trajectories.get(entity_id)
    }

    /// Get memory statistics
    pub fn get_memory_statistics(&self) -> crate::memory::holographic_memory::MemoryStatistics {
        self.holographic_memory.get_statistics()
    }

    /// Get Free Will statistics for an entity
    pub fn get_free_will_statistics(
        &self,
        _entity_id: &EntityId,
    ) -> Option<crate::consciousness::free_will::FreeWillStatistics> {
        self.free_will_kernels
            .values()
            .next()
            .map(|kernel| kernel.get_statistics())
    }

    /// Get lifecycle statistics
    pub fn get_statistics(&self) -> &LifecycleStatistics {
        &self.statistics
    }

    /// Get final evolution result
    pub fn get_final_evolution_result(&self) -> EvolutionResult {
        EvolutionResult {
            steps: self.simulation_time,
            transitions: self.statistics.total_transitions,
            spectrum_access_upgrades: self.statistics.total_experiences as usize,
            final_statistics: self.statistics.clone(),
            execution_time: std::time::Duration::ZERO,
        }
    }
} // End of impl EntityLifecycleManager

// ========================================================================
// PHASE 3: TELEOLOGICAL CALCULATION HELPER FUNCTIONS
// ========================================================================

/// Calculate entity's purpose alignment
///
/// Measures how aligned the entity is with returning to source.
fn calculate_entity_purpose_alignment(entity_data: &EntityLifecycleData) -> f64 {
    // Spectrum access: More oneness (time/space) = more aligned
    let spectrum_alignment = entity_data.current_spectrum_access.oneness_access;

    // Veil transparency: Thinner veil = closer to source
    let veil_alignment = entity_data.current_spectrum_access.veil_transparency;

    // Evolutionary direction: Higher density = closer to source
    let density_progress = match &entity_data.current_density {
        Density::First(_) => 0.125,
        Density::Second(_) => 0.25,
        Density::Third => 0.375,
        Density::Fourth => 0.5,
        Density::Fifth => 0.625,
        Density::Sixth => 0.75,
        Density::Seventh => 0.875,
        Density::Eighth => 1.0,
    };

    // Consciousness level: Higher consciousness = more aligned
    let consciousness_alignment = entity_data.current_state.consciousness_level;

    // Weighted average
    spectrum_alignment * 0.3
        + veil_alignment * 0.3
        + density_progress * 0.2
        + consciousness_alignment * 0.2
}

/// Calculate entity's coherence with source
///
/// Measures resonance with Intelligent-Infinity.
fn calculate_entity_coherence_with_source(entity_data: &EntityLifecycleData) -> f64 {
    // Oneness access (time/space dominance = closer to source)
    let oneness_access = entity_data.current_spectrum_access.oneness_access;

    // Space/time access (inverse of oneness)
    let space_time_access = entity_data.current_spectrum_access.space_time_access;

    // Coherence = oneness - space/time
    let base_coherence = oneness_access - space_time_access;

    // Modulate by consciousness level (higher consciousness = higher coherence)
    let consciousness_factor = entity_data.current_state.consciousness_level;

    // Combine factors
    let coherence = (base_coherence + 1.0) / 2.0; // Normalize to 0-1
    let coherence = coherence * 0.7 + consciousness_factor * 0.3;

    coherence.clamp(0.0, 1.0)
}

/// Calculate entity's wisdom
///
/// Wisdom is integrated experience that guides future choices.
fn calculate_entity_wisdom(entity_data: &EntityLifecycleData) -> f64 {
    // Base wisdom from experience accumulation
    let base_wisdom = entity_data.current_state.experience_accumulation;

    // Learning progress increases wisdom efficiency
    let learning_factor = 1.0 + entity_data.current_state.learning_progress;

    // Consciousness level increases wisdom quality
    let consciousness_factor = 1.0 + entity_data.current_state.consciousness_level;

    // Karmic pattern resolution increases wisdom
    let resolved_patterns = entity_data
        .karmic_patterns
        .iter()
        .filter(|p| matches!(p.resolution_status, ResolutionStatus::Resolved))
        .count();
    let karmic_factor = 1.0 + (resolved_patterns as f64 * 0.1);

    // Wisdom = experience × efficiency × quality × karmic integration
    base_wisdom * learning_factor * consciousness_factor * karmic_factor
}

impl Default for EvolutionResult {
    fn default() -> Self {
        EvolutionResult {
            steps: 0,
            transitions: 0,
            spectrum_access_upgrades: 0,
            final_statistics: LifecycleStatistics::default(),
            execution_time: std::time::Duration::ZERO,
        }
    }
}

impl Default for LifecycleStatistics {
    fn default() -> Self {
        LifecycleStatistics {
            total_entities: 0,
            entities_by_density: HashMap::new(),
            entities_by_access_level: HashMap::new(),
            avg_consciousness_level: 0.0,
            total_experiences: 0,
            total_transitions: 0,
            avg_developmental_level: 0.0,
            polarization_distribution: PolarizationDistribution {
                sto_count: 0,
                sts_count: 0,
                unpolarized_count: 0,
                avg_sto_score: 0.0,
                avg_sts_score: 0.0,
            },
            // Phase 3: Add teleological metrics
            average_purpose_alignment: 0.0,
            average_coherence_with_source: 0.0,
            average_service_orientation: 0.0,
            attractor_effectiveness_history: Vec::new(),
        }
    }
}

impl Default for EntityLifecycleData {
    fn default() -> Self {
        EntityLifecycleData {
            entity_id: EntityId::new("default".to_string()),
            current_state: EntityState {
                vibrational_state: crate::entity_layer7::layer7::VibrationalState {
                    frequency: 0.5,
                    amplitude: 0.5,
                    coherence: 0.5,
                    density: crate::evolution_density_octave::density_octave::Density::First(
                        crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
                    ),
                    potential_energy: 0.5,
                    kinetic_energy: 0.5,
                },
                polarity_state: crate::entity_layer7::layer7::PolarityState {
                    polarity_bias: 0.0,
                    polarization_strength: 0.0,
                },
                consciousness_level: 0.1,
                experience_accumulation: 0.0,
                learning_progress: 0.0,
            },
            current_spectrum_access: EntitySpectrumAccess {
                space_time_access: 0.5,
                oneness_access: 0.5,
                veil_transparency: 0.5,
                evolutionary_level: SpectrumAccessLevel::ThirdDensity,
                space_time_ratio: 0.5,
                time_space_ratio: 0.5,
                spectrum_position: 0.5,
            },
            current_density: crate::evolution_density_octave::density_octave::Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            ),
            density_octave: DensityOctave::new(),
            evolutionary_trajectory: EvolutionaryTrajectory::new(),
            evolutionary_rate: 1.0,
            karmic_patterns: Vec::new(),
            total_experiences: 0,
            density_transitions: Vec::new(),
            original_spectrum_configuration: EntitySpectrumAccess {
                space_time_access: 0.5,
                oneness_access: 0.5,
                veil_transparency: 0.5,
                evolutionary_level: SpectrumAccessLevel::ThirdDensity,
                space_time_ratio: 0.5,
                time_space_ratio: 0.5,
                spectrum_position: 0.5,
            }, // Phase 1: Add default original configuration
            polarization: crate::polarization::PolarizationProgress::new(), // Phase 1: Add polarization
            teleological_progress: TeleologicalProgress::new(0.1, 0.1, 0.0, 0.0), // Phase 3: Add teleological progress
            field_coherence: None, // Phase 2: Default None
        }
    }
}

/// Single Evolution Result
///
/// Result of evolving a single entity for one step.
#[derive(Debug, Clone)]
struct SingleEvolutionResult {
    transitions: usize,
    spectrum_access_upgrades: usize,
}

#[cfg(test)]
fn create_test_entity_state() -> EntityState {
    EntityState {
        vibrational_state: crate::entity_layer7::layer7::VibrationalState {
            frequency: 0.5,
            amplitude: 0.5,
            coherence: 0.5,
            density: crate::evolution_density_octave::density_octave::Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            ),
            potential_energy: 0.5,
            kinetic_energy: 0.5,
        },
        polarity_state: crate::entity_layer7::layer7::PolarityState {
            polarity_bias: 0.3,
            polarization_strength: 0.5,
        },
        consciousness_level: 0.3,
        experience_accumulation: 0.0,
        learning_progress: 0.0,
    }
}

#[cfg(test)]
fn create_test_evolutionary_trajectory() -> EvolutionaryTrajectory {
    EvolutionaryTrajectory::new()
}

#[cfg(test)]
fn create_test_free_will_kernel() -> FreeWillKernel {
    let violet_realm = crate::foundation::VioletRealm::new();
    let intelligent_infinity = crate::foundation::IntelligentInfinity::from_violet(violet_realm);
    FreeWillKernel::new(intelligent_infinity.archetype22.clone())
}

#[test]
fn test_entity_lifecycle_manager_creation() {
    let manager = EntityLifecycleManager::new();
    assert_eq!(manager.entities.len(), 0);
    assert_eq!(manager.evolution_trajectories.len(), 0);
    assert_eq!(manager.simulation_time, 0);
}

#[test]
fn test_add_entity() {
    let mut manager = EntityLifecycleManager::new();

    let entity_id = EntityId::new("1".to_string());
    let entity_state = create_test_entity_state();
    let evolutionary_trajectory = create_test_evolutionary_trajectory();
    let free_will_kernel = create_test_free_will_kernel();

    // All entities start at First density
    use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};
    let initial_density = Density::First(Density1SubLevel::Quantum);

    // Phase 1: Add spectrum configuration parameter
    let spectrum_configuration = EntitySpectrumAccess {
        space_time_access: 0.5,
        oneness_access: 0.5,
        veil_transparency: 0.5,
        evolutionary_level: SpectrumAccessLevel::ThirdDensity,
        space_time_ratio: 1.0,
        time_space_ratio: 1.0,
        spectrum_position: 0.5,
    };

    manager.add_entity(
        entity_id.clone(),
        entity_state,
        evolutionary_trajectory,
        free_will_kernel,
        initial_density,
        spectrum_configuration,
    );

    assert_eq!(manager.entities.len(), 1);
    assert!(manager.entities.contains_key(&entity_id));
}

#[test]
fn test_evolve_entities() {
    let mut manager = EntityLifecycleManager::new();

    for i in 0..5 {
        let entity_id = EntityId::new(format!("{}", i));
        let entity_state = create_test_entity_state();
        let evolutionary_trajectory = create_test_evolutionary_trajectory();
        let free_will_kernel = create_test_free_will_kernel();
        // All entities start at First density
        use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};
        let initial_density = match i {
            0 => Density::First(Density1SubLevel::Quantum),
            1 => Density::First(Density1SubLevel::Atomic),
            2 => Density::First(Density1SubLevel::Molecular),
            3 => Density::First(Density1SubLevel::Planetary),
            _ => Density::First(Density1SubLevel::Quantum),
        };

        // Phase 1: Add spectrum configuration parameter
        let spectrum_configuration = EntitySpectrumAccess {
            space_time_access: 0.5,
            oneness_access: 0.5,
            veil_transparency: 0.5,
            evolutionary_level: SpectrumAccessLevel::ThirdDensity,
            space_time_ratio: 1.0,
            time_space_ratio: 1.0,
            spectrum_position: 0.5,
        };

        manager.add_entity(
            entity_id,
            entity_state,
            evolutionary_trajectory,
            free_will_kernel,
            initial_density,
            spectrum_configuration,
        );
    }

    let result = manager.evolve_entities(10);
    assert_eq!(result.steps, 10);
    assert!(result.final_statistics.total_experiences > 0);
}

#[test]
fn test_lifecycle_statistics() {
    let mut manager = EntityLifecycleManager::new();

    for i in 0..10 {
        let entity_id = EntityId::new(format!("{}", i));
        let entity_state = create_test_entity_state();
        let evolutionary_trajectory = create_test_evolutionary_trajectory();
        let free_will_kernel = create_test_free_will_kernel();
        // All entities start at First density
        use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};
        let initial_density = match i {
            0 | 1 => Density::First(Density1SubLevel::Quantum),
            2 | 3 => Density::First(Density1SubLevel::Atomic),
            4 | 5 => Density::First(Density1SubLevel::Molecular),
            6 | 7 => Density::First(Density1SubLevel::Planetary),
            _ => Density::First(Density1SubLevel::Quantum),
        };

        // Phase 1: Add spectrum configuration parameter
        let spectrum_configuration = EntitySpectrumAccess {
            space_time_access: 0.5,
            oneness_access: 0.5,
            veil_transparency: 0.5,
            evolutionary_level: SpectrumAccessLevel::ThirdDensity,
            space_time_ratio: 1.0,
            time_space_ratio: 1.0,
            spectrum_position: 0.5,
        };

        manager.add_entity(
            entity_id,
            entity_state,
            evolutionary_trajectory,
            free_will_kernel,
            initial_density,
            spectrum_configuration,
        );
    }

    manager.evolve_entities(20);

    let stats = &manager.statistics;
    assert_eq!(stats.total_entities, 10);
    assert!(stats.total_experiences > 0);
    assert!(stats.avg_consciousness_level > 0.0);
}

#[test]
fn test_polarization_distribution() {
    let mut manager = EntityLifecycleManager::new();

    for i in 0..10 {
        let entity_id = EntityId::new(format!("{}", i));
        let mut entity_state = create_test_entity_state();

        if i % 3 == 0 {
            entity_state.polarity_state.polarity_bias = 0.7;
        } else if i % 3 == 1 {
            entity_state.polarity_state.polarity_bias = -0.7;
        }

        let evolutionary_trajectory = create_test_evolutionary_trajectory();
        let free_will_kernel = create_test_free_will_kernel();
        // All entities start at First density
        use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};
        let initial_density = match i {
            0 | 1 | 2 => Density::First(Density1SubLevel::Quantum),
            3 | 4 | 5 => Density::First(Density1SubLevel::Atomic),
            _ => Density::First(Density1SubLevel::Molecular),
        };

        // Phase 1: Add spectrum configuration parameter
        let spectrum_configuration = EntitySpectrumAccess {
            space_time_access: 0.5,
            oneness_access: 0.5,
            veil_transparency: 0.5,
            evolutionary_level: SpectrumAccessLevel::ThirdDensity,
            space_time_ratio: 1.0,
            time_space_ratio: 1.0,
            spectrum_position: 0.5,
        };

        manager.add_entity(
            entity_id,
            entity_state,
            evolutionary_trajectory,
            free_will_kernel,
            initial_density,
            spectrum_configuration,
        );
    }

    manager.evolve_entities(10);

    let polarization = &manager.statistics.polarization_distribution;
    assert!(polarization.sto_count > 0 || polarization.sts_count > 0);
}
