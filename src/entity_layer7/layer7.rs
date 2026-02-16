// Layer 7: Sub-Sub-Logos (Individual Entity Inheritance)
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "All creation is alive, everything is a sub-sub-Logos down to the limits of observation"
//
// This module implements individual entities that:
// 1. Inherit the complete archetypical mind system from their Solar-Logos
// 2. Have unique spectrum configurations that determine their evolutionary path
// 3. Contain the holographic blueprint with the complete evolutionary trajectory
// 4. Apply "transcend and include" to all previous layers

use crate::entity_layer7::dna_encoding::DNAPattern;
use crate::entity_layer7::holographic_blueprint::HolographicBlueprint;
use crate::foundation::{
    IntelligentInfinity as IndigoRealm, LightLoveField as GreenRealm, Logos as BlueRealm,
    VioletRealm,
};
use crate::spectrum::{ArchetypicalMind, OrangeRealm, RedRealm, SpectrumRatio, YellowRealm};
use crate::types::Float;
use std::collections::HashMap;

// EntityScale enum removed - scale is now derived from current Density and Density sub-level
// This is the CORRECT architecture: density determines scale, not the other way around

/// Entity type (individual, collective, environmental, or Logos)
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// The Logos Hierarchy consists of:
/// - Galactic-scale Logoi (Orange-Ray) - create galaxy patterns
/// - Solar-scale Logoi (Red-Ray) - create solar systems and archetypical minds
/// - Individual entities (Layer 7) - inherit from Solar-Logos
///
/// Phase 4 Refactor: Added Logos types to model the hierarchical creation sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType {
    /// Galactic-scale Logos (Orange-Ray)
    ///
    /// Galactic-scale Logoi configure the spectrum at galactic scale, creating
    /// the patterns that galaxies will follow when physical matter forms.
    /// They are the first level of the Logos hierarchy.
    GalacticLogos,
    /// Solar-scale Logos (Red-Ray)
    ///
    /// Solar-scale Logoi configure the spectrum at solar-system scale and develop
    /// specific archetypical mind systems (10 or 22 archetypes). They are children
    /// of Galactic-scale Logoi and parents of individual entities.
    SolarLogos,
    /// Individual entity (particle, atom, molecule, cell, organism, being)
    ///
    /// Individual entities exist at all scales simultaneously and are composed
    /// of lower-density entities through hierarchical composition. They inherit
    /// from Solar-scale Logoi.
    Individual,
    /// Collective entity (field, ecosystem, society)
    ///
    /// Collective entities are formed from multiple individual entities and
    /// exhibit collective consciousness (the whole is more than the sum of parts).
    Collective,
    /// Environmental entity (planet, star, galaxy)
    ///
    /// Environmental entities are created from 1st Density materials and provide
    /// the substrate for higher densities to exist. Without environment, higher
    /// densities cannot emerge.
    Environmental,
}

impl EntityType {
    /// Check if this is a Galactic-scale Logos
    pub fn is_galactic_logos(&self) -> bool {
        matches!(self, EntityType::GalacticLogos)
    }

    /// Check if this is a Solar-scale Logos
    pub fn is_solar_logos(&self) -> bool {
        matches!(self, EntityType::SolarLogos)
    }

    /// Check if this is a Logos (Galactic or Solar)
    pub fn is_logos(&self) -> bool {
        matches!(self, EntityType::GalacticLogos | EntityType::SolarLogos)
    }

    /// Check if this is an individual entity
    pub fn is_individual(&self) -> bool {
        matches!(self, EntityType::Individual)
    }

    /// Check if this is a collective entity
    pub fn is_collective(&self) -> bool {
        matches!(self, EntityType::Collective)
    }

    /// Check if this is an environmental entity
    pub fn is_environmental(&self) -> bool {
        matches!(self, EntityType::Environmental)
    }

    /// Get the display name for this entity type
    pub fn display_name(&self) -> &'static str {
        match self {
            EntityType::GalacticLogos => "Galactic Logos",
            EntityType::SolarLogos => "Solar Logos",
            EntityType::Individual => "Individual",
            EntityType::Collective => "Collective",
            EntityType::Environmental => "Environmental",
        }
    }

    /// Get the scale level for this entity type
    pub fn scale_level(&self) -> usize {
        match self {
            EntityType::GalacticLogos => 0, // Highest scale
            EntityType::SolarLogos => 1,    // High scale
            EntityType::Environmental => 2, // Medium scale
            EntityType::Collective => 3,    // Lower scale
            EntityType::Individual => 4,    // Lowest scale
        }
    }
}

/// Spectrum access information
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// The Space/Time and Time/Space spectrum is a continuous range of reciprocal ratios
/// (v = s/t to v = t/s) with a qualitative break at v = 1 (the Veil).
#[derive(Debug, Clone)]
pub struct SpectrumAccess {
    /// Spectrum ratio (v = s/t or v = t/s)
    /// Values > 1.0 indicate space/time dominance
    /// Values < 1.0 indicate time/space dominance
    /// Value of 1.0 is at the Veil
    pub ratio: f64,

    /// Is the Veil active for this entity?
    /// The Veil limits access to the oneness side of the spectrum
    pub veil_active: bool,

    /// Space/time access level (0.0 to 1.0)
    /// Higher values indicate more access to space/time (separation consciousness)
    pub space_time_access: f64,

    /// Time/space access level (0.0 to 1.0)
    /// Higher values indicate more access to time/space (unity consciousness)
    pub time_space_access: f64,

    /// Mannyness access level (0.0 to 1.0)
    /// Access to the "many-ness" aspect of consciousness
    pub mannyness_access: f64,
}

impl SpectrumAccess {
    /// Create a default spectrum access
    pub fn default() -> Self {
        SpectrumAccess {
            ratio: 1.0,
            veil_active: false,
            space_time_access: 0.5,
            time_space_access: 0.5,
            mannyness_access: 0.5,
        }
    }

    /// Create spectrum access for space/time dominant entities
    pub fn space_time_dominant() -> Self {
        SpectrumAccess {
            ratio: 20.0,
            veil_active: true,
            space_time_access: 0.95,
            time_space_access: 0.05,
            mannyness_access: 0.1,
        }
    }

    /// Create spectrum access for time/space dominant entities
    pub fn time_space_dominant() -> Self {
        SpectrumAccess {
            ratio: 0.05,
            veil_active: false,
            space_time_access: 0.05,
            time_space_access: 0.95,
            mannyness_access: 0.9,
        }
    }

    /// Create spectrum access for balanced entities (near the Veil)
    pub fn balanced() -> Self {
        SpectrumAccess {
            ratio: 1.0,
            veil_active: false,
            space_time_access: 0.5,
            time_space_access: 0.5,
            mannyness_access: 0.5,
        }
    }
}

// ============================================================================
// MIGRATION 8: Choice Context and Modifier (from entity.rs)
// Phase 4.5 Migration 8: Migrated from entity.rs to entity_layer7/layer7.rs
// ============================================================================

/// Choice Context - context in which a choice is made
///
/// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
/// "Third density is the density of choice"
#[derive(Debug, Clone)]
pub struct ChoiceContext {
    /// Archetypes involved in the choice
    pub involved_archetypes: Vec<usize>,

    /// Catalyst triggering the choice
    pub catalyst: crate::entity_state::Catalyst,

    /// Environmental factors
    pub environmental_factors: HashMap<String, Float>,
}

impl ChoiceContext {
    /// Create a new choice context
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    pub fn new(
        involved_archetypes: Vec<usize>,
        catalyst: crate::entity_state::Catalyst,
        environmental_factors: HashMap<String, Float>,
    ) -> Self {
        Self {
            involved_archetypes,
            catalyst,
            environmental_factors,
        }
    }

    /// Create a minimal choice context
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    pub fn minimal(catalyst: crate::entity_state::Catalyst) -> Self {
        Self {
            involved_archetypes: Vec::new(),
            catalyst,
            environmental_factors: HashMap::new(),
        }
    }
}

/// Choice Modifier - how polarization affects choices
///
/// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
/// Knowledge Base Reference: Densities/D3. Third Density.json
/// "Third density is the density of choice"
#[derive(Debug, Clone)]
pub enum ChoiceModifier {
    /// Strengthen STO tendency
    StrengthenSTO(Float),

    /// Strengthen STS tendency
    StrengthenSTS(Float),

    /// No modification
    Neutral,
}

impl ChoiceModifier {
    /// Create a STO modifier
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    pub fn sto(intensity: Float) -> Self {
        Self::StrengthenSTO(intensity)
    }

    /// Create a STS modifier
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    pub fn sts(intensity: Float) -> Self {
        Self::StrengthenSTS(intensity)
    }

    /// Create a neutral modifier
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    pub fn neutral() -> Self {
        Self::Neutral
    }

    /// Check if modifier is STO
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    pub fn is_sto(&self) -> bool {
        matches!(self, ChoiceModifier::StrengthenSTO(_))
    }

    /// Check if modifier is STS
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    pub fn is_sts(&self) -> bool {
        matches!(self, ChoiceModifier::StrengthenSTS(_))
    }

    /// Check if modifier is neutral
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    pub fn is_neutral(&self) -> bool {
        matches!(self, ChoiceModifier::Neutral)
    }

    /// Get modifier intensity
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    pub fn intensity(&self) -> Float {
        match self {
            ChoiceModifier::StrengthenSTO(i) => *i,
            ChoiceModifier::StrengthenSTS(i) => *i,
            ChoiceModifier::Neutral => 0.0,
        }
    }
}

// ============================================================================
// END MIGRATION 8
// ============================================================================

/// Individual Entity (Sub-Sub-Logos)
///
/// Each entity inherits the complete cosmological architecture from all previous layers
/// and adds individual entity consciousness with inherited archetypical mind system.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each entity contains within it all densities and sub-densities of the octave"
///
/// This is a RESULT of the "transcend and include" principle operating at every stage.
///
/// IMPORTANT: Entity scale is now DERIVED from current Density, not stored separately.
/// This is the CORRECT architecture: density determines scale, not the other way around.
///
/// Phase 0 Refactor: Added composition and environment_id fields to model:
/// - Hierarchical composition (entities composed of other entities)
/// - Entity-environment relationships (entities exist in environments)
/// - Environment creation from 1st Density materials
#[derive(Debug, Clone)]
pub struct SubSubLogos {
    /// Unique identifier for this entity
    pub entity_id: EntityId,

    /// Entity type (individual, collective, or environmental)
    pub entity_type: EntityType,

    /// Parent entity ID (for hierarchical relationships - many individuals to one collective)
    pub parent_id: Option<EntityId>,

    /// Child entity IDs (for collective entities - tracks member entities)
    pub children: Vec<EntityId>,

    /// Composition - what entities is this entity composed of?
    ///
    /// Phase 0 Refactor: This models the hierarchical material composition:
    /// - An atom is composed of quantum particles
    /// - A molecule is composed of atoms
    /// - A cell is composed of molecules
    /// - An organism is composed of cells
    /// - A being's body is composed of organisms
    ///
    /// Each component is itself a conscious entity.
    /// The whole is more than the sum of parts (collective consciousness).
    pub composition: Vec<EntityId>,

    /// Environment ID - what environment does this entity exist in?
    ///
    /// Phase 0 Refactor: This models entity-environment relationships:
    /// - Entities exist IN environments
    /// - Environment is created from 1st Density materials
    /// - Without environment, higher densities cannot exist
    /// - Environment affects entity evolution
    ///
    /// For environmental entities (planets, stars, galaxies), this is None.
    pub environment_id: Option<EntityId>,

    /// Spectrum access information
    pub spectrum_access: SpectrumAccess,

    /// Physical entity (Phase 4: Physical Scale Integration)
    /// Links the consciousness entity to its physical manifestation
    pub physical_entity: Option<crate::matter::Matter>,

    /// INCLUDES: All previous layers
    pub violet_realm: VioletRealm,
    pub indigo_realm: IndigoRealm,
    pub blue_realm: BlueRealm,
    pub green_realm: GreenRealm,
    pub yellow_realm: YellowRealm,
    pub orange_realm: OrangeRealm,
    pub red_realm: RedRealm,

    /// TRANSCENDS: Individual entity consciousness with inherited archetypical mind system
    pub spectrum_configuration: IndividualSpectrumConfiguration,
    pub archetypical_mind: ArchetypicalMind,
    pub holographic_blueprint: HolographicBlueprint,
    pub dna_patterns: Vec<DNAPattern>,

    /// EVOLVES INTO: Attractor-field for Density Octave evolution
    pub evolutionary_attractor: EvolutionaryAttractorField,

    /// Current state of entity development
    pub current_state: EntityState,

    /// Phase 5: Individual Variation and Organic Emergence
    /// Evolutionary rate (0.5x to 1.5x) - individual variation in evolutionary speed
    pub evolutionary_rate: f64,

    /// Phase 5: Unique karmic patterns for this entity
    /// Each entity has unique karmic patterns that influence its evolutionary path
    pub karmic_patterns: Vec<Layer7KarmicPattern>,

    /// PHASE 1: Evolution Clock - Tracks individual entity evolution progress
    ///
    /// Each entity has its own evolution clock that determines when it is ready
    /// to make evolutionary choices. This enables true individualization where
    /// entities evolve independently rather than serially.
    ///
    /// The clock advances based on:
    /// - Evolutionary rate (faster entities = faster clock)
    /// - Spectrum configuration (unique spectrum affects clock speed)
    /// - Consciousness level (higher consciousness = faster clock)
    /// - Experience accumulation (more experience = faster clock)
    ///
    /// Evolution occurs when the clock reaches a threshold, not on global steps.
    pub evolution_clock: f64,

    /// Phase 1: Polarity Progression - Tracks entity's polarization journey
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Archetype 22 (The Choice): Creates polarity by choosing between Service-to-Others (STO) and Service-to-Self (STS)"
    /// "All development flows from this moment"
    /// "Polarity is required for density progression beyond 3rd density"
    ///
    /// Each entity starts unpolarized and through repeated choices (via catalyst and Free Will),
    /// develops a polarization orientation (STO or STS). When sufficiently polarized (51%+ for STO,
    /// 95%+ for STS), the entity becomes harvestable for density transition.
    pub polarization: crate::polarization::PolarizationProgress,

