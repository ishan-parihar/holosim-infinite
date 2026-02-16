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
use crate::holographic::universal_template::{
    ArchetypeActivationProfile, SpectrumConfiguration, UniversalTemplate,
};
use crate::holographic::HolographicField;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
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
        let current_density = entity.density.clone();
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
    let density = subsublogos.current_density.clone();

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
    let current_density = entity.density.clone();

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evolution_density_octave::density_octave::Density;
    use crate::holographic::universal_template::{
        FromConfig, SpectrumConfiguration, TemplateConfig,
    };

    fn create_test_holographic_field() -> HolographicField {
        use crate::holographic::complex_vectors::ComplexArchetype;

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
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::default();

        let entity_data = EntityData::from_config(&config);

        assert_eq!(entity_data.entity_type, EntityType::Individual);
        assert_eq!(entity_data.evolution_clock, 0.0);
        assert_eq!(entity_data.evolutionary_rate, 1.0);
    }

    #[test]
    fn test_subsublogos_adapter_creation() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::default();

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
        let config = TemplateConfig::default();

        let entity = UniversalTemplate::from_config(
            field,
            config,
            EntityData::new(EntityId::new("test-2".to_string()), EntityType::Individual),
        );

        let mut adapter = SubSubLogosAdapter::new(entity);
        adapter.advance_evolution_clock(1.0);

        assert_eq!(adapter.evolution_clock(), 1.0);
    }
}
