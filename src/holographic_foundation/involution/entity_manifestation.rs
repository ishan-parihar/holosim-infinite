//! Entity Manifestation - Creation of entities from hierarchy parameters
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Entity manifestation from hierarchy parameters
//!  Entity properties derive from position in hierarchy"

use super::cosmic_hierarchy::{CosmicHierarchy, HierarchyPath};
use super::logos_config::{ManifestationParameters, PlanetaryLogosConfig};
use super::propagation::{FieldConfiguration, InvolutionFlow};
use super::HierarchyLevel;
use crate::holographic_foundation::distortions::FieldState;
use crate::holographic_foundation::spectrum::SpectrumState;
use crate::types::Float;

#[derive(Debug, Clone)]
pub struct EntityManifestation {
    pub planetary_context: PlanetaryLogosConfig,
    pub field_configuration: FieldConfiguration,
    pub manifestation_count: u64,
}

impl EntityManifestation {
    pub fn new(planetary: PlanetaryLogosConfig) -> Self {
        Self {
            planetary_context: planetary,
            field_configuration: FieldConfiguration::new(),
            manifestation_count: 0,
        }
    }

    pub fn from_hierarchy(hierarchy: &CosmicHierarchy, planetary_id: usize) -> Option<Self> {
        let planetary = hierarchy.planetary_logoi.get(planetary_id)?;

        let flow = InvolutionFlow::new();
        let path = flow.build_path_to_planetary(hierarchy, planetary_id)?;

        let mut field = FieldConfiguration::new();
        for config in path.configs {
            field = match config {
                super::cosmic_hierarchy::LevelConfig::Primary(p) => {
                    FieldConfiguration::from_primary(&p)
                }
                super::cosmic_hierarchy::LevelConfig::Galactic(g) => field.refine_from_galactic(&g),
                super::cosmic_hierarchy::LevelConfig::Solar(s) => field.refine_from_solar(&s),
                super::cosmic_hierarchy::LevelConfig::Planetary(p) => {
                    field.refine_from_planetary(&p)
                }
            };
        }

        Some(Self {
            planetary_context: planetary.clone(),
            field_configuration: field,
            manifestation_count: 0,
        })
    }

    /// Manifest a new entity from the hierarchy parameters
    pub fn manifest_entity(&mut self) -> EntitySeed {
        self.manifestation_count += 1;

        let params = self.planetary_context.derive_entity_parameters();

        EntitySeed {
            entity_id: self.manifestation_count,
            parameters: params,
            field_state: FieldState::uninitialized(),
            source_planetary: self.planetary_context.planetary_id,
            hierarchy_depth: HierarchyLevel::Planetary.depth(),
        }
    }

    /// Manifest multiple entities
    pub fn manifest_entities(&mut self, count: usize) -> Vec<EntitySeed> {
        (0..count).map(|_| self.manifest_entity()).collect()
    }

    /// Check if manifestation capacity allows more entities
    pub fn can_manifest_more(&self) -> bool {
        (self.manifestation_count as usize) < self.planetary_context.inherited_capacity
    }

    /// Get remaining capacity
    pub fn remaining_capacity(&self) -> usize {
        self.planetary_context
            .inherited_capacity
            .saturating_sub(self.manifestation_count as usize)
    }
}

#[derive(Debug, Clone)]
pub struct EntitySeed {
    pub entity_id: u64,
    pub parameters: ManifestationParameters,
    pub field_state: FieldState,
    pub source_planetary: u64,
    pub hierarchy_depth: usize,
}

impl EntitySeed {
    /// Initialize the entity's field state from the manifestation parameters
    pub fn initialize_field(&mut self) {
        self.field_state.coherence = self.parameters.archetype_profile.iter().sum::<Float>()
            / self.parameters.archetype_profile.len() as Float;
    }

    /// Check if the entity is properly initialized
    pub fn is_initialized(&self) -> bool {
        self.field_state.coherence > 0.0
    }

