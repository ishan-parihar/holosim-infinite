//! Migration Module - Convert between SubSubLogos and UniversalTemplate<Entity>
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 1 (Weeks 3-4):
//! "Migrate SubSubLogos to use UniversalTemplate<EntityData>"
//! "Preserve all existing functionality"
//!
//! This module provides:
//! - SubSubLogosAdapter: Wrapper for backward compatibility
//! - Conversion functions: SubSubLogos ↔ Entity
//! - FromConfig implementation for EntityData
//! - ExtractedEntityPotential conversion (Phase 1.5)
//! - EntityBehavior trait for unified interface

// Core types from layer7 (publicly accessible)
use crate::entity_layer7::layer7::{
    EntityId, EntityState, EntityType, EvolutionaryAttractorField, IndividualSpectrumConfiguration,
    SpectrumAccess, SubSubLogos, VeilInfo,
};
// HolographicBlueprint from its module
use crate::entity_layer7::holographic_blueprint::HolographicBlueprint;
// Foundation realms - use type aliases and direct imports
use crate::foundation::blue_realm::Logos;
use crate::foundation::green_realm::LightLoveField as GreenRealm;
use crate::foundation::indigo_realm::IntelligentInfinity;
use crate::foundation::violet_realm::VioletRealm;
// Spectrum realms - direct imports
use crate::spectrum::archetypical_mind::ArchetypicalMind;
use crate::spectrum::orange_realm::OrangeRealm;
use crate::spectrum::red_realm::RedRealm;
use crate::spectrum::yellow_realm::YellowRealm;
// Polarization progress
use crate::polarization::PolarizationProgress;
// Holographic template
use crate::holographic::holographic_field::ExtractedEntityPotential;
use crate::holographic::universal_template::{
    ArchetypeActivationProfile, SpectrumConfiguration, UniversalTemplate,
};
use crate::holographic::HolographicField;
use std::hash::Hasher;
use std::sync::Arc;
use super::entity_data::{Entity, EntityData};

/// Adapter for SubSubLogos to work with UniversalTemplate
///
/// This provides backward compatibility during migration.
/// Eventually, SubSubLogos will be replaced by Entity (UniversalTemplate<EntityData>).
#[derive(Debug, Clone)]
pub struct SubSubLogosAdapter {
    /// The underlying entity using UniversalTemplate
    pub entity: Entity,

    /// Current density (for backward compatibility)
    pub current_density: crate::evolution_density_octave::density_octave::Density,
}

impl SubSubLogosAdapter {
    /// Create a new SubSubLogosAdapter from an Entity
    pub fn new(entity: Entity) -> Self {
        let current_density = entity.density;
        Self {
            entity,
            current_density,
        }
    }

    /// Get the entity ID
    pub fn entity_id(&self) -> &EntityId {
        &self.entity.component_data.entity_id
    }

    /// Get the entity type
    pub fn entity_type(&self) -> EntityType {
        self.entity.component_data.entity_type
    }

    /// Get the evolution clock
    pub fn evolution_clock(&self) -> f64 {
        self.entity.component_data.evolution_clock
    }

    /// Advance the evolution clock
    pub fn advance_evolution_clock(&mut self, delta: f64) {
        self.entity.component_data.advance_evolution_clock(delta);
    }

    /// Evolve the spectrum using template method
    pub fn evolve_spectrum(&mut self, delta: f64) {
        self.entity.evolve_spectrum(delta);
    }

    /// Process archetypes using template method
    pub fn process_archetypes(
        &self,
    ) -> crate::holographic::universal_template::ArchetypicalInterference {
        self.entity.process_archetypes()
    }

    /// Update backward compatibility fields
    pub fn update_backward_compatibility_fields(&mut self) {
        self.entity
            .component_data
            .update_backward_compatibility_fields();
    }
}

