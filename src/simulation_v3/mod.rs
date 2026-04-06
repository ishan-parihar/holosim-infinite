pub mod advanced_game_mechanics;
pub mod archetype_basis;
pub mod archetype_interference_cache;
pub mod archetypical_crafting;
pub mod archetypical_interference_engine;
pub mod catalyst_system;
pub mod causal_inversion;
pub mod collective_dynamics;
pub mod collective_emergence_visualizer;
pub mod collective_manifestation;
pub mod collective_statistics;
pub mod collective_system;
pub mod collective_visualizer;
pub mod collision_detection;
pub mod consciousness_processor;
pub mod density_mechanics;
pub mod density_region_generator;
pub mod density_switching;
pub mod density_transition_system;
pub mod distributed_system;
pub mod dynamic_world_evolution;
pub mod embodied_body;
pub mod emergent_behavior;
pub mod emergent_behavior_visualizer;
pub mod entity_lifecycle;
pub mod environment;
pub mod fractal_cache;
pub mod free_will_seed;
pub mod holographic_field;
pub mod holographic_game_loop;
pub mod holographic_inventory;
pub mod holographic_material;
pub mod holographic_memory;
pub mod holographic_physics;
pub mod holographic_world_generator;
pub mod individual_progression;
pub mod individual_progression_visualizer;
pub mod individual_variation_statistics;
pub mod individual_variation_visualizer;
pub mod inter_scale_interactions;
pub mod inter_scale_interactions_visualizer;
pub mod interactive_explorer;
pub mod interactive_interface;
pub mod involution_sequence;
pub mod living_environment;
pub mod memory_profiler;
pub mod mera_network;
pub mod multiscale_camera;
pub mod multiscale_field;
pub mod multiscale_world;
pub mod observer_effect;
pub mod observer_registry;
pub mod persistence;
pub mod physical_structure_visualizer;
pub mod position_adapter;
pub mod predictive_loading;
pub mod quantum_statistics;
pub mod quantum_visualizer;
pub mod reporter;
pub mod resonance_visualizer;
pub mod scale_physics;
pub mod scale_specific_physics;
pub mod scale_transition_optimizer;
pub mod simulation_runner;
pub mod social_processor;
pub mod spectrum_visualizer;
pub mod statistics;
pub mod visualization;