    // ============================================================================
    // BACKWARD COMPATIBILITY FIELDS
    // ============================================================================
    /// Current density (from vibrational state)
    pub current_density: crate::evolution_density_octave::density_octave::Density,

    /// Consciousness level (from current_state)
    pub consciousness_level: f64,

    /// Experience accumulation (from current_state)
    pub experience_accumulation: f64,

    /// Learning progress (from current_state)
    pub learning_progress: f64,

    /// Archetype activations (from archetypical mind)
    pub archetype_activations: [f64; 22],

    /// Veil transparency (from spectrum access)
    pub veil_transparency: f64,

    /// Space/time ratio (from spectrum access)
    pub space_time_ratio: f64,

    /// Time/space ratio (calculated from space_time_ratio)
    pub time_space_ratio: f64,

    /// Spectrum position (calculated from space_time_ratio)
    pub spectrum_position: f64,

    /// Potential energy (from vibrational state)
    pub potential_energy: f64,

    /// Kinetic energy (from vibrational state)
    pub kinetic_energy: f64,

    /// Total energy (potential + kinetic)
    pub energy: f64,

    /// Veil information for backward compatibility
    pub veil: VeilInfo,
}

impl SubSubLogos {
    /// Create a new Sub-Sub-Logos entity
    ///
    /// This represents the birth of an entity that inherits
    /// the complete cosmological architecture from all previous layers.
    ///
    /// IMPORTANT: All entities START at First density and EVOLVE through densities.
    /// Scale is derived from current density, not passed as a parameter.
    ///
    /// Phase 0 Refactor: Added composition and environment_id parameters to model
    /// hierarchical composition and entity-environment relationships.
    pub fn new(
        entity_id: EntityId,
        entity_type: EntityType,
        parent_id: Option<EntityId>,
        composition: Vec<EntityId>,
        environment_id: Option<EntityId>,
        violet_realm: VioletRealm,
        indigo_realm: IndigoRealm,
        blue_realm: BlueRealm,
        green_realm: GreenRealm,
        yellow_realm: YellowRealm,
        orange_realm: OrangeRealm,
        red_realm: RedRealm,
        spectrum_configuration: IndividualSpectrumConfiguration,
    ) -> Self {
        // Inherit archetypical mind system from Solar-Logos
        let archetypical_mind = red_realm.get_archetypical_mind().clone();

        // Convert ArchetypicalMind to ArchetypicalMindBlueprint
        let archetypical_mind_blueprint =
            crate::entity_layer7::holographic_blueprint::ArchetypicalMindBlueprint::from_archetypical_mind(
                &archetypical_mind
            );

        // Generate holographic blueprint based on spectrum configuration
        let holographic_blueprint = HolographicBlueprint::from_spectrum_configuration(
            &spectrum_configuration,
            &archetypical_mind_blueprint,
        );

        // Generate DNA/RNA patterns from holographic blueprint
        let dna_patterns = holographic_blueprint.generate_dna_patterns();

        // Phase 5: Generate unique karmic patterns for this entity
        let karmic_patterns = Self::generate_unique_karmic_patterns(&entity_id);

        // Create evolutionary attractor field
        let evolutionary_attractor = EvolutionaryAttractorField::new(&holographic_blueprint);

        // Initialize entity state
        let current_state = EntityState::new(&spectrum_configuration);

        // Calculate spectrum access based on entity type (scale will be derived from density)
        let spectrum_access = Self::calculate_initial_spectrum_access(&entity_type);

        // Initialize physical entity as None (will be created later in Phase 4)
        let physical_entity = None;

        // Phase 5: Generate unique evolutionary rate (0.5x to 1.5x)
        let evolutionary_rate = Self::generate_evolutionary_rate();

        // Phase 1: Initialize evolution clock
        // Each entity starts with a clock value of 0.0
        // The clock advances based on evolutionary rate and spectrum configuration
        let evolution_clock = 0.0;

        // Phase 1: Initialize polarization progression
        // All entities start unpolarized and will develop polarization through choices
        let polarization = crate::polarization::PolarizationProgress::new();

        // Get archetype activations for backward compatibility
        let archetype_activations = archetypical_mind.get_activations();

        // Calculate backward compatibility fields
        let current_density = current_state.vibrational_state.density.clone();
        let consciousness_level = current_state.consciousness_level;
        let experience_accumulation = current_state.experience_accumulation;
        let learning_progress = current_state.learning_progress;
        let veil_transparency = if spectrum_access.veil_active {
            0.0
        } else {
            spectrum_access.time_space_access
        };
        let space_time_ratio = spectrum_access.ratio;
        let time_space_ratio = if space_time_ratio > 0.0 {
            1.0 / space_time_ratio
        } else {
            0.0
        };
        let spectrum_position = if space_time_ratio > 1.0 {
            (space_time_ratio - 1.0) / 9.0
        } else if space_time_ratio < 1.0 {
            (1.0 - space_time_ratio) / 9.0
        } else {
            0.5
        };
        let potential_energy = current_state.vibrational_state.potential_energy;
        let kinetic_energy = current_state.vibrational_state.kinetic_energy;
        let energy = potential_energy + kinetic_energy;

        // Create veil info for backward compatibility
        let veil = VeilInfo {
            transparency: veil_transparency,
            active: spectrum_access.veil_active,
            illusion_strength: 1.0 - veil_transparency,
            access_control: VeilAccessControl {
                time_space_access: veil_transparency,
                holographic_connection_access: veil_transparency * 0.8,
                higher_consciousness_access: veil_transparency * 0.6,
            },
        };

        SubSubLogos {
            entity_id,
            entity_type,
            parent_id,
            children: Vec::new(),
            composition,
            environment_id,
            spectrum_access,
            physical_entity,
            violet_realm,
            indigo_realm,
            blue_realm,
            green_realm,
            yellow_realm,
            orange_realm,
            red_realm,
            spectrum_configuration,
            archetypical_mind: archetypical_mind.clone(),
            holographic_blueprint,
            dna_patterns,
            evolutionary_attractor,
            current_state,
            evolutionary_rate,
            karmic_patterns,
            evolution_clock,
            polarization,
            current_density,
            consciousness_level,
            experience_accumulation,
            learning_progress,
            archetype_activations,
            veil_transparency,
            space_time_ratio,
            time_space_ratio,
            spectrum_position,
            potential_energy,
            kinetic_energy,
            energy,
            veil,
        }
    }