/// Convert SubSubLogos to Entity (UniversalTemplate<EntityData>)
///
/// This function migrates a SubSubLogos entity to use the UniversalTemplate pattern.
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 1:
/// "Migrate SubSubLogos to use UniversalTemplate<EntityData>"
pub fn convert_subsublogos_to_entity(
    subsublogos: SubSubLogos,
    holographic_field: Arc<HolographicField>,
) -> Entity {
    // Extract spectrum configuration from SubSubLogos
    let spectrum_configuration = subsublogos.spectrum_configuration.clone();

    // Convert to SpectrumConfiguration for UniversalTemplate
    let spectrum = SpectrumConfiguration::new(
        spectrum_configuration.ratio,
        subsublogos.veil_transparency,
        subsublogos.spectrum_access.space_time_access,
        subsublogos.spectrum_access.time_space_access,
    );

    // Extract archetype activations from SubSubLogos
    let archetype_activations = subsublogos.archetype_activations;
    let archetype_activation = ArchetypeActivationProfile::new(archetype_activations);

    // Get density from SubSubLogos
    let density = subsublogos.current_density;

    // Generate free will seed from entity ID
    let free_will_seed = generate_free_will_seed(&subsublogos.entity_id);

    // Create EntityData from SubSubLogos
    let entity_data = EntityData {
        entity_id: subsublogos.entity_id,
        entity_type: subsublogos.entity_type,
        parent_id: subsublogos.parent_id,
        children: subsublogos.children,
        composition: subsublogos.composition,
        environment_id: subsublogos.environment_id,
        spectrum_access: subsublogos.spectrum_access,
        physical_entity: subsublogos.physical_entity,
        violet_realm: subsublogos.violet_realm,
        indigo_realm: subsublogos.indigo_realm,
        blue_realm: subsublogos.blue_realm,
        green_realm: subsublogos.green_realm,
        yellow_realm: subsublogos.yellow_realm,
        orange_realm: subsublogos.orange_realm,
        red_realm: subsublogos.red_realm,
        spectrum_configuration: subsublogos.spectrum_configuration,
        archetypical_mind: subsublogos.archetypical_mind,
        holographic_blueprint: subsublogos.holographic_blueprint,
        dna_patterns: subsublogos.dna_patterns,
        evolutionary_attractor: subsublogos.evolutionary_attractor,
        current_state: subsublogos.current_state,
        evolutionary_rate: subsublogos.evolutionary_rate,
        karmic_patterns: subsublogos.karmic_patterns,
        evolution_clock: subsublogos.evolution_clock,
        polarization: subsublogos.polarization,
        consciousness_level: subsublogos.consciousness_level,
        experience_accumulation: subsublogos.experience_accumulation,
        learning_progress: subsublogos.learning_progress,
        archetype_activations: subsublogos.archetype_activations,
        veil_transparency: subsublogos.veil_transparency,
        space_time_ratio: subsublogos.space_time_ratio,
        time_space_ratio: subsublogos.time_space_ratio,
        spectrum_position: subsublogos.spectrum_position,
        potential_energy: subsublogos.potential_energy,
        kinetic_energy: subsublogos.kinetic_energy,
        energy: subsublogos.energy,
        veil: subsublogos.veil,
    };

    // Create UniversalTemplate with shared holographic field
    UniversalTemplate::new(
        holographic_field,
        spectrum,
        archetype_activation,
        density,
        free_will_seed,
        entity_data,
    )
}

/// Convert Entity (UniversalTemplate<EntityData>) to SubSubLogos
///
/// This function converts an Entity back to SubSubLogos for backward compatibility.
pub fn convert_entity_to_subsublogos(entity: Entity) -> SubSubLogos {
    let data = entity.component_data;
    let current_density = entity.density;

    SubSubLogos {
        entity_id: data.entity_id,
        entity_type: data.entity_type,
        parent_id: data.parent_id,
        children: data.children,
        composition: data.composition,
        environment_id: data.environment_id,
        spectrum_access: data.spectrum_access,
        physical_entity: data.physical_entity,
        violet_realm: data.violet_realm,
        indigo_realm: data.indigo_realm,
        blue_realm: data.blue_realm,
        green_realm: data.green_realm,
        yellow_realm: data.yellow_realm,
        orange_realm: data.orange_realm,
        red_realm: data.red_realm,
        spectrum_configuration: data.spectrum_configuration,
        archetypical_mind: data.archetypical_mind,
        holographic_blueprint: data.holographic_blueprint,
        dna_patterns: data.dna_patterns,
        evolutionary_attractor: data.evolutionary_attractor,
        current_state: data.current_state,
        evolutionary_rate: data.evolutionary_rate,
        karmic_patterns: data.karmic_patterns,
        evolution_clock: data.evolution_clock,
        polarization: data.polarization,
        current_density,
        consciousness_level: data.consciousness_level,
        experience_accumulation: data.experience_accumulation,
        learning_progress: data.learning_progress,
        archetype_activations: data.archetype_activations,
        veil_transparency: data.veil_transparency,
        space_time_ratio: data.space_time_ratio,
        time_space_ratio: data.time_space_ratio,
        spectrum_position: data.spectrum_position,
        potential_energy: data.potential_energy,
        kinetic_energy: data.kinetic_energy,
        energy: data.energy,
        veil: data.veil,
    }
}

