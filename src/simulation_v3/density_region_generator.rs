//! Density-Specific Region Generator - Week 73-76: Density-Specific Regions
//!
//! From MASTER_R&D_ROADMAP.md Week 73-76:
//! - Implement 1st Density regions (matter formation)
//! - Implement 2nd Density regions (evolution zones)
//! - Implement 3rd Density regions (polarized civilizations)
//! - Implement 4th Density regions (collective consciousness)
//! - Implement 5th Density regions (wisdom centers)
//! - Implement 6th Density regions (unity complexes)
//! - Implement 7th Density regions (gateways)
//! - Implement 8th Density regions (return points)
//!
//! From GAMING_ENGINE_ROADMAP_v2.md:
//! "World regions have density-specific properties"
//!
//! This module provides:
//! - DensityRegionGenerator: Generates regions for all 8 densities
//! - Unique properties for each density region
//! - Holographic continuity between density regions
//! - Transition zones between adjacent densities
//! - Density-specific entity spawning
//! - Density-specific environmental effects

use crate::holographic::field_address::HolographicAddress;
use crate::simulation_v3::density_mechanics::Density;
use crate::simulation_v3::multiscale_camera::ScaleLevel;
use crate::types::Float;
use std::collections::HashMap;
use std::time::Duration;

/// Maximum number of regions per density
pub const MAX_REGIONS_PER_DENSITY: usize = 64;

/// Minimum region size (in world units)
pub const MIN_REGION_SIZE: Float = 100.0;

/// Maximum region size
pub const MAX_REGION_SIZE: Float = 100000.0;

/// Default transition zone width between densities
pub const DEFAULT_TRANSITION_WIDTH: Float = 1000.0;

/// Region ID type
pub type RegionId = u64;

/// Density region configuration
#[derive(Debug, Clone, PartialEq)]
pub struct DensityRegionConfig {
    /// Primary density of this region
    pub density: Density,

    /// Holographic address for this region (Phase 1)
    /// Replaces bounded coordinates with infinite field addressing
    pub holographic_address: Option<HolographicAddress>,

    /// Region bounds [min_x, max_x, min_y, max_y, min_z, min_z]
    pub bounds: [Float; 6],

    /// Spectrum configuration override
    pub spectrum_modifiers: SpectrumModifiers,

    /// Environmental effects
    pub environmental_effects: EnvironmentalEffects,

    /// Entity spawning rules
    pub spawn_rules: SpawnRules,

    /// Transition zones to adjacent densities
    pub transition_zones: Vec<TransitionZone>,
}

impl DensityRegionConfig {
    pub fn new(density: Density, bounds: [Float; 6]) -> Self {
        DensityRegionConfig {
            density,
            bounds,
            spectrum_modifiers: SpectrumModifiers::default_for_density(density),
            environmental_effects: EnvironmentalEffects::default_for_density(density),
            spawn_rules: SpawnRules::default_for_density(density),
            transition_zones: Vec::new(),
            holographic_address: None,
        }
    }
}

/// Spectrum modifiers for a density region
#[derive(Debug, Clone, PartialEq)]
pub struct SpectrumModifiers {
    /// Space/Time multiplier
    pub space_time_multiplier: Float,

    /// Time/Space multiplier
    pub time_space_multiplier: Float,

    /// Quantum probability modifier
    pub quantum_probability: Float,

    /// Veil transparency modifier
    pub veil_transparency_modifier: Float,
}

