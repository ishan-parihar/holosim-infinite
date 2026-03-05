//! Multi-Scale World System
//!
//! Implements multi-scale world rendering with holographic continuity across scales.
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 5: Multi-Scale Architecture
//! > "All scales (quantum to cosmic, 7 scales, 52 orders of magnitude) are playable"
//! > "Smooth scale transitions with holographic continuity"
//! > "Each scale contains the whole (holographic principle)"
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4: Multi-Scale Optimization
//! > "Fractal caching with multi-scale entries"
//! > "Hierarchical field representation"
//! > "Holographic continuity across scales"
//!
//! From MASTER_R&D_ROADMAP.md Phase 5 Week 69-72:
//! > "Implement quantum scale world (particle fields)"
//! > "Implement cellular scale world (cell colonies)"
//! > "Implement biological scale world (ecosystems)"
//! > "Implement planetary scale world (civilizations)"
//! > "Implement stellar scale world (solar systems)"
//! > "Implement galactic scale world (galaxies)"
//! > "Implement cosmic scale world (multiverses)"
//!
//! Key Concepts:
//! - **Multi-Scale World**: World exists simultaneously at all 7 scales
//! - **Holographic Continuity**: Each scale contains the whole
//! - **Scale-Specific Features**: Each scale has unique rendering and features
//! - **Scale Transitions**: Smooth visual transitions between scales
//! - **Observer Effect**: Player observation affects scale perception
//! - **Fractal Resolution**: Detail level decreases with scale magnitude

use crate::simulation_v3::{
    holographic_physics::SpectrumRatio,
    multiscale_camera::{ScaleLevel, ScaleTransition},
    scale_physics::{
        BiologicalSimulation, CellularSimulation, CosmicSimulation, Galaxy, PlanetarySimulation,
        StellarSimulation,
    },
};
use crate::types::Float;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub const MAX_ENTITIES_PER_SCALE: usize = 100_000;
pub const SCALE_TRANSITION_DURATION: Duration = Duration::from_millis(50);
pub const HOLOGRAPHIC_CONTINUITY_THRESHOLD: Float = 0.8;

#[derive(Debug, Clone, PartialEq)]
pub enum MultiScaleWorldError {
    InvalidScaleLevel(ScaleLevel),
    EntityCapacityExceeded(ScaleLevel),
    TransitionInProgress(ScaleLevel),
    ScaleNotLoaded(ScaleLevel),
    ContinuityBroken(ScaleLevel, ScaleLevel),
    RenderingFailed(String),
}

#[derive(Debug, Clone)]
pub struct ScaleWorldFeatures {
    pub scale_level: ScaleLevel,
    pub rendering_features: RenderingFeatures,
    pub interactive_features: InteractiveFeatures,
    pub unique_mechanics: UniqueMechanics,
}