/// Generate a free will seed from an entity ID
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Free Will as seed (8 bytes vs 100+ bytes)"
fn generate_free_will_seed(entity_id: &EntityId) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    let mut hasher = DefaultHasher::new();
    entity_id.hash(&mut hasher);
    hasher.finish()
}

/// FromConfig implementation for EntityData
///
/// This allows TemplateFactory to create EntityData from TemplateConfig.
impl crate::holographic::universal_template::FromConfig for EntityData {
    fn from_config(config: &crate::holographic::universal_template::TemplateConfig) -> Self {
        // Create a default entity with entity-specific data
        let entity_id = EntityId::new(format!("entity-{:x}", config.free_will_seed));
        let entity_type = EntityType::Individual;

        // Initialize spectrum access from template config
        let spectrum_access = SpectrumAccess {
            ratio: config.spectrum.ratio.calculate_ratio(),
            space_time_access: config.spectrum.space_time_access,
            time_space_access: config.spectrum.time_space_access,
            veil_active: config.spectrum.veil_transparency < 0.5,
            mannyness_access: 0.5,
        };

        // Create individual spectrum configuration
        let spectrum_configuration =
            IndividualSpectrumConfiguration::new(config.spectrum.ratio.clone());

        // Initialize archetypical mind with archetype activations from config
        let mut archetypical_mind = ArchetypicalMind::new_from_logos();
        for (i, &activation) in config.archetype_activation.coefficients.iter().enumerate() {
            let _ = archetypical_mind.activate_archetype(i + 1, activation);
        }

        // Create holographic blueprint from spectrum configuration
        let archetypical_mind_blueprint =
            crate::entity_layer7::holographic_blueprint::ArchetypicalMindBlueprint::default();
        let holographic_blueprint = HolographicBlueprint::from_spectrum_configuration(
            &spectrum_configuration,
            &archetypical_mind_blueprint,
        );

        // Generate DNA patterns
        let dna_patterns = holographic_blueprint.generate_dna_patterns();

        // Create evolutionary attractor field
        let evolutionary_attractor = EvolutionaryAttractorField::new(&holographic_blueprint);

        // Initialize entity state
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

        // Extract energy values before moving current_state
        let potential_energy = current_state.vibrational_state.potential_energy;
        let kinetic_energy = current_state.vibrational_state.kinetic_energy;
        let energy = potential_energy + kinetic_energy;
        let consciousness_level = current_state.consciousness_level;
        let experience_accumulation = current_state.experience_accumulation;
        let learning_progress = current_state.learning_progress;

        // Initialize veil info
        let veil_transparency = config.spectrum.veil_transparency;
        let veil = VeilInfo {
            transparency: veil_transparency,
            active: veil_transparency < 0.5,
            illusion_strength: 1.0 - veil_transparency,
            access_control: crate::entity_layer7::layer7::VeilAccessControl {
                time_space_access: veil_transparency,
                holographic_connection_access: veil_transparency * 0.8,
                higher_consciousness_access: veil_transparency * 0.6,
            },
        };

        let mut entity_data = EntityData {
            entity_id,
            entity_type,
            parent_id: None,
            children: Vec::new(),
            composition: Vec::new(),
            environment_id: None,
            spectrum_access,
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
            dna_patterns,
            evolutionary_attractor,
            current_state,
            evolutionary_rate: 1.0,
            karmic_patterns: Vec::new(),
            evolution_clock: 0.0,
            polarization: crate::polarization::PolarizationProgress::new(),
            consciousness_level,
            experience_accumulation,
            learning_progress,
            archetype_activations: config.archetype_activation.coefficients,
            veil_transparency,
            space_time_ratio: config.spectrum.ratio.calculate_ratio(),
            time_space_ratio: 1.0 / config.spectrum.ratio.calculate_ratio(),
            spectrum_position: 0.5,
            potential_energy,
            kinetic_energy,
            energy,
            veil,
        };

        // Update backward compatibility fields
        entity_data.update_backward_compatibility_fields();

        entity_data
    }
}

// =============================================================================
// PHASE 1.5: ExtractedEntityPotential Conversion
// =============================================================================

