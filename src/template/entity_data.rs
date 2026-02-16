//! EntityData - Component-Specific Data for UniversalTemplate<Entity>
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 1 (Weeks 3-4):
//! "Migrate SubSubLogos to use UniversalTemplate<EntityData>"
//!
//! This module provides EntityData struct which holds all entity-specific fields.
//! The UniversalTemplate provides:
//! - field: Arc<HolographicField> (shared reference)
//! - spectrum: SpectrumConfiguration (spectrum ratio, veil transparency, access levels)
//! - archetype_activation: ArchetypeActivationProfile (22 coefficients)
//! - density: Density (1st → 8th)
//! - free_will_seed: u64 (non-deterministic choice seed)
//!
//! EntityData provides everything else specific to entities:
//! - Identity: entity_id, entity_type, parent_id, children
//! - Relationships: composition, environment_id
//! - Layer Realms: violet, indigo, blue, green, yellow, orange, red
//! - Extended spectrum configuration (localized, evolutionary parameters, consciousness-first encoding)
//! - Archetypical mind system (full ArchetypicalMind, not just activation profile)
//! - Holographic blueprint and DNA patterns
//! - Evolution: evolutionary_attractor, current_state, evolutionary_rate, karmic_patterns
//! - Individual progression: evolution_clock, polarization
//! - Backward compatibility fields
//! - Veil info
//!
//! # Key Design Decision
//!
//! UniversalTemplate stores:
//! - spectrum: SpectrumConfiguration (simplified, for template methods)
//! - archetype_activation: ArchetypeActivationProfile (22 coefficients, for interference calculation)
//!
//! EntityData stores:
//! - spectrum_configuration: IndividualSpectrumConfiguration (extended, for entity-specific logic)
//! - archetypical_mind: ArchetypicalMind (full system, for archetype processing)
//!
//! The two spectrum configurations are kept synchronized during migration.
//! The two archetype systems represent different aspects (activation profile vs full system).

// Core types from layer7 (publicly accessible)
use crate::entity_layer7::layer7::{
    EntityId, EntityState, EntityType, EvolutionaryAttractorField, IndividualSpectrumConfiguration,
    Layer7KarmicPattern, SpectrumAccess, VeilInfo,
};
// DNAPattern from its module
use crate::entity_layer7::dna_encoding::DNAPattern;
// HolographicBlueprint from its module
use crate::entity_layer7::holographic_blueprint::HolographicBlueprint;
// Foundation realms - use type aliases and direct imports
use crate::foundation::blue_realm::Logos;
use crate::foundation::green_realm::LightLoveField as GreenRealm;
use crate::foundation::indigo_realm::IntelligentInfinity;
use crate::foundation::violet_realm::VioletRealm;
// Spectrum realms - direct imports
use crate::spectrum::orange_realm::OrangeRealm;
use crate::spectrum::red_realm::RedRealm;
use crate::spectrum::yellow_realm::YellowRealm;
// Archetypical mind
use crate::spectrum::archetypical_mind::ArchetypicalMind;
// Polarization progress
use crate::polarization::PolarizationProgress;

/// Entity-specific data for UniversalTemplate
///
/// This struct contains all fields that are specific to entities,
/// as opposed to the shared fields in UniversalTemplate.
#[derive(Debug, Clone)]
pub struct EntityData {
    // =========================================================================
    // IDENTITY & RELATIONSHIPS
    // =========================================================================
    /// Unique identifier for this entity
    pub entity_id: EntityId,

    /// Entity type (individual, collective, or environmental)
    pub entity_type: EntityType,

    /// Parent entity ID (for hierarchical relationships)
    pub parent_id: Option<EntityId>,

    /// Child entity IDs (for collective entities)
    pub children: Vec<EntityId>,

    /// Composition - what entities is this entity composed of?
    pub composition: Vec<EntityId>,

    /// Environment ID - what environment does this entity exist in?
    pub environment_id: Option<EntityId>,

    /// Spectrum access information
    pub spectrum_access: SpectrumAccess,

    /// Physical entity (Phase 4: Physical Scale Integration)
    pub physical_entity: Option<crate::matter::Matter>,

    // =========================================================================
    // LAYER REALMS (INCLUDES: All previous layers)
    // =========================================================================
    /// Violet Realm (Unity/Source)
    pub violet_realm: VioletRealm,

    /// Indigo Realm (Intelligent Infinity/Awareness)
    pub indigo_realm: IntelligentInfinity,

    /// Blue Realm (Creative Principle/Logos)
    pub blue_realm: Logos,

    /// Green Realm (Field of Potential/Light-Love)
    pub green_realm: GreenRealm,