impl SpectrumModifiers {
    pub fn default_for_density(density: Density) -> Self {
        match density {
            Density::First => SpectrumModifiers {
                space_time_multiplier: 1.0,
                time_space_multiplier: 0.1,
                quantum_probability: 0.05,
                veil_transparency_modifier: 0.0,
            },
            Density::Second => SpectrumModifiers {
                space_time_multiplier: 0.9,
                time_space_multiplier: 0.2,
                quantum_probability: 0.1,
                veil_transparency_modifier: 0.0,
            },
            Density::Third => SpectrumModifiers {
                space_time_multiplier: 0.8,
                time_space_multiplier: 0.3,
                quantum_probability: 0.15,
                veil_transparency_modifier: 0.1,
            },
            Density::Fourth => SpectrumModifiers {
                space_time_multiplier: 0.6,
                time_space_multiplier: 0.5,
                quantum_probability: 0.25,
                veil_transparency_modifier: 0.3,
            },
            Density::Fifth => SpectrumModifiers {
                space_time_multiplier: 0.4,
                time_space_multiplier: 0.7,
                quantum_probability: 0.4,
                veil_transparency_modifier: 0.6,
            },
            Density::Sixth => SpectrumModifiers {
                space_time_multiplier: 0.2,
                time_space_multiplier: 0.9,
                quantum_probability: 0.6,
                veil_transparency_modifier: 1.0,
            },
            Density::Seventh => SpectrumModifiers {
                space_time_multiplier: 0.1,
                time_space_multiplier: 1.0,
                quantum_probability: 0.8,
                veil_transparency_modifier: 1.0,
            },
            Density::Eighth => SpectrumModifiers {
                space_time_multiplier: 0.05,
                time_space_multiplier: 1.0,
                quantum_probability: 1.0,
                veil_transparency_modifier: 1.0,
            },
        }
    }
}

/// Environmental effects for a density region
#[derive(Debug, Clone, PartialEq)]
pub struct EnvironmentalEffects {
    /// Visual effects (fog, particles, etc.)
    pub visual_effects: Vec<VisualEffect>,

    /// Audio effects
    pub audio_effects: Vec<AudioEffect>,

    /// Physics modifiers
    pub physics_modifiers: PhysicsModifiers,

    /// Time dilation factor
    pub time_dilation: Float,

    /// Gravity modifier
    pub gravity_modifier: Float,
}

impl EnvironmentalEffects {
    pub fn default_for_density(density: Density) -> Self {
        let visual_effects = match density {
            Density::First => vec![VisualEffect::ParticleField, VisualEffect::AtomicGlow],
            Density::Second => vec![VisualEffect::OrganicGrowth, VisualEffect::CellularDivision],
            Density::Third => vec![VisualEffect::WeatherSystem, VisualEffect::DayNightCycle],
            Density::Fourth => vec![VisualEffect::CollectiveAura, VisualEffect::LoveRadiance],
            Density::Fifth => vec![VisualEffect::WisdomLight, VisualEffect::ClarityField],
            Density::Sixth => vec![VisualEffect::UnityField, VisualEffect::HarmonyRipple],
            Density::Seventh => vec![VisualEffect::GatewayPortal, VisualEffect::CosmicEnergy],
            Density::Eighth => vec![VisualEffect::InfiniteWhite, VisualEffect::Silence],
        };

        let audio_effects = match density {
            Density::First => vec![AudioEffect::AtomicVibration, AudioEffect::QuantumNoise],
            Density::Second => vec![AudioEffect::OrganicSounds, AudioEffect::Heartbeat],
            Density::Third => vec![AudioEffect::NatureAmbient, AudioEffect::CivilizationSounds],
            Density::Fourth => vec![AudioEffect::HarmonicResonance, AudioEffect::CollectiveVoice],
            Density::Fifth => vec![AudioEffect::WisdomTone, AudioEffect::ClarityChime],
            Density::Sixth => vec![AudioEffect::UnityChorus, AudioEffect::HarmonyOrchestra],
            Density::Seventh => vec![AudioEffect::GatewayHum, AudioEffect::CosmicWhisper],
            Density::Eighth => vec![AudioEffect::Silence, AudioEffect::InfiniteResonance],
        };

        let time_dilation = match density {
            Density::First => 0.1,
            Density::Second => 0.5,
            Density::Third => 1.0,
            Density::Fourth => 2.0,
            Density::Fifth => 5.0,
            Density::Sixth => 10.0,
            Density::Seventh => 100.0,
            Density::Eighth => Float::INFINITY,
        };

        let gravity_modifier = match density {
            Density::First => 10.0,
            Density::Second => 2.0,
            Density::Third => 1.0,
            Density::Fourth => 0.5,
            Density::Fifth => 0.2,
            Density::Sixth => 0.1,
            Density::Seventh => 0.01,
            Density::Eighth => 0.0,
        };

        EnvironmentalEffects {
            visual_effects,
            audio_effects,
            physics_modifiers: PhysicsModifiers::default_for_density(density),
            time_dilation,
            gravity_modifier,
        }
    }
}