impl ExtractedEntityPotential {
    /// Convert a field-extracted potential into an Entity template
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Entities manifest at stable interference nodes"
    ///
    /// This implements the Phase 1.5 migration where entities emerge from
    /// the holographic field rather than being created directly.
    ///
    /// # Arguments
    ///
    /// * `field` - Shared reference to the holographic field
    ///
    /// # Returns
    ///
    /// A new Entity (UniversalTemplate<EntityData>) created from the potential
    pub fn into_entity(self, field: Arc<HolographicField>) -> Entity {
        // Create EntityData from the extracted potential
        let entity_data = EntityData::from_extracted_potential(self.clone());

        // Create UniversalTemplate with shared holographic field
        UniversalTemplate::new(
            field,
            self.spectrum,
            self.archetype_activation,
            self.density,
            self.free_will_seed,
            entity_data,
        )
    }
}

impl EntityData {
    /// Create EntityData from an extracted field potential
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Field is primary reality"
    ///
    /// This creates entity-specific data from a potential extracted from
    /// the holographic field at a stable interference node.
    ///
    /// # Arguments
    ///
    /// * `potential` - The extracted entity potential from the field
    ///
    /// # Returns
    ///
    /// A new EntityData initialized from the potential
    pub fn from_extracted_potential(potential: ExtractedEntityPotential) -> Self {
        use crate::entity_layer7::layer7::PolarityState;
        use crate::entity_layer7::layer7::VibrationalState;

        // Create spectrum access from the potential's spectrum configuration
        let spectrum_access = SpectrumAccess {
            ratio: potential.spectrum.ratio.calculate_ratio(),
            veil_active: potential.spectrum.veil_transparency < 0.5,
            space_time_access: potential.spectrum.space_time_access,
            time_space_access: potential.spectrum.time_space_access,
            mannyness_access: potential.coherence,
        };

        // Create individual spectrum configuration
        let spectrum_configuration =
            IndividualSpectrumConfiguration::new(potential.spectrum.ratio.clone());

        // Create archetypical mind blueprint
        let archetypical_mind_blueprint =
            crate::entity_layer7::holographic_blueprint::ArchetypicalMindBlueprint::default();

        // Create holographic blueprint
        let holographic_blueprint = HolographicBlueprint::from_spectrum_configuration(
            &spectrum_configuration,
            &archetypical_mind_blueprint,
        );

        // Create archetypical mind
        let mut archetypical_mind = ArchetypicalMind::new_from_logos();
        for (i, &activation) in potential
            .archetype_activation
            .coefficients
            .iter()
            .enumerate()
        {
            let _ = archetypical_mind.activate_archetype(i + 1, activation);
        }

        // Generate DNA patterns
        let dna_patterns = holographic_blueprint.generate_dna_patterns();

        // Create evolutionary attractor field
        let evolutionary_attractor = EvolutionaryAttractorField::new(&holographic_blueprint);

        // Create entity state based on potential
        let current_state = EntityState {
            vibrational_state: VibrationalState::new(),
            polarity_state: PolarityState {
                polarity_bias: 0.0,
                polarization_strength: 0.0,
            },
            consciousness_level: potential.coherence,
            experience_accumulation: 0.0,
            learning_progress: 0.0,
        };

        // Create veil info
        let veil = VeilInfo {
            transparency: potential.spectrum.veil_transparency,
            active: potential.spectrum.veil_transparency < 0.5,
            illusion_strength: 1.0 - potential.spectrum.veil_transparency,
            access_control: crate::entity_layer7::layer7::VeilAccessControl {
                time_space_access: potential.spectrum.time_space_access,
                holographic_connection_access: potential.spectrum.time_space_access * 0.8,
                higher_consciousness_access: potential.spectrum.time_space_access * 0.6,
            },
        };

        // Extract energy values
        let potential_energy = current_state.vibrational_state.potential_energy;
        let kinetic_energy = current_state.vibrational_state.kinetic_energy;
        let energy = potential_energy + kinetic_energy;

        // Create entity ID from potential address
        // Use free_will_seed and coherence_path length as basis for the entity ID
        let path_hash = potential.address.coherence_path.len();
        let entity_id = EntityId::new(format!(
            "entity-{:x}-{}",
            potential.free_will_seed, path_hash
        ));

        Self {
            entity_id,
            entity_type: EntityType::Individual,
            parent_id: None,
            children: Vec::new(),
            composition: Vec::new(),
            environment_id: None,
            spectrum_access,
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
            dna_patterns,
            evolutionary_attractor,
            current_state,
            evolutionary_rate: 1.0,
            karmic_patterns: Vec::new(),
            evolution_clock: 0.0,
            polarization: PolarizationProgress::new(),
            consciousness_level: potential.coherence,
            experience_accumulation: 0.0,
            learning_progress: 0.0,
            archetype_activations: potential.archetype_activation.coefficients,
            veil_transparency: potential.spectrum.veil_transparency,
            space_time_ratio: potential.spectrum.ratio.calculate_ratio(),
            time_space_ratio: 1.0 / potential.spectrum.ratio.calculate_ratio().max(0.001),
            spectrum_position: 0.5,
            potential_energy,
            kinetic_energy,
            energy,
            veil,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evolution_density_octave::density_octave::Density;
    use crate::holographic::complex_vectors::ComplexArchetype;
    use crate::holographic::universal_template::{
        FromConfig, SpectrumConfiguration, TemplateConfig,
    };
    use crate::template::entity_behavior::EntityBehavior;

    fn create_test_holographic_field() -> HolographicField {
        let archetypes = {
            let mut a = [ComplexArchetype {
                amplitude: 0.0,
                phase: 0.0,
            }; 22];
            for i in 0..22 {
                a[i] = ComplexArchetype {
                    amplitude: (i as f64 + 1.0) / 22.0,
                    phase: (i as f64) * std::f64::consts::PI / 11.0,
                };
            }
            a
        };
        HolographicField::new(crate::holographic::InvolutionLayer::Green, archetypes)
    }

    #[test]
    fn test_free_will_seed_generation() {
        let entity_id = EntityId::new("test-entity".to_string());
        let seed1 = generate_free_will_seed(&entity_id);
        let seed2 = generate_free_will_seed(&entity_id);

        // Same entity ID should generate same seed
        assert_eq!(seed1, seed2);

        // Different entity IDs should generate different seeds
        let entity_id2 = EntityId::new("test-entity-2".to_string());
        let seed3 = generate_free_will_seed(&entity_id2);
        assert_ne!(seed1, seed3);
    }

    #[test]
    fn test_from_config() {
        let _field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let entity_data = EntityData::from_config(&config);

        assert_eq!(entity_data.entity_type, EntityType::Individual);
        assert_eq!(entity_data.evolution_clock, 0.0);
        assert_eq!(entity_data.evolutionary_rate, 1.0);
    }

    #[test]
    fn test_subsublogos_adapter_creation() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let entity = UniversalTemplate::from_config(
            field,
            config,
            EntityData::new(EntityId::new("test-1".to_string()), EntityType::Individual),
        );

        let adapter = SubSubLogosAdapter::new(entity);

        assert_eq!(adapter.entity_id().uuid, "test-1");
        assert_eq!(adapter.entity_type(), EntityType::Individual);
        assert_eq!(adapter.evolution_clock(), 0.0);
    }

    #[test]
    fn test_subsublogos_adapter_evolution_clock() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let entity = UniversalTemplate::from_config(
            field,
            config,
            EntityData::new(EntityId::new("test-2".to_string()), EntityType::Individual),
        );

        let mut adapter = SubSubLogosAdapter::new(entity);
        adapter.advance_evolution_clock(1.0);

        assert_eq!(adapter.evolution_clock(), 1.0);
    }

