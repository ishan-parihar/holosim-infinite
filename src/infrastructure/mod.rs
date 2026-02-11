// Infrastructure Modules
//
// This directory contains supporting infrastructure for the simulation:
// - Simulation management and execution
// - State management (world state, snapshots, views)
// - Common types and utilities

// Re-export from source modules
pub use crate::complete_simulation::{
    CompleteSimulation, EntityEvolution, EvolutionStep, PolarizationDistribution, SimulationResult,
    SimulationStatistics,
};
pub use crate::simulation_audit::SimulationAudit;
// pub use crate::simulation_context::SimulationContext; // DELETED - replaced by simulation_v3/ (Phase 4.2)
pub use crate::simulation_v3::simulation_runner::SimulationRunner; // Use V3.0 instead
pub use crate::snapshot;
pub use crate::types;
pub use crate::views;
pub use crate::world_state;