pub use archetype_basis::{
    ArchetypeActivationProfile, ArchetypeBasis, ArchetypeBasisError, ArchetypeBasisStatistics,
    ArchetypeVector, ArchetypicalPattern, NUM_ARCHETYPES, PROFILE_SIZE_BYTES,
};
pub use archetype_interference_cache::{
    ArchetypeCacheError, ArchetypeCacheStatistics, ArchetypeInterferenceCache, ArchetypeKey,
    ArchetypicalInterference, MAX_CACHE_SIZE, QUANTIZATION_LEVELS,
};
pub use archetypical_crafting::{
    ArchetypicalCrafting, CraftingError, CraftingInterferencePattern, CraftingResult,
    CRAFTING_RESONANCE_THRESHOLD, MAX_CRAFTING_ITEMS, MIN_CRAFTING_ITEMS,
    MIN_INTERFERENCE_COHERENCE,
};
pub use archetypical_interference_engine::{
    ActionVector, ArchetypicalInterferenceEngine, EmergentBehavior, InterferenceEngineError,
    InterferenceEngineStatistics, InterferencePattern,
};
pub use collective_dynamics::CollectiveInfluenceStatistics;
pub use collective_manifestation::{
    CollectiveResonanceCalculator, CollectiveResonanceResult, FunctionalProperties,
    GeometricPattern, GeometricPatternType, HolographicStructure, ManifestationError,
    ManifestationManager, ManifestationProject, MaterialAppearance, StructureProperties,
    StructureType, VisualProperties,
};
pub use collision_detection::{
    CollisionEvent, CollisionExceptionType, CollisionType, DensityCollisionException,
    DensityCollisionExceptions, HolographicCollisionSystem,
    InterferencePattern as CollisionInterferencePattern, SpectrumCollisionRules,
    DENSITY_COLLISION_EXCEPTION_THRESHOLD, INTERFERENCE_COLLISION_THRESHOLD,
    OBSERVER_COLLISION_INFLUENCE, SPECTRUM_COLLISION_MODIFIER,
};
pub use density_mechanics::{
    ActionSpace, Catalyst, Density, DensityMechanics, DensityMechanicsError,
    DensityMechanicsStatistics, EighthDensityMechanics, FifthDensityMechanics,
    FirstDensityMechanics, FourthDensityMechanics, InteractionRules, PerceptionFilter, Polarity,
    SecondDensityMechanics, SeventhDensityMechanics, SixthDensityMechanics, ThirdDensityMechanics,
    MAX_CATALYST_INTENSITY, MAX_VEIL_TRANSPARENCY, MIN_VEIL_TRANSPARENCY, NUM_DENSITIES,
};
pub use density_region_generator::{
    AudioEffect, DensityRegionConfig, DensityRegionError, DensityRegionGenerator,
    DensityRegionStatistics, EnvironmentalEffects, PhysicsModifiers, Polarity as RegionPolarity,
    ScaleIntegration, SpawnRules, SpectrumModifiers,
    TransitionInterpolationMode as RegionTransitionMode, TransitionZone, VisualEffect,
    MAX_REGIONS_PER_DENSITY, MAX_REGION_SIZE, MIN_REGION_SIZE,
};
pub use density_switching::{
    DensityStateMachine, DensityStateUpdateResult, DensityTransition, DensityTransitionError,
    GameplayDensity, GameplaySpectrumAccess, GameplayVeilState, HolographicContinuity,
    InterpolatedDensityState, TransitionInterpolationMode, TransitionUpdateResult,
};
pub use density_transition_system::{
    DensityMemoryEntry, DensityTransitionError as DtsError,
    DensityTransitionState as TransitionState, DensityTransitionStatistics,
    DensityTransitionSystem, TransitionInterpolationMode as DtsInterpolationMode,
    TransitionUpdateResult as DtsUpdateResult, DEFAULT_TRANSITION_DURATION, MAX_DENSITY_MEMORIES,
    MAX_TRANSITION_DURATION, MIN_TRANSITION_DURATION,
};
pub use dynamic_world_evolution::{
    CompressedBlueprint, CompressedSnapshot, DynamicWorldEvolution, EvolutionEvent,
    EvolutionStatus, EvolutionTrigger, RegenerationResult, RegionState, WorldEvolution,
    WorldEvolutionError, WorldEvolutionStatistics, WorldPersistenceData, WorldSnapshot,
    EVOLUTION_COMPLETION_THRESHOLD, MAX_REGENERATION_ATTEMPTS, MAX_WORLD_HISTORY_SNAPSHOTS,
    MIN_SPECTRUM_CHANGE_THRESHOLD, PERSISTENCE_VERSION,
};
pub use environment::EnvironmentalStatistics;
pub use fractal_cache::{
    EvictionPolicy, FractalCache, FractalCacheEntry, FractalCacheError, FractalCacheKey,
    FractalCacheStatistics, FractalData, FractalRefinement, RefinementMode,
};
pub use free_will_seed::{
    deterministic_choice_reconstruction, generate_possibility_space, ActionType,
    ArchetypeConstraint, Choice, FreeWillChoiceEngine, FreeWillError, FreeWillSeed,
    FreeWillStatistics, Possibility, PossibilitySpace,
};
pub use holographic_game_loop::{GameLoopResult, HolographicGameLoop};
pub use holographic_inventory::{
    ArchetypicalItemSignature, HolographicInventory, HolographicItem, HolographicItemBlueprint,
    InventoryError, InventoryStatistics, ItemCategory, ItemId, ResonancePattern,
    DEFAULT_RESONANCE_CAPACITY, MAX_INVENTORY_SIZE, MIN_RESONANCE_COST, RESONANCE_DECAY_RATE,
    RESONANCE_REGEN_RATE,
};
pub use holographic_material::{
    HolographicMaterial, HolographicMaterialError, HolographicMaterialSystem, MaterialId,
    MaterialPhase, MaterialProperties, MaterialSystemStatistics, ResonanceCache, ResonanceKey,
    BIOCOMPATIBILITY_MAX, BIOCOMPATIBILITY_MIN, CHEMICAL_REACTIVITY_MAX, CHEMICAL_REACTIVITY_MIN,
    CONDUCTIVITY_MAX, CONDUCTIVITY_MIN, DEFAULT_CACHE_SIZE, DENSITY_MAX, DENSITY_MIN,
    DENSITY_MULTIPLIER, DIELECTRIC_CONSTANT_MAX, DIELECTRIC_CONSTANT_MIN, ELASTIC_MODULUS_MAX,
    ELASTIC_MODULUS_MIN, ELECTRICAL_RESISTIVITY_MAX, ELECTRICAL_RESISTIVITY_MIN, HARDNESS_MAX,
    HARDNESS_MIN, MAGNETIC_PERMEABILITY_MAX, MAGNETIC_PERMEABILITY_MIN, MAX_MATERIALS,
    REFRACTIVE_INDEX_MAX, REFRACTIVE_INDEX_MIN, RESONANCE_MULTIPLIER, SPECIFIC_HEAT_MAX,
    SPECIFIC_HEAT_MIN, THERMAL_CONDUCTIVITY_MAX, THERMAL_CONDUCTIVITY_MIN,
};
pub use holographic_memory::{
    HolographicMemoryError, HolographicMemoryStatistics, HolographicMemorySystem,
    HolographicSignature, MemoryEntry, MemoryKey, MemoryQuery, MemoryType, SoulStream,
    SoulStreamId, TranscendedMemory,
};
pub use holographic_physics::{
    HolographicEntity, HolographicPhysicsEngine, HolographicPhysicsError, HolographicPhysicsMode,
    Interaction, InteractionType, PhysicsEngineStatistics, PhysicsInterferencePattern,
    SpectrumRatio, SpectrumThresholds,
};
pub use holographic_world_generator::{
    DensityRegionType, HolographicBlueprint, HolographicWorld, HolographicWorldGenerator, RegionId,
    ScaleWorld, SpectrumConfigurationOverride, UnfoldingProgress, WorldGeneratorError,
    WorldGeneratorStatistics, WorldId, WorldRegion, DENSITY_REGION_TRANSITION_RATE,
    MAX_REGIONS_PER_WORLD, MAX_WORLD_ENTITIES, WORLD_UNFOLDING_DURATION,
};
pub use memory_profiler::{
    CacheMemoryStats, CompressionMetrics, MemoryProfiler, MemorySnapshot, HOLOGRAPHIC_ENTITY_SIZE,
    HOLOGRAPHIC_SCALING_EXPONENT, TRADITIONAL_ENTITY_SIZE,
};
pub use mera_network::{
    MeraCompressionResult, MeraDecompressionResult, MeraError, MeraLayer, MeraNetwork, MeraQuery,
    MeraScale, MeraStatistics, QueryType, Tensor, WaveletCoefficients,
};
pub use multiscale_camera::{
    InterpolatedScale, InterpolatedScaleView, InterpolationMode, MultiScaleCamera, PhysicsMode,
    ScaleLevel, ScaleTransition, ScaleView,
};
pub use multiscale_field::{
    HolographicView, MultiScaleField, MultiScaleFieldError, MultiScaleFieldStatistics,
};
pub use multiscale_world::{
    BiologicalMechanics, CellularMechanics, CosmicMechanics, FieldVisualizationMode,
    GalacticMechanics, HolographicContinuity as MultiScaleContinuity, MultiScaleWorldError,
    MultiScaleWorldRenderer, MultiScaleWorldStatistics, PlanetaryMechanics, PostProcessingEffects,
    QuantumMechanics, RenderCache, RenderingFeatures, ScaleSimulationData,
    ScaleTransitionVisualization, ScaleWorldFeatures, ScaleWorldState, ShadowQuality,
    StellarMechanics, TransitionEffects, UniqueMechanics, HOLOGRAPHIC_CONTINUITY_THRESHOLD,
    MAX_ENTITIES_PER_SCALE, SCALE_TRANSITION_DURATION,
};
pub use observer_effect::{
    CollapsedState, ObservationCache, ObservationCacheStatistics, ObservationEngine,
    ObserverEffect, ObserverEffectError, ObserverKey,
};
pub use predictive_loading::{
    LoadPriority, PredictiveLoader, PredictiveLoaderError, PredictiveLoaderStatistics,
};
pub use scale_physics::HolographicContinuity as PhysicsContinuity;
pub use scale_physics::{
    Action, AminoAcid, ArtType, ArtWork, Atmosphere, BasisState, BiologicalChange,
    BiologicalSimulation, BlackHole, Brane, CellCycle, CellularChange, CellularSimulation,
    Civilization, CorrelationType, CosmicBackground, CosmicChange, CosmicSimulation, CosmicVoid,
    CulturalEvolution, CulturalTrait, DarkMatterDistribution, DarkMatterFilament, DarkMatterHalo,
    DensityProfile, DimensionalStructure, EconomicSystem, EnergyFlow, EnergyType, Entanglement,
    Galaxy, GalaxyCluster, GalaxyType, GeneExpression, GovernmentType, IntelligentInfinity, Jets,
    LargeScaleStructure, Meme, MetabolicState, Needs, OrbitalPath, PerformanceMetrics, Planet,
    PlanetType, PlanetaryChange, PlanetarySimulation, PopulationDynamics, Protein, QuantumChange,
    QuantumPhysics, ResearchProject, ResourceDeposit, ResourceType, ScalePhysicsError,
    ScaleSpecificPhysics, SensoryInput, SimulationResult, SpectralType, SpinState, Star,
    StarFormationRegion, StellarChange, StellarEvolution, StellarPhase, StellarSimulation,
    StringVibration, Supercluster, Superposition, TechCategory, TechnologyLevel, TradeNetwork,
    TradeRoute, Universe, UniverseGeometry, WaveFunction,
};
pub use scale_specific_physics::{
    ClassicalPhysicsMode as ScaleClassicalPhysicsMode,
    MetaphysicalPhysicsMode as ScaleMetaphysicalPhysicsMode,
    QuantumPhysicsMode as ScaleQuantumPhysicsMode, ScaleLevel as ScaleSpecificLevel,
    ScaleSpecificPhysics as ScaleSpecificPhysicsSystem, WaveFunction as ScaleWaveFunction,
};
pub use scale_transition_optimizer::{
    OptimizedTransition, RefinementState, ScaleTransitionOptimizer, TransitionCacheEntry,
    TransitionKey, TransitionMetrics, TransitionOptimizerError, TransitionOptimizerStatistics,
};
pub use simulation_runner::SimulationParameters;