    // =========================================================================
    // PHASE 1.5 TESTS: EntityBehavior and ExtractedEntityPotential
    // =========================================================================

    #[test]
    fn test_entity_behavior_identity() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let entity = UniversalTemplate::from_config(
            field,
            config,
            EntityData::new(
                EntityId::new("behavior-test".to_string()),
                EntityType::Individual,
            ),
        );

        // Test EntityBehavior trait methods
        assert_eq!(entity.entity_id().uuid, "behavior-test");
        assert_eq!(entity.entity_type(), EntityType::Individual);
    }

    #[test]
    fn test_entity_behavior_composition() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let mut entity = UniversalTemplate::from_config(
            field,
            config,
            EntityData::new(
                EntityId::new("comp-test".to_string()),
                EntityType::Individual,
            ),
        );

        let component_id = EntityId::new("component-1".to_string());

        // Test adding components
        entity.add_component(component_id.clone());
        assert_eq!(entity.component_count(), 1);
        assert!(entity.has_component(&component_id));

        // Test removing components
        assert!(entity.remove_component(&component_id));
        assert_eq!(entity.component_count(), 0);
        assert!(!entity.has_component(&component_id));

        // Test removing non-existent component
        assert!(!entity.remove_component(&component_id));
    }

    #[test]
    fn test_entity_behavior_hierarchy() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let mut entity = UniversalTemplate::from_config(
            field,
            config,
            EntityData::new(
                EntityId::new("hier-test".to_string()),
                EntityType::Collective,
            ),
        );

        let child_id = EntityId::new("child-1".to_string());

        // Test parent relationship
        assert!(entity.is_root());
        assert!(!entity.has_parent());

        let parent_id = EntityId::new("parent-1".to_string());
        entity.set_parent(Some(parent_id.clone()));
        assert!(entity.has_parent());
        assert!(!entity.is_root());
        assert_eq!(entity.parent(), Some(parent_id));

        // Test children relationship
        assert!(entity.is_leaf());
        entity.add_child(child_id.clone());
        assert!(!entity.is_leaf());
        assert_eq!(entity.children().len(), 1);

        entity.remove_child(&child_id);
        assert!(entity.is_leaf());
    }

    #[test]
    fn test_entity_behavior_evolution() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let mut entity = UniversalTemplate::from_config(
            field,
            config,
            EntityData::new(
                EntityId::new("evo-test".to_string()),
                EntityType::Individual,
            ),
        );

        // Test evolution clock
        assert_eq!(entity.evolution_clock(), 0.0);

        entity.advance_evolution_clock(1.0);
        assert_eq!(entity.evolution_clock(), 1.0);

        entity.advance_evolution_clock(0.5);
        assert_eq!(entity.evolution_clock(), 1.5);
    }

    #[test]
    fn test_entity_behavior_archetype() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let mut entity = UniversalTemplate::from_config(
            field,
            config,
            EntityData::new(
                EntityId::new("arch-test".to_string()),
                EntityType::Individual,
            ),
        );

        // Test archetype activations
        let activations = entity.archetype_activations();
        assert_eq!(activations.len(), 22);

        // Test mutable access
        {
            let activations_mut = entity.archetype_activations_mut();
            activations_mut[0] = 0.8;
        }

        assert_eq!(entity.archetype_activations()[0], 0.8);
    }

    #[test]
    fn test_entity_behavior_spectrum() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let entity = UniversalTemplate::from_config(
            field,
            config,
            EntityData::new(
                EntityId::new("spec-test".to_string()),
                EntityType::Individual,
            ),
        );

        // Test spectrum access
        let ratio = entity.space_time_ratio();
        assert!(ratio > 0.0);

        let transparency = entity.veil_transparency();
        assert!((0.0..=1.0).contains(&transparency));
    }

    #[test]
    fn test_extracted_entity_potential_into_entity() {
        use crate::holographic::field_address::HolographicAddress;
        use crate::holographic::holographic_field::ExtractedEntityPotential;
        use crate::holographic::universal_template::ArchetypeActivationProfile;

        let field = Arc::new(create_test_holographic_field());

        // Create a potential
        let potential = ExtractedEntityPotential {
            address: HolographicAddress::cosmic_origin(),
            archetype_activation: ArchetypeActivationProfile::new([0.5; 22]),
            spectrum: SpectrumConfiguration::balanced(),
            density: Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            ),
            coherence: 0.7,
            free_will_seed: 12345,
        };

        // Convert to entity
        let entity = potential.into_entity(field);

        // Verify entity was created correctly
        assert!(entity.entity_id().uuid.starts_with("entity-"));
        assert_eq!(entity.entity_type(), EntityType::Individual);
        assert_eq!(entity.evolution_clock(), 0.0);
    }

    #[test]
    fn test_entity_data_from_extracted_potential() {
        use crate::holographic::field_address::HolographicAddress;
        use crate::holographic::holographic_field::ExtractedEntityPotential;
        use crate::holographic::universal_template::ArchetypeActivationProfile;
        use crate::spectrum::larson_framework::SpectrumRatio;

        // Create a potential with specific values
        let potential = ExtractedEntityPotential {
            address: HolographicAddress::cosmic_origin(),
            archetype_activation: ArchetypeActivationProfile::new([0.6; 22]),
            spectrum: SpectrumConfiguration::new(
                SpectrumRatio::space_time(2.0, 1.0),
                0.3,
                0.7,
                0.3,
            ),
            density: Density::Third,
            coherence: 0.8,
            free_will_seed: 54321,
        };

        // Create EntityData
        let entity_data = EntityData::from_extracted_potential(potential);

        // Verify creation
        assert_eq!(entity_data.entity_type, EntityType::Individual);
        assert_eq!(entity_data.consciousness_level, 0.8);
        assert_eq!(entity_data.veil_transparency, 0.3);
        assert_eq!(entity_data.evolution_clock, 0.0);
    }
}