/// Visual effect type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualEffect {
    ParticleField,
    AtomicGlow,
    OrganicGrowth,
    CellularDivision,
    WeatherSystem,
    DayNightCycle,
    CollectiveAura,
    LoveRadiance,
    WisdomLight,
    ClarityField,
    UnityField,
    HarmonyRipple,
    GatewayPortal,
    CosmicEnergy,
    InfiniteWhite,
    Silence,
}

/// Audio effect type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioEffect {
    AtomicVibration,
    QuantumNoise,
    OrganicSounds,
    Heartbeat,
    NatureAmbient,
    CivilizationSounds,
    HarmonicResonance,
    CollectiveVoice,
    WisdomTone,
    ClarityChime,
    UnityChorus,
    HarmonyOrchestra,
    GatewayHum,
    CosmicWhisper,
    Silence,
    InfiniteResonance,
}

/// Physics modifiers for a density region
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicsModifiers {
    /// Collision detection precision
    pub collision_precision: Float,

    /// Entanglement probability
    pub entanglement_probability: Float,

    /// Superposition duration
    pub superposition_duration: Duration,

    /// Non-local interaction range
    pub non_local_range: Float,
}

impl PhysicsModifiers {
    pub fn default_for_density(density: Density) -> Self {
        match density {
            Density::First => PhysicsModifiers {
                collision_precision: 1e-35,
                entanglement_probability: 0.95,
                superposition_duration: Duration::from_millis(10),
                non_local_range: Float::INFINITY,
            },
            Density::Second => PhysicsModifiers {
                collision_precision: 1e-12,
                entanglement_probability: 0.7,
                superposition_duration: Duration::from_millis(100),
                non_local_range: 1e-6,
            },
            Density::Third => PhysicsModifiers {
                collision_precision: 1e-5,
                entanglement_probability: 0.3,
                superposition_duration: Duration::from_secs(1),
                non_local_range: 100.0,
            },
            Density::Fourth => PhysicsModifiers {
                collision_precision: 1e-3,
                entanglement_probability: 0.5,
                superposition_duration: Duration::from_secs(10),
                non_local_range: 1000.0,
            },
            Density::Fifth => PhysicsModifiers {
                collision_precision: 1e-2,
                entanglement_probability: 0.7,
                superposition_duration: Duration::from_secs(60),
                non_local_range: 10000.0,
            },
            Density::Sixth => PhysicsModifiers {
                collision_precision: 0.1,
                entanglement_probability: 0.9,
                superposition_duration: Duration::from_secs(600),
                non_local_range: 100000.0,
            },
            Density::Seventh => PhysicsModifiers {
                collision_precision: 1.0,
                entanglement_probability: 1.0,
                superposition_duration: Duration::from_secs(3600),
                non_local_range: Float::INFINITY,
            },
            Density::Eighth => PhysicsModifiers {
                collision_precision: Float::INFINITY,
                entanglement_probability: 1.0,
                superposition_duration: Duration::MAX,
                non_local_range: Float::INFINITY,
            },
        }
    }
}

/// Entity spawning rules for a density region
#[derive(Debug, Clone, PartialEq)]
pub struct SpawnRules {
    /// Maximum entities allowed
    pub max_entities: usize,

    /// Spawn rate (entities per second)
    pub spawn_rate: Float,

    /// Required archetype affinity
    pub archetype_affinity: [Float; 22],

    /// Required polarity
    pub required_polarity: Option<Polarity>,

    /// Catalyst generation rate
    pub catalyst_rate: Float,
}