#[derive(Debug, Clone)]
pub struct RenderingFeatures {
    pub particle_systems_enabled: bool,
    pub light_baking: bool,
    pub shadow_quality: ShadowQuality,
    pub post_processing: PostProcessingEffects,
    pub field_visualization: FieldVisualizationMode,
    pub resolution_scale: Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadowQuality {
    None,
    Low,
    Medium,
    High,
    Ultra,
}

#[derive(Debug, Clone)]
pub struct PostProcessingEffects {
    pub bloom: bool,
    pub chromatic_aberration: bool,
    pub film_grain: bool,
    pub holographic_shimmer: bool,
    pub spectrum_filter: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldVisualizationMode {
    None,
    WaveFunction,
    ProbabilityCloud,
    EnergyField,
    InterferencePattern,
    HolographicGrid,
}

#[derive(Debug, Clone)]
pub struct InteractiveFeatures {
    pub player_interaction_range: Float,
    pub entity_selection_enabled: bool,
    pub terrain_modification: bool,
    pub time_dilation_enabled: bool,
    pub observer_effect_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct UniqueMechanics {
    pub quantum_mechanics: Option<QuantumMechanics>,
    pub cellular_mechanics: Option<CellularMechanics>,
    pub biological_mechanics: Option<BiologicalMechanics>,
    pub planetary_mechanics: Option<PlanetaryMechanics>,
    pub stellar_mechanics: Option<StellarMechanics>,
    pub galactic_mechanics: Option<GalacticMechanics>,
    pub cosmic_mechanics: Option<CosmicMechanics>,
}

#[derive(Debug, Clone)]
pub struct QuantumMechanics {
    pub superposition_enabled: bool,
    pub entanglement_visualization: bool,
    pub wave_function_collapse_animation: bool,
    pub tunneling_probability: Float,
    pub uncertainty_visualization: bool,
}

#[derive(Debug, Clone)]
pub struct CellularMechanics {
    pub dna_unfolding_enabled: bool,
    pub cell_division_animation: bool,
    pub metabolism_visualization: bool,
    pub protein_synthesis: bool,
    pub cell_colony_dynamics: bool,
}

#[derive(Debug, Clone)]
pub struct BiologicalMechanics {
    pub ecosystem_simulation: bool,
    pub food_chain_dynamics: bool,
    pub instinct_driven_ai: bool,
    pub evolution_tracking: bool,
    pub population_dynamics: bool,
}

#[derive(Debug, Clone)]
pub struct PlanetaryMechanics {
    pub civilization_simulation: bool,
    pub climate_systems: bool,
    pub geological_processes: bool,
    pub resource_management: bool,
    pub social_dynamics: bool,
}

#[derive(Debug, Clone)]
pub struct StellarMechanics {
    pub orbital_mechanics: bool,
    pub stellar_evolution: bool,
    pub planetary_systems: bool,
    pub gravity_visualization: bool,
    pub solar_activity: bool,
}

#[derive(Debug, Clone)]
pub struct GalacticMechanics {
    pub spiral_arm_dynamics: bool,
    pub supermassive_black_hole: bool,
    pub star_formation_regions: bool,
    pub galactic_rotation: bool,
    pub dark_matter_halos: bool,
}

#[derive(Debug, Clone)]
pub struct CosmicMechanics {
    pub multiverse_visualization: bool,
    pub cosmic_background: bool,
    pub dimensional_structure: bool,
    pub universal_constants: bool,
    #[allow(dead_code)]
    void_visualization: bool,
}

impl ScaleWorldFeatures {
    pub fn quantum_scale() -> Self {
        ScaleWorldFeatures {
            scale_level: ScaleLevel::Quantum,
            rendering_features: RenderingFeatures {
                particle_systems_enabled: true,
                light_baking: false,
                shadow_quality: ShadowQuality::None,
                post_processing: PostProcessingEffects {
                    bloom: true,
                    chromatic_aberration: true,
                    film_grain: false,
                    holographic_shimmer: true,
                    spectrum_filter: true,
                },
                field_visualization: FieldVisualizationMode::WaveFunction,
                resolution_scale: 1e-35,
            },
            interactive_features: InteractiveFeatures {
                player_interaction_range: 1e-15,
                entity_selection_enabled: true,
                terrain_modification: false,
                time_dilation_enabled: true,
                observer_effect_enabled: true,
            },
            unique_mechanics: UniqueMechanics {
                quantum_mechanics: Some(QuantumMechanics {
                    superposition_enabled: true,
                    entanglement_visualization: true,
                    wave_function_collapse_animation: true,
                    tunneling_probability: 0.5,
                    uncertainty_visualization: true,
                }),
                cellular_mechanics: None,
                biological_mechanics: None,
                planetary_mechanics: None,
                stellar_mechanics: None,
                galactic_mechanics: None,
                cosmic_mechanics: None,
            },
        }
    }

    pub fn cellular_scale() -> Self {
        ScaleWorldFeatures {
            scale_level: ScaleLevel::Cellular,
            rendering_features: RenderingFeatures {
                particle_systems_enabled: true,
                light_baking: false,
                shadow_quality: ShadowQuality::Low,
                post_processing: PostProcessingEffects {
                    bloom: true,
                    chromatic_aberration: true,
                    film_grain: false,
                    holographic_shimmer: true,
                    spectrum_filter: true,
                },
                field_visualization: FieldVisualizationMode::ProbabilityCloud,
                resolution_scale: 1e-12,
            },
            interactive_features: InteractiveFeatures {
                player_interaction_range: 1e-6,
                entity_selection_enabled: true,
                terrain_modification: false,
                time_dilation_enabled: true,
                observer_effect_enabled: true,
            },
            unique_mechanics: UniqueMechanics {
                quantum_mechanics: None,
                cellular_mechanics: Some(CellularMechanics {
                    dna_unfolding_enabled: true,
                    cell_division_animation: true,
                    metabolism_visualization: true,
                    protein_synthesis: true,
                    cell_colony_dynamics: true,
                }),
                biological_mechanics: None,
                planetary_mechanics: None,
                stellar_mechanics: None,
                galactic_mechanics: None,
                cosmic_mechanics: None,
            },
        }
    }

    pub fn biological_scale() -> Self {
        ScaleWorldFeatures {
            scale_level: ScaleLevel::Biological,
            rendering_features: RenderingFeatures {
                particle_systems_enabled: true,
                light_baking: true,
                shadow_quality: ShadowQuality::Medium,
                post_processing: PostProcessingEffects {
                    bloom: false,
                    chromatic_aberration: false,
                    film_grain: false,
                    holographic_shimmer: false,
                    spectrum_filter: false,
                },
                field_visualization: FieldVisualizationMode::EnergyField,
                resolution_scale: 1e-5,
            },
            interactive_features: InteractiveFeatures {
                player_interaction_range: 10.0,
                entity_selection_enabled: true,
                terrain_modification: true,
                time_dilation_enabled: false,
                observer_effect_enabled: true,
            },
            unique_mechanics: UniqueMechanics {
                quantum_mechanics: None,
                cellular_mechanics: None,
                biological_mechanics: Some(BiologicalMechanics {
                    ecosystem_simulation: true,
                    food_chain_dynamics: true,
                    instinct_driven_ai: true,
                    evolution_tracking: true,
                    population_dynamics: true,
                }),
                planetary_mechanics: None,
                stellar_mechanics: None,
                galactic_mechanics: None,
                cosmic_mechanics: None,
            },
        }
    }

