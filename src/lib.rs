//! Holonic Realms - A comprehensive simulation of the Law of One cosmological principles
//!
//! This library implements holographic architecture, density octave evolution,
//! and the emergence of consciousness through free will based on the Law of One.

// Foundation modules (Layers 0-3)
pub mod entity_layer7;
pub mod foundation;
pub mod spectrum;

// Cosmological Module (Top-Down Flow - Phase 2)
pub mod cosmological;

// Unified Module - Physics + Biology + Consciousness (Phase 3)
pub mod unified;

// Consciousness and Free Will
pub mod consciousness;
pub mod free_will_capacity;
pub mod free_will_integration;

// Evolution and Density
pub mod evolution;
pub mod evolution_chain;
pub mod evolution_density_octave;
pub mod involution;

// Physical Manifestation and Matter
pub mod matter;
pub mod physical_manifestation;

// Holographic and Energy Systems
pub mod energy_center_interface;
pub mod energy_fields;
pub mod energy_flow_system;
pub mod energy_ray_centers;
pub mod holographic;
pub mod template;
pub mod compression;
pub mod sacred_geometry;

// Simulation and Entity Management
pub mod entity;
pub mod entity_holon_integration;
pub mod entity_state;
pub mod holon;
pub mod simulation_v3;

// Attractors and Patterns
pub mod attractor_pattern_system;
pub mod attractors;

// Archetypes
pub mod archetypes;

// Holographic Properties
pub mod holographic_complex;
pub mod holographic_reference;
pub mod holographic_seed;

// Veil and Density
pub mod veil;

// Coordinates and Navigation
pub mod coordinates;

// Light and Photon
pub mod light;

// Memory and Social Systems
pub mod memory;
pub mod social_memory;

// Physics
pub mod physics;
pub mod physics_derivation;
pub mod physics_engine;

// Polarity
pub mod polarization;

// Multi-scale Architecture
pub mod multi_scale;

// Exploration
pub mod exploration;

// Validation
pub mod validation;

// Complete simulation
pub mod complete_simulation;

// HPO System (Phase 1: Hyperparameter Optimization)
pub mod hpo;

// Dynamic Mechanisms (Phase 2)
pub mod dynamic_mechanisms;

// Biological Emergence (Phase 3)
pub mod biology;

// Noospheric Systems (Phase 4)
pub mod noosphere;

// Gaia Systems (Phase 4)
pub mod gaia;

// GUI System (Phase 5)
pub mod gui;

// Integrated System (Phase 6)
pub mod emergence_validator;
pub mod integrated_system;
pub mod performance_optimizer;
pub mod user_acceptance;

// Simulation audit (disabled - needs refactoring)

// Additional modules
pub mod decision_engine;
pub mod dual_dimensional_integration;
pub mod fractal_holographic_structure;
pub mod intelligent_infinity;
pub mod natural_laws;
pub mod organic_reality_generator;
pub mod physical_dimension;
pub mod probability;
pub mod scale_architecture;
pub mod transformation_engine;

// Adapters for compatibility
pub mod adapters;

// Performance testing infrastructure (Phase 8)
#[cfg(test)]
pub mod performance_tests;

// Core systems
pub mod complex;
pub mod solar_system;
pub mod spirit_channel;
pub mod types;

// Re-exports for convenience
pub use foundation::{
    indigo_realm::{
        Archetype22, IntelligentInfinity, PolarityChoice, Possibility, PossibilitySpace,
    },
    violet_realm::VioletRealm,
};

pub use entity_layer7::layer7::{
    EntityId, EntityType, IndividualSpectrumConfiguration, SubSubLogos,
};

pub use spectrum::{
    larson_framework::{SpectrumRatio, SpectrumSide},
    orange_realm::OrangeRealm,
    red_realm::RedRealm,
    yellow_realm::YellowRealm,
};