// Causal Inversion (Phase 0)
pub use causal_inversion::{
    CausalInversionConfig, CausalInversionRunner, CausalSimulationResult, CausalStatistics,
    CausalTickResult, EntityData, InfinityPulseState, ManifestationPotential, PotentialEntityType,
    ValidationResult, EXPECTED_PHASE_COUNT, EXPECTED_PHASE_ORDER,
};

// Observer Registry (Phase 1)
pub use observer_registry::{CacheStats, ObserverRegistry, ObserverRegistryConfig};

// Embodied Body (Phase 3, P1)
pub use embodied_body::{BodyEnvironment, DeathCause, EmbodiedBody, SensoryField, SurvivalStatus};

// Living Environment (Phase 4)
pub use living_environment::{
    EntitySpatialPosition, LivingEnvironment, LivingEnvironmentConfig, LivingEnvironmentStats,
    ResourceDistribution, SpatialCell,
};

// Position Adapter (Phase 9 - Unified Position System)
pub use position_adapter::{PositionIntegration, SpatialPositionAdapter, UnifiedPositionAdapter};

// Social Processor (Phase 6)
pub use social_processor::{SocialOutput, SocialProcessor};

// Interactive Interface (Phase 8)
pub use interactive_interface::{
    Bookmark, EntityInspector, EventNarrator, InspectorTab, InteractiveInterface, ObserverMode,
    ScaleController, SimulationEvent,
};

// Advanced Game Mechanics (Phase 6, Week 93-96)
pub use advanced_game_mechanics::{
    AdvancedGameMechanicsError, ArchetypeId, CatalystAmount, CombatId, FactionId, QuestId,
    ResonanceCompatibility, Result as AdvancedMechanicsResult, Timestamp, TradeId,
};

// Distributed System (Phase 7, Weeks 97-112)
pub use distributed_system::{
    ChoiceId, ConnectionStatus, FieldSignature, Latency, MessagePriority, NetworkConfig,
    NetworkError, ObservationId, PeerId, Result as DistributedSystemResult, SessionId, UpdateId,
    Version,
};

#[cfg(test)]
mod tests {
    #[test]
    fn test_simulation_v3_module_exists() {
        // Test that the module compiles and exports exist
        // Test passes if module exists
    }
}
