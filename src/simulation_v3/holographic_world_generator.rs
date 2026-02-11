//! Holographic World Generator
//!
//! Implements world generation from holographic unfolding, not procedural algorithms.
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 7: True Emergence Mechanics
//! > "World emerges from holographic unfolding, not procedural algorithms"
//! > "Holographic field unfolding algorithm"
//! > "Spectrum configuration for world generation"
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 3.1:
//! > "UniversalTemplate<T> - Build once, instantiate many"
//! > "Every element follows the same holographic template"
//!
//! From MASTER_R&D_ROADMAP.md Phase 5:
//! > "Goal: World emerges from holographic unfolding, not procedural algorithms"
//! > "Deliverables: Holographic field unfolding algorithm, Spectrum configuration,
//! > Archetypical pattern emergence, Multi-scale world, Density-specific regions"
//!
//! Key Concepts:
//! - **Holographic Blueprint**: The pre-existing whole from which the world unfolds
//! - **Unfolding Process**: Not creation, but revelation of what already exists
//! - **Spectrum Configuration**: Determines world properties (Space/Time vs Time/Space)
//! - **Observer Effect**: Player observation affects world collapse
//! - **Multi-Scale World**: World exists simultaneously at all scales
//! - **Density Regions**: Different world regions have density-specific properties
//! - **Dynamic Evolution**: World changes based on spectrum shifts and memory

use crate::simulation_v3::{
    density_mechanics::Density,
    holographic_memory::{HolographicMemorySystem, HolographicSignature, SoulStreamId},
    holographic_physics::{HolographicPhysicsMode, SpectrumRatio},
    multiscale_camera::ScaleLevel,
    observer_effect::{ObservationEngine, ObserverKey},
};
use crate::types::Float;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

pub type WorldId = u64;
pub type RegionId = u64;

pub const MAX_WORLD_ENTITIES: usize = 1_000_000;
pub const MAX_REGIONS_PER_WORLD: usize = 100;
pub const WORLD_UNFOLDING_DURATION: Duration = Duration::from_secs(60);
pub const DENSITY_REGION_TRANSITION_RATE: Float = 0.001;