    /// Phase 5: Generate unique evolutionary rate
    ///
    /// Each entity has a unique evolutionary rate that affects how quickly
    /// it progresses through the density octave.
    // Phase 1 Final: Adjusted range to 0.3x-1.7x for target funnel
    // Expected distribution after 50 steps: ~60 at 1st, ~30 at 2nd, ~10 at 3rd
    fn generate_evolutionary_rate() -> f64 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(0.3..1.7)
    }

    /// Phase 5: Generate unique karmic patterns for this entity
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

    /// Calculate initial spectrum access based on entity type
    ///
    /// IMPORTANT: Spectrum access is based on CURRENT DENSITY, not scale.
    /// All entities START at First density, so initial spectrum access is
    /// based on First density characteristics.
    ///
    /// Spectrum access will be updated as entities transition to higher densities.
    ///
    /// Phase 4 Refactor: Added Logos entity types handling.
    fn calculate_initial_spectrum_access(entity_type: &EntityType) -> SpectrumAccess {
        // All entities start at First density
        // First density entities have limited spectrum access, veil is not yet fully active
        match entity_type {
            EntityType::GalacticLogos => {
                // Galactic-scale Logoi: High spectrum access, can configure at galactic scale
                SpectrumAccess {
                    ratio: 1.0, // Balanced - can access both sides of spectrum
                    veil_active: false,
                    space_time_access: 0.8,
                    time_space_access: 0.8,
                    mannyness_access: 0.9,
                }
            }
            EntityType::SolarLogos => {
                // Solar-scale Logoi: High spectrum access, can configure at solar scale
                SpectrumAccess {
                    ratio: 1.0, // Balanced - can access both sides of spectrum
                    veil_active: false,
                    space_time_access: 0.8,
                    time_space_access: 0.8,
                    mannyness_access: 0.9,
                }
            }
            EntityType::Individual => {
                // Individual entities at First density: balanced access, veil inactive
                SpectrumAccess {
                    ratio: 1.0, // Near the Veil
                    veil_active: false,
                    space_time_access: 0.5,
                    time_space_access: 0.5,
                    mannyness_access: 0.3,
                }
            }
            EntityType::Collective => {
                // Collective entities at First density: slightly more balanced
                SpectrumAccess {
                    ratio: 1.0,
                    veil_active: false,
                    space_time_access: 0.5,
                    time_space_access: 0.5,
                    mannyness_access: 0.6,
                }
            }
            EntityType::Environmental => {
                // Environmental entities (planets, stars, galaxies) at First density:
                // More space/time dominant as they provide the physical substrate
                SpectrumAccess {
                    ratio: 5.0, // More space/time dominant
                    veil_active: false,
                    space_time_access: 0.8,
                    time_space_access: 0.2,
                    mannyness_access: 0.4,
                }
            }
        }
    }

    /// Verify holographic completeness
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Each entity contains the whole"
    ///
    /// This is a RESULT of the "transcend and include" principle.
    pub fn verify_holographic_completeness(&self) -> HolographicCompletenessReport {
        let mut report = HolographicCompletenessReport::default();

        // Check if all layers are present
        report.violet_present = true;
        report.indigo_present = true;
        report.blue_present = true;
        report.green_present = true;
        report.yellow_present = true;
        report.orange_present = true;
        report.red_present = true;

        // Check if holographic blueprint is complete
        report.blueprint_complete = self.holographic_blueprint.is_complete();

        // Check if archetypical mind is inherited
        report.archetypical_mind_inherited = true;

        // Check if DNA/RNA patterns are present
        report.dna_patterns_present = !self.dna_patterns.is_empty();

        // Check if evolutionary attractor field is present
        report.evolutionary_attractor_present = true;

        // Calculate overall completeness
        let total_checks = 11; // Fixed: was 10, should be 11 (count of all checks below)
        let passed_checks = [
            report.violet_present,
            report.indigo_present,
            report.blue_present,
            report.green_present,
            report.yellow_present,
            report.orange_present,
            report.red_present,
            report.blueprint_complete,
            report.archetypical_mind_inherited,
            report.dna_patterns_present,
            report.evolutionary_attractor_present,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        report.completeness_percentage = (passed_checks as f64 / total_checks as f64) * 100.0;

        report
    }

    /// Access the spectrum based on evolutionary state
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Evolution is not about moving from Space/Time to Time/Space—it's about
    /// accessing more of the spectrum"
    pub fn access_spectrum(&self, access_level: SpectrumAccessLevel) -> EntitySpectrumAccess {
        let _current_ratio = self.spectrum_configuration.ratio.clone();

        // Phase 4: Veil Enhancement - Use same transparency mapping as Veil::from_density()
        // From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
        // "Veil transparency varies based on density (0.0 in 3rd, 1.0 in 7th)"
        let veil_transparency = match access_level {
            SpectrumAccessLevel::ThirdDensity => 0.1, // Veil mostly active (small access)
            SpectrumAccessLevel::FourthDensity => 0.4, // Veil starts to thin
            SpectrumAccessLevel::FifthDensity => 0.7, // Veil mostly dissolved
            SpectrumAccessLevel::SixthDensity => 1.0, // Veil fully gone
            SpectrumAccessLevel::SeventhDensity => 1.0, // Veil fully dissolved
        };

        // Phase 4: Space/Time ratio is influenced by veil transparency
        // Thick veil (< 0.5 transparency): More Space/Time dominant (1.0 to 10.0)
        // Thin veil (> 0.5 transparency): More balanced or Time/Space dominant (1.0 to 0.1)
        let (space_time_ratio, time_space_ratio) = if veil_transparency < 0.5 {
            // Veil is thick, more Space/Time dominant
            // Range: 1.0 to 10.0
            let st_ratio = 1.0 + (1.0 - veil_transparency) * 9.0;
            let ts_ratio = 1.0 / st_ratio;
            (st_ratio, ts_ratio)
        } else {
            // Veil is thin, more balanced or Time/Space dominant
            // Range: 1.0 to 0.1
            let ts_ratio = 1.0 / (1.0 + veil_transparency * 9.0);
            let st_ratio = 1.0 / ts_ratio;
            (st_ratio, ts_ratio)
        };

        // Normalize ratios to ensure they sum to 1.0 for spectrum position
        let total_ratio = space_time_ratio + time_space_ratio;
        let normalized_space_time = if total_ratio > 0.0 {
            space_time_ratio / total_ratio
        } else {
            0.5 // Default to middle if both are 0
        };

        // Oneness access is proportional to veil transparency (higher transparency = more oneness)
        let oneness_access = veil_transparency;

        // Space/Time access is inverse of oneness access
        let space_time_access = 1.0 - oneness_access;

        // Spectrum position: 0.0 (pure space/time) to 1.0 (pure time/space)
        let spectrum_position = normalized_space_time;

        EntitySpectrumAccess {
            space_time_access,
            oneness_access,
            veil_transparency,
            evolutionary_level: access_level,
            space_time_ratio,
            time_space_ratio,
            spectrum_position,
        }
    }

    /// Get the evolutionary trajectory from holographic blueprint
    pub fn get_evolutionary_trajectory(&self) -> &EvolutionaryTrajectory {
        &self.holographic_blueprint.evolutionary_trajectory
    }

    /// Update entity state based on experience
    pub fn update_state(&mut self, experience: EntityExperience) {
        self.current_state.apply_experience(experience);

        // Update spectrum access based on evolutionary progress
        let access_level = self.current_state.calculate_access_level();
        let spectrum_access = self.access_spectrum(access_level);

        // Update evolutionary attractor field
        self.evolutionary_attractor
            .update(&self.current_state, &spectrum_access);
    }

    /// Check if entity is ready for density transition
    pub fn check_density_transition_readiness(&self) -> DensityTransitionReadiness {
        self.evolutionary_attractor
            .check_transition_readiness(&self.current_state)
    }

    /// Create physical entity for this entity (Phase 4: Physical Scale Integration)
    ///
    /// This creates a physical manifestation of the entity based on its CURRENT DENSITY.
    /// The physical entity is created from the consciousness patterns encoded in the
    /// holographic blueprint and spectrum configuration.
    ///
    /// IMPORTANT: Physical form EVOLVES with density:
    /// - 1st density: Quantum particles, atoms, molecules
    /// - 2nd density: Cells, organisms
    /// - 3rd density: Physical bodies (humanoids)
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 4:
    /// "Integrate matter module with entity system to track physical properties"
    pub fn create_physical_entity(&mut self) -> Result<(), String> {
        use crate::matter::{Atom, Cell, Coordinate3D, Matter, Nucleus, Particle};

        // Generate random position for the physical entity
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let position = Coordinate3D::new(
            rng.gen_range(-1000.0..1000.0),
            rng.gen_range(-1000.0..1000.0),
            rng.gen_range(-1000.0..1000.0),
        );

        // Create physical entity based on CURRENT DENSITY (not scale)
        let physical = match self.evolutionary_attractor.current_density {
            DensityLevel::First => {
                // 1st density: Quantum particles, atoms, molecules
                // Use consciousness level to determine sub-level
                let consciousness = self.current_state.consciousness_level;
                if consciousness < 0.25 {
                    // Quantum particles
                    let archetype_activation = self.generate_archetype_activation_for_density();
                    let particle = Particle::from_archetype_activation(
                        self.entity_id.uuid.parse().unwrap_or(0),
                        archetype_activation,
                        position,
                    );
                    Matter::Particle(particle)
                } else if consciousness < 0.5 {
                    // Atoms
                    let (protons, neutrons) = self.determine_atomic_properties();
                    let nucleus = Nucleus::new(protons, neutrons);
                    let atom = Atom::new(
                        protons as u32,
                        (protons + neutrons) as u32,
                        nucleus,
                        position,
                    );
                    Matter::Atom(atom)
                } else if consciousness < 0.75 {
                    // Molecules
                    let atom1 =
                        Atom::new(1, 1, Nucleus::new(1, 0), Coordinate3D::new(0.0, 0.0, 0.0));
                    let atom2 =
                        Atom::new(1, 1, Nucleus::new(1, 0), Coordinate3D::new(0.74, 0.0, 0.0));
                    let molecule = crate::matter::Molecule {
                        atoms: vec![atom1, atom2],
                        bonds: vec![(0, 1, crate::matter::BondType::Covalent)],
                    };
                    Matter::Molecule(molecule)
                } else {
                    // Planetary structures (represented as heavy atoms)
                    let protons = 92; // Uranium-like (heavy)
                    let neutrons = 146;
                    let nucleus = Nucleus::new(protons, neutrons);
                    let atom = Atom::new(
                        protons as u32,
                        (protons + neutrons) as u32,
                        nucleus,
                        position,
                    );
                    Matter::Atom(atom)
                }
            }
            DensityLevel::Second => {
                // 2nd density: Cells, organisms
                let consciousness = self.current_state.consciousness_level;
                if consciousness < 0.6 {
                    // Cellular
                    let cell = Cell {
                        organelles: vec![
                            (
                                crate::matter::OrganelleType::Nucleus,
                                "Cell nucleus".to_string(),
                            ),
                            (
                                crate::matter::OrganelleType::Mitochondria,
                                "Mitochondria".to_string(),
                            ),
                            (
                                crate::matter::OrganelleType::Ribosome,
                                "Ribosomes".to_string(),
                            ),
                        ],
                    };
                    Matter::Cell(cell)
                } else if consciousness < 0.8 {
                    // Simple Life
                    let cell = Cell {
                        organelles: vec![
                            (
                                crate::matter::OrganelleType::Nucleus,
                                "Cell nucleus".to_string(),
                            ),
                            (
                                crate::matter::OrganelleType::Mitochondria,
                                "Mitochondria".to_string(),
                            ),
                            (
                                crate::matter::OrganelleType::Ribosome,
                                "Ribosomes".to_string(),
                            ),
                            (
                                crate::matter::OrganelleType::EndoplasmicReticulum,
                                "ER".to_string(),
                            ),
                        ],
                    };
                    Matter::Cell(cell)
                } else {
                    // Complex Life
                    let cell = Cell {
                        organelles: vec![
                            (
                                crate::matter::OrganelleType::Nucleus,
                                "Cell nucleus".to_string(),
                            ),
                            (
                                crate::matter::OrganelleType::Mitochondria,
                                "Mitochondria".to_string(),
                            ),
                            (
                                crate::matter::OrganelleType::Ribosome,
                                "Ribosomes".to_string(),
                            ),
                            (
                                crate::matter::OrganelleType::EndoplasmicReticulum,
                                "ER".to_string(),
                            ),
                            (
                                crate::matter::OrganelleType::GolgiApparatus,
                                "Golgi".to_string(),
                            ),
                        ],
                    };
                    Matter::Cell(cell)
                }
            }
            DensityLevel::Third => {
                // 3rd density: Physical bodies (humanoids)
                let cell = Cell {
                    organelles: vec![
                        (
                            crate::matter::OrganelleType::Nucleus,
                            "Neuron nucleus".to_string(),
                        ),
                        (
                            crate::matter::OrganelleType::Mitochondria,
                            "Neuron mitochondria".to_string(),
                        ),
                        (
                            crate::matter::OrganelleType::Ribosome,
                            "Neuron ribosomes".to_string(),
                        ),
                    ],
                };
                Matter::Cell(cell)
            }
            _ => {
                // Higher density entities (4th+) - no physical manifestation in 3D
                return Err(format!(
                    "{} density entities have no physical manifestation in 3D space",
                    self.evolutionary_attractor.current_density
                ));
            }
        };

        self.physical_entity = Some(physical);
        Ok(())
    }

    /// Generate archetype activation pattern for current density (Phase 4 helper)
    ///
    /// Phase 2 Update: Now includes entity-specific variation using entity ID as seed,
    /// and incorporates karmic patterns to create unique archetype activation per entity.
    ///
    /// Phase 4 Update: Made public so holographic field can calculate archetype similarity.
    pub fn generate_archetype_activation_for_density(&self) -> [crate::types::Float; 22] {
        use rand::Rng;
        use rand::SeedableRng;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut activation = [0.0 as crate::types::Float; 22];

        // Use entity consciousness level to modulate archetype activation
        let consciousness = self.current_state.consciousness_level;

        // Phase 2: Use entity ID to seed the random number generator for entity-specific variation
        // This ensures that each entity has a unique but consistent archetype activation pattern
        let mut hasher = DefaultHasher::new();
        self.entity_id.uuid.hash(&mut hasher);
        let seed = hasher.finish();
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        // Base activation pattern - all archetypes have some activation
        // Phase 2: Add entity-specific random variation to base activation
        for i in 0..22 {
            let random_variation: f64 = rng.gen_range(-0.05..0.05); // ±5% variation
            activation[i] = (0.1 + consciousness * 0.5 + random_variation).clamp(0.0_f64, 1.0_f64);
        }

        // Phase 2: Calculate total karmic intensity for this entity
        let _karmic_intensity: f64 = self
            .karmic_patterns
            .iter()
            .map(|p| p.intensity)
            .sum::<f64>()
            / self.karmic_patterns.len().max(1) as f64;

        // Density-specific archetype activations
        match self.evolutionary_attractor.current_density {
            DensityLevel::First => {
                // 1st density: High Great Way (stability) and Choice
                // Phase 2: Add entity-specific variation
                let variation1: f64 = rng.gen_range(-0.10..0.10);
                activation[6] = (0.95 + variation1).clamp(0.0_f64, 1.0_f64); // A7: Great Way
                let variation2: f64 = rng.gen_range(-0.10..0.10);
                activation[13] = (0.95 + variation2).clamp(0.0_f64, 1.0_f64); // A14: Great Way
                let variation3: f64 = rng.gen_range(-0.10..0.10);
                activation[20] = (0.95 + variation3).clamp(0.0_f64, 1.0_f64); // A21: Great Way
                let variation4: f64 = rng.gen_range(-0.05..0.00);
                activation[21] = (1.0 + variation4).clamp(0.0_f64, 1.0_f64); // A22: Choice (slightly variable)
            }
            DensityLevel::Second => {
                // 2nd density: High Matrix, Catalyst, Experience
                // Phase 2: Add entity-specific variation
                let variation1: f64 = rng.gen_range(-0.05..0.00);
                activation[14] = (1.0 + variation1).clamp(0.0_f64, 1.0_f64); // A15: Matrix
                let variation2: f64 = rng.gen_range(-0.15..0.15);
                activation[15] = (0.7 + variation2).clamp(0.0_f64, 1.0_f64); // A16: Potentiator
                let variation3: f64 = rng.gen_range(-0.15..0.15);
                activation[16] = (0.8 + variation3).clamp(0.0_f64, 1.0_f64); // A17: Catalyst
                let variation4: f64 = rng.gen_range(-0.10..0.10);
                activation[17] = (0.9 + variation4).clamp(0.0_f64, 1.0_f64); // A18: Experience
            }
            DensityLevel::Third => {
                // 3rd density: High Choice and Great Way (conscious choice)
                // Phase 2: Add entity-specific variation, including polarity influence
                let polarity_bias = self.current_state.polarity_state.polarity_bias;

                // STO bias (positive) increases archetypes related to service
                // STS bias (negative) increases archetypes related to control
                let polarity_modulation = polarity_bias * 0.1;

                let variation1: f64 = rng.gen_range(-0.05..0.05);
                activation[6] = (1.0 + variation1 + polarity_modulation).clamp(0.0_f64, 1.0_f64); // A7: Great Way
                let variation2: f64 = rng.gen_range(-0.05..0.05);
                activation[13] = (1.0 + variation2 + polarity_modulation).clamp(0.0_f64, 1.0_f64); // A14: Great Way
                let variation3: f64 = rng.gen_range(-0.05..0.05);
                activation[20] = (1.0 + variation3 + polarity_modulation).clamp(0.0_f64, 1.0_f64); // A21: Great Way
                let variation4: f64 = rng.gen_range(-0.05..0.05);
                activation[21] = (1.0 + variation4).clamp(0.0_f64, 1.0_f64); // A22: Choice
            }
            _ => {
                // Higher density: Maximum activation for higher density
                // Phase 2: Add entity-specific variation even at higher densities
                for i in 0..22 {
                    let random_variation: f64 = rng.gen_range(-0.10..0.10);
                    activation[i] = (1.0 + random_variation).clamp(0.0_f64, 1.0_f64);
                }
            }
        }

        // Phase 2: Apply karmic pattern influence to archetype activation
        // Karmic patterns create specific archetype activations that attract relevant catalysts
        for (i, pattern) in self.karmic_patterns.iter().enumerate() {
            if pattern.resolution_status != ResolutionStatus::Resolved {
                // Unresolved karmic patterns create strong activations in specific archetypes
                // The archetype index is determined by the pattern ID hash
                let archetype_index = (i + (pattern.pattern_id.len() % 22)) % 22;

                // Higher intensity karmic patterns create stronger archetype activations
                let karmic_activation = pattern.intensity * 0.3; // Up to 30% additional activation
                activation[archetype_index] =
                    (activation[archetype_index] + karmic_activation).clamp(0.0_f64, 1.0_f64);
            }
        }

        // Phase 2: Apply evolutionary rate to archetype activation
        // Entities with higher evolutionary rates have more balanced archetype activation
        let rate_modulation = (self.evolutionary_rate - 1.0) * 0.05; // ±5% modulation
        for i in 0..22 {
            activation[i] = (activation[i] + rate_modulation).clamp(0.0_f64, 1.0_f64);
        }

        activation
    }

    /// Determine atomic properties (protons, neutrons) for this entity (Phase 4 helper)
    fn determine_atomic_properties(&self) -> (u8, u8) {
        let consciousness = self.current_state.consciousness_level;

        // Use consciousness level to determine atomic number
        // Higher consciousness = heavier atoms
        let base_protons = (consciousness * 118.0) as u8; // Up to oganesson (118)
        let protons = if base_protons < 1 { 1 } else { base_protons };

        // Neutrons typically similar to protons
        let neutrons = protons;

        (protons, neutrons)
    }

    // ========================================================================
    // PHASE 1: MATERIAL COMPOSITION AND HIERARCHY
    // ========================================================================

    /// Add a component entity to this entity's composition
    ///
    /// Phase 1: Material Composition Tracking
    ///
    /// This models hierarchical material composition:
    /// - An atom adds quantum particles to its composition
    /// - A molecule adds atoms to its composition
    /// - A cell adds molecules to its composition
    /// - An organism adds cells to its composition
    /// - A being's body adds organisms to its composition
    ///
    /// Each component is itself a conscious entity.
    /// The whole is more than the sum of parts (collective consciousness).
    ///
    /// # Arguments
    /// * `component_id` - The ID of the entity to add to this entity's composition
    pub fn add_component(&mut self, component_id: EntityId) {
        if !self.composition.contains(&component_id) {
            self.composition.push(component_id);
        }
    }

    /// Remove a component entity from this entity's composition
    ///
    /// Phase 1: Material Composition Tracking
    ///
    /// This models entities separating or decomposing:
    /// - A molecule might lose atoms (chemical reaction)
    /// - An organism might lose cells (damage, death)
    /// - A being might lose organisms (body loss)
    ///
    /// # Arguments
    /// * `component_id` - The ID of the entity to remove from this entity's composition
    ///
    /// # Returns
    /// * `true` if the component was found and removed, `false` otherwise
    pub fn remove_component(&mut self, component_id: &EntityId) -> bool {
        if let Some(pos) = self.composition.iter().position(|id| id == component_id) {
            self.composition.remove(pos);
            true
        } else {
            false
        }
    }

    /// Check if this entity is composed of a specific entity
    ///
    /// Phase 1: Material Composition Tracking
    ///
    /// # Arguments
    /// * `component_id` - The ID of the entity to check
    ///
    /// # Returns
    /// * `true` if this entity contains the specified component, `false` otherwise
    pub fn has_component(&self, component_id: &EntityId) -> bool {
        self.composition.contains(component_id)
    }

    /// Get the number of components in this entity's composition
    ///
    /// Phase 1: Material Composition Tracking
    ///
    /// # Returns
    /// * The number of entities in this entity's composition
    pub fn component_count(&self) -> usize {
        self.composition.len()
    }

    /// Check if this entity is composed of any entities
    ///
    /// Phase 1: Material Composition Tracking
    ///
    /// # Returns
    /// * `true` if this entity has any components, `false` otherwise
    pub fn has_components(&self) -> bool {
        !self.composition.is_empty()
    }

    /// Get a reference to this entity's composition
    ///
    /// Phase 1: Material Composition Tracking
    ///
    /// # Returns
    /// * A slice of EntityIds representing this entity's composition
    pub fn composition(&self) -> &[EntityId] {
        &self.composition
    }

    /// Replace the entire composition of this entity
    ///
    /// Phase 1: Material Composition Tracking
    ///
    /// This is used during entity creation or major reconfiguration.
    ///
    /// # Arguments
    /// * `new_composition` - The new composition for this entity
    pub fn set_composition(&mut self, new_composition: Vec<EntityId>) {
        self.composition = new_composition;
    }

    /// Calculate the total complexity of this entity based on composition
    ///
    /// Phase 1: Material Complexity Measurement
    ///
    /// Complexity is calculated as the sum of:
    /// - Direct components (entities in composition)
    /// - Indirect components (entities composed by direct components)
    ///
    /// This measures the hierarchical depth and breadth of the entity.
    ///
    /// # Arguments
    /// * `get_entity_fn` - A function that returns an entity by ID
    ///
    /// # Returns
    /// * The total complexity score for this entity
    pub fn calculate_complexity<F>(&self, get_entity_fn: &F) -> usize
    where
        F: Fn(&EntityId) -> Option<Self>,
    {
        // Direct components
        let direct_complexity = self.composition.len();

        // Indirect components (recursive)
        let indirect_complexity: usize = self
            .composition
            .iter()
            .filter_map(|component_id| get_entity_fn(component_id))
            .map(|component| component.calculate_complexity(get_entity_fn))
            .sum();

        direct_complexity + indirect_complexity
    }

    /// Check if this entity is a fundamental entity (no composition)
    ///
    /// Phase 1: Material Composition Classification
    ///
    /// Fundamental entities are the building blocks of reality.
    /// In the Law of One cosmology, quantum particles are fundamental.
    ///
    /// # Returns
    /// * `true` if this entity is fundamental (has no composition), `false` otherwise
    pub fn is_fundamental(&self) -> bool {
        self.composition.is_empty()
    }

    /// Check if this entity is a composite entity (has composition)
    ///
    /// Phase 1: Material Composition Classification
    ///
    /// Composite entities are formed from other entities.
    ///
    /// # Returns
    /// * `true` if this entity is composite (has composition), `false` otherwise
    pub fn is_composite(&self) -> bool {
        !self.composition.is_empty()
    }

    /// Get the composition depth of this entity
    ///
    /// Phase 1: Material Composition Measurement
    ///
    /// Depth measures how many levels of composition exist.
    /// - Fundamental entities: depth 0
    /// - Entities composed of fundamental entities: depth 1
    /// - Entities composed of depth-1 entities: depth 2
    /// - And so on...
    ///
    /// # Arguments
    /// * `get_entity_fn` - A function that returns an entity by ID
    ///
    /// # Returns
    /// * The composition depth of this entity
    pub fn composition_depth<F>(&self, get_entity_fn: &F) -> usize
    where
        F: Fn(&EntityId) -> Option<Self>,
    {
        if self.composition.is_empty() {
            return 0;
        }

        let max_child_depth: usize = self
            .composition
            .iter()
            .filter_map(|component_id| get_entity_fn(component_id))
            .map(|component| component.composition_depth(get_entity_fn))
            .max()
            .unwrap_or(0);

        max_child_depth + 1
    }

    // ========================================================================
    // PHASE 1: HIERARCHICAL RELATIONSHIPS
    // ========================================================================

    /// Add a child entity to this entity's children
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// This models the parent/child relationship for collective entities:
    /// - A collective entity tracks its member entities (children)
    /// - Many individual entities can have the same parent (many-to-one)
    /// - Example: Many cells belong to one organism
    ///
    /// # Arguments
    /// * `child_id` - The ID of the entity to add as a child
    pub fn add_child(&mut self, child_id: EntityId) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    /// Remove a child entity from this entity's children
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// This models entities leaving a collective:
    /// - Cells leaving an organism (death, separation)
    /// - Organisms leaving a society
    ///
    /// # Arguments
    /// * `child_id` - The ID of the entity to remove from children
    ///
    /// # Returns
    /// * `true` if the child was found and removed, `false` otherwise
    pub fn remove_child(&mut self, child_id: &EntityId) -> bool {
        if let Some(pos) = self.children.iter().position(|id| id == child_id) {
            self.children.remove(pos);
            true
        } else {
            false
        }
    }

    /// Check if this entity has a specific child
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// # Arguments
    /// * `child_id` - The ID of the entity to check
    ///
    /// # Returns
    /// * `true` if this entity contains the specified child, `false` otherwise
    pub fn has_child(&self, child_id: &EntityId) -> bool {
        self.children.contains(child_id)
    }

    /// Get the number of children of this entity
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// # Returns
    /// * The number of entities that have this entity as their parent
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Check if this entity has any children
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// # Returns
    /// * `true` if this entity has any children, `false` otherwise
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Get a reference to this entity's children
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// # Returns
    /// * A slice of EntityIds representing this entity's children
    pub fn children(&self) -> &[EntityId] {
        &self.children
    }

    /// Set the parent entity for this entity
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// This models the many-to-one relationship where many entities
    /// can have the same parent.
    ///
    /// # Arguments
    /// * `parent_id` - The ID of the parent entity (None for no parent)
    pub fn set_parent(&mut self, parent_id: Option<EntityId>) {
        self.parent_id = parent_id;
    }

    /// Get the parent entity ID
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// # Returns
    /// * The ID of the parent entity, or None if this entity has no parent
    pub fn parent(&self) -> Option<&EntityId> {
        self.parent_id.as_ref()
    }

    /// Check if this entity has a parent
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// # Returns
    /// * `true` if this entity has a parent, `false` otherwise
    pub fn has_parent(&self) -> bool {
        self.parent_id.is_some()
    }

    /// Get the hierarchical depth of this entity
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// Depth measures how many levels of parent relationships exist.
    /// - Root entities (no parent): depth 0
    /// - Children of root entities: depth 1
    /// - Grandchildren of root entities: depth 2
    /// - And so on...
    ///
    /// # Arguments
    /// * `get_entity_fn` - A function that returns an entity by ID
    ///
    /// # Returns
    /// * The hierarchical depth of this entity
    pub fn hierarchical_depth<F>(&self, get_entity_fn: &F) -> usize
    where
        F: Fn(&EntityId) -> Option<Self>,
    {
        match &self.parent_id {
            None => 0,
            Some(parent_id) => {
                if let Some(parent) = get_entity_fn(parent_id) {
                    parent.hierarchical_depth(get_entity_fn) + 1
                } else {
                    0
                }
            }
        }
    }

    /// Get all descendants of this entity
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// This recursively collects all children, grandchildren, etc.
    ///
    /// # Arguments
    /// * `get_entity_fn` - A function that returns an entity by ID
    ///
    /// # Returns
    /// * A vector of EntityIds representing all descendants
    pub fn get_descendants<F>(&self, get_entity_fn: &F) -> Vec<EntityId>
    where
        F: Fn(&EntityId) -> Option<Self>,
    {
        let mut descendants = Vec::new();

        for child_id in &self.children {
            descendants.push(child_id.clone());

            if let Some(child) = get_entity_fn(child_id) {
                descendants.extend(child.get_descendants(get_entity_fn));
            }
        }

        descendants
    }

    /// Check if this entity is a root entity (no parent)
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// # Returns
    /// * `true` if this entity is a root entity (no parent), `false` otherwise
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Check if this entity is a leaf entity (no children)
    ///
    /// Phase 1: Hierarchical Relationships
    ///
    /// # Returns
    /// * `true` if this entity is a leaf entity (no children), `false` otherwise
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Calculate the collective consciousness strength of this entity
    ///
    /// Phase 1: Collective Consciousness
    ///
    /// From Law of One: "The whole is more than the sum of parts."
    ///
    /// Collective consciousness strength is calculated as:
    /// - Base strength: number of components
    /// - Coherence multiplier: based on vibrational coherence
    /// - Emergence bonus: additional strength from hierarchical organization
    ///
    /// # Arguments
    /// * `get_entity_fn` - A function that returns an entity by ID
    ///
    /// # Returns
    /// * The collective consciousness strength (0.0 to infinity)
    pub fn collective_consciousness_strength<F>(&self, get_entity_fn: &F) -> f64
    where
        F: Fn(&EntityId) -> Option<Self>,
    {
        if self.composition.is_empty() {
            // Fundamental entities have individual consciousness strength
            return self.current_state.consciousness_level;
        }

        // Calculate base strength from components
        let base_strength: f64 = self
            .composition
            .iter()
            .filter_map(|component_id| get_entity_fn(component_id))
            .map(|component| component.collective_consciousness_strength(get_entity_fn))
            .sum();

        // Coherence multiplier (0.5 to 1.5)
        let coherence_multiplier = 0.5 + self.current_state.vibrational_state.coherence;

        // Emergence bonus based on hierarchical depth
        let emergence_bonus = 1.0 + (self.composition_depth(get_entity_fn) as f64 * 0.1);

        (base_strength * coherence_multiplier * emergence_bonus).max(0.0)
    }

    /// Update collective consciousness based on component states
    ///
    /// Phase 1: Collective Consciousness
    ///
    /// This updates the entity's consciousness based on the collective
    /// influence of its components. The whole is more than the sum of parts.
    ///
    /// # Arguments
    /// * `get_entity_fn` - A function that returns an entity by ID
    pub fn update_collective_consciousness<F>(&mut self, get_entity_fn: &F)
    where
        F: Fn(&EntityId) -> Option<Self>,
    {
        if self.composition.is_empty() {
            // Fundamental entities don't have collective consciousness to update
            return;
        }

        // Calculate average consciousness of components
        let component_consciousness: Vec<f64> = self
            .composition
            .iter()
            .filter_map(|component_id| get_entity_fn(component_id))
            .map(|component| component.current_state.consciousness_level)
            .collect();

        if !component_consciousness.is_empty() {
            let avg_component_consciousness: f64 =
                component_consciousness.iter().sum::<f64>() / component_consciousness.len() as f64;

            // The collective consciousness is higher than the average (emergence)
            let emergence_factor = 1.0 + (self.composition.len() as f64 * 0.05);

            // Blend current consciousness with collective influence
            self.current_state.consciousness_level = (self.current_state.consciousness_level * 0.7)
                + (avg_component_consciousness * emergence_factor * 0.3);
        }
    }

    // ========================================================================
    // PHASE 1: ENVIRONMENT INTERACTION
    // ========================================================================

    /// Set the environment for this entity
    ///
    /// Phase 1: Environment Interaction
    ///
    /// This models entity-environment relationships:
    /// - Entities exist IN environments
    /// - Environment is created from 1st Density materials
    /// - Without environment, higher densities cannot exist
    ///
    /// # Arguments
    /// * `environment_id` - The ID of the environment entity (None for no environment)
    pub fn set_environment(&mut self, environment_id: Option<EntityId>) {
        self.environment_id = environment_id;
    }

    /// Get the environment entity ID
    ///
    /// Phase 1: Environment Interaction
    ///
    /// # Returns
    /// * The ID of the environment entity, or None if this entity has no environment
    pub fn environment(&self) -> Option<&EntityId> {
        self.environment_id.as_ref()
    }

    /// Check if this entity has an environment
    ///
    /// Phase 1: Environment Interaction
    ///
    /// # Returns
    /// * `true` if this entity has an environment, `false` otherwise
    pub fn has_environment(&self) -> bool {
        self.environment_id.is_some()
    }

    /// Check if this entity is in a specific environment
    ///
    /// Phase 1: Environment Interaction
    ///
    /// # Arguments
    /// * `environment_id` - The ID of the environment to check
    ///
    /// # Returns
    /// * `true` if this entity is in the specified environment, `false` otherwise
    pub fn is_in_environment(&self, environment_id: &EntityId) -> bool {
        match &self.environment_id {
            Some(env_id) => env_id == environment_id,
            None => false,
        }
    }

    /// Get environmental influence on this entity
    ///
    /// Phase 1: Environment Interaction
    ///
    /// From Law of One: Environment affects entity evolution.
    /// - 1st Density environments (planets) support 2nd Density life
    /// - 2nd Density environments (ecosystems) support 3rd Density consciousness
    ///
    /// Environmental influence is calculated as:
    /// - Base influence: environment's consciousness level
    /// - Density compatibility: how well entity's density matches environment's capabilities
    /// - Coherence factor: how well entity is in harmony with environment
    ///
    /// # Arguments
    /// * `get_entity_fn` - A function that returns an entity by ID
    ///
    /// # Returns
    /// * The environmental influence factor (0.0 to 2.0)
    pub fn environmental_influence<F>(&self, get_entity_fn: &F) -> f64
    where
        F: Fn(&EntityId) -> Option<Self>,
    {
        match &self.environment_id {
            None => 0.0, // No environment = no influence
            Some(env_id) => {
                if let Some(environment) = get_entity_fn(env_id) {
                    // Base influence from environment's consciousness
                    let base_influence = environment.current_state.consciousness_level;

                    // Density compatibility (simplified - based on entity type)
                    let density_compatibility = match self.entity_type {
                        EntityType::Environmental => 1.0,
                        EntityType::Individual => 0.8,
                        EntityType::Collective => 0.9,
                        EntityType::GalacticLogos => 1.0,
                        EntityType::SolarLogos => 1.0,
                    };

                    // Coherence factor
                    let coherence_factor = self.current_state.vibrational_state.coherence;

                    (base_influence * density_compatibility * coherence_factor * 2.0).min(2.0)
                } else {
                    0.0
                }
            }
        }
    }

    /// Apply environmental influence to this entity
    ///
    /// Phase 1: Environment Interaction
    ///
    /// This updates the entity's state based on environmental influence.
    /// Environment affects:
    /// - Consciousness level (environment provides substrate for awareness)
    /// - Experience accumulation (environment provides catalysts)
    /// - Learning progress (environment provides lessons)
    ///
    /// # Arguments
    /// * `get_entity_fn` - A function that returns an entity by ID
    pub fn apply_environmental_influence<F>(&mut self, get_entity_fn: &F)
    where
        F: Fn(&EntityId) -> Option<Self>,
    {
        let influence = self.environmental_influence(get_entity_fn);

        if influence > 0.0 {
            // Environment supports consciousness expansion
            self.current_state.consciousness_level += influence * 0.01;

            // Environment provides catalysts for experience
            self.current_state.experience_accumulation += influence * 0.02;

            // Environment provides lessons for learning
            self.current_state.learning_progress += influence * 0.015;

            // Environment affects vibrational coherence
            self.current_state.vibrational_state.coherence =
                (self.current_state.vibrational_state.coherence + influence * 0.005).min(1.0);
        }
    }

    /// Get entities that affect this entity's environment
    ///
    /// Phase 1: Environment Interaction
    ///
    /// This finds all entities that compose the environment this entity exists in.
    ///
    /// # Arguments
    /// * `get_entity_fn` - A function that returns an entity by ID
    ///
    /// # Returns
    /// * A vector of EntityIds representing entities that compose the environment
    pub fn get_environmental_components<F>(&self, get_entity_fn: &F) -> Vec<EntityId>
    where
        F: Fn(&EntityId) -> Option<Self>,
    {
        match &self.environment_id {
            None => Vec::new(),
            Some(env_id) => {
                if let Some(environment) = get_entity_fn(env_id) {
                    environment.composition.clone()
                } else {
                    Vec::new()
                }
            }
        }
    }

    /// Check if this entity is compatible with its environment
    ///
    /// Phase 1: Environment Interaction
    ///
    /// Compatibility is determined by:
    /// - Entity type matches environment type
    /// - Entity's density is supported by environment's capabilities
    ///
    /// # Arguments
    /// * `get_entity_fn` - A function that returns an entity by ID
    ///
    /// # Returns
    /// * `true` if this entity is compatible with its environment, `false` otherwise
    pub fn is_environment_compatible<F>(&self, get_entity_fn: &F) -> bool
    where
        F: Fn(&EntityId) -> Option<Self>,
    {
        match &self.environment_id {
            None => false, // No environment = not compatible
            Some(env_id) => {
                if let Some(environment) = get_entity_fn(env_id) {
                    // Check if environment is Environmental type
                    if environment.entity_type != EntityType::Environmental {
                        return false;
                    }

                    // Check density compatibility (simplified)
                    // For now, all entities are compatible with their environment
                    // In future phases, this will check if entity's density matches environment's capabilities
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Move this entity to a new environment
    ///
    /// Phase 1: Environment Interaction
    ///
    /// This models entities moving between environments.
    ///
    /// # Arguments
    /// * `new_environment_id` - The ID of the new environment (None for no environment)
    pub fn move_to_environment(&mut self, new_environment_id: Option<EntityId>) {
        self.environment_id = new_environment_id;
    }

    /// Calculate environmental stability for this entity
    ///
    /// Phase 1: Environment Interaction
    ///
    /// Stability measures how stable and supportive the environment is.
    ///
    /// # Arguments
    /// * `get_entity_fn` - A function that returns an entity by ID
    ///
    /// # Returns
    /// * The environmental stability score (0.0 to 1.0)
    pub fn environmental_stability<F>(&self, get_entity_fn: &F) -> f64
    where
        F: Fn(&EntityId) -> Option<Self>,
    {
        match &self.environment_id {
            None => 0.0, // No environment = no stability
            Some(env_id) => {
                if let Some(environment) = get_entity_fn(env_id) {
                    // Stability based on environment's vibrational coherence
                    let base_stability = environment.current_state.vibrational_state.coherence;

                    // Stability increases with environment's consciousness level
                    let consciousness_factor = environment.current_state.consciousness_level * 0.5;

                    // Stability increases with environmental complexity
                    let complexity_factor = (environment.component_count() as f64 * 0.01).min(0.5);

                    (base_stability + consciousness_factor + complexity_factor).min(1.0)
                } else {
                    0.0
                }
            }
        }
    }

    // ========================================================================
    // PHASE 1: INDEPENDENT EVOLUTION AND DECISION-MAKING
    // ========================================================================

    /// Check if this entity is ready to evolve
    ///
    /// Phase 1: Independent Evolution
    ///
    /// Each entity evolves independently based on its own evolution clock.
    /// The clock advances based on:
    /// - Evolutionary rate (faster entities = faster clock)
    /// - Spectrum configuration (unique spectrum affects clock speed)
    /// - Consciousness level (higher consciousness = faster clock)
    /// - Experience accumulation (more experience = faster clock)
    ///
    /// Evolution occurs when the clock reaches a threshold (100.0), not on global steps.
    ///
    /// # Returns
    /// * `true` if the entity is ready to evolve, `false` otherwise
    pub fn is_ready_to_evolve(&self) -> bool {
        // Evolution threshold is 100.0
        // When the clock reaches this threshold, the entity is ready to make an evolutionary choice
        self.evolution_clock >= 100.0
    }

    /// Advance the evolution clock
    ///
    /// Phase 1: Independent Evolution
    ///
    /// The clock advances based on multiple factors:
    /// - Evolutionary rate (0.3x to 1.7x multiplies base advancement)
    /// - Spectrum configuration (unique ratio affects clock speed)
    /// - Consciousness level (higher consciousness = faster advancement)
    /// - Experience accumulation (more experience = faster advancement)
    /// - Karmic intensity (higher intensity = faster advancement)
    ///
    /// # Arguments
    /// * `time_step` - The time step size (typically 1.0)
    pub fn advance_evolution_clock(&mut self, time_step: f64) {
        // Base advancement: 1.0 per time step
        let base_advancement = time_step;

        // Evolutionary rate multiplier (0.3x to 1.7x)
        let rate_multiplier = self.evolutionary_rate;

        // Spectrum configuration multiplier
        // Entities with unique spectrum ratios advance at different rates
        let spectrum_ratio = self.spectrum_configuration.ratio.calculate_ratio();
        let spectrum_multiplier = if spectrum_ratio >= 1.0 {
            // Space/time dominant: faster advancement (separation consciousness)
            1.0 + (spectrum_ratio - 1.0) * 0.1
        } else {
            // Time/space dominant: slower advancement (unity consciousness)
            0.8 + spectrum_ratio * 0.2
        };

        // Consciousness level multiplier (0.0 to 1.0)
        // Higher consciousness = faster advancement
        let consciousness_multiplier = 1.0 + self.current_state.consciousness_level * 0.5;

        // Experience accumulation multiplier
        // More experience = faster advancement
        let experience_multiplier = 1.0 + self.current_state.experience_accumulation * 0.01;

        // Karmic intensity multiplier
        // Higher karmic intensity = faster advancement (more catalysts)
        let karmic_intensity: f64 = if !self.karmic_patterns.is_empty() {
            self.karmic_patterns
                .iter()
                .map(|p| p.intensity)
                .sum::<f64>()
                / self.karmic_patterns.len() as f64
        } else {
            0.0
        };
        let karmic_multiplier = 1.0 + karmic_intensity * 0.3;

        // Calculate total advancement
        let total_advancement = base_advancement
            * rate_multiplier
            * spectrum_multiplier
            * consciousness_multiplier
            * experience_multiplier
            * karmic_multiplier;

        // Advance the clock
        self.evolution_clock += total_advancement;
    }

    /// Make an evolutionary choice
    ///
    /// Phase 1: Independent Decision-Making
    ///
    /// When an entity is ready to evolve, it makes an evolutionary choice using
    /// its Free Will kernel (Archetype 22). This choice determines:
    /// - Whether to attempt a density transition
    /// - What type of catalyst to seek
    /// - How to polarize (STO or STS)
    /// - What experiences to attract
    ///
    /// This method uses the Free Will kernel to make non-deterministic choices
    /// that are not predetermined but also not random.
    ///
    /// # Arguments
    /// * `free_will_kernel` - The entity's Free Will kernel
    ///
    /// # Returns
    /// * The evolutionary choice made by the entity
    pub fn make_evolutionary_choice(
        &mut self,
        free_will_kernel: &mut crate::consciousness::free_will::FreeWillKernel,
    ) -> EvolutionaryChoice {
        use crate::consciousness::free_will::{ChoiceContext, PolarityPreference};

        // Determine polarity preference based on current state
        let polarity_preference = if self.current_state.polarity_state.polarity_bias > 0.1 {
            PolarityPreference::ServiceToOthers
        } else if self.current_state.polarity_state.polarity_bias < -0.1 {
            PolarityPreference::ServiceToSelf
        } else {
            PolarityPreference::Neutral
        };

        // Create choice context
        let context = ChoiceContext {
            polarity_preference,
            environmental_constraints: Vec::new(),
            experience_bias: self.current_state.experience_accumulation / 1000.0,
        };

        // Exercise Free Will to make the choice
        // Phase 0: Use default catalyst intensity and veil transparency for now
        let catalyst_intensity = 0.5;
        let veil_transparency = self.spectrum_access.time_space_access;
        let choice_result = free_will_kernel.exercise_free_will(
            &self.current_state,
            &context,
            catalyst_intensity,
            veil_transparency,
        );

        // Determine the evolutionary choice based on Free Will result
        let evolutionary_choice = if self.is_ready_for_density_transition() {
            // Entity is ready for density transition
            match choice_result.choice.sto_alignment {
                align if align > 0.5 => EvolutionaryChoice::AttemptDensityTransitionSTO,
                align if align < -0.5 => EvolutionaryChoice::AttemptDensityTransitionSTS,
                _ => EvolutionaryChoice::AttemptDensityTransitionNeutral,
            }
        } else {
            // Entity is not ready for density transition
            // Seek catalyst instead
            match choice_result.choice.sto_alignment {
                align if align > 0.3 => EvolutionaryChoice::SeekCatalystSTO,
                align if align < -0.3 => EvolutionaryChoice::SeekCatalystSTS,
                _ => EvolutionaryChoice::SeekCatalystNeutral,
            }
        };

        // Update entity state based on choice
        self.current_state.polarity_state.polarity_bias +=
            choice_result.choice.sto_alignment * 0.01;
        self.current_state.polarity_state.polarization_strength +=
            choice_result.choice.confidence * 0.02;

        // Reset evolution clock after making a choice
        self.evolution_clock = 0.0;

        evolutionary_choice
    }

    /// Check if entity is ready for density transition (Phase 1 helper)
    ///
    /// Phase 1: Density Transition Readiness
    ///
    /// This checks if the entity has sufficient development to attempt
    /// a density transition.
    ///
    /// # Returns
    /// * `true` if ready for density transition, `false` otherwise
    fn is_ready_for_density_transition(&self) -> bool {
        // Check consciousness level
        let consciousness_ready = self.current_state.consciousness_level >= 0.5;

        // Check learning progress
        let learning_ready = self.current_state.learning_progress >= 10.0;

        // Check polarization strength (3rd density and above)
        let polarization_ready = match self.evolutionary_attractor.current_density {
            crate::entity_layer7::layer7::DensityLevel::First
            | crate::entity_layer7::layer7::DensityLevel::Second => true,
            _ => self.current_state.polarity_state.polarization_strength >= 0.3,
        };

        consciousness_ready && learning_ready && polarization_ready
    }

    /// Get the current evolution clock value
    ///
    /// Phase 1: Independent Evolution
    ///
    /// # Returns
    /// * The current evolution clock value (0.0 to 100.0+)
    pub fn get_evolution_clock(&self) -> f64 {
        self.evolution_clock
    }

    /// Get the evolution clock progress percentage
    ///
    /// Phase 1: Independent Evolution
    ///
    /// # Returns
    /// * The progress percentage (0.0 to 1.0)
    pub fn get_evolution_clock_progress(&self) -> f64 {
        (self.evolution_clock / 100.0).min(1.0)
    }

    // ============================================================================
    // MIGRATION 8: Choice Context and Modifier Methods (from entity.rs)
    // Phase 4.5 Migration 8: Migrated from entity.rs to entity_layer7/layer7.rs
    // ============================================================================

    /// Get polarization bias for decision-making
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    ///
    /// Knowledge Base Reference: Densities/D3. Third Density.json
    /// "Third density is the density of choice"
    ///
    /// # Returns
    /// * The polarization bias value (-1.0 to 1.0)
    pub fn get_polarization_bias(&self) -> f64 {
        self.polarization.polarity_bias()
    }

    /// Apply polarization markers to decision-making
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    ///
    /// Knowledge Base Reference: Densities/D3. Third Density.json
    /// "Third density is the density of choice"
    ///
    /// This method determines how polarization affects the choices an entity makes.
    /// A strongly polarized entity will have its choices biased toward its
    /// polarity orientation.
    ///
    /// # Arguments
    /// * `context` - The context in which the choice is being made
    ///
    /// # Returns
    /// * A ChoiceModifier indicating how to modify the choice based on polarization
    pub fn apply_polarization_to_choices(&self, _context: &ChoiceContext) -> ChoiceModifier {
        // Get STO and STS intensities from polarization
        let sto_intensity = self.polarization.sto_intensity();
        let sts_intensity = self.polarization.sts_intensity();

        // Determine modifier based on polarization strength
        if sto_intensity > 0.7 {
            // Strong STO bias - strengthen STO tendency
            ChoiceModifier::StrengthenSTO(sto_intensity)
        } else if sts_intensity > 0.95 {
            // Strong STS bias - strengthen STS tendency
            // Note: STS requires 95%+ for harvest, so we use a higher threshold
            ChoiceModifier::StrengthenSTS(sts_intensity)
        } else if sto_intensity > 0.51 {
            // Moderate STO bias - slight STO strengthening
            ChoiceModifier::StrengthenSTO(sto_intensity * 0.5)
        } else if sts_intensity > 0.51 {
            // Moderate STS bias - slight STS strengthening
            ChoiceModifier::StrengthenSTS(sts_intensity * 0.5)
        } else {
            // Weak or no polarization - neutral modifier
            ChoiceModifier::Neutral
        }
    }

    /// Check if entity is harvest-ready
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    ///
    /// Knowledge Base Reference: Densities/D4. Fourth Density.json
    /// "Harvestability"
    ///
    /// An entity is harvest-ready when:
    /// - It has sufficient polarization (51%+ STO or 95%+ STS)
    /// - It has sufficient archetype activation
    /// - It has sufficient vibrational level
    /// - It has basic self-hood (for third density)
    ///
    /// # Returns
    /// * `true` if harvest-ready, `false` otherwise
    pub fn is_harvest_ready(&self) -> bool {
        // Check polarization
        let is_polarized = self.polarization.is_polarized();

        // Check vibrational level (51%+ for harvest)
        let high_vibration = self.current_state.consciousness_level >= 0.51;

        // Check archetype activation (15+ archetypes active)
        let active_archetypes = self.archetypical_mind.archetype_count();
        let sufficient_archetypes = active_archetypes >= 15;

        // Check basic self-hood (for third density)
        let has_self_hood = self.current_state.polarity_state.polarization_strength >= 0.3;

        is_polarized && high_vibration && sufficient_archetypes && has_self_hood
    }

    /// Check if entity has achieved basic self-hood
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    ///
    /// Knowledge Base Reference: Densities/D3. Third Density.json
    /// "Self-hood exploration"
    ///
    /// # Returns
    /// * `true` if entity has basic self-hood, `false` otherwise
    pub fn has_basic_self_hood(&self) -> bool {
        self.current_state.polarity_state.polarization_strength >= 0.3
    }

    /// Get overall growth progress
    ///
    /// MIGRATION_NOTE: Migrated from src/entity.rs (Phase 4.5 Migration 8)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// "Evolution is not linear - entities can spiral"
    ///
    /// # Returns
    /// * The overall growth progress (0.0 to 1.0)
    pub fn get_growth_progress(&self) -> f64 {
        // Growth progress is based on:
        // - Consciousness level (40%)
        // - Polarity strength (30%)
        // - Learning progress (20%)
        // - Experience accumulation (10%)

        let consciousness_score = self.current_state.consciousness_level * 0.4;
        let polarization_score = self.current_state.polarity_state.polarization_strength * 0.3;
        let learning_score = self.current_state.learning_progress * 0.2;
        let experience_score = (self.current_state.experience_accumulation / 1000.0).min(1.0) * 0.1;

        consciousness_score + polarization_score + learning_score + experience_score
    }

    // ============================================================================
    // BACKWARD COMPATIBILITY ACCESSORS
    // ============================================================================

    /// Get current density (returns reference to field)
    pub fn current_density(&self) -> &crate::evolution_density_octave::density_octave::Density {
        &self.current_density
    }

    /// Get consciousness level (computed from current_state)
    pub fn consciousness_level(&self) -> f64 {
        self.current_state.consciousness_level
    }

    /// Get experience accumulation (computed from current_state)
    pub fn experience_accumulation(&self) -> f64 {
        self.current_state.experience_accumulation
    }

    /// Get learning progress (computed from current_state)
    pub fn learning_progress(&self) -> f64 {
        self.current_state.learning_progress
    }

    /// Get archetype activations (from archetypical mind)
    pub fn archetype_activations(&self) -> [f64; 22] {
        self.archetypical_mind.get_activations()
    }

    /// Get archetype activations as reference
    pub fn get_archetype_activations_ref(&self) -> [f64; 22] {
        self.archetypical_mind.get_activations()
    }

    /// Get veil transparency (from spectrum access)
    pub fn veil_transparency(&self) -> f64 {
        if self.spectrum_access.veil_active {
            0.0
        } else {
            self.spectrum_access.time_space_access
        }
    }

    /// Get space/time ratio (from spectrum access)
    pub fn space_time_ratio(&self) -> f64 {
        self.spectrum_access.ratio
    }

    /// Get time/space ratio (computed from space_time_ratio)
    pub fn time_space_ratio(&self) -> f64 {
        if self.spectrum_access.ratio > 0.0 {
            1.0 / self.spectrum_access.ratio
        } else {
            0.0
        }
    }

    /// Get spectrum position (computed from space_time_ratio)
    pub fn spectrum_position(&self) -> f64 {
        if self.spectrum_access.ratio > 1.0 {
            (self.spectrum_access.ratio - 1.0) / 9.0
        } else if self.spectrum_access.ratio < 1.0 {
            (1.0 - self.spectrum_access.ratio) / 9.0
        } else {
            0.5
        }
    }

    /// Get veil (as a struct with transparency)
    pub fn veil(&self) -> VeilInfo {
        let transparency = self.veil_transparency();
        VeilInfo {
            transparency,
            active: self.spectrum_access.veil_active,
            illusion_strength: 1.0 - transparency,
            access_control: VeilAccessControl {
                time_space_access: transparency,
                holographic_connection_access: transparency * 0.8,
                higher_consciousness_access: transparency * 0.6,
            },
        }
    }

    /// Get spectrum access struct
    pub fn spectrum_access_info(&self) -> SpectrumAccessInfo {
        SpectrumAccessInfo {
            space_time_ratio: self.space_time_ratio(),
            time_space_ratio: self.time_space_ratio(),
            spectrum_position: self.spectrum_position(),
            veil_transparency: self.veil_transparency(),
        }
    }

    /// Get potential energy (derived from vibrational state)
    pub fn potential_energy(&self) -> f64 {
        self.current_state.vibrational_state.potential_energy
    }

    /// Get kinetic energy (derived from vibrational state)
    pub fn kinetic_energy(&self) -> f64 {
        self.current_state.vibrational_state.kinetic_energy
    }

    /// Get energy field (derived from vibrational state)
    pub fn energy(&self) -> f64 {
        self.current_state.vibrational_state.potential_energy
            + self.current_state.vibrational_state.kinetic_energy
    }

    /// Create a test entity for testing purposes
    ///
    /// This creates a minimal entity with default values that can be customized
    /// for testing. This is primarily used in unit tests.
    #[cfg(test)]
    pub fn create_test_entity() -> Self {
        use uuid::Uuid;

        let entity_id = EntityId {
            uuid: Uuid::new_v4().to_string(),
            incarnation_number: 1,
        };

        let entity_type = EntityType::Individual;
        let spectrum_ratio = SpectrumRatio::space_time(0.6, 0.4);
        let spectrum_configuration = IndividualSpectrumConfiguration::new(spectrum_ratio);

        // Create minimal default realms for testing
        let violet_realm = VioletRealm::default();
        let indigo_realm = IndigoRealm::default();
        let blue_realm = BlueRealm::default();
        let green_realm = GreenRealm::default();
        let yellow_realm = YellowRealm::default();
        let orange_realm = OrangeRealm::default();
        let red_realm = RedRealm::default();

        let mut entity = SubSubLogos::new(
            entity_id,
            entity_type,
            None,
            Vec::new(),
            None,
            violet_realm,
            indigo_realm,
            blue_realm,
            green_realm,
            yellow_realm,
            orange_realm,
            red_realm,
            spectrum_configuration,
        );

        // Set entity to 5th density for holographic connection tests
        // 5th density has 0.5 veil transparency, which meets the 0.3 requirement
        use crate::evolution_density_octave::density_octave::Density;
        entity.current_density = Density::Fifth;
        entity.current_state.vibrational_state.density = Density::Fifth;
        entity.veil_transparency = 0.5;

        entity
    }

    // ============================================================================
    // END MIGRATION 8
    // ============================================================================
}

/// Veil information struct for backward compatibility
#[derive(Debug, Clone)]
pub struct VeilInfo {
    pub transparency: f64,
    pub active: bool,
    pub illusion_strength: f64,
    pub access_control: VeilAccessControl,
}

/// Veil access control for backward compatibility
#[derive(Debug, Clone)]
pub struct VeilAccessControl {
    pub time_space_access: f64,
    pub holographic_connection_access: f64,
    pub higher_consciousness_access: f64,
}

/// Spectrum access information struct for backward compatibility
#[derive(Debug, Clone)]
pub struct SpectrumAccessInfo {
    pub space_time_ratio: f64,
    pub time_space_ratio: f64,
    pub spectrum_position: f64,
    pub veil_transparency: f64,
}

/// Entity identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntityId {
    pub uuid: String,
    pub incarnation_number: usize,
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.uuid, self.incarnation_number)
    }
}

/// Evolutionary Choice
///
/// Phase 1: Independent Decision-Making
///
/// Represents the choice an entity makes when it is ready to evolve.
/// This choice is made using the Free Will kernel (Archetype 22) and
/// determines the entity's evolutionary path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvolutionaryChoice {
    /// Attempt density transition with STO polarity
    AttemptDensityTransitionSTO,
    /// Attempt density transition with STS polarity
    AttemptDensityTransitionSTS,
    /// Attempt density transition with neutral polarity
    AttemptDensityTransitionNeutral,
    /// Seek catalyst with STO polarity
    SeekCatalystSTO,
    /// Seek catalyst with STS polarity
    SeekCatalystSTS,
    /// Seek catalyst with neutral polarity
    SeekCatalystNeutral,
}