    /// Yellow Realm (Dimensions/Veil)
    pub yellow_realm: YellowRealm,

    /// Orange Realm (Galactic Logoi)
    pub orange_realm: OrangeRealm,

    /// Red Realm (Solar Logoi + Archetypical Mind)
    pub red_realm: RedRealm,

    // =========================================================================
    // TRANSCENDS: Individual entity consciousness
    // =========================================================================
    /// Extended spectrum configuration (individual-specific)
    pub spectrum_configuration: IndividualSpectrumConfiguration,

    /// Full archetypical mind system (not just activation profile)
    pub archetypical_mind: ArchetypicalMind,

    /// Holographic blueprint for this entity
    pub holographic_blueprint: HolographicBlueprint,

    /// DNA/RNA patterns generated from holographic blueprint
    pub dna_patterns: Vec<DNAPattern>,

    // =========================================================================
    // EVOLVES INTO: Attractor-field for Density Octave evolution
    // =========================================================================
    /// Evolutionary attractor field
    pub evolutionary_attractor: EvolutionaryAttractorField,

    /// Current state of entity development
    pub current_state: EntityState,

    /// Phase 5: Individual Variation - Evolutionary rate (0.5x to 1.5x)
    pub evolutionary_rate: f64,

    /// Phase 5: Unique karmic patterns for this entity
    pub karmic_patterns: Vec<Layer7KarmicPattern>,

    /// Phase 1: Evolution Clock - Tracks individual entity evolution progress
    pub evolution_clock: f64,

    /// Phase 1: Polarity Progression - Tracks entity's polarization journey
    pub polarization: PolarizationProgress,

    // =========================================================================
    // BACKWARD COMPATIBILITY FIELDS
    // =========================================================================
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

impl EntityData {
    /// Create a new EntityData
    ///
    /// This is a minimal constructor. For full entity creation with layer realms,
    /// use the FromConfig trait or convert from SubSubLogos.
    pub fn new(entity_id: EntityId, entity_type: EntityType) -> Self {
        // Create spectrum configuration
        let spectrum_configuration = IndividualSpectrumConfiguration::new(
            crate::spectrum::larson_framework::SpectrumRatio::space_time(1.0, 1.0),
        );

        // Create archetypical mind blueprint
        let archetypical_mind_blueprint =
            crate::entity_layer7::holographic_blueprint::ArchetypicalMindBlueprint::new_from_logos(
            );

        // Create holographic blueprint
        let holographic_blueprint = HolographicBlueprint::from_spectrum_configuration(
            &spectrum_configuration,
            &archetypical_mind_blueprint,
        );

        // Create archetypical mind
        let archetypical_mind = ArchetypicalMind::new_from_logos();

        // Create evolutionary attractor field
        let evolutionary_attractor = EvolutionaryAttractorField::new(&holographic_blueprint);

        // Create entity state
        let current_state = EntityState {
            vibrational_state: crate::entity_layer7::layer7::VibrationalState::new(),
            polarity_state: crate::entity_layer7::layer7::PolarityState {
                polarity_bias: 0.0,
                polarization_strength: 0.0,
            },
            consciousness_level: 0.1,
            experience_accumulation: 0.0,
            learning_progress: 0.0,
        };

        EntityData {
            entity_id,
            entity_type,
            parent_id: None,
            children: Vec::new(),
            composition: Vec::new(),
            environment_id: None,
            spectrum_access: SpectrumAccess::default(),
            physical_entity: None,
            violet_realm: VioletRealm::default(),
            indigo_realm: IntelligentInfinity::default(),
            blue_realm: Logos::default(),
            green_realm: GreenRealm::default(),
            yellow_realm: YellowRealm::default(),
            orange_realm: OrangeRealm::default(),
            red_realm: RedRealm::default(),
            spectrum_configuration,
            archetypical_mind,
            holographic_blueprint,
            dna_patterns: Vec::new(),
            evolutionary_attractor,
            current_state,
            evolutionary_rate: 1.0,
            karmic_patterns: Vec::new(),
            evolution_clock: 0.0,
            polarization: PolarizationProgress::new(),
            consciousness_level: 0.0,
            experience_accumulation: 0.0,
            learning_progress: 0.0,
            archetype_activations: [0.5; 22],
            veil_transparency: 0.5,
            space_time_ratio: 1.0,
            time_space_ratio: 1.0,
            spectrum_position: 0.5,
            potential_energy: 0.0,
            kinetic_energy: 0.0,
            energy: 0.0,
            veil: VeilInfo {
                transparency: 0.5,
                active: false,
                illusion_strength: 0.5,
                access_control: crate::entity_layer7::layer7::VeilAccessControl {
                    time_space_access: 0.5,
                    holographic_connection_access: 0.4,
                    higher_consciousness_access: 0.3,
                },
            },
        }
    }

