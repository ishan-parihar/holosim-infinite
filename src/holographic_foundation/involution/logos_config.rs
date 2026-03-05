//! Logos Configuration - Parameters for each level of the cosmic hierarchy
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Each level imposes boundary conditions on lower levels
//!  Lower levels inherit + refine patterns"

use super::HierarchyLevel;
use crate::holographic_foundation::distortions::UnifiedFieldConfig;
use crate::holographic_foundation::spectrum::{DensityPosition, VelocityRatio};
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct PrimaryLogosConfig {
    pub universal_constants: UniversalConstants,
    pub base_distortion_config: UnifiedFieldConfig,
    pub octave_structure: OctaveStructure,
}

impl PrimaryLogosConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn derive_galactic_parameters(&self) -> GalacticLogosConfig {
        GalacticLogosConfig::from_primary(self)
    }

    pub fn hierarchy_level(&self) -> HierarchyLevel {
        HierarchyLevel::Primary
    }
}

#[derive(Debug, Clone)]
pub struct UniversalConstants {
    pub base_frequency: Float,
    pub love_strength: Float,
    pub light_speed: Float,
    pub free_will_amplitude: Float,
    pub coherence_threshold: Float,
}

impl Default for UniversalConstants {
    fn default() -> Self {
        Self {
            base_frequency: 1.0,
            love_strength: 1.0,
            light_speed: 1.0,
            free_will_amplitude: 0.1,
            coherence_threshold: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OctaveStructure {
    pub num_densities: usize,
    pub density_ratios: Vec<Float>,
    pub transition_points: Vec<Float>,
}

impl Default for OctaveStructure {
    fn default() -> Self {
        Self {
            num_densities: 8,
            density_ratios: vec![1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0],
            transition_points: vec![0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875, 1.0],
        }
    }
}

#[derive(Debug, Clone)]
pub struct GalacticLogosConfig {
    pub galactic_id: u64,
    pub archetype_selection: ArchetypeSelection,
    pub spectrum_configuration: SpectrumConfiguration,
    pub density_range: (Float, Float),
    pub inherited_constants: UniversalConstants,
}

impl GalacticLogosConfig {
    pub fn from_primary(primary: &PrimaryLogosConfig) -> Self {
        Self {
            galactic_id: 0,
            archetype_selection: ArchetypeSelection::default(),
            spectrum_configuration: SpectrumConfiguration::from_constants(
                &primary.universal_constants,
            ),
            density_range: (1.0, 8.0),
            inherited_constants: primary.universal_constants.clone(),
        }
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.galactic_id = id;
        self
    }

    pub fn derive_solar_parameters(&self) -> SolarLogosConfig {
        SolarLogosConfig::from_galactic(self)
    }

    pub fn hierarchy_level(&self) -> HierarchyLevel {
        HierarchyLevel::Galactic
    }
}

#[derive(Debug, Clone)]
pub struct ArchetypeSelection {
    pub primary_archetypes: Vec<usize>,
    pub archetype_weights: HashMap<usize, Float>,
    pub activation_threshold: Float,
}

impl Default for ArchetypeSelection {
    fn default() -> Self {
        let mut weights = HashMap::new();
        for i in 0..22 {
            weights.insert(i, 1.0 / 22.0);
        }
        Self {
            primary_archetypes: (0..22).collect(),
            archetype_weights: weights,
            activation_threshold: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpectrumConfiguration {
    pub base_velocity_ratio: Float,
    pub density_coupling: Float,
    pub veil_base_transparency: Float,
}

impl SpectrumConfiguration {
    pub fn from_constants(constants: &UniversalConstants) -> Self {
        Self {
            base_velocity_ratio: 1.0,
            density_coupling: constants.love_strength * 0.5,
            veil_base_transparency: 0.0,
        }
    }
}

impl Default for SpectrumConfiguration {
    fn default() -> Self {
        Self {
            base_velocity_ratio: 1.0,
            density_coupling: 0.5,
            veil_base_transparency: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SolarLogosConfig {
    pub solar_id: u64,
    pub parent_galactic_id: u64,
    pub archetype_system: ArchetypeSystem,
    pub veil_parameters: VeilParameters,
    pub entity_capacity: usize,
    pub inherited_spectrum: SpectrumConfiguration,
}

impl SolarLogosConfig {
    pub fn from_galactic(galactic: &GalacticLogosConfig) -> Self {
        Self {
            solar_id: 0,
            parent_galactic_id: galactic.galactic_id,
            archetype_system: ArchetypeSystem::from_selection(&galactic.archetype_selection),
            veil_parameters: VeilParameters::default(),
            entity_capacity: 1000,
            inherited_spectrum: galactic.spectrum_configuration.clone(),
        }
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.solar_id = id;
        self
    }

    pub fn derive_planetary_parameters(&self) -> PlanetaryLogosConfig {
        PlanetaryLogosConfig::from_solar(self)
    }

    pub fn hierarchy_level(&self) -> HierarchyLevel {
        HierarchyLevel::Solar
    }
}

#[derive(Debug, Clone)]
pub struct ArchetypeSystem {
    pub active_archetypes: Vec<usize>,
    pub archetype_interactions: HashMap<(usize, usize), Float>,
    pub system_name: String,
}

impl ArchetypeSystem {
    pub fn from_selection(selection: &ArchetypeSelection) -> Self {
        let mut interactions = HashMap::new();
        for &a in &selection.primary_archetypes {
            for &b in &selection.primary_archetypes {
                if a < b {
                    let weight_a = selection.archetype_weights.get(&a).copied().unwrap_or(0.0);
                    let weight_b = selection.archetype_weights.get(&b).copied().unwrap_or(0.0);
                    interactions.insert((a, b), (weight_a * weight_b).sqrt());
                }
            }
        }
        Self {
            active_archetypes: selection.primary_archetypes.clone(),
            archetype_interactions: interactions,
            system_name: "Default Archetype System".to_string(),
        }
    }
}

impl Default for ArchetypeSystem {
    fn default() -> Self {
        Self {
            active_archetypes: (0..22).collect(),
            archetype_interactions: HashMap::new(),
            system_name: "Default".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VeilParameters {
    pub base_thickness: Float,
    pub thinning_rate: Float,
    pub crossing_threshold: Float,
    pub transparency_per_density: Vec<Float>,
}

impl Default for VeilParameters {
    fn default() -> Self {
        Self {
            base_thickness: 1.0,
            thinning_rate: 0.01,
            crossing_threshold: 0.5,
            transparency_per_density: vec![0.05, 0.10, 0.10, 0.30, 0.50, 0.80, 1.00, 1.00],
        }
    }
}

impl VeilParameters {
    pub fn transparency_for_density(&self, density: Float) -> Float {
        let idx = density.floor() as usize;
        if idx < self.transparency_per_density.len() {
            let base = self.transparency_per_density[idx];
            let phase = density.fract();
            let next_idx = (idx + 1).min(self.transparency_per_density.len() - 1);
            let next = self.transparency_per_density[next_idx];
            base + (next - base) * phase
        } else {
            1.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlanetaryLogosConfig {
    pub planetary_id: u64,
    pub parent_solar_id: u64,
    pub blueprint: EntityBlueprint,
    pub entity_density_range: (Float, Float),
    pub catalyst_intensity: Float,
    pub inherited_veil: VeilParameters,
    pub inherited_capacity: usize,
}

impl PlanetaryLogosConfig {
    pub fn from_solar(solar: &SolarLogosConfig) -> Self {
        Self {
            planetary_id: 0,
            parent_solar_id: solar.solar_id,
            blueprint: EntityBlueprint::default(),
            entity_density_range: (1.0, 3.5),
            catalyst_intensity: 1.0,
            inherited_veil: solar.veil_parameters.clone(),
            inherited_capacity: solar.entity_capacity,
        }
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.planetary_id = id;
        self
    }

    pub fn hierarchy_level(&self) -> HierarchyLevel {
        HierarchyLevel::Planetary
    }

    pub fn derive_entity_parameters(&self) -> ManifestationParameters {
        ManifestationParameters::from_planetary(self)
    }
}

#[derive(Debug, Clone)]
pub struct EntityBlueprint {
    pub base_spectrum_position: Float,
    pub base_density: Float,
    pub archetype_activation_profile: [Float; 22],
    pub free_will_capacity: Float,
}

impl Default for EntityBlueprint {
    fn default() -> Self {
        Self {
            base_spectrum_position: 1.0,
            base_density: 3.0,
            archetype_activation_profile: [0.5; 22],
            free_will_capacity: 0.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManifestationParameters {
    pub spectrum_position: VelocityRatio,
    pub density_position: DensityPosition,
    pub archetype_profile: [Float; 22],
    pub free_will_capacity: Float,
    pub veil_transparency: Float,
    pub catalyst_sensitivity: Float,
}

impl ManifestationParameters {
    pub fn from_planetary(planetary: &PlanetaryLogosConfig) -> Self {
        let density = planetary.blueprint.base_density;
        Self {
            spectrum_position: VelocityRatio::new(planetary.blueprint.base_spectrum_position),
            density_position: DensityPosition::new(density),
            archetype_profile: planetary.blueprint.archetype_activation_profile,
            free_will_capacity: planetary.blueprint.free_will_capacity,
            veil_transparency: planetary.inherited_veil.transparency_for_density(density),
            catalyst_sensitivity: planetary.catalyst_intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_logos_creation() {
        let primary = PrimaryLogosConfig::new();
        assert_eq!(primary.hierarchy_level(), HierarchyLevel::Primary);
    }

    #[test]
    fn test_hierarchy_derivation() {
        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        assert_eq!(galactic.hierarchy_level(), HierarchyLevel::Galactic);
        assert_eq!(solar.hierarchy_level(), HierarchyLevel::Solar);
        assert_eq!(planetary.hierarchy_level(), HierarchyLevel::Planetary);
    }

    #[test]
    fn test_galactic_derives_from_primary() {
        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();

        assert_eq!(
            galactic.inherited_constants.base_frequency,
            primary.universal_constants.base_frequency
        );
    }

    #[test]
    fn test_veil_transparency_by_density() {
        let veil = VeilParameters::default();

        assert!(veil.transparency_for_density(1.0) < veil.transparency_for_density(5.0));
        assert!(veil.transparency_for_density(5.0) < veil.transparency_for_density(7.0));
    }

    #[test]
    fn test_entity_manifestation_from_planetary() {
        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();
        let entity = planetary.derive_entity_parameters();

        assert_eq!(entity.density_position.value, 3.0);
        assert!(entity.free_will_capacity > 0.0);
    }

    #[test]
    fn test_archetype_system_creation() {
        let selection = ArchetypeSelection::default();
        let system = ArchetypeSystem::from_selection(&selection);

        assert_eq!(system.active_archetypes.len(), 22);
    }
}