impl EntityId {
    pub fn new(uuid: String) -> Self {
        EntityId {
            uuid,
            incarnation_number: 1,
        }
    }

    pub fn next_incarnation(&self) -> Self {
        EntityId {
            uuid: self.uuid.clone(),
            incarnation_number: self.incarnation_number + 1,
        }
    }

    /// Get a numeric representation of the entity ID
    ///
    /// This generates a u64 hash from the uuid for use in validation results.
    pub fn as_u64(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.uuid.hash(&mut hasher);
        self.incarnation_number.hash(&mut hasher);
        hasher.finish()
    }
}

/// Individual spectrum configuration
///
/// Each entity has unique spectrum configurations that determine their evolutionary path.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Spectrum patterns (information) exist BEFORE physical matter"
#[derive(Debug, Clone)]
pub struct IndividualSpectrumConfiguration {
    /// Unique spectrum ratio for this entity
    pub ratio: SpectrumRatio,

    /// Individual-scale spectrum configuration
    pub localized_configuration: LocalizedConfiguration,

    /// Evolutionary parameters encoded in spectrum
    pub evolutionary_parameters: EvolutionaryParameters,

    /// Consciousness-first encoding (information before matter)
    pub consciousness_first_encoding: ConsciousnessFirstEncoding,
}