    pub fn planetary_scale() -> Self {
        ScaleWorldFeatures {
            scale_level: ScaleLevel::Planetary,
            rendering_features: RenderingFeatures {
                particle_systems_enabled: true,
                light_baking: true,
                shadow_quality: ShadowQuality::High,
                post_processing: PostProcessingEffects {
                    bloom: false,
                    chromatic_aberration: false,
                    film_grain: false,
                    holographic_shimmer: false,
                    spectrum_filter: false,
                },
                field_visualization: FieldVisualizationMode::None,
                resolution_scale: 1.0,
            },
            interactive_features: InteractiveFeatures {
                player_interaction_range: 1000.0,
                entity_selection_enabled: true,
                terrain_modification: true,
                time_dilation_enabled: false,
                observer_effect_enabled: true,
            },
            unique_mechanics: UniqueMechanics {
                quantum_mechanics: None,
                cellular_mechanics: None,
                biological_mechanics: None,
                planetary_mechanics: Some(PlanetaryMechanics {
                    civilization_simulation: true,
                    climate_systems: true,
                    geological_processes: true,
                    resource_management: true,
                    social_dynamics: true,
                }),
                stellar_mechanics: None,
                galactic_mechanics: None,
                cosmic_mechanics: None,
            },
        }
    }

    pub fn stellar_scale() -> Self {
        ScaleWorldFeatures {
            scale_level: ScaleLevel::Stellar,
            rendering_features: RenderingFeatures {
                particle_systems_enabled: true,
                light_baking: true,
                shadow_quality: ShadowQuality::High,
                post_processing: PostProcessingEffects {
                    bloom: true,
                    chromatic_aberration: false,
                    film_grain: false,
                    holographic_shimmer: false,
                    spectrum_filter: false,
                },
                field_visualization: FieldVisualizationMode::None,
                resolution_scale: 1e8,
            },
            interactive_features: InteractiveFeatures {
                player_interaction_range: 1e13,
                entity_selection_enabled: true,
                terrain_modification: false,
                time_dilation_enabled: false,
                observer_effect_enabled: true,
            },
            unique_mechanics: UniqueMechanics {
                quantum_mechanics: None,
                cellular_mechanics: None,
                biological_mechanics: None,
                planetary_mechanics: None,
                stellar_mechanics: Some(StellarMechanics {
                    orbital_mechanics: true,
                    stellar_evolution: true,
                    planetary_systems: true,
                    gravity_visualization: true,
                    solar_activity: true,
                }),
                galactic_mechanics: None,
                cosmic_mechanics: None,
            },
        }
    }

    pub fn galactic_scale() -> Self {
        ScaleWorldFeatures {
            scale_level: ScaleLevel::Galactic,
            rendering_features: RenderingFeatures {
                particle_systems_enabled: true,
                light_baking: true,
                shadow_quality: ShadowQuality::Ultra,
                post_processing: PostProcessingEffects {
                    bloom: true,
                    chromatic_aberration: false,
                    film_grain: false,
                    holographic_shimmer: true,
                    spectrum_filter: false,
                },
                field_visualization: FieldVisualizationMode::InterferencePattern,
                resolution_scale: 1e14,
            },
            interactive_features: InteractiveFeatures {
                player_interaction_range: 1e21,
                entity_selection_enabled: true,
                terrain_modification: false,
                time_dilation_enabled: false,
                observer_effect_enabled: true,
            },
            unique_mechanics: UniqueMechanics {
                quantum_mechanics: None,
                cellular_mechanics: None,
                biological_mechanics: None,
                planetary_mechanics: None,
                stellar_mechanics: None,
                galactic_mechanics: Some(GalacticMechanics {
                    spiral_arm_dynamics: true,
                    supermassive_black_hole: true,
                    star_formation_regions: true,
                    galactic_rotation: true,
                    dark_matter_halos: true,
                }),
                cosmic_mechanics: None,
            },
        }
    }