impl SpawnRules {
    pub fn default_for_density(density: Density) -> Self {
        let max_entities = match density {
            Density::First => 1000000,
            Density::Second => 100000,
            Density::Third => 10000,
            Density::Fourth => 1000,
            Density::Fifth => 100,
            Density::Sixth => 10,
            Density::Seventh => 1,
            Density::Eighth => 1,
        };

        let spawn_rate = match density {
            Density::First => 1000.0,
            Density::Second => 100.0,
            Density::Third => 10.0,
            Density::Fourth => 1.0,
            Density::Fifth => 0.1,
            Density::Sixth => 0.01,
            Density::Seventh => 0.001,
            Density::Eighth => 0.0,
        };

        let catalyst_rate = match density {
            Density::First => 0.01,
            Density::Second => 0.1,
            Density::Third => 1.0,
            Density::Fourth => 2.0,
            Density::Fifth => 3.0,
            Density::Sixth => 4.0,
            Density::Seventh => 5.0,
            Density::Eighth => 10.0,
        };

        SpawnRules {
            max_entities,
            spawn_rate,
            archetype_affinity: [0.5; 22],
            required_polarity: None,
            catalyst_rate,
        }
    }
}

/// Polarity type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity {
    ServiceToOthers,
    ServiceToSelf,
    Undecided,
}

/// Transition zone between density regions
#[derive(Debug, Clone, PartialEq)]
pub struct TransitionZone {
    /// From density
    pub from_density: Density,

    /// To density
    pub to_density: Density,

    /// Zone bounds
    pub bounds: [Float; 6],

    /// Width of transition zone
    pub width: Float,

    /// Interpolation mode
    pub interpolation_mode: TransitionInterpolationMode,

    /// Transition duration
    pub transition_duration: Duration,
}

/// Transition interpolation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionInterpolationMode {
    Linear,
    Smoothstep,
    EaseIn,
    EaseOut,
    EaseInOut,
    QuantumTunnel,
    DimensionalRift,
}

/// Density region generator
#[derive(Debug, Clone)]
pub struct DensityRegionGenerator {
    /// Region configurations by density
    pub regions: HashMap<Density, Vec<DensityRegionConfig>>,

    /// Next region ID
    pub next_region_id: RegionId,

    /// Scale integration
    pub scale_integration: ScaleIntegration,

    /// Statistics
    pub statistics: DensityRegionStatistics,
}

/// Scale integration for density regions
#[derive(Debug, Clone)]
pub struct ScaleIntegration {
    /// Which scales each density appears on
    pub density_scales: HashMap<Density, Vec<ScaleLevel>>,

    /// Scale-specific density multipliers
    pub scale_density_multipliers: HashMap<(Density, ScaleLevel), Float>,
}

impl ScaleIntegration {
    pub fn new() -> Self {
        let mut density_scales = HashMap::new();

        // Each density appears on all scales, but with different prominence
        for density in [
            Density::First,
            Density::Second,
            Density::Third,
            Density::Fourth,
            Density::Fifth,
            Density::Sixth,
            Density::Seventh,
            Density::Eighth,
        ] {
            density_scales.insert(
                density,
                vec![
                    ScaleLevel::Quantum,
                    ScaleLevel::Cellular,
                    ScaleLevel::Biological,
                    ScaleLevel::Planetary,
                    ScaleLevel::Stellar,
                    ScaleLevel::Galactic,
                    ScaleLevel::Cosmic,
                ],
            );
        }

        let mut scale_density_multipliers = HashMap::new();

        // Quantum scale favors 1st and 2nd density
        scale_density_multipliers.insert((Density::First, ScaleLevel::Quantum), 2.0);
        scale_density_multipliers.insert((Density::Second, ScaleLevel::Quantum), 1.5);

        // Biological scale favors 2nd and 3rd density
        scale_density_multipliers.insert((Density::Second, ScaleLevel::Biological), 2.0);
        scale_density_multipliers.insert((Density::Third, ScaleLevel::Biological), 1.5);

        // Planetary scale favors 3rd and 4th density
        scale_density_multipliers.insert((Density::Third, ScaleLevel::Planetary), 2.0);
        scale_density_multipliers.insert((Density::Fourth, ScaleLevel::Planetary), 1.5);

        // Cosmic scale favors 6th, 7th, and 8th density
        scale_density_multipliers.insert((Density::Sixth, ScaleLevel::Cosmic), 1.5);
        scale_density_multipliers.insert((Density::Seventh, ScaleLevel::Cosmic), 2.0);
        scale_density_multipliers.insert((Density::Eighth, ScaleLevel::Cosmic), 3.0);

        ScaleIntegration {
            density_scales,
            scale_density_multipliers,
        }
    }
}