impl IndividualSpectrumConfiguration {
    pub fn new(ratio: SpectrumRatio) -> Self {
        let ratio_clone = ratio.clone();
        IndividualSpectrumConfiguration {
            ratio,
            localized_configuration: LocalizedConfiguration::new(&ratio_clone),
            evolutionary_parameters: EvolutionaryParameters::default(),
            consciousness_first_encoding: ConsciousnessFirstEncoding::new(),
        }
    }
}

/// Localized configuration for individual entity
#[derive(Debug, Clone)]
pub struct LocalizedConfiguration {
    /// Local space/time ratio
    pub space_time_ratio: f64,

    /// Local time/space ratio
    pub time_space_ratio: f64,

    /// Unique coordinate in galactic spectrum
    pub galactic_coordinate: SpectrumCoordinate,

    /// Unique coordinate in solar spectrum
    pub solar_coordinate: SpectrumCoordinate,
}

impl LocalizedConfiguration {
    fn new(ratio: &SpectrumRatio) -> Self {
        LocalizedConfiguration {
            space_time_ratio: if ratio.calculate_ratio() >= 1.0 {
                ratio.calculate_ratio()
            } else {
                1.0 / ratio.calculate_ratio()
            },
            time_space_ratio: if ratio.calculate_ratio() >= 1.0 {
                1.0 / ratio.calculate_ratio()
            } else {
                ratio.calculate_ratio()
            },
            galactic_coordinate: SpectrumCoordinate::random(),
            solar_coordinate: SpectrumCoordinate::random(),
        }
    }
}

