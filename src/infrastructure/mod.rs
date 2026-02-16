// Infrastructure Modules
//
// This directory contains supporting infrastructure for the simulation:
// - Simulation management and execution
// - State management (world state, snapshots, views)
// - Common types and utilities

// ============================================================================
// SIMULATION V3 EXPORTS (New System - Primary)
// ============================================================================

pub use crate::simulation_v3::simulation_runner::{
    SimulationRunner, SimulationParameters, SimulationResult as V3SimulationResult,
};

pub use crate::simulation_v3::involution_sequence::{
    InvolutionSequenceRunner, InvolutionResult, InvolutionStage,
};

pub use crate::simulation_v3::entity_lifecycle::{
    EntityLifecycleManager, EvolutionResult, LifecycleStatistics,
};

pub use crate::simulation_v3::holographic_field::{
    HolographicFieldManager, HolographicFieldResult, HolographicFieldConfig,
};

pub use crate::simulation_v3::collective_dynamics::CollectiveDynamicsManager;

pub use crate::simulation_v3::statistics::{
    SimulationStatistics, PolarizationDistribution as V3PolarityDistribution,
    EmergentProperties,
};

// ============================================================================
// LEGACY EXPORTS (Old System - Deprecated, kept for backward compatibility)
// ============================================================================

#[deprecated(note = "Use simulation_v3::SimulationRunner instead")]
pub use crate::complete_simulation::{
    CompleteSimulation, EntityEvolution, EvolutionStep, PolarizationDistribution as LegacyPolarityDistribution, 
    SimulationResult as LegacySimulationResult, SimulationStatistics as LegacySimulationStatistics,
};

pub use crate::simulation_audit::SimulationAudit;
pub use crate::snapshot;
pub use crate::types;
pub use crate::views;
pub use crate::world_state;