impl Default for ScaleIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl DensityRegionGenerator {
    pub fn new() -> Self {
        DensityRegionGenerator {
            regions: HashMap::new(),
            next_region_id: 1,
            scale_integration: ScaleIntegration::new(),
            statistics: DensityRegionStatistics::new(),
        }
    }

    /// Generate a density region
    pub fn generate_region(
        &mut self,
        density: Density,
        bounds: [Float; 6],
    ) -> Result<DensityRegionConfig, DensityRegionError> {
        if self.regions.get(&density).map_or(0, |r| r.len()) >= MAX_REGIONS_PER_DENSITY {
            return Err(DensityRegionError::RegionCapacityExceeded {
                density,
                current: self.regions.get(&density).map_or(0, |r| r.len()),
                max: MAX_REGIONS_PER_DENSITY,
            });
        }

        let region_size =
            (bounds[1] - bounds[0]) * (bounds[3] - bounds[2]) * (bounds[5] - bounds[4]);

        // From COSMOLOGICAL-ARCHITECTURE.md: Each density has unique characteristics and limitations
        // Lower densities have tighter region size limits, following the pattern of max_entities
        let max_size = match density {
            Density::First => 1_000_000_000_000.0, // 1e12 - matter formation has tightest limits
            Density::Second => 10_000_000_000_000.0, // 1e13 - evolution zones
            Density::Third => 100_000_000_000_000.0, // 1e14 - polarized civilizations
            Density::Fourth => 1_000_000_000_000_000.0, // 1e15 - collective consciousness
            Density::Fifth => 10_000_000_000_000_000.0, // 1e16 - wisdom centers
            Density::Sixth => 100_000_000_000_000_000.0, // 1e17 - unity complexes
            Density::Seventh => 1_000_000_000_000_000_000.0, // 1e18 - gateways
            Density::Eighth => 10_000_000_000_000_000_000.0, // 1e19 - return points
        };

        if region_size < MIN_REGION_SIZE.powi(3) {
            return Err(DensityRegionError::RegionTooSmall {
                density,
                size: region_size,
                min_size: MIN_REGION_SIZE.powi(3),
            });
        }

        if region_size >= max_size {
            return Err(DensityRegionError::RegionTooLarge {
                density,
                size: region_size,
                max_size,
            });
        }

        let config = DensityRegionConfig::new(density, bounds);

        self.regions
            .entry(density)
            .or_default()
            .push(config.clone());

        self.statistics.regions_generated += 1;
        self.statistics.total_regions = self.regions.values().map(|v| v.len()).sum();

        Ok(config)
    }

