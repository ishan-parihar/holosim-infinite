//! Involution Flow Propagation - Top-down causal flow
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Each level imposes boundary conditions on lower levels
//!  Lower levels inherit + refine patterns"
//!
//! KEY PRINCIPLE: "Transcend and Include"
//! - Each level includes all previous development
//! - Propagation is NOT simple inheritance but hierarchical refinement

use super::cosmic_hierarchy::{CosmicHierarchy, HierarchyPath, LevelConfig};
use super::logos_config::{
    GalacticLogosConfig, PlanetaryLogosConfig, PrimaryLogosConfig, SolarLogosConfig,
};
use super::{HierarchyLevel, PropagationMode};
use crate::holographic_foundation::spectrum::SpectrumState;
use crate::types::Float;

#[derive(Debug, Clone)]
pub struct InvolutionFlow {
    pub mode: PropagationMode,
    pub propagation_strength: Float,
    pub refinement_rate: Float,
}

impl Default for InvolutionFlow {
    fn default() -> Self {
        Self {
            mode: PropagationMode::Resonant,
            propagation_strength: 1.0,
            refinement_rate: 0.1,
        }
    }
}

impl InvolutionFlow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mode(mut self, mode: PropagationMode) -> Self {
        self.mode = mode;
        self
    }

    /// Propagate parameters from top of hierarchy down to entities
    ///
    /// The causal chain:
    /// Primary → Galactic → Solar → Planetary → Entity
    pub fn propagate_downward(&self, hierarchy: &CosmicHierarchy) -> PropagationResult {
        let mut result = PropagationResult::new();

        result.add_level(HierarchyLevel::Primary, PropagationStep::Base);

        if !hierarchy.galactic_logoi.is_empty() {
            result.add_level(
                HierarchyLevel::Galactic,
                PropagationStep::Inherit {
                    from_level: HierarchyLevel::Primary,
                    refinement: self.refinement_rate,
                },
            );
        }

        if !hierarchy.solar_logoi.is_empty() {
            result.add_level(
                HierarchyLevel::Solar,
                PropagationStep::Inherit {
                    from_level: HierarchyLevel::Galactic,
                    refinement: self.refinement_rate,
                },
            );
        }

        if !hierarchy.planetary_logoi.is_empty() {
            result.add_level(
                HierarchyLevel::Planetary,
                PropagationStep::Inherit {
                    from_level: HierarchyLevel::Solar,
                    refinement: self.refinement_rate,
                },
            );
        }

        result
    }

    /// Build the complete path from Primary to a specific planetary level
    pub fn build_path_to_planetary(
        &self,
        hierarchy: &CosmicHierarchy,
        planetary_id: usize,
    ) -> Option<HierarchyPath> {
        let (solar_id, galactic_id) = hierarchy.find_parent_chain(planetary_id)?;

        let mut path = HierarchyPath::new();

        path.push(
            HierarchyLevel::Primary,
            LevelConfig::Primary(hierarchy.primary_logos.clone()),
        );

        if galactic_id < hierarchy.galactic_logoi.len() {
            path.push(
                HierarchyLevel::Galactic,
                LevelConfig::Galactic(hierarchy.galactic_logoi[galactic_id].clone()),
            );
        }

        if solar_id < hierarchy.solar_logoi.len() {
            path.push(
                HierarchyLevel::Solar,
                LevelConfig::Solar(hierarchy.solar_logoi[solar_id].clone()),
            );
        }

        if planetary_id < hierarchy.planetary_logoi.len() {
            path.push(
                HierarchyLevel::Planetary,
                LevelConfig::Planetary(hierarchy.planetary_logoi[planetary_id].clone()),
            );
        }

        Some(path)
    }

    /// Apply transcending inclusion at a level transition
    ///
    /// "Transcend and Include": Each level includes all previous
    pub fn apply_transcend_include(
        &self,
        incoming: &FieldConfiguration,
        refinement: Float,
    ) -> FieldConfiguration {
        FieldConfiguration {
            distortion_config: incoming.distortion_config.clone(),
            spectrum_state: SpectrumState::new(),
            refinement_factor: refinement,
            included_levels: incoming.included_levels + 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropagationResult {
    pub levels: Vec<(HierarchyLevel, PropagationStep)>,
    pub success: bool,
}

impl PropagationResult {
    pub fn new() -> Self {
        Self {
            levels: Vec::new(),
            success: true,
        }
    }

    pub fn add_level(&mut self, level: HierarchyLevel, step: PropagationStep) {
        self.levels.push((level, step));
    }

    pub fn depth(&self) -> usize {
        self.levels.len()
    }

    pub fn final_level(&self) -> Option<HierarchyLevel> {
        self.levels.last().map(|(level, _)| *level)
    }
}

impl Default for PropagationResult {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum PropagationStep {
    Base,
    Inherit {
        from_level: HierarchyLevel,
        refinement: Float,
    },
    Transform {
        from_level: HierarchyLevel,
        transformation: String,
    },
}

#[derive(Debug, Clone)]
pub struct FieldConfiguration {
    pub distortion_config: crate::holographic_foundation::distortions::UnifiedFieldConfig,
    pub spectrum_state: SpectrumState,
    pub refinement_factor: Float,
    pub included_levels: usize,
}

impl Default for FieldConfiguration {
    fn default() -> Self {
        Self {
            distortion_config:
                crate::holographic_foundation::distortions::UnifiedFieldConfig::default(),
            spectrum_state: SpectrumState::new(),
            refinement_factor: 1.0,
            included_levels: 0,
        }
    }
}

impl FieldConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_primary(primary: &PrimaryLogosConfig) -> Self {
        Self {
            distortion_config: primary.base_distortion_config.clone(),
            spectrum_state: SpectrumState::new(),
            refinement_factor: 1.0,
            included_levels: 1,
        }
    }

    pub fn refine_from_galactic(&self, galactic: &GalacticLogosConfig) -> Self {
        Self {
            distortion_config: self.distortion_config.clone(),
            spectrum_state: SpectrumState::at_density(galactic.density_range.0),
            refinement_factor: self.refinement_factor * 0.9,
            included_levels: self.included_levels + 1,
        }
    }

    pub fn refine_from_solar(&self, _solar: &SolarLogosConfig) -> Self {
        Self {
            distortion_config: self.distortion_config.clone(),
            spectrum_state: SpectrumState::new(),
            refinement_factor: self.refinement_factor * 0.95,
            included_levels: self.included_levels + 1,
        }
    }

    pub fn refine_from_planetary(&self, planetary: &PlanetaryLogosConfig) -> Self {
        Self {
            distortion_config: self.distortion_config.clone(),
            spectrum_state: SpectrumState::at_density(planetary.entity_density_range.0),
            refinement_factor: self.refinement_factor * 0.98,
            included_levels: self.included_levels + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_involution_flow_creation() {
        let flow = InvolutionFlow::new();
        assert_eq!(flow.mode, PropagationMode::Resonant);
    }

    #[test]
    fn test_propagate_downward() {
        let flow = InvolutionFlow::new();
        let hierarchy = CosmicHierarchy::new();

        let result = flow.propagate_downward(&hierarchy);

        assert!(result.success);
        assert_eq!(result.depth(), 1);
        assert_eq!(result.final_level(), Some(HierarchyLevel::Primary));
    }

    #[test]
    fn test_propagate_full_hierarchy() {
        let flow = InvolutionFlow::new();
        let mut hierarchy = CosmicHierarchy::new();

        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        hierarchy.add_galactic_logos(galactic);
        hierarchy.add_solar_logos(solar, 0);
        hierarchy.add_planetary_logos(planetary, 0);

        let result = flow.propagate_downward(&hierarchy);

        assert_eq!(result.depth(), 4);
        assert_eq!(result.final_level(), Some(HierarchyLevel::Planetary));
    }

    #[test]
    fn test_build_path_to_planetary() {
        let flow = InvolutionFlow::new();
        let mut hierarchy = CosmicHierarchy::new();

        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        hierarchy.add_galactic_logos(galactic);
        hierarchy.add_solar_logos(solar, 0);
        hierarchy.add_planetary_logos(planetary, 0);

        let path = flow.build_path_to_planetary(&hierarchy, 0);

        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.depth(), 4);
    }

    #[test]
    fn test_transcend_include() {
        let flow = InvolutionFlow::new();
        let incoming = FieldConfiguration::new();

        let result = flow.apply_transcend_include(&incoming, 0.1);

        assert_eq!(result.included_levels, incoming.included_levels + 1);
    }

    #[test]
    fn test_field_configuration_refinement() {
        let primary = PrimaryLogosConfig::new();
        let galactic = primary.derive_galactic_parameters();
        let solar = galactic.derive_solar_parameters();
        let planetary = solar.derive_planetary_parameters();

        let field = FieldConfiguration::from_primary(&primary);
        assert_eq!(field.included_levels, 1);

        let field = field.refine_from_galactic(&galactic);
        assert_eq!(field.included_levels, 2);

        let field = field.refine_from_solar(&solar);
        assert_eq!(field.included_levels, 3);

        let field = field.refine_from_planetary(&planetary);
        assert_eq!(field.included_levels, 4);
    }
}