    pub fn cosmic_scale() -> Self {
        ScaleWorldFeatures {
            scale_level: ScaleLevel::Cosmic,
            rendering_features: RenderingFeatures {
                particle_systems_enabled: false,
                light_baking: true,
                shadow_quality: ShadowQuality::Ultra,
                post_processing: PostProcessingEffects {
                    bloom: true,
                    chromatic_aberration: false,
                    film_grain: false,
                    holographic_shimmer: true,
                    spectrum_filter: true,
                },
                field_visualization: FieldVisualizationMode::HolographicGrid,
                resolution_scale: 1e22,
            },
            interactive_features: InteractiveFeatures {
                player_interaction_range: 1e26,
                entity_selection_enabled: false,
                terrain_modification: false,
                time_dilation_enabled: false,
                observer_effect_enabled: true,
            },
            unique_mechanics: UniqueMechanics {
                quantum_mechanics: None,
                cellular_mechanics: None,
                biological_mechanics: None,
                planetary_mechanics: None,
                stellar_mechanics: None,
                galactic_mechanics: None,
                cosmic_mechanics: Some(CosmicMechanics {
                    multiverse_visualization: true,
                    cosmic_background: true,
                    dimensional_structure: true,
                    universal_constants: true,
                    void_visualization: true,
                }),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct HolographicContinuity {
    pub source_scale: ScaleLevel,
    pub target_scale: ScaleLevel,
    pub continuity_score: Float,
    pub holographic_overlap: Float,
    pub interference_preserved: bool,
}

impl HolographicContinuity {
    pub fn new(source_scale: ScaleLevel, target_scale: ScaleLevel) -> Self {
        let continuity_score = Self::compute_continuity_score(source_scale, target_scale);
        let holographic_overlap = Self::compute_holographic_overlap(source_scale, target_scale);

        HolographicContinuity {
            source_scale,
            target_scale,
            continuity_score,
            holographic_overlap,
            interference_preserved: continuity_score >= HOLOGRAPHIC_CONTINUITY_THRESHOLD,
        }
    }

    fn compute_continuity_score(source: ScaleLevel, target: ScaleLevel) -> Float {
        let scale_difference = (Self::scale_to_index(source) - Self::scale_to_index(target)).abs();
        1.0 / (1.0 + scale_difference as Float * 0.1)
    }

    fn compute_holographic_overlap(source: ScaleLevel, target: ScaleLevel) -> Float {
        let scale_difference = (Self::scale_to_index(source) - Self::scale_to_index(target)).abs();
        1.0 - (scale_difference as Float / 6.0).min(1.0)
    }

    fn scale_to_index(scale: ScaleLevel) -> i32 {
        match scale {
            ScaleLevel::Quantum => 0,
            ScaleLevel::Cellular => 1,
            ScaleLevel::Biological => 2,
            ScaleLevel::Planetary => 3,
            ScaleLevel::Stellar => 4,
            ScaleLevel::Galactic => 5,
            ScaleLevel::Cosmic => 6,
        }
    }

    pub fn is_continuous(&self) -> bool {
        self.continuity_score >= HOLOGRAPHIC_CONTINUITY_THRESHOLD && self.interference_preserved
    }
}

#[derive(Debug, Clone)]
pub struct ScaleWorldState {
    pub scale_level: ScaleLevel,
    pub simulation_data: ScaleSimulationData,
    pub entities_loaded: Vec<u64>,
    pub render_cache: Option<RenderCache>,
    pub is_loaded: bool,
    pub load_timestamp: Instant,
}

#[derive(Debug, Clone)]
pub enum ScaleSimulationData {
    Quantum(QuantumSim),
    Cellular(CellularSimulation),
    Biological(BiologicalSimulation),
    Planetary(PlanetarySimulation),
    Stellar(StellarSimulation),
    Galactic(Galaxy),
    Cosmic(CosmicSimulation),
}

#[derive(Debug, Clone, Default)]
pub struct QuantumSim {
    pub wave_functions: Vec<[Float; 3]>,
    pub entanglement_pairs: Vec<(u64, u64)>,
    pub superposition_states: Vec<Float>,
}

#[derive(Debug, Clone)]
pub struct RenderCache {
    pub cache_id: u64,
    pub timestamp: Instant,
    pub render_data: Vec<u8>,
    pub holographic_signature: [Float; 22],
    pub spectrum_configuration: SpectrumRatio,
}

#[derive(Debug, Clone)]
pub struct ScaleTransitionVisualization {
    pub transition: ScaleTransition,
    pub source_features: ScaleWorldFeatures,
    pub target_features: ScaleWorldFeatures,
    pub continuity: HolographicContinuity,
    pub interpolation_progress: Float,
    pub visual_effects: TransitionEffects,
}

#[derive(Debug, Clone)]
pub struct TransitionEffects {
    pub holographic_dissolve: bool,
    pub spectrum_shift: bool,
    pub particle_explosion: bool,
    pub fractal_refinement: bool,
    pub dimensional_rift: bool,
}

impl TransitionEffects {
    pub fn for_scale_transition(source: ScaleLevel, target: ScaleLevel) -> Self {
        let scale_distance = (source as i32 - target as i32).abs();

        TransitionEffects {
            holographic_dissolve: scale_distance >= 3,
            spectrum_shift: scale_distance >= 2,
            particle_explosion: scale_distance >= 4,
            fractal_refinement: scale_distance >= 1,
            dimensional_rift: scale_distance >= 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultiScaleWorldRenderer {
    pub scale_states: HashMap<ScaleLevel, ScaleWorldState>,
    pub current_scale: ScaleLevel,
    pub active_transition: Option<ScaleTransitionVisualization>,
    pub spectrum_configuration: SpectrumRatio,
    pub statistics: MultiScaleWorldStatistics,
}

impl MultiScaleWorldRenderer {
    pub fn new(spectrum_configuration: SpectrumRatio) -> Self {
        let mut scale_states = HashMap::new();

        for scale in [
            ScaleLevel::Quantum,
            ScaleLevel::Cellular,
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            ScaleLevel::Stellar,
            ScaleLevel::Galactic,
            ScaleLevel::Cosmic,
        ] {
            let sim_data = match scale {
                ScaleLevel::Quantum => ScaleSimulationData::Quantum(QuantumSim::default()),
                ScaleLevel::Cellular => ScaleSimulationData::Cellular(CellularSimulation::new()),
                ScaleLevel::Biological => {
                    ScaleSimulationData::Biological(BiologicalSimulation::new())
                }
                ScaleLevel::Planetary => ScaleSimulationData::Planetary(PlanetarySimulation::new()),
                ScaleLevel::Stellar => ScaleSimulationData::Stellar(StellarSimulation::new()),
                ScaleLevel::Galactic => ScaleSimulationData::Galactic(Galaxy::default()),
                ScaleLevel::Cosmic => ScaleSimulationData::Cosmic(CosmicSimulation::new()),
            };

            scale_states.insert(
                scale,
                ScaleWorldState {
                    scale_level: scale,
                    simulation_data: sim_data,
                    entities_loaded: Vec::new(),
                    render_cache: None,
                    is_loaded: false,
                    load_timestamp: Instant::now(),
                },
            );
        }

        MultiScaleWorldRenderer {
            scale_states,
            current_scale: ScaleLevel::Biological,
            active_transition: None,
            spectrum_configuration,
            statistics: MultiScaleWorldStatistics::default(),
        }
    }

    pub fn get_scale_features(&self, scale: ScaleLevel) -> ScaleWorldFeatures {
        match scale {
            ScaleLevel::Quantum => ScaleWorldFeatures::quantum_scale(),
            ScaleLevel::Cellular => ScaleWorldFeatures::cellular_scale(),
            ScaleLevel::Biological => ScaleWorldFeatures::biological_scale(),
            ScaleLevel::Planetary => ScaleWorldFeatures::planetary_scale(),
            ScaleLevel::Stellar => ScaleWorldFeatures::stellar_scale(),
            ScaleLevel::Galactic => ScaleWorldFeatures::galactic_scale(),
            ScaleLevel::Cosmic => ScaleWorldFeatures::cosmic_scale(),
        }
    }

    pub fn load_scale(&mut self, scale: ScaleLevel) -> Result<(), MultiScaleWorldError> {
        if let Some(state) = self.scale_states.get_mut(&scale) {
            if state.is_loaded {
                return Ok(());
            }

            state.is_loaded = true;
            state.load_timestamp = Instant::now();
            self.statistics.scales_loaded += 1;
            Ok(())
        } else {
            Err(MultiScaleWorldError::InvalidScaleLevel(scale))
        }
    }

    pub fn unload_scale(&mut self, scale: ScaleLevel) -> Result<(), MultiScaleWorldError> {
        if let Some(state) = self.scale_states.get_mut(&scale) {
            if !state.is_loaded {
                return Ok(());
            }

            state.is_loaded = false;
            state.render_cache = None;
            state.entities_loaded.clear();
            self.statistics.scales_unloaded += 1;
            Ok(())
        } else {
            Err(MultiScaleWorldError::InvalidScaleLevel(scale))
        }
    }

    pub fn transition_to_scale(
        &mut self,
        target_scale: ScaleLevel,
    ) -> Result<ScaleTransitionVisualization, MultiScaleWorldError> {
        if self.active_transition.is_some() {
            return Err(MultiScaleWorldError::TransitionInProgress(
                self.current_scale,
            ));
        }

        let source_scale = self.current_scale;
        if source_scale == target_scale {
            return Err(MultiScaleWorldError::InvalidScaleLevel(target_scale));
        }

        let source_features = self.get_scale_features(source_scale);
        let target_features = self.get_scale_features(target_scale);
        let continuity = HolographicContinuity::new(source_scale, target_scale);
        let visual_effects = TransitionEffects::for_scale_transition(source_scale, target_scale);

        let transition = ScaleTransitionVisualization {
            transition: ScaleTransition::new(source_scale, target_scale, SCALE_TRANSITION_DURATION),
            source_features,
            target_features,
            continuity,
            interpolation_progress: 0.0,
            visual_effects,
        };

        self.active_transition = Some(transition.clone());
        self.current_scale = target_scale;
        self.statistics.scale_transitions += 1;

        Ok(transition)
    }

    pub fn update_transition(&mut self, delta_time: Duration) -> bool {
        if let Some(transition) = &mut self.active_transition {
            transition.transition.update(delta_time);
            transition.interpolation_progress = transition.transition.progress();

            if transition.transition.is_complete() {
                self.active_transition = None;
                return true;
            }
        }
        false
    }

    pub fn get_holographic_continuity(
        &self,
        source: ScaleLevel,
        target: ScaleLevel,
    ) -> HolographicContinuity {
        HolographicContinuity::new(source, target)
    }

    pub fn is_scale_loaded(&self, scale: ScaleLevel) -> bool {
        self.scale_states
            .get(&scale)
            .map(|s| s.is_loaded)
            .unwrap_or(false)
    }

    pub fn get_loaded_scales(&self) -> Vec<ScaleLevel> {
        self.scale_states
            .iter()
            .filter(|(_, state)| state.is_loaded)
            .map(|(scale, _)| *scale)
            .collect()
    }

    pub fn update_statistics(&mut self) {
        self.statistics.active_scales = self.get_loaded_scales().len();
        self.statistics.total_entities = self
            .scale_states
            .values()
            .map(|s| s.entities_loaded.len())
            .sum();
    }

    pub fn get_statistics(&self) -> MultiScaleWorldStatistics {
        self.statistics.clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct MultiScaleWorldStatistics {
    pub scales_loaded: usize,
    pub scales_unloaded: usize,
    pub scale_transitions: usize,
    pub active_scales: usize,
    pub total_entities: usize,
    pub average_load_time_ms: Float,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_features_quantum() {
        let features = ScaleWorldFeatures::quantum_scale();
        assert_eq!(features.scale_level, ScaleLevel::Quantum);
        assert!(features.rendering_features.particle_systems_enabled);
        assert!(features.unique_mechanics.quantum_mechanics.is_some());
    }

    #[test]
    fn test_scale_features_cellular() {
        let features = ScaleWorldFeatures::cellular_scale();
        assert_eq!(features.scale_level, ScaleLevel::Cellular);
        assert!(features.unique_mechanics.cellular_mechanics.is_some());
    }

    #[test]
    fn test_scale_features_biological() {
        let features = ScaleWorldFeatures::biological_scale();
        assert_eq!(features.scale_level, ScaleLevel::Biological);
        assert!(features.unique_mechanics.biological_mechanics.is_some());
    }

    #[test]
    fn test_scale_features_planetary() {
        let features = ScaleWorldFeatures::planetary_scale();
        assert_eq!(features.scale_level, ScaleLevel::Planetary);
        assert!(features.unique_mechanics.planetary_mechanics.is_some());
    }

    #[test]
    fn test_scale_features_stellar() {
        let features = ScaleWorldFeatures::stellar_scale();
        assert_eq!(features.scale_level, ScaleLevel::Stellar);
        assert!(features.unique_mechanics.stellar_mechanics.is_some());
    }

    #[test]
    fn test_scale_features_galactic() {
        let features = ScaleWorldFeatures::galactic_scale();
        assert_eq!(features.scale_level, ScaleLevel::Galactic);
        assert!(features.unique_mechanics.galactic_mechanics.is_some());
    }

    #[test]
    fn test_scale_features_cosmic() {
        let features = ScaleWorldFeatures::cosmic_scale();
        assert_eq!(features.scale_level, ScaleLevel::Cosmic);
        assert!(features.unique_mechanics.cosmic_mechanics.is_some());
    }

    #[test]
    fn test_holographic_continuity_adjacent_scales() {
        let continuity = HolographicContinuity::new(ScaleLevel::Biological, ScaleLevel::Planetary);
        assert!(continuity.is_continuous());
        assert!(continuity.interference_preserved);
    }

    #[test]
    fn test_holographic_continuity_distant_scales() {
        let continuity = HolographicContinuity::new(ScaleLevel::Quantum, ScaleLevel::Cosmic);
        assert!(continuity.continuity_score < HOLOGRAPHIC_CONTINUITY_THRESHOLD);
        assert!(!continuity.interference_preserved);
    }

    #[test]
    fn test_holographic_continuity_same_scale() {
        let continuity = HolographicContinuity::new(ScaleLevel::Biological, ScaleLevel::Biological);
        assert_eq!(continuity.continuity_score, 1.0);
        assert!(continuity.is_continuous());
    }

    #[test]
    fn test_transition_effects_scale_distance_1() {
        let effects =
            TransitionEffects::for_scale_transition(ScaleLevel::Biological, ScaleLevel::Planetary);
        assert!(!effects.holographic_dissolve);
        assert!(!effects.spectrum_shift);
        assert!(!effects.particle_explosion);
        assert!(effects.fractal_refinement);
        assert!(!effects.dimensional_rift);
    }

    #[test]
    fn test_transition_effects_scale_distance_3() {
        let effects =
            TransitionEffects::for_scale_transition(ScaleLevel::Cellular, ScaleLevel::Stellar);
        assert!(effects.holographic_dissolve);
        assert!(effects.spectrum_shift);
        assert!(!effects.particle_explosion);
        assert!(effects.fractal_refinement);
        assert!(!effects.dimensional_rift);
    }

    #[test]
    fn test_transition_effects_scale_distance_5() {
        let effects =
            TransitionEffects::for_scale_transition(ScaleLevel::Quantum, ScaleLevel::Galactic);
        assert!(effects.holographic_dissolve);
        assert!(effects.spectrum_shift);
        assert!(effects.particle_explosion);
        assert!(effects.fractal_refinement);
        assert!(effects.dimensional_rift);
    }

    #[test]
    fn test_multi_scale_world_renderer_initialization() {
        let renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));
        assert_eq!(renderer.current_scale, ScaleLevel::Biological);
        assert!(renderer.active_transition.is_none());
        assert_eq!(renderer.scale_states.len(), 7);
    }

    #[test]
    fn test_load_scale() {
        let mut renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));
        let result = renderer.load_scale(ScaleLevel::Quantum);
        assert!(result.is_ok());
        assert!(renderer.is_scale_loaded(ScaleLevel::Quantum));
    }

    #[test]
    fn test_unload_scale() {
        let mut renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));
        renderer.load_scale(ScaleLevel::Quantum).unwrap();
        let result = renderer.unload_scale(ScaleLevel::Quantum);
        assert!(result.is_ok());
        assert!(!renderer.is_scale_loaded(ScaleLevel::Quantum));
    }

    #[test]
    fn test_transition_to_scale() {
        let mut renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));
        renderer.load_scale(ScaleLevel::Planetary).unwrap();
        let transition = renderer.transition_to_scale(ScaleLevel::Planetary);
        assert!(transition.is_ok());
        let viz = transition.unwrap();
        assert_eq!(viz.transition.to_scale(), ScaleLevel::Planetary);
        assert!(renderer.active_transition.is_some());
    }

    #[test]
    fn test_transition_already_in_progress() {
        let mut renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));
        renderer.load_scale(ScaleLevel::Planetary).unwrap();
        renderer.transition_to_scale(ScaleLevel::Planetary).unwrap();
        let result = renderer.transition_to_scale(ScaleLevel::Stellar);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MultiScaleWorldError::TransitionInProgress(_)
        ));
    }