    /// Generate transition zones between density regions
    pub fn generate_transition_zones(
        &mut self,
        density_a: Density,
        density_b: Density,
        width: Float,
    ) -> Result<TransitionZone, DensityRegionError> {
        if density_a == density_b {
            return Err(DensityRegionError::SameDensityTransition { density: density_a });
        }

        let regions_a = self
            .regions
            .get(&density_a)
            .ok_or(DensityRegionError::DensityRegionsNotFound { density: density_a })?;

        let regions_b = self
            .regions
            .get(&density_b)
            .ok_or(DensityRegionError::DensityRegionsNotFound { density: density_b })?;

        let region_a = regions_a
            .first()
            .ok_or(DensityRegionError::DensityRegionsNotFound { density: density_a })?;

        let region_b = regions_b
            .first()
            .ok_or(DensityRegionError::DensityRegionsNotFound { density: density_b })?;

        let bounds_a = region_a.bounds;
        let bounds_b = region_b.bounds;

        let transition_bounds = [
            bounds_a[1].min(bounds_b[0]),
            bounds_a[1].max(bounds_b[0]) + width,
            bounds_a[2].min(bounds_b[2]),
            bounds_a[3].max(bounds_b[3]),
            bounds_a[4].min(bounds_b[4]),
            bounds_a[5].max(bounds_b[5]),
        ];

        let interpolation_mode = match (density_a, density_b) {
            (Density::First, _) | (_, Density::First) => TransitionInterpolationMode::QuantumTunnel,
            (Density::Eighth, _) | (_, Density::Eighth) => {
                TransitionInterpolationMode::DimensionalRift
            }
            (Density::Fourth, _) | (_, Density::Fourth) => TransitionInterpolationMode::EaseInOut,
            (Density::Sixth, _) | (_, Density::Sixth) => TransitionInterpolationMode::Smoothstep,
            _ => TransitionInterpolationMode::Linear,
        };

        let transition_duration = match (density_a, density_b) {
            (Density::First, Density::Eighth) | (Density::Eighth, Density::First) => {
                Duration::from_secs(60)
            }
            (Density::Third, Density::Fourth) | (Density::Fourth, Density::Third) => {
                Duration::from_secs(10)
            }
            _ => Duration::from_secs(30),
        };

        let transition = TransitionZone {
            from_density: density_a,
            to_density: density_b,
            bounds: transition_bounds,
            width,
            interpolation_mode,
            transition_duration,
        };

        self.statistics.transition_zones_generated += 1;

        Ok(transition)
    }