    /// Get the entity ID
    pub fn entity_id(&self) -> &EntityId {
        &self.entity_id
    }

    /// Get the entity type
    pub fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    /// Get the evolution clock value
    pub fn evolution_clock(&self) -> f64 {
        self.evolution_clock
    }

    /// Advance the evolution clock
    pub fn advance_evolution_clock(&mut self, delta: f64) {
        self.evolution_clock += delta * self.evolutionary_rate;
    }

    /// Get the polarization progress
    pub fn polarization(&self) -> &PolarizationProgress {
        &self.polarization
    }

    /// Get mutable reference to polarization progress
    pub fn polarization_mut(&mut self) -> &mut PolarizationProgress {
        &mut self.polarization
    }

    /// Get the current state
    pub fn current_state(&self) -> &EntityState {
        &self.current_state
    }

    /// Get mutable reference to current state
    pub fn current_state_mut(&mut self) -> &mut EntityState {
        &mut self.current_state
    }

    /// Update backward compatibility fields
    ///
    /// This should be called after modifying current_state, spectrum_access, or archetypical_mind
    /// to keep backward compatibility fields in sync.
    pub fn update_backward_compatibility_fields(&mut self) {
        self.consciousness_level = self.current_state.consciousness_level;
        self.experience_accumulation = self.current_state.experience_accumulation;
        self.learning_progress = self.current_state.learning_progress;
        self.archetype_activations = self.archetypical_mind.get_activations();
        self.veil_transparency = if self.spectrum_access.veil_active {
            0.0
        } else {
            self.spectrum_access.time_space_access
        };
        self.space_time_ratio = self.spectrum_access.ratio;
        self.time_space_ratio = if self.space_time_ratio > 0.0 {
            1.0 / self.space_time_ratio
        } else {
            0.0
        };
        self.spectrum_position = if self.space_time_ratio > 1.0 {
            (self.space_time_ratio - 1.0) / 9.0
        } else if self.space_time_ratio < 1.0 {
            (1.0 - self.space_time_ratio) / 9.0
        } else {
            0.5
        };
        self.potential_energy = self.current_state.vibrational_state.potential_energy;
        self.kinetic_energy = self.current_state.vibrational_state.kinetic_energy;
        self.energy = self.potential_energy + self.kinetic_energy;
        self.veil.transparency = self.veil_transparency;
        self.veil.active = self.spectrum_access.veil_active;
        self.veil.illusion_strength = 1.0 - self.veil_transparency;
    }
}

/// Type alias for Entity using UniversalTemplate
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 1:
/// "All component types use the SAME template"
/// "pub type Entity = UniversalTemplate<EntityData>;"
pub type Entity = crate::holographic::universal_template::UniversalTemplate<EntityData>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evolution_density_octave::density_octave::Density;
    use crate::holographic::universal_template::{SpectrumConfiguration, TemplateConfig};

    #[test]
    fn test_entity_data_creation() {
        let entity_id = EntityId::new("test-entity-1".to_string());
        let entity_data = EntityData::new(entity_id.clone(), EntityType::Individual);

        assert_eq!(entity_data.entity_id, entity_id);
        assert_eq!(entity_data.entity_type, EntityType::Individual);
        assert_eq!(entity_data.evolution_clock, 0.0);
    }

    #[test]
    fn test_entity_data_evolution_clock() {
        let entity_id = EntityId::new("test-entity-2".to_string());
        let mut entity_data = EntityData::new(entity_id, EntityType::Individual);
        entity_data.evolutionary_rate = 1.5;

        entity_data.advance_evolution_clock(1.0);
        assert_eq!(entity_data.evolution_clock, 1.5);

        entity_data.advance_evolution_clock(2.0);
        assert_eq!(entity_data.evolution_clock, 4.5);
    }

    #[test]
    fn test_entity_data_backward_compatibility() {
        let entity_id = EntityId::new("test-entity-3".to_string());
        let mut entity_data = EntityData::new(entity_id, EntityType::Individual);

        // Modify current state
        entity_data.current_state.consciousness_level = 0.8;
        entity_data.current_state.experience_accumulation = 100.0;
        entity_data.current_state.learning_progress = 0.7;

        // Update backward compatibility fields
        entity_data.update_backward_compatibility_fields();

        // Verify backward compatibility fields are updated
        assert_eq!(entity_data.consciousness_level, 0.8);
        assert_eq!(entity_data.experience_accumulation, 100.0);
        assert_eq!(entity_data.learning_progress, 0.7);
    }
}