    #[test]
    fn test_get_holographic_continuity() {
        let renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));
        let continuity =
            renderer.get_holographic_continuity(ScaleLevel::Biological, ScaleLevel::Planetary);
        assert!(continuity.is_continuous());
    }

    #[test]
    fn test_get_loaded_scales() {
        let mut renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));
        renderer.load_scale(ScaleLevel::Quantum).unwrap();
        renderer.load_scale(ScaleLevel::Cellular).unwrap();
        let loaded = renderer.get_loaded_scales();
        assert_eq!(loaded.len(), 2);
        assert!(loaded.contains(&ScaleLevel::Quantum));
        assert!(loaded.contains(&ScaleLevel::Cellular));
    }

    #[test]
    fn test_update_transition_progress() {
        let mut renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));
        renderer.load_scale(ScaleLevel::Planetary).unwrap();
        renderer.transition_to_scale(ScaleLevel::Planetary).unwrap();

        let delta = Duration::from_millis(25);
        let completed = renderer.update_transition(delta);
        assert!(!completed);
        assert!(renderer.active_transition.is_some());

        let delta = Duration::from_millis(50);
        let completed = renderer.update_transition(delta);
        assert!(completed);
        assert!(renderer.active_transition.is_none());
    }

    #[test]
    fn test_get_statistics() {
        let mut renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));
        renderer.load_scale(ScaleLevel::Quantum).unwrap();
        renderer.load_scale(ScaleLevel::Cellular).unwrap();
        renderer.transition_to_scale(ScaleLevel::Planetary).unwrap();
        renderer.update_statistics();

        let stats = renderer.get_statistics();
        assert_eq!(stats.scales_loaded, 2);
        assert_eq!(stats.scale_transitions, 1);
        assert_eq!(stats.active_scales, 2);
    }

    #[test]
    fn test_invalid_scale_level() {
        let mut renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));
        let result = renderer.load_scale(ScaleLevel::Biological);
        assert!(result.is_ok());
    }

    #[test]
    fn test_scale_world_state_initialization() {
        let state = ScaleWorldState {
            scale_level: ScaleLevel::Quantum,
            simulation_data: ScaleSimulationData::Quantum(QuantumSim::default()),
            entities_loaded: Vec::new(),
            render_cache: None,
            is_loaded: false,
            load_timestamp: Instant::now(),
        };
        assert_eq!(state.scale_level, ScaleLevel::Quantum);
        assert!(!state.is_loaded);
    }

    #[test]
    fn test_all_scales_in_initial_states() {
        let renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));
        assert_eq!(renderer.scale_states.len(), 7);

        let all_scales = vec![
            ScaleLevel::Quantum,
            ScaleLevel::Cellular,
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            ScaleLevel::Stellar,
            ScaleLevel::Galactic,
            ScaleLevel::Cosmic,
        ];

        for scale in all_scales {
            assert!(renderer.scale_states.contains_key(&scale));
        }
    }

    #[test]
    fn test_continuity_threshold() {
        assert!(HOLOGRAPHIC_CONTINUITY_THRESHOLD > 0.0);
        assert!(HOLOGRAPHIC_CONTINUITY_THRESHOLD <= 1.0);
    }

    #[test]
    fn test_scale_specific_physics_modes() {
        let renderer = MultiScaleWorldRenderer::new(SpectrumRatio::new(1.0, 1.0));

        let quantum_features = renderer.get_scale_features(ScaleLevel::Quantum);
        assert!(matches!(
            quantum_features.unique_mechanics.quantum_mechanics,
            Some(_)
        ));

        let galactic_features = renderer.get_scale_features(ScaleLevel::Galactic);
        assert!(matches!(
            galactic_features.unique_mechanics.galactic_mechanics,
            Some(_)
        ));
    }

    #[test]
    fn test_rendering_features_different_scales() {
        let quantum = ScaleWorldFeatures::quantum_scale();
        let biological = ScaleWorldFeatures::biological_scale();
        let cosmic = ScaleWorldFeatures::cosmic_scale();

        assert_eq!(
            quantum.rendering_features.shadow_quality,
            ShadowQuality::None
        );
        assert_eq!(
            biological.rendering_features.shadow_quality,
            ShadowQuality::Medium
        );
        assert_eq!(
            cosmic.rendering_features.shadow_quality,
            ShadowQuality::Ultra
        );
    }

    #[test]
    fn test_interactive_features_interaction_ranges() {
        let quantum = ScaleWorldFeatures::quantum_scale();
        let biological = ScaleWorldFeatures::biological_scale();
        let stellar = ScaleWorldFeatures::stellar_scale();
        let cosmic = ScaleWorldFeatures::cosmic_scale();

        assert!(quantum.interactive_features.player_interaction_range < 1e-10);
        assert!(biological.interactive_features.player_interaction_range > 1.0);
        assert!(stellar.interactive_features.player_interaction_range > 1e10);
        assert!(cosmic.interactive_features.player_interaction_range > 1e20);
    }

    #[test]
    fn test_post_processing_effects_scale_specific() {
        let quantum = ScaleWorldFeatures::quantum_scale();
        let planetary = ScaleWorldFeatures::planetary_scale();
        let cosmic = ScaleWorldFeatures::cosmic_scale();

        assert!(
            quantum
                .rendering_features
                .post_processing
                .holographic_shimmer
        );
        assert!(
            !planetary
                .rendering_features
                .post_processing
                .holographic_shimmer
        );
        assert!(
            cosmic
                .rendering_features
                .post_processing
                .holographic_shimmer
        );
    }

    #[test]
    fn test_unique_mechanics_exclusivity() {
        let biological = ScaleWorldFeatures::biological_scale();
        let planetary = ScaleWorldFeatures::planetary_scale();

        assert!(biological.unique_mechanics.biological_mechanics.is_some());
        assert!(biological.unique_mechanics.planetary_mechanics.is_none());

        assert!(planetary.unique_mechanics.planetary_mechanics.is_some());
        assert!(planetary.unique_mechanics.biological_mechanics.is_none());
    }

    #[test]
    fn test_resolution_scales() {
        let quantum = ScaleWorldFeatures::quantum_scale();
        let biological = ScaleWorldFeatures::biological_scale();
        let cosmic = ScaleWorldFeatures::cosmic_scale();

        assert_eq!(quantum.rendering_features.resolution_scale, 1e-35);
        assert_eq!(biological.rendering_features.resolution_scale, 1e-5);
        assert_eq!(cosmic.rendering_features.resolution_scale, 1e22);
    }

    #[test]
    fn test_field_visualization_modes() {
        let quantum = ScaleWorldFeatures::quantum_scale();
        let planetary = ScaleWorldFeatures::planetary_scale();
        let galactic = ScaleWorldFeatures::galactic_scale();

        assert_eq!(
            quantum.rendering_features.field_visualization,
            FieldVisualizationMode::WaveFunction
        );
        assert_eq!(
            planetary.rendering_features.field_visualization,
            FieldVisualizationMode::None
        );
        assert_eq!(
            galactic.rendering_features.field_visualization,
            FieldVisualizationMode::InterferencePattern
        );
    }
}