    /// Get regions for a specific density
    pub fn get_regions(&self, density: Density) -> &[DensityRegionConfig] {
        self.regions
            .get(&density)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get all regions
    pub fn get_all_regions(&self) -> Vec<(Density, &DensityRegionConfig)> {
        let mut all = Vec::new();
        for (density, regions) in &self.regions {
            for region in regions {
                all.push((*density, region));
            }
        }
        all
    }

    /// Get density at a specific point
    pub fn get_density_at_point(&self, point: [Float; 3]) -> Option<Density> {
        for (density, regions) in &self.regions {
            for region in regions {
                if region.bounds[0] <= point[0]
                    && point[0] <= region.bounds[1]
                    && region.bounds[2] <= point[1]
                    && point[1] <= region.bounds[3]
                    && region.bounds[4] <= point[2]
                    && point[2] <= region.bounds[5]
                {
                    return Some(*density);
                }
            }
        }
        None
    }

    /// Get density transition at a specific point
    pub fn get_transition_at_point(&self, point: [Float; 3]) -> Option<(TransitionZone, Float)> {
        for regions in self.regions.values() {
            for region in regions {
                for transition in &region.transition_zones {
                    if transition.bounds[0] <= point[0]
                        && point[0] <= transition.bounds[1]
                        && transition.bounds[2] <= point[1]
                        && point[1] <= transition.bounds[3]
                        && transition.bounds[4] <= point[2]
                        && point[2] <= transition.bounds[5]
                    {
                        let progress = (point[0] - transition.bounds[0]) / transition.width;
                        return Some((transition.clone(), progress));
                    }
                }
            }
        }
        None
    }

    /// Clear all regions
    pub fn clear(&mut self) {
        self.regions.clear();
        self.next_region_id = 1;
        self.statistics = DensityRegionStatistics::new();
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &DensityRegionStatistics {
        &self.statistics
    }
}

impl Default for DensityRegionGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Density region statistics
#[derive(Debug, Clone)]
pub struct DensityRegionStatistics {
    pub regions_generated: usize,
    pub total_regions: usize,
    pub transition_zones_generated: usize,
    pub total_area: Float,
    pub average_region_size: Float,
}

impl Default for DensityRegionStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl DensityRegionStatistics {
    pub fn new() -> Self {
        DensityRegionStatistics {
            regions_generated: 0,
            total_regions: 0,
            transition_zones_generated: 0,
            total_area: 0.0,
            average_region_size: 0.0,
        }
    }
}

/// Density region error
#[derive(Debug, Clone, PartialEq)]
pub enum DensityRegionError {
    RegionCapacityExceeded {
        density: Density,
        current: usize,
        max: usize,
    },
    RegionTooSmall {
        density: Density,
        size: Float,
        min_size: Float,
    },
    RegionTooLarge {
        density: Density,
        size: Float,
        max_size: Float,
    },
    DensityRegionsNotFound {
        density: Density,
    },
    SameDensityTransition {
        density: Density,
    },
    InvalidBounds {
        bounds: [Float; 6],
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_region_generator_new() {
        let generator = DensityRegionGenerator::new();
        assert_eq!(generator.regions.len(), 0);
        assert_eq!(generator.next_region_id, 1);
    }

    #[test]
    fn test_generate_first_density_region() {
        let mut generator = DensityRegionGenerator::new();
        let bounds = [0.0, 1000.0, 0.0, 1000.0, 0.0, 1000.0];

        let region = generator
            .generate_region(Density::First, bounds)
            .expect("Failed to generate region");

        assert_eq!(region.density, Density::First);
        assert_eq!(region.bounds, bounds);
        assert_eq!(generator.regions.get(&Density::First).unwrap().len(), 1);
    }

    #[test]
    fn test_generate_all_density_regions() {
        let mut generator = DensityRegionGenerator::new();
        let bounds = [0.0, 1000.0, 0.0, 1000.0, 0.0, 1000.0];

        for density in [
            Density::First,
            Density::Second,
            Density::Third,
            Density::Fourth,
            Density::Fifth,
            Density::Sixth,
            Density::Seventh,
            Density::Eighth,
        ] {
            generator
                .generate_region(density, bounds)
                .unwrap_or_else(|_| panic!("Failed to generate {:?} region", density));
        }

        assert_eq!(generator.regions.len(), 8);
        assert_eq!(generator.statistics.regions_generated, 8);
    }

    #[test]
    fn test_generate_region_too_small() {
        let mut generator = DensityRegionGenerator::new();
        let bounds = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0];

        let result = generator.generate_region(Density::First, bounds);

        assert_eq!(
            result,
            Err(DensityRegionError::RegionTooSmall {
                density: Density::First,
                size: 1.0,
                min_size: 1000000.0
            })
        );
    }

    #[test]
    fn test_generate_region_too_large() {
        let mut generator = DensityRegionGenerator::new();
        let bounds = [0.0, 10000.0, 0.0, 10000.0, 0.0, 10000.0];

        let result = generator.generate_region(Density::First, bounds);

        assert_eq!(
            result,
            Err(DensityRegionError::RegionTooLarge {
                density: Density::First,
                size: 1e12,
                max_size: 1_000_000_000_000.0 // First density has tighter limits (1e12)
            })
        );
    }

    #[test]
    fn test_generate_transition_zone() {
        let mut generator = DensityRegionGenerator::new();
        let bounds_a = [0.0, 1000.0, 0.0, 1000.0, 0.0, 1000.0];
        let bounds_b = [2000.0, 3000.0, 0.0, 1000.0, 0.0, 1000.0];

        generator
            .generate_region(Density::Third, bounds_a)
            .expect("Failed to generate 3rd density region");
        generator
            .generate_region(Density::Fourth, bounds_b)
            .expect("Failed to generate 4th density region");

        let transition = generator
            .generate_transition_zones(Density::Third, Density::Fourth, 500.0)
            .expect("Failed to generate transition zone");

        assert_eq!(transition.from_density, Density::Third);
        assert_eq!(transition.to_density, Density::Fourth);
        assert_eq!(transition.width, 500.0);
    }

    #[test]
    fn test_generate_transition_same_density() {
        let mut generator = DensityRegionGenerator::new();

        let result = generator.generate_transition_zones(Density::Third, Density::Third, 500.0);

        assert_eq!(
            result,
            Err(DensityRegionError::SameDensityTransition {
                density: Density::Third
            })
        );
    }

    #[test]
    fn test_get_density_at_point() {
        let mut generator = DensityRegionGenerator::new();
        let bounds = [0.0, 1000.0, 0.0, 1000.0, 0.0, 1000.0];

        generator
            .generate_region(Density::Third, bounds)
            .expect("Failed to generate region");

        let point_inside = [500.0, 500.0, 500.0];
        let point_outside = [2000.0, 2000.0, 2000.0];

        assert_eq!(
            generator.get_density_at_point(point_inside),
            Some(Density::Third)
        );
        assert_eq!(generator.get_density_at_point(point_outside), None);
    }

    #[test]
    fn test_spectrum_modifiers_default() {
        let first_modifiers = SpectrumModifiers::default_for_density(Density::First);
        assert_eq!(first_modifiers.space_time_multiplier, 1.0);
        assert_eq!(first_modifiers.time_space_multiplier, 0.1);
        assert_eq!(first_modifiers.quantum_probability, 0.05);
        assert_eq!(first_modifiers.veil_transparency_modifier, 0.0);

        let eighth_modifiers = SpectrumModifiers::default_for_density(Density::Eighth);
        assert_eq!(eighth_modifiers.space_time_multiplier, 0.05);
        assert_eq!(eighth_modifiers.time_space_multiplier, 1.0);
        assert_eq!(eighth_modifiers.quantum_probability, 1.0);
        assert_eq!(eighth_modifiers.veil_transparency_modifier, 1.0);
    }

    #[test]
    fn test_environmental_effects_default() {
        let first_effects = EnvironmentalEffects::default_for_density(Density::First);
        assert!(first_effects
            .visual_effects
            .contains(&VisualEffect::ParticleField));
        assert!(first_effects
            .visual_effects
            .contains(&VisualEffect::AtomicGlow));

        let fourth_effects = EnvironmentalEffects::default_for_density(Density::Fourth);
        assert!(fourth_effects
            .visual_effects
            .contains(&VisualEffect::CollectiveAura));
        assert!(fourth_effects
            .visual_effects
            .contains(&VisualEffect::LoveRadiance));
    }

    #[test]
    fn test_spawn_rules_default() {
        let first_rules = SpawnRules::default_for_density(Density::First);
        assert_eq!(first_rules.max_entities, 1000000);
        assert_eq!(first_rules.spawn_rate, 1000.0);
        assert_eq!(first_rules.catalyst_rate, 0.01);

        let eighth_rules = SpawnRules::default_for_density(Density::Eighth);
        assert_eq!(eighth_rules.max_entities, 1);
        assert_eq!(eighth_rules.spawn_rate, 0.0);
        assert_eq!(eighth_rules.catalyst_rate, 10.0);
    }

    #[test]
    fn test_scale_integration() {
        let integration = ScaleIntegration::new();

        assert!(integration
            .density_scales
            .get(&Density::First)
            .unwrap()
            .contains(&ScaleLevel::Quantum));

        assert!(integration
            .density_scales
            .get(&Density::Eighth)
            .unwrap()
            .contains(&ScaleLevel::Cosmic));
    }

    #[test]
    fn test_clear_regions() {
        let mut generator = DensityRegionGenerator::new();
        let bounds = [0.0, 1000.0, 0.0, 1000.0, 0.0, 1000.0];

        generator
            .generate_region(Density::Third, bounds)
            .expect("Failed to generate region");

        assert_eq!(generator.regions.len(), 1);
        assert_eq!(generator.statistics.regions_generated, 1);

        generator.clear();

        assert_eq!(generator.regions.len(), 0);
        assert_eq!(generator.statistics.regions_generated, 0);
    }

    #[test]
    fn test_region_capacity_exceeded() {
        let mut generator = DensityRegionGenerator::new();
        let bounds = [0.0, 1000.0, 0.0, 1000.0, 0.0, 1000.0];

        for _ in 0..MAX_REGIONS_PER_DENSITY {
            generator
                .generate_region(Density::Third, bounds)
                .expect("Failed to generate region");
        }

        let result = generator.generate_region(Density::Third, bounds);

        assert_eq!(
            result,
            Err(DensityRegionError::RegionCapacityExceeded {
                density: Density::Third,
                current: MAX_REGIONS_PER_DENSITY,
                max: MAX_REGIONS_PER_DENSITY
            })
        );
    }
}