pub fn create_observer_key(observer_id: u64, target_id: u64) -> ObserverKey {
    ObserverKey::simple(observer_id, target_id)
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorldGeneratorError {
    InvalidBlueprint,
    InvalidSpectrumConfiguration,
    ObserverNotFound(ObserverKey),
    WorldCapacityExceeded { requested: usize, capacity: usize },
    RegionCapacityExceeded { requested: usize, capacity: usize },
    UnfoldingFailed(String),
    MemoryNotAvailable(SoulStreamId),
}

#[derive(Debug, Clone)]
pub struct HolographicBlueprint {
    pub blueprint_id: WorldId,
    pub blueprint_signature: HolographicSignature,
    pub base_archetypical_pattern: [Float; 22],
    pub base_spectrum_configuration: SpectrumRatio,
    pub world_seed: u64,
    pub holographic_completeness: Float,
}

impl HolographicBlueprint {
    pub fn new(blueprint_id: WorldId, world_seed: u64) -> Self {
        HolographicBlueprint {
            blueprint_id,
            blueprint_signature: HolographicSignature::new(),
            base_archetypical_pattern: [0.0; 22],
            base_spectrum_configuration: SpectrumRatio::new(1.0, 1.0),
            world_seed,
            holographic_completeness: 1.0,
        }
    }

    pub fn compute_resonance_at_point(&self, position: [Float; 3]) -> Float {
        let distance_from_center =
            (position[0].powi(2) + position[1].powi(2) + position[2].powi(2)).sqrt();
        let resonance =
            (self.holographic_completeness / (1.0 + distance_from_center * 0.01)).max(0.1);
        resonance
    }

    pub fn compute_interference_at_point(&self, position: [Float; 3]) -> Float {
        let x = position[0];
        let y = position[1];
        let z = position[2];

        let mut interference = 0.0;
        for i in 0..22 {
            let angle = 2.0 * std::f64::consts::PI * i as Float / 22.0;
            interference += self.base_archetypical_pattern[i]
                * (x * angle.cos() + y * angle.sin() + z * angle).sin();
        }

        interference / 22.0
    }

    pub fn compute_density_affinity(&self, density: Density) -> Float {
        let spectrum_pos = self.base_spectrum_configuration.spectrum_position;

        match density {
            Density::First => (1.0 - spectrum_pos).powi(2),
            Density::Second => (1.0 - spectrum_pos).abs(),
            Density::Third => 1.0 - (spectrum_pos - 0.5).abs() * 2.0,
            Density::Fourth => (spectrum_pos - 0.5).abs() * 2.0,
            Density::Fifth => spectrum_pos.abs(),
            Density::Sixth => spectrum_pos.powi(2),
            Density::Seventh => spectrum_pos.powi(3),
            Density::Eighth => spectrum_pos.powi(4),
        }
        .min(1.0)
        .max(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct SpectrumConfigurationOverride {
    pub space_time_multiplier: Float,
    pub time_space_multiplier: Float,
    pub quantum_probability: Float,
}

impl Default for SpectrumConfigurationOverride {
    fn default() -> Self {
        SpectrumConfigurationOverride {
            space_time_multiplier: 1.0,
            time_space_multiplier: 1.0,
            quantum_probability: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorldRegion {
    pub region_id: RegionId,
    pub region_type: DensityRegionType,
    pub bounds: [Float; 6],
    pub spectrum_configuration: SpectrumRatio,
    pub density_affinity: Density,
    pub holographic_signature: HolographicSignature,
    pub region_memory: Vec<HolographicSignature>,
    pub observer_count: usize,
}

impl WorldRegion {
    pub fn new(region_id: RegionId, region_type: DensityRegionType, bounds: [Float; 6]) -> Self {
        WorldRegion {
            region_id,
            region_type,
            bounds,
            spectrum_configuration: SpectrumRatio::new(1.0, 1.0),
            density_affinity: Density::Third,
            holographic_signature: HolographicSignature::new(),
            region_memory: Vec::new(),
            observer_count: 0,
        }
    }

    pub fn contains_point(&self, point: [Float; 3]) -> bool {
        point[0] >= self.bounds[0]
            && point[0] <= self.bounds[1]
            && point[1] >= self.bounds[2]
            && point[1] <= self.bounds[3]
            && point[2] >= self.bounds[4]
            && point[2] <= self.bounds[5]
    }

    pub fn compute_region_physics_mode(&self) -> HolographicPhysicsMode {
        if self.spectrum_configuration.is_space_time_dominant() {
            HolographicPhysicsMode::SpaceTime
        } else if self.spectrum_configuration.is_time_space_dominant() {
            HolographicPhysicsMode::TimeSpace
        } else {
            HolographicPhysicsMode::Quantum
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DensityRegionType {
    FirstDensityMatter,
    SecondDensityEvolution,
    ThirdDensityVeiled,
    FourthDensityLove,
    FifthDensityWisdom,
    SixthDensityUnity,
    SeventhDensityGateway,
    EighthDensityInfinity,
    TransitionZone,
}

impl DensityRegionType {
    pub fn from_density(density: Density) -> Self {
        match density {
            Density::First => DensityRegionType::FirstDensityMatter,
            Density::Second => DensityRegionType::SecondDensityEvolution,
            Density::Third => DensityRegionType::ThirdDensityVeiled,
            Density::Fourth => DensityRegionType::FourthDensityLove,
            Density::Fifth => DensityRegionType::FifthDensityWisdom,
            Density::Sixth => DensityRegionType::SixthDensityUnity,
            Density::Seventh => DensityRegionType::SeventhDensityGateway,
            Density::Eighth => DensityRegionType::EighthDensityInfinity,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScaleWorld {
    pub scale_level: ScaleLevel,
    pub entities: Vec<u64>,
    pub world_signature: HolographicSignature,
    pub scale_physics_mode: HolographicPhysicsMode,
    pub resolution: Float,
}

impl ScaleWorld {
    pub fn new(scale_level: ScaleLevel) -> Self {
        let resolution = match scale_level {
            ScaleLevel::Quantum => 1e-35,
            ScaleLevel::Cellular => 1e-12,
            ScaleLevel::Biological => 1e-5,
            ScaleLevel::Planetary => 1e0,
            ScaleLevel::Stellar => 1e8,
            ScaleLevel::Galactic => 1e14,
            ScaleLevel::Cosmic => 1e22,
        };

        ScaleWorld {
            scale_level,
            entities: Vec::new(),
            world_signature: HolographicSignature::new(),
            scale_physics_mode: HolographicPhysicsMode::SpaceTime,
            resolution,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HolographicWorld {
    pub world_id: WorldId,
    pub blueprint: HolographicBlueprint,
    pub spectrum_configuration: SpectrumRatio,
    pub regions: HashMap<RegionId, WorldRegion>,
    pub scale_worlds: HashMap<ScaleLevel, ScaleWorld>,
    pub world_memory: HolographicMemorySystem,
    pub observer_keys: Vec<ObserverKey>,
    pub world_age: Duration,
    pub evolution_stage: Float,
    pub holographic_coherence: Float,
}

impl HolographicWorld {
    pub fn new(world_id: WorldId, blueprint: HolographicBlueprint) -> Self {
        HolographicWorld {
            world_id,
            spectrum_configuration: blueprint.base_spectrum_configuration.clone(),
            regions: HashMap::new(),
            scale_worlds: HashMap::new(),
            world_memory: HolographicMemorySystem::new(),
            observer_keys: Vec::new(),
            world_age: Duration::ZERO,
            evolution_stage: 0.0,
            holographic_coherence: 1.0,
            blueprint,
        }
    }

    pub fn add_region(&mut self, region: WorldRegion) -> Result<(), WorldGeneratorError> {
        if self.regions.len() >= MAX_REGIONS_PER_WORLD {
            return Err(WorldGeneratorError::RegionCapacityExceeded {
                requested: self.regions.len() + 1,
                capacity: MAX_REGIONS_PER_WORLD,
            });
        }
        self.regions.insert(region.region_id, region);
        Ok(())
    }

    pub fn add_scale_world(&mut self, scale_world: ScaleWorld) {
        self.scale_worlds
            .insert(scale_world.scale_level, scale_world);
    }

    pub fn entity_count(&self) -> usize {
        self.scale_worlds.values().map(|sw| sw.entities.len()).sum()
    }

    pub fn evolve(&mut self, delta_time: Duration) {
        self.world_age += delta_time;
        self.evolution_stage =
            (self.world_age.as_secs_f64() / WORLD_UNFOLDING_DURATION.as_secs_f64()).min(1.0);

        let delta = delta_time.as_secs_f64() * DENSITY_REGION_TRANSITION_RATE;
        for region in self.regions.values_mut() {
            for i in 0..22 {
                region.holographic_signature.interference_pattern[i] +=
                    delta * (rand::random::<Float>() - 0.5);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnfoldingProgress {
    pub total_stages: u32,
    pub current_stage: u32,
    pub stage_name: String,
    pub progress_percentage: Float,
    pub estimated_remaining_time: Duration,
}

#[derive(Debug, Clone)]
pub struct WorldGeneratorStatistics {
    pub worlds_generated: u32,
    pub total_regions_created: u32,
    pub total_entities_generated: u32,
    pub average_unfolding_time: Duration,
    pub memory_usage_bytes: usize,
}

impl Default for WorldGeneratorStatistics {
    fn default() -> Self {
        WorldGeneratorStatistics {
            worlds_generated: 0,
            total_regions_created: 0,
            total_entities_generated: 0,
            average_unfolding_time: Duration::ZERO,
            memory_usage_bytes: 0,
        }
    }
}

pub struct HolographicWorldGenerator {
    pub observation_engine: ObservationEngine,
    pub statistics: WorldGeneratorStatistics,
}

impl HolographicWorldGenerator {
    pub fn new() -> Self {
        HolographicWorldGenerator {
            observation_engine: ObservationEngine::new(1000),
            statistics: WorldGeneratorStatistics::default(),
        }
    }

    pub fn unfold(
        &mut self,
        blueprint: HolographicBlueprint,
        observer: Option<ObserverKey>,
    ) -> Result<HolographicWorld, WorldGeneratorError> {
        let start_time = SystemTime::now();

        let mut world = HolographicWorld::new(blueprint.blueprint_id, blueprint.clone());

        if let Some(obs) = observer {
            world.observer_keys.push(obs);
        }

        self.apply_spectrum_configuration(&mut world, &blueprint)?;
        self.apply_observer_collapse(&mut world, observer)?;
        self.unfold_all_scales(&mut world, &blueprint)?;
        self.unfold_density_regions(&mut world, &blueprint)?;

        let unfolding_time = start_time.elapsed().unwrap_or(Duration::ZERO);
        self.update_statistics(unfolding_time);

        Ok(world)
    }

    fn apply_spectrum_configuration(
        &mut self,
        world: &mut HolographicWorld,
        blueprint: &HolographicBlueprint,
    ) -> Result<(), WorldGeneratorError> {
        world.spectrum_configuration = blueprint.base_spectrum_configuration.clone();
        Ok(())
    }

    fn apply_observer_collapse(
        &mut self,
        world: &mut HolographicWorld,
        observer: Option<ObserverKey>,
    ) -> Result<(), WorldGeneratorError> {
        if let Some(_obs_key) = observer {
            let mut collapsed_signature = HolographicSignature::new();

            collapsed_signature.interference_pattern[0] = 0.8;
            collapsed_signature.coherence_level = 0.9;
            collapsed_signature.resonance_frequency = 440.0;

            for region in world.regions.values_mut() {
                region.holographic_signature = collapsed_signature.clone();
                region.observer_count += 1;
            }
        }

        Ok(())
    }

    fn unfold_all_scales(
        &mut self,
        world: &mut HolographicWorld,
        blueprint: &HolographicBlueprint,
    ) -> Result<(), WorldGeneratorError> {
        let scales = [
            ScaleLevel::Quantum,
            ScaleLevel::Cellular,
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            ScaleLevel::Stellar,
            ScaleLevel::Galactic,
            ScaleLevel::Cosmic,
        ];

        for scale in scales {
            let mut scale_world = ScaleWorld::new(scale);
            scale_world.world_signature = blueprint.blueprint_signature.clone();

            scale_world.scale_physics_mode = if blueprint
                .base_spectrum_configuration
                .is_space_time_dominant()
            {
                HolographicPhysicsMode::SpaceTime
            } else if blueprint
                .base_spectrum_configuration
                .is_time_space_dominant()
            {
                HolographicPhysicsMode::TimeSpace
            } else {
                HolographicPhysicsMode::Quantum
            };

            world.add_scale_world(scale_world);
        }

        Ok(())
    }

    fn unfold_density_regions(
        &mut self,
        world: &mut HolographicWorld,
        blueprint: &HolographicBlueprint,
    ) -> Result<(), WorldGeneratorError> {
        let densities = [
            Density::First,
            Density::Second,
            Density::Third,
            Density::Fourth,
            Density::Fifth,
            Density::Sixth,
            Density::Seventh,
            Density::Eighth,
        ];

        for (i, density) in densities.iter().enumerate() {
            let region_id = (world.world_id * 10 + i as u64) as RegionId;

            let bounds = [
                (i as Float) * 10.0,
                (i as Float + 1.0) * 10.0,
                0.0,
                10.0,
                0.0,
                10.0,
            ];

            let mut region =
                WorldRegion::new(region_id, DensityRegionType::from_density(*density), bounds);
            region.density_affinity = *density;
            region.spectrum_configuration = blueprint.base_spectrum_configuration.clone();
            region.holographic_signature = blueprint.blueprint_signature.clone();

            world.add_region(region)?;
        }

        Ok(())
    }

    pub fn regenerate_world(
        &mut self,
        world: &mut HolographicWorld,
    ) -> Result<(), WorldGeneratorError> {
        let blueprint = world.blueprint.clone();

        let observer = if !world.observer_keys.is_empty() {
            Some(world.observer_keys[0])
        } else {
            None
        };

        *world = self.unfold(blueprint, observer)?;

        Ok(())
    }

    pub fn compute_holographic_continuity(
        &self,
        world_a: &HolographicWorld,
        world_b: &HolographicWorld,
    ) -> Float {
        let blueprint_continuity = world_a
            .blueprint
            .blueprint_signature
            .compute_similarity(&world_b.blueprint.blueprint_signature);

        let spectrum_continuity = 1.0
            - (world_a.spectrum_configuration.space_time_ratio
                - world_b.spectrum_configuration.space_time_ratio)
                .abs()
                * 0.5
            - (world_a.spectrum_configuration.time_space_ratio
                - world_b.spectrum_configuration.time_space_ratio)
                .abs()
                * 0.5;

        blueprint_continuity * 0.7 + spectrum_continuity * 0.3
    }

    fn update_statistics(&mut self, unfolding_time: Duration) {
        self.statistics.worlds_generated += 1;
        self.statistics.total_regions_created += 8;

        let total_time = self.statistics.average_unfolding_time.as_nanos() as u128
            + unfolding_time.as_nanos() as u128;
        self.statistics.average_unfolding_time =
            Duration::from_nanos((total_time / self.statistics.worlds_generated as u128) as u64);
    }

    pub fn get_statistics(&self) -> WorldGeneratorStatistics {
        self.statistics.clone()
    }

    pub fn estimate_unfolding_progress(
        &self,
        current_stage: u32,
        total_stages: u32,
    ) -> UnfoldingProgress {
        let progress = current_stage as Float / total_stages as Float * 100.0;
        let stage_names = vec![
            "Blueprint Initialization",
            "Spectrum Configuration",
            "Observer Collapse",
            "Scale Unfolding",
            "Density Region Creation",
            "Entity Generation",
            "Memory Integration",
            "Finalization",
        ];

        let stage_name = stage_names
            .get(current_stage as usize)
            .unwrap_or(&"Unknown Stage")
            .to_string();

        let avg_time = self.statistics.average_unfolding_time.as_secs_f64();
        let remaining_stages = (total_stages - current_stage) as Float;
        let estimated_remaining =
            Duration::from_secs_f64(avg_time * remaining_stages / total_stages as Float);

        UnfoldingProgress {
            total_stages,
            current_stage,
            stage_name,
            progress_percentage: progress,
            estimated_remaining_time: estimated_remaining,
        }
    }
}

impl Default for HolographicWorldGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holographic_blueprint_creation() {
        let blueprint = HolographicBlueprint::new(1, 42);
        assert_eq!(blueprint.blueprint_id, 1);
        assert_eq!(blueprint.world_seed, 42);
        assert_eq!(blueprint.holographic_completeness, 1.0);
    }

    #[test]
    fn test_blueprint_resonance_computation() {
        let blueprint = HolographicBlueprint::new(1, 42);
        let resonance_center = blueprint.compute_resonance_at_point([0.0, 0.0, 0.0]);
        let resonance_far = blueprint.compute_resonance_at_point([100.0, 100.0, 100.0]);

        assert!(resonance_center > resonance_far);
        assert!(resonance_center > 0.0);
        assert!(resonance_far > 0.0);
    }

    #[test]
    fn test_blueprint_interference_computation() {
        let blueprint = HolographicBlueprint::new(1, 42);
        blueprint.base_archetypical_pattern[0] = 1.0;

        let interference = blueprint.compute_interference_at_point([1.0, 0.0, 0.0]);
        assert!(interference >= -1.0 && interference <= 1.0);
    }

    #[test]
    fn test_density_affinity_computation() {
        let mut blueprint = HolographicBlueprint::new(1, 42);
        blueprint.base_spectrum_configuration = SpectrumRatio::new(2.0, 1.0);

        let first_density_affinity = blueprint.compute_density_affinity(Density::First);
        let eighth_density_affinity = blueprint.compute_density_affinity(Density::Eighth);

        assert!(first_density_affinity < eighth_density_affinity);
    }

    #[test]
    fn test_world_region_creation() {
        let region = WorldRegion::new(
            1,
            DensityRegionType::ThirdDensityVeiled,
            [0.0, 10.0, 0.0, 10.0, 0.0, 10.0],
        );
        assert_eq!(region.region_id, 1);
        assert_eq!(region.region_type, DensityRegionType::ThirdDensityVeiled);
        assert_eq!(region.observer_count, 0);
    }

    #[test]
    fn test_region_contains_point() {
        let region = WorldRegion::new(
            1,
            DensityRegionType::ThirdDensityVeiled,
            [0.0, 10.0, 0.0, 10.0, 0.0, 10.0],
        );

        assert!(region.contains_point([5.0, 5.0, 5.0]));
        assert!(!region.contains_point([15.0, 5.0, 5.0]));
        assert!(!region.contains_point([5.0, 15.0, 5.0]));
        assert!(!region.contains_point([5.0, 5.0, 15.0]));
    }

    #[test]
    fn test_region_physics_mode() {
        let mut region = WorldRegion::new(
            1,
            DensityRegionType::ThirdDensityVeiled,
            [0.0, 10.0, 0.0, 10.0, 0.0, 10.0],
        );

        region.spectrum_configuration = SpectrumRatio::new(2.0, 1.0);
        let mode_st = region.compute_region_physics_mode();
        assert_eq!(mode_st, HolographicPhysicsMode::SpaceTime);

        region.spectrum_configuration = SpectrumRatio::new(1.0, 2.0);
        let mode_ts = region.compute_region_physics_mode();
        assert_eq!(mode_ts, HolographicPhysicsMode::TimeSpace);

        region.spectrum_configuration = SpectrumRatio::new(1.0, 1.0);
        let mode_q = region.compute_region_physics_mode();
        assert_eq!(mode_q, HolographicPhysicsMode::Quantum);
    }

    #[test]
    fn test_scale_world_creation() {
        let quantum_scale = ScaleWorld::new(ScaleLevel::Quantum);
        assert_eq!(quantum_scale.scale_level, ScaleLevel::Quantum);
        assert_eq!(quantum_scale.resolution, 1e-35);

        let cosmic_scale = ScaleWorld::new(ScaleLevel::Cosmic);
        assert_eq!(cosmic_scale.scale_level, ScaleLevel::Cosmic);
        assert_eq!(cosmic_scale.resolution, 1e22);
    }

    #[test]
    fn test_holographic_world_creation() {
        let blueprint = HolographicBlueprint::new(1, 42);
        let world = HolographicWorld::new(1, blueprint);

        assert_eq!(world.world_id, 1);
        assert_eq!(world.evolution_stage, 0.0);
        assert_eq!(world.world_age, Duration::ZERO);
        assert_eq!(world.holographic_coherence, 1.0);
    }

    #[test]
    fn test_world_add_region() {
        let blueprint = HolographicBlueprint::new(1, 42);
        let mut world = HolographicWorld::new(1, blueprint);
        let region = WorldRegion::new(
            1,
            DensityRegionType::ThirdDensityVeiled,
            [0.0, 10.0, 0.0, 10.0, 0.0, 10.0],
        );

        let result = world.add_region(region);
        assert!(result.is_ok());
        assert_eq!(world.regions.len(), 1);
    }

    #[test]
    fn test_world_add_scale_world() {
        let blueprint = HolographicBlueprint::new(1, 42);
        let mut world = HolographicWorld::new(1, blueprint);
        let scale_world = ScaleWorld::new(ScaleLevel::Quantum);

        world.add_scale_world(scale_world);
        assert_eq!(world.scale_worlds.len(), 1);
        assert!(world.scale_worlds.contains_key(&ScaleLevel::Quantum));
    }

    #[test]
    fn test_world_evolution() {
        let blueprint = HolographicBlueprint::new(1, 42);
        let mut world = HolographicWorld::new(1, blueprint);
        let region = WorldRegion::new(
            1,
            DensityRegionType::ThirdDensityVeiled,
            [0.0, 10.0, 0.0, 10.0, 0.0, 10.0],
        );
        world.add_region(region).unwrap();

        let initial_age = world.world_age;
        world.evolve(Duration::from_secs(10));

        assert!(world.world_age > initial_age);
        assert!(world.evolution_stage > 0.0);
    }

    #[test]
    fn test_world_generator_creation() {
        let generator = HolographicWorldGenerator::new();
        assert_eq!(generator.statistics.worlds_generated, 0);
    }

    #[test]
    fn test_world_unfolding() {
        let mut generator = HolographicWorldGenerator::new();
        let blueprint = HolographicBlueprint::new(1, 42);

        let world = generator.unfold(blueprint, None);
        assert!(world.is_ok());

        let world = world.unwrap();
        assert_eq!(world.world_id, 1);
        assert_eq!(world.scale_worlds.len(), 7);
        assert_eq!(world.regions.len(), 8);
    }

    #[test]
    fn test_world_unfolding_with_observer() {
        let mut generator = HolographicWorldGenerator::new();
        let blueprint = HolographicBlueprint::new(1, 42);

        let observer_key = ObserverKey::new(1, 2);
        let world = generator.unfold(blueprint, Some(observer_key));

        assert!(world.is_ok());
        let world = world.unwrap();
        assert_eq!(world.observer_keys.len(), 1);

        for region in world.regions.values() {
            assert_eq!(region.observer_count, 1);
        }
    }

    #[test]
    fn test_world_regeneration() {
        let mut generator = HolographicWorldGenerator::new();
        let blueprint = HolographicBlueprint::new(1, 42);

        let mut world = generator.unfold(blueprint.clone(), None).unwrap();
        world.evolve(Duration::from_secs(100));
        let evolved_age = world.world_age;

        let result = generator.regenerate_world(&mut world);
        assert!(result.is_ok());
        assert_eq!(world.world_id, 1);
        assert_eq!(world.world_age, Duration::ZERO);
    }

    #[test]
    fn test_holographic_continuity() {
        let mut generator = HolographicWorldGenerator::new();
        let blueprint = HolographicBlueprint::new(1, 42);

        let world_a = generator.unfold(blueprint.clone(), None).unwrap();
        let world_b = generator.unfold(blueprint.clone(), None).unwrap();

        let continuity = generator.compute_holographic_continuity(&world_a, &world_b);
        assert!(continuity > 0.0);
        assert!(continuity <= 1.0);
    }

    #[test]
    fn test_statistics_tracking() {
        let mut generator = HolographicWorldGenerator::new();
        let blueprint = HolographicBlueprint::new(1, 42);

        generator.unfold(blueprint.clone(), None).unwrap();
        generator.unfold(blueprint.clone(), None).unwrap();

        let stats = generator.get_statistics();
        assert_eq!(stats.worlds_generated, 2);
        assert_eq!(stats.total_regions_created, 16);
    }

    #[test]
    fn test_unfolding_progress_estimation() {
        let generator = HolographicWorldGenerator::new();
        let progress = generator.estimate_unfolding_progress(2, 8);

        assert_eq!(progress.total_stages, 8);
        assert_eq!(progress.current_stage, 2);
        assert_eq!(progress.progress_percentage, 25.0);
        assert!(!progress.stage_name.is_empty());
    }

    #[test]
    fn test_region_capacity_limit() {
        let blueprint = HolographicBlueprint::new(1, 42);
        let mut world = HolographicWorld::new(1, blueprint);

        for i in 0..MAX_REGIONS_PER_WORLD {
            let region = WorldRegion::new(
                i as RegionId,
                DensityRegionType::ThirdDensityVeiled,
                [0.0, 10.0, 0.0, 10.0, 0.0, 10.0],
            );
            world.add_region(region).unwrap();
        }

        let overflow_region = WorldRegion::new(
            999,
            DensityRegionType::ThirdDensityVeiled,
            [0.0, 10.0, 0.0, 10.0, 0.0, 10.0],
        );
        let result = world.add_region(overflow_region);
        assert!(result.is_err());
    }

    #[test]
    fn test_density_region_type_from_density() {
        assert_eq!(
            DensityRegionType::from_density(Density::First),
            DensityRegionType::FirstDensityMatter
        );
        assert_eq!(
            DensityRegionType::from_density(Density::Fourth),
            DensityRegionType::FourthDensityLove
        );
        assert_eq!(
            DensityRegionType::from_density(Density::Eighth),
            DensityRegionType::EighthDensityInfinity
        );
    }

    #[test]
    fn test_spectrum_configuration_override_default() {
        let override_config = SpectrumConfigurationOverride::default();
        assert_eq!(override_config.space_time_multiplier, 1.0);
        assert_eq!(override_config.time_space_multiplier, 1.0);
        assert_eq!(override_config.quantum_probability, 0.5);
    }
}