/// Coordinate in spectrum
#[derive(Debug, Clone)]
pub struct SpectrumCoordinate {
    pub dimension: usize,
    pub coordinate: [f64; 3],
}

impl SpectrumCoordinate {
    fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        SpectrumCoordinate {
            dimension: rng.gen_range(0..8),
            coordinate: [rng.gen(), rng.gen(), rng.gen()],
        }
    }
}

/// Evolutionary parameters encoded in spectrum
#[derive(Debug, Clone)]
pub struct EvolutionaryParameters {
    /// Polarity bias (service-to-others or service-to-self)
    pub polarity_bias: f64, // -1.0 to 1.0

    /// Learning rate
    pub learning_rate: f64,

    /// Evolutionary potential
    pub evolutionary_potential: f64,

    /// Karmic patterns encoded in spectrum
    pub karmic_patterns: Vec<Layer7KarmicPattern>,
}

impl Default for EvolutionaryParameters {
    fn default() -> Self {
        EvolutionaryParameters {
            polarity_bias: 0.0,
            learning_rate: 0.5,
            evolutionary_potential: 1.0,
            karmic_patterns: Vec::new(),
        }
    }
}

/// Karmic pattern encoded in spectrum
#[derive(Debug, Clone)]
pub struct Layer7KarmicPattern {
    pub pattern_id: String,
    pub intensity: f64,
    pub resolution_status: ResolutionStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolutionStatus {
    Unresolved,
    InProgress,
    Resolved,
}

/// Consciousness-first encoding
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Spectrum patterns (information) exist BEFORE physical matter"
///
/// The physical universe is the manifestation of pre-existing spectrum configuration.
#[derive(Debug, Clone)]
pub struct ConsciousnessFirstEncoding {
    /// Information encoding (primary)
    pub information_encoding: Vec<InformationPattern>,