pub use entity_layer7::layer7::EntitySpectrumAccess;
pub use evolution_density_octave::density_octave::{Density, Density1SubLevel, Density2SubLevel};

// Re-export all types for backward compatibility
pub use types::{
    ComplexType, EnvironmentID, HealthStatus, HolonID, HolonicLevel, Octant, Polarity, Rung,
};

// Re-export ComplexType as `complex_type` for backward compatibility (avoid naming conflict with complex module)
pub use types::ComplexType as complex_type;

// Re-export HPO system types for convenience
pub use hpo::{
    ComplexityMetrics, ConfigError, ConfigId, EmergenceMetrics, FailureType, FitnessScore,
    Float as HpoFloat, HpoSystem, ParameterSpace, SelectionAlgorithm, SelectionParameters,
    SimulationConfig, SimulationConfigBuilder, SimulationId, SimulationResult, SimulationRunner,
    StabilityMetrics,
};

// Re-export HPO sub-module types
pub use hpo::fitness_evaluator::FitnessStatistics;
pub use hpo::hpo_system::OptimizationResult;
pub use hpo::selection::{ConvergenceStatus, SelectionStatistics};

// ============================================================================
// SIMULATION V3 EXPORTS (Complete Cosmological Architecture)
// ============================================================================

// ============================================================================
// SIMULATION V3 EXPORTS (Complete Cosmological Architecture)
// ============================================================================

// Main simulation runner
pub use simulation_v3::simulation_runner::{
    SimulationRunner as V3SimulationRunner, 
    SimulationParameters, 
    SimulationResult as V3SimulationResult,
};

// Involution sequence
pub use simulation_v3::involution_sequence::{
    InvolutionSequenceRunner, InvolutionResult, InvolutionStage,
};

// Entity lifecycle
pub use simulation_v3::entity_lifecycle::{
    EntityLifecycleManager, EvolutionResult, LifecycleStatistics,
};

// Holographic field
pub use simulation_v3::holographic_field::{
    HolographicFieldManager, HolographicFieldResult, HolographicFieldConfig,
};

// Collective dynamics
pub use simulation_v3::collective_dynamics::CollectiveDynamicsManager;

// Scale physics
pub use simulation_v3::scale_physics::{
    ScaleSpecificPhysics, SimulationResult as ScaleSimulationResult,
};

// Statistics
pub use simulation_v3::statistics::{
    SimulationStatistics as V3SimulationStatistics, 
    PolarizationDistribution as V3PolarizationDistribution, 
    EmergentProperties,
};

// ============================================================================
// PHASE 4: ENTITY-ENVIRONMENT COUPLING & SPATIAL EMERGENCE
// ============================================================================

// Spatial module - Spectrum-derived positions instead of golden ratio
pub mod spatial;
pub use spatial::{
    SpectrumSpatialDynamics,
    SpectrumSpatialConfig,
    CoherenceDerivedPosition,
    SpatialPerspective,
    SpatialStatistics,
    VeilTransform,
    VeilTransformConfig,
    EntityExperience,
    Perspective,
    TimeFlowType,
    PerceptionType,
    TransformedPosition,
    VeilStatistics,
};

// Environment module - Entity-planet coupling
pub mod environment;
pub use environment::{
    PlanetId,
    EnvironmentId,
    EntityEnvironmentState,
    WeatherPattern,
    EnvironmentTerrain,
    EntityBridge,
    EntityBridgeConfig,
    EntityPlanetAssignment,
    EntityBridgeStatistics,
    EnvironmentalEffects,
    EnvironmentalEffectsConfig,
    EntityMutableState,
    ConsciousnessState,
    EnvironmentalEffectsStatistics,
};

// Emergent attractors module
pub use attractors::{
    EmergentAttractors,
    EmergentAttractorsConfig,
    CoherencePeak,
    EmergentAttractorType,
    EmergentAttractor,
    EmergentAttractorStatistics,
};