    /// Get the entity's initial density position
    pub fn initial_density(&self) -> Float {
        self.parameters.density_position.value
    }

    /// Get the entity's initial spectrum position
    pub fn initial_spectrum_position(&self) -> Float {
        self.parameters.spectrum_position.value
    }

    /// Get the entity's veil transparency
    pub fn veil_transparency(&self) -> Float {
        self.parameters.veil_transparency
    }

    /// Get the entity's free will capacity
    pub fn free_will_capacity(&self) -> Float {
        self.parameters.free_will_capacity
    }

    /// Get the entity's catalyst sensitivity
    pub fn catalyst_sensitivity(&self) -> Float {
        self.parameters.catalyst_sensitivity
    }
}

#[derive(Debug, Clone, Default)]
pub struct ManifestationStatistics {
    pub total_manifested: u64,
    pub by_density: [u64; 8],
    pub average_coherence: Float,
}

impl ManifestationStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_manifestation(&mut self, seed: &EntitySeed) {
        self.total_manifested += 1;

        let density_idx = (seed.parameters.density_position.value.floor() as usize).min(7);
        self.by_density[density_idx] += 1;

        let n = self.total_manifested as Float;
        self.average_coherence =
            self.average_coherence * (n - 1.0) / n + seed.field_state.coherence / n;
    }
}

#[cfg(test)]
mod tests {
    use super::super::logos_config::PrimaryLogosConfig;
    use super::*;

    #[test]
    fn test_entity_manifestation_creation() {
        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        let manifestation = EntityManifestation::new(planetary);
        assert_eq!(manifestation.manifestation_count, 0);
    }

    #[test]
    fn test_manifest_entity() {
        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        let mut manifestation = EntityManifestation::new(planetary);
        let seed = manifestation.manifest_entity();

        assert_eq!(seed.entity_id, 1);
        assert_eq!(manifestation.manifestation_count, 1);
    }

    #[test]
    fn test_manifest_multiple_entities() {
        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        let mut manifestation = EntityManifestation::new(planetary);
        let seeds = manifestation.manifest_entities(5);

        assert_eq!(seeds.len(), 5);
        assert_eq!(manifestation.manifestation_count, 5);
    }

    #[test]
    fn test_entity_seed_initialization() {
        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        let mut manifestation = EntityManifestation::new(planetary);
        let mut seed = manifestation.manifest_entity();

        assert!(!seed.is_initialized());

        seed.initialize_field();

        assert!(seed.is_initialized());
    }

    #[test]
    fn test_manifestation_from_hierarchy() {
        let mut hierarchy = CosmicHierarchy::new();

        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        hierarchy.add_galactic_logos(galactic);
        hierarchy.add_solar_logos(solar, 0);
        hierarchy.add_planetary_logos(planetary, 0);

        let manifestation = EntityManifestation::from_hierarchy(&hierarchy, 0);

        assert!(manifestation.is_some());
    }

    #[test]
    fn test_capacity_checking() {
        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let mut planetary = solar.derive_planetary_parameters();
        planetary.inherited_capacity = 3;

        let mut manifestation = EntityManifestation::new(planetary);

        assert!(manifestation.can_manifest_more());
        assert_eq!(manifestation.remaining_capacity(), 3);

        manifestation.manifest_entities(3);

        assert!(!manifestation.can_manifest_more());
        assert_eq!(manifestation.remaining_capacity(), 0);
    }

    #[test]
    fn test_manifestation_statistics() {
        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        let mut manifestation = EntityManifestation::new(planetary);
        let mut stats = ManifestationStatistics::new();

        for _ in 0..5 {
            let mut seed = manifestation.manifest_entity();
            seed.initialize_field();
            stats.record_manifestation(&seed);
        }

        assert_eq!(stats.total_manifested, 5);
        assert!(stats.average_coherence > 0.0);
    }
}