    /// Physical manifestation encoding (secondary, unfolds from information)
    pub physical_manifestation_encoding: Vec<PhysicalPattern>,
}

impl ConsciousnessFirstEncoding {
    fn new() -> Self {
        ConsciousnessFirstEncoding {
            information_encoding: Vec::new(),
            physical_manifestation_encoding: Vec::new(),
        }
    }
}

/// Information pattern (primary encoding)
#[derive(Debug, Clone)]
pub struct InformationPattern {
    pub pattern_id: String,
    pub information_density: f64,
    pub holographic_signature: Vec<f64>,
}

/// Physical pattern (secondary encoding, unfolds from information)
#[derive(Debug, Clone)]
pub struct PhysicalPattern {
    pub pattern_id: String,
    pub matter_density: f64,
    pub energy_signature: Vec<f64>,
}

/// Evolutionary attractor field
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each stage creates attractor-fields—'spiritual gravity' that pulls the
/// previous stage toward the next level of vibration/frequency"
#[derive(Debug, Clone)]
pub struct EvolutionaryAttractorField {
    /// Attractor strength (pulls toward next density)
    pub attractor_strength: f64,

    /// Current density level
    pub current_density: DensityLevel,

    /// Target density level (next stage in evolution)
    pub target_density: DensityLevel,

    /// Evolutionary progress
    pub evolutionary_progress: f64,
}

impl EvolutionaryAttractorField {
    pub fn new(_blueprint: &HolographicBlueprint) -> Self {
        EvolutionaryAttractorField {
            attractor_strength: 0.5,
            current_density: DensityLevel::First,
            target_density: DensityLevel::Second,
            evolutionary_progress: 0.0,
        }
    }

    fn update(&mut self, state: &EntityState, spectrum_access: &EntitySpectrumAccess) {
        // From COSMOLOGICAL-ARCHITECTURE.md: "Evolution is gradual and cumulative"
        // Evolutionary progress increases through experience, not just access level
        // The veil transparency influences the RATE of progress, not the absolute progress

        // Progress is based on experience accumulation
        // First->Second transition requires substantial experience (threshold at 0.25)
        // Using 25.0 as normalization ensures that even with max veil transparency,
        // 5.0 experience (10 iterations of 0.5) stays below 0.25 threshold
        // This aligns with holographic principle: evolution requires depth of experience
        let experience_factor = state.experience_accumulation / 25.0; // Normalize to 0-1 range
        let veil_multiplier = 0.5 + (spectrum_access.oneness_access * 0.5); // 0.5 to 1.0 range

        // Update evolutionary progress (gradual accumulation)
        self.evolutionary_progress = (experience_factor * veil_multiplier).min(1.0);

        // Update attractor strength based on evolutionary progress
        self.attractor_strength = self.evolutionary_progress * 0.8 + 0.2;

        // Update current density based on progress
        self.current_density = DensityLevel::from_progress(self.evolutionary_progress);

        // Update target density
        self.target_density = self.current_density.next();
    }

    fn check_transition_readiness(&self, _state: &EntityState) -> DensityTransitionReadiness {
        let progress_threshold = match self.target_density {
            DensityLevel::Second => 0.25,
            DensityLevel::Third => 0.50,
            DensityLevel::Fourth => 0.75,
            DensityLevel::Fifth => 0.85,
            DensityLevel::Sixth => 0.95,
            DensityLevel::Seventh => 0.99,
            DensityLevel::Eighth => 1.0,
            DensityLevel::First => 0.0,
        };

        let is_ready = self.evolutionary_progress >= progress_threshold;

        DensityTransitionReadiness {
            current_density: self.current_density.clone(),
            target_density: self.target_density.clone(),
            is_ready,
            progress_percentage: self.evolutionary_progress * 100.0,
            required_progress: progress_threshold * 100.0,
        }
    }
}

/// Density level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DensityLevel {
    First,   // Red Ray
    Second,  // Orange Ray
    Third,   // Yellow Ray
    Fourth,  // Green Ray
    Fifth,   // Blue Ray
    Sixth,   // Indigo-Ray
    Seventh, // Violet-Ray
    Eighth,  // Return to IntelligentInfinity
}

impl std::fmt::Display for DensityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DensityLevel::First => write!(f, "1st"),
            DensityLevel::Second => write!(f, "2nd"),
            DensityLevel::Third => write!(f, "3rd"),
            DensityLevel::Fourth => write!(f, "4th"),
            DensityLevel::Fifth => write!(f, "5th"),
            DensityLevel::Sixth => write!(f, "6th"),
            DensityLevel::Seventh => write!(f, "7th"),
            DensityLevel::Eighth => write!(f, "8th"),
        }
    }
}

impl DensityLevel {
    fn from_progress(progress: f64) -> Self {
        if progress < 0.25 {
            DensityLevel::First
        } else if progress < 0.50 {
            DensityLevel::Second
        } else if progress < 0.75 {
            DensityLevel::Third
        } else if progress < 0.85 {
            DensityLevel::Fourth
        } else if progress < 0.95 {
            DensityLevel::Fifth
        } else if progress < 0.99 {
            DensityLevel::Sixth
        } else if progress < 1.0 {
            DensityLevel::Seventh
        } else {
            DensityLevel::Eighth
        }
    }

    pub fn next(&self) -> Self {
        match self {
            DensityLevel::First => DensityLevel::Second,
            DensityLevel::Second => DensityLevel::Third,
            DensityLevel::Third => DensityLevel::Fourth,
            DensityLevel::Fourth => DensityLevel::Fifth,
            DensityLevel::Fifth => DensityLevel::Sixth,
            DensityLevel::Sixth => DensityLevel::Seventh,
            DensityLevel::Seventh => DensityLevel::Eighth,
            DensityLevel::Eighth => DensityLevel::Eighth,
        }
    }
}

/// Entity state
#[derive(Debug, Clone, PartialEq)]
pub struct EntityState {
    /// Vibrational state
    pub vibrational_state: VibrationalState,

    /// Polarity state
    pub polarity_state: PolarityState,

    /// Consciousness level
    pub consciousness_level: f64,

    /// Experience accumulation
    pub experience_accumulation: f64,

    /// Learning progress
    pub learning_progress: f64,
}

impl EntityState {
    fn new(spectrum_config: &IndividualSpectrumConfiguration) -> Self {
        EntityState {
            vibrational_state: VibrationalState::from_params(
                &spectrum_config.evolutionary_parameters,
            ),
            polarity_state: PolarityState::new(&spectrum_config.evolutionary_parameters),
            consciousness_level: 0.1,
            experience_accumulation: 0.0,
            learning_progress: 0.0,
        }
    }

    fn apply_experience(&mut self, experience: EntityExperience) {
        self.experience_accumulation += experience.intensity;
        self.learning_progress += experience.learning_value;
        self.consciousness_level =
            (self.consciousness_level + experience.consciousness_expansion).min(1.0);
        self.vibrational_state.apply_experience(&experience);
        self.polarity_state.apply_experience(&experience);
    }

    pub fn calculate_access_level(&self) -> SpectrumAccessLevel {
        if self.consciousness_level < 0.3 {
            SpectrumAccessLevel::ThirdDensity
        } else if self.consciousness_level < 0.6 {
            SpectrumAccessLevel::FourthDensity
        } else if self.consciousness_level < 0.8 {
            SpectrumAccessLevel::FifthDensity
        } else if self.consciousness_level <= 0.95 {
            SpectrumAccessLevel::SixthDensity
        } else {
            SpectrumAccessLevel::SeventhDensity
        }
    }
}

/// Vibrational state
#[derive(Debug, Clone, PartialEq)]
pub struct VibrationalState {
    pub frequency: f64,
    pub amplitude: f64,
    pub coherence: f64,
    /// Current density level
    pub density: crate::evolution_density_octave::density_octave::Density,
    /// Potential energy (derived from frequency)
    pub potential_energy: f64,
    /// Kinetic energy (derived from amplitude)
    pub kinetic_energy: f64,
}

impl VibrationalState {
    /// Create a new vibrational state with default values
    pub fn new() -> Self {
        VibrationalState {
            frequency: 0.5,
            amplitude: 0.5,
            coherence: 0.5,
            density: crate::evolution_density_octave::density_octave::Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            ),
            potential_energy: 0.5,
            kinetic_energy: 0.5,
        }
    }

    fn from_params(_params: &EvolutionaryParameters) -> Self {
        VibrationalState {
            frequency: 0.5,
            amplitude: 0.5,
            coherence: 0.5,
            density: crate::evolution_density_octave::density_octave::Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            ),
            potential_energy: 0.5,
            kinetic_energy: 0.5,
        }
    }

    fn apply_experience(&mut self, experience: &EntityExperience) {
        self.frequency = (self.frequency + experience.frequency_shift * 0.1).min(1.0);
        self.amplitude = (self.amplitude + experience.amplitude_change * 0.1).min(1.0);
        self.coherence = (self.coherence + experience.coherence_improvement * 0.1).min(1.0);
    }
}

/// Polarity state
#[derive(Debug, Clone, PartialEq)]
pub struct PolarityState {
    pub polarity_bias: f64, // -1.0 (STS) to 1.0 (STO)
    pub polarization_strength: f64,
}

impl PolarityState {
    fn new(params: &EvolutionaryParameters) -> Self {
        PolarityState {
            polarity_bias: params.polarity_bias,
            polarization_strength: 0.0,
        }
    }

    fn apply_experience(&mut self, experience: &EntityExperience) {
        self.polarity_bias =
            (self.polarity_bias + experience.polarity_impact * 0.1).clamp(-1.0, 1.0);
        self.polarization_strength =
            (self.polarization_strength + experience.polarization_strength_change * 0.1).min(1.0);
    }
}

/// Entity experience
#[derive(Debug, Clone)]
pub struct EntityExperience {
    pub intensity: f64,
    pub learning_value: f64,
    pub consciousness_expansion: f64,
    pub frequency_shift: f64,
    pub amplitude_change: f64,
    pub coherence_improvement: f64,
    pub polarity_impact: f64,
    pub polarization_strength_change: f64,
}

/// Spectrum access level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectrumAccessLevel {
    ThirdDensity,
    FourthDensity,
    FifthDensity,
    SixthDensity,
    SeventhDensity,
}

/// Spectrum access
#[derive(Debug, Clone)]
pub struct EntitySpectrumAccess {
    /// Space/Time access (v = s/t - Many-ness dominant)
    pub space_time_access: f64,

    /// Oneness/Time/Space access (v = t/s - Oneness dominant)
    pub oneness_access: f64,

    /// Veil transparency (0.0 = fully opaque, 1.0 = fully transparent)
    pub veil_transparency: f64,

    /// Evolutionary level
    pub evolutionary_level: SpectrumAccessLevel,

    // Phase 7: Space/Time vs Time/Space differentiation
    /// Space/Time ratio (v = s/t - Many-ness dominant)
    pub space_time_ratio: f64,

    /// Time/Space ratio (v = t/s - Oneness dominant)
    pub time_space_ratio: f64,

    /// Position on spectrum continuum (v = s/t ↔ v = t/s)
    /// Scale: 0.0 (pure space/time) to 1.0 (pure time/space)
    /// Veil position: 0.5 (qualitative break at v=1)
    pub spectrum_position: f64,
}

impl EntitySpectrumAccess {
    /// Calculate spectrum position on v = s/t ↔ v = t/s continuum
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The spectrum is a continuum v = s/t ↔ v = t/s with a qualitative break at the Veil (v=1)"
    ///
    /// Returns:
    /// - 0.0 to 0.5: Space/Time dominant (v = s/t)
    /// - 0.5: Veil position (v = 1)
    /// - 0.5 to 1.0: Time/Space dominant (v = t/s)
    pub fn calculate_spectrum_position(&self) -> f64 {
        // Normalize ratios to ensure they sum to 1.0
        let total_ratio = self.space_time_ratio + self.time_space_ratio;
        let normalized_space_time = if total_ratio > 0.0 {
            self.space_time_ratio / total_ratio
        } else {
            0.5 // Default to middle if both are 0
        };

        // Spectrum position: 0.0 (pure space/time) to 1.0 (pure time/space)
        // The Veil is at position 0.5 (v = 1)
        normalized_space_time
    }

    /// Check if entity is space/time dominant
    pub fn is_space_time_dominant(&self) -> bool {
        self.spectrum_position < 0.5
    }

    /// Check if entity is time/space dominant
    pub fn is_time_space_dominant(&self) -> bool {
        self.spectrum_position > 0.5
    }

    /// Check if entity is at the Veil position
    pub fn is_at_veil(&self) -> bool {
        (self.spectrum_position - 0.5).abs() < 0.1
    }

    /// Update spectrum access as entity evolves (veil thins)
    pub fn update_spectrum_access(&mut self) {
        // As entity evolves, veil thins (transparency increases)
        self.veil_transparency = (self.veil_transparency + 0.01).min(1.0);

        // Entity can access more of the spectrum (time/space increases)
        self.time_space_ratio = (self.time_space_ratio * 1.01).min(1.0);

        // Space/time ratio decreases as entity accesses more time/space
        self.space_time_ratio = (self.space_time_ratio * 0.99).max(0.0);

        // Recalculate spectrum position
        self.spectrum_position = self.calculate_spectrum_position();

        // Update access values based on new ratios
        self.space_time_access = self.space_time_ratio;
        self.oneness_access = self.time_space_ratio;
    }
}

/// Holographic completeness report
#[derive(Debug, Clone, Default)]
pub struct HolographicCompletenessReport {
    pub violet_present: bool,
    pub indigo_present: bool,
    pub blue_present: bool,
    pub green_present: bool,
    pub yellow_present: bool,
    pub orange_present: bool,
    pub red_present: bool,
    pub blueprint_complete: bool,
    pub archetypical_mind_inherited: bool,
    pub dna_patterns_present: bool,
    pub evolutionary_attractor_present: bool,
    pub completeness_percentage: f64,
}

/// Evolutionary trajectory
#[derive(Debug, Clone)]
pub struct EvolutionaryTrajectory {
    pub stages: Vec<EvolutionaryStage>,
    pub current_stage: usize,
}

impl EvolutionaryTrajectory {
    pub fn new() -> Self {
        EvolutionaryTrajectory {
            stages: vec![
                EvolutionaryStage::QuantumRealm,
                EvolutionaryStage::AtomicRealm,
                EvolutionaryStage::MolecularRealm,
                EvolutionaryStage::CellularRealm,
                EvolutionaryStage::ConsciousLifeRealm,
                EvolutionaryStage::FourthDensityRealm,
                EvolutionaryStage::FifthDensityRealm,
                EvolutionaryStage::SixthDensityRealm,
                EvolutionaryStage::SeventhDensityRealm,
            ],
            current_stage: 0,
        }
    }

    pub fn current_stage(&self) -> &EvolutionaryStage {
        &self.stages[self.current_stage]
    }

    pub fn advance(&mut self) {
        if self.current_stage < self.stages.len() - 1 {
            self.current_stage += 1;
        }
    }
}

/// Evolutionary stage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvolutionaryStage {
    QuantumRealm,
    AtomicRealm,
    MolecularRealm,
    PlanetaryRealm,
    CellularRealm,
    SimpleLifeRealm,
    ComplexLifeRealm,
    ConsciousLifeRealm,
    SocietalRealm,
    FourthDensityRealm,
    FifthDensityRealm,
    SixthDensityRealm,
    SeventhDensityRealm,
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
/// MIGRATED FROM: src/holographic_complex.rs (Phase 4.5 Migration 4)
#[derive(Debug, Clone)]
pub struct UnifiedConsciousnessStructure {
    /// All 22 archetypes as a unified structure
    ///
    /// Indices 0-6: Mind Complex (A1-A7)
    /// Indices 7-13: Body Complex (A8-A14)
    /// Indices 14-20: Spirit Complex (A15-A21)
    /// Index 21: Archetype 22 (The Choice - Free Will)
    ///
    /// CRITICAL: These are NOT separate arrays - they are ONE unified array
    /// that can be VIEWED through different aspects.
    pub archetype_states: [crate::entity_state::ArchetypeState; 22],

    /// Activation levels for each archetype (0.0 to 1.0)
    ///
    /// This tracks how much each archetype has been activated during evolution.
    pub activation_levels: [crate::types::Float; 22],

    /// Wisdom extracted from each archetype (0.0 to 1.0)
    ///
    /// This tracks the wisdom gained from processing catalyst through each archetype.
    pub wisdom_levels: [crate::types::Float; 22],
}

impl Default for UnifiedConsciousnessStructure {
    fn default() -> Self {
        Self {
            archetype_states: [crate::entity_state::ArchetypeState::Latent; 22],
            activation_levels: [0.0; 22],
            wisdom_levels: [0.0; 22],
        }
    }
}

impl UnifiedConsciousnessStructure {
    /// Create a new unified consciousness structure
    pub fn new() -> Self {
        Self::default()
    }

    /// Get archetype state at index
    pub fn get_state(&self, index: usize) -> Option<crate::entity_state::ArchetypeState> {
        self.archetype_states.get(index).copied()
    }

    /// Set archetype state at index
    pub fn set_state(&mut self, index: usize, state: crate::entity_state::ArchetypeState) -> bool {
        if index < 22 {
            self.archetype_states[index] = state;
            true
        } else {
            false
        }
    }

    /// Get activation level at index
    pub fn get_activation(&self, index: usize) -> Option<crate::types::Float> {
        self.activation_levels.get(index).copied()
    }

    /// Set activation level at index
    pub fn set_activation(&mut self, index: usize, level: crate::types::Float) -> bool {
        if index < 22 {
            self.activation_levels[index] = level.clamp(0.0, 1.0);
            true
        } else {
            false
        }
    }

    /// Get wisdom level at index
    pub fn get_wisdom(&self, index: usize) -> Option<crate::types::Float> {
        self.wisdom_levels.get(index).copied()
    }

    /// Set wisdom level at index
    pub fn set_wisdom(&mut self, index: usize, level: crate::types::Float) -> bool {
        if index < 22 {
            self.wisdom_levels[index] = level.clamp(0.0, 1.0);
            true
        } else {
            false
        }
    }

    /// Get all archetype states
    pub fn get_all_states(&self) -> &[crate::entity_state::ArchetypeState; 22] {
        &self.archetype_states
    }

    /// Get all activation levels
    pub fn get_all_activations(&self) -> &[crate::types::Float; 22] {
        &self.activation_levels
    }

    /// Get all wisdom levels
    pub fn get_all_wisdom(&self) -> &[crate::types::Float; 22] {
        &self.wisdom_levels
    }
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
/// MIGRATED FROM: src/holographic_complex.rs (Phase 4.5 Migration 4)
#[derive(Debug, Clone)]
pub struct CrossCouplingState {
    /// Mind-Body coupling (0.0 to 1.0)
    ///
    /// How strongly Mind and Body are intertwined
    pub mind_body_coupling: crate::types::Float,

    /// Body-Spirit coupling (0.0 to 1.0)
    ///
    /// How strongly Body and Spirit are intertwined
    pub body_spirit_coupling: crate::types::Float,

    /// Mind-Spirit coupling (0.0 to 1.0)
    ///
    /// How strongly Mind and Spirit are intertwined
    pub mind_spirit_coupling: crate::types::Float,

    /// Overall intertwining (0.0 to 1.0)
    ///
    /// The degree to which all three aspects are inextricably intertwined
    pub overall_intertwining: crate::types::Float,

    /// Coupling strength (0.0 to 1.0)
    ///
    /// How strong the coupling is overall
    pub coupling_strength: crate::types::Float,
}

impl Default for CrossCouplingState {
    fn default() -> Self {
        Self::new()
    }
}

impl CrossCouplingState {
    /// Create a new cross-coupling state
    pub fn new() -> Self {
        Self {
            mind_body_coupling: 0.5,
            body_spirit_coupling: 0.5,
            mind_spirit_coupling: 0.5,
            overall_intertwining: 0.5,
            coupling_strength: 0.5,
        }
    }

    /// Check if the aspects are inextricably intertwined
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.2
    /// "Mind, Body, and Spirit are not separate axes but 'inextricably intertwined'"
    pub fn is_inextricably_intertwined(&self) -> bool {
        // Aspects are inextricably intertwined when coupling is high
        self.overall_intertwining >= 0.7
    }

    /// Get coupling for a specific pair of aspects
    pub fn get_coupling(
        &self,
        aspect1: crate::types::ComplexType,
        aspect2: crate::types::ComplexType,
    ) -> crate::types::Float {
        match (aspect1, aspect2) {
            (crate::types::ComplexType::Mind, crate::types::ComplexType::Body)
            | (crate::types::ComplexType::Body, crate::types::ComplexType::Mind) => {
                self.mind_body_coupling
            }
            (crate::types::ComplexType::Body, crate::types::ComplexType::Spirit)
            | (crate::types::ComplexType::Spirit, crate::types::ComplexType::Body) => {
                self.body_spirit_coupling
            }
            (crate::types::ComplexType::Mind, crate::types::ComplexType::Spirit)
            | (crate::types::ComplexType::Spirit, crate::types::ComplexType::Mind) => {
                self.mind_spirit_coupling
            }
            _ => 0.0,
        }
    }

    /// Calculate overall balance
    pub fn calculate_balance(&mut self) {
        self.overall_intertwining =
            (self.mind_body_coupling + self.body_spirit_coupling + self.mind_spirit_coupling) / 3.0;
        self.coupling_strength = self.overall_intertwining;
    }
}

/// Density transition readiness
#[derive(Debug, Clone)]
pub struct DensityTransitionReadiness {
    pub current_density: DensityLevel,
    pub target_density: DensityLevel,
    pub is_ready: bool,
    pub progress_percentage: f64,
    pub required_progress: f64,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectrum::{OrangeRealm, RedRealm, SpectrumRatio, SpectrumSide, YellowRealm};

    #[test]
    fn test_sub_sub_logos_creation() {
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();
        let yellow = YellowRealm::new(green.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        let entity = SubSubLogos::new(
            EntityId::new("test-entity".to_string()),
            EntityType::Individual,
            None,
            Vec::new(), // composition
            None,       // environment_id
            violet.clone(),
            indigo.clone(),
            blue.clone(),
            green.clone(),
            yellow.clone(),
            orange.clone(),
            red.clone(),
            spectrum_config,
        );

        assert_eq!(entity.entity_id.uuid, "test-entity");
        assert!(entity.holographic_blueprint.is_complete());
        assert!(!entity.dna_patterns.is_empty());
    }

    #[test]
    fn test_holographic_completeness() {
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();
        let yellow = YellowRealm::new(green.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        let entity = SubSubLogos::new(
            EntityId::new("test-entity".to_string()),
            EntityType::Individual,
            None,
            Vec::new(), // composition
            None,       // environment_id
            violet,
            indigo,
            blue,
            green,
            yellow,
            orange,
            red,
            spectrum_config,
        );

        let report = entity.verify_holographic_completeness();
        assert_eq!(report.completeness_percentage, 100.0);
        assert!(report.blueprint_complete);
        assert!(report.archetypical_mind_inherited);
        assert!(report.dna_patterns_present);
    }

    #[test]
    fn test_spectrum_access_levels() {
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();
        let yellow = YellowRealm::new(green.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        let entity = SubSubLogos::new(
            EntityId::new("test-entity".to_string()),
            EntityType::Individual,
            None,
            Vec::new(), // composition
            None,       // environment_id
            violet,
            indigo,
            blue,
            green,
            yellow,
            orange,
            red,
            spectrum_config,
        );

        // Test 3rd density access
        let access_3rd = entity.access_spectrum(SpectrumAccessLevel::ThirdDensity);
        assert_eq!(access_3rd.oneness_access, 0.1);
        assert_eq!(access_3rd.veil_transparency, 0.1);

        // Test 6th density access
        let access_6th = entity.access_spectrum(SpectrumAccessLevel::SixthDensity);
        assert_eq!(access_6th.oneness_access, 1.0);
        assert_eq!(access_6th.veil_transparency, 1.0);
    }

    #[test]
    fn test_entity_state_updates() {
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();
        let yellow = YellowRealm::new(green.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        let mut entity = SubSubLogos::new(
            EntityId::new("test-entity".to_string()),
            EntityType::Individual,
            None,
            Vec::new(), // composition
            None,       // environment_id
            violet,
            indigo,
            blue,
            green,
            yellow,
            orange,
            red,
            spectrum_config,
        );

        let experience = EntityExperience {
            intensity: 0.5,
            learning_value: 0.3,
            consciousness_expansion: 0.2,
            frequency_shift: 0.1,
            amplitude_change: 0.1,
            coherence_improvement: 0.1,
            polarity_impact: 0.2,
            polarization_strength_change: 0.1,
        };

        entity.update_state(experience);

        assert!(entity.current_state.experience_accumulation > 0.0);
        assert!(entity.current_state.learning_progress > 0.0);
        assert!(entity.current_state.consciousness_level > 0.1);
    }

    #[test]
    fn test_density_transition_readiness() {
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();
        let yellow = YellowRealm::new(green.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        let mut entity = SubSubLogos::new(
            EntityId::new("test-entity".to_string()),
            EntityType::Individual,
            None,
            Vec::new(), // composition
            None,       // environment_id
            violet,
            indigo,
            blue,
            green,
            yellow,
            orange,
            red,
            spectrum_config,
        );

        // Simulate evolution to 4th density
        let experience = EntityExperience {
            intensity: 0.8,
            learning_value: 0.7,
            consciousness_expansion: 0.5,
            frequency_shift: 0.3,
            amplitude_change: 0.3,
            coherence_improvement: 0.3,
            polarity_impact: 0.2,
            polarization_strength_change: 0.2,
        };

        entity.update_state(experience);

        let readiness = entity.check_density_transition_readiness();
        assert_eq!(readiness.current_density, DensityLevel::First);
        assert_eq!(readiness.target_density, DensityLevel::Second);
    }

    #[test]
    fn test_entity_id_incarnation() {
        let id1 = EntityId::new("test-entity".to_string());
        let id2 = id1.next_incarnation();

        assert_eq!(id1.uuid, id2.uuid);
        assert_eq!(id1.incarnation_number, 1);
        assert_eq!(id2.incarnation_number, 2);
    }

    #[test]
    fn test_density_level_progression() {
        assert_eq!(DensityLevel::from_progress(0.1), DensityLevel::First);
        assert_eq!(DensityLevel::from_progress(0.3), DensityLevel::Second);
        assert_eq!(DensityLevel::from_progress(0.6), DensityLevel::Third);
        assert_eq!(DensityLevel::from_progress(0.8), DensityLevel::Fourth);
        assert_eq!(DensityLevel::from_progress(0.9), DensityLevel::Fifth);
        assert_eq!(DensityLevel::from_progress(0.97), DensityLevel::Sixth);
        assert_eq!(DensityLevel::from_progress(0.995), DensityLevel::Seventh);
        assert_eq!(DensityLevel::from_progress(1.0), DensityLevel::Eighth);
    }

    #[test]
    fn test_evolutionary_trajectory() {
        let trajectory = EvolutionaryTrajectory::new();
        assert_eq!(trajectory.stages.len(), 9);
        assert_eq!(trajectory.current_stage, 0);

        let mut trajectory = trajectory;
        trajectory.advance();
        assert_eq!(trajectory.current_stage, 1);
    }
}
