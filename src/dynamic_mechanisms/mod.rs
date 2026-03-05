//! Dynamic Mechanisms Module (Phase 2)
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 2:
//! "Replace placeholder implementations with real dynamic behavior"
//!
//! This module implements the four dynamic mechanisms that drive simulation evolution:
//! 1. CatalystGenerator - Generate catalyst events with proper probability distribution
//! 2. ArchetypeProcessor - Process 22 archetypes through the mind matrix
//! 3. FreeWillEngine - Enable free will decisions affecting entity trajectories
//! 4. HolographicUnfolding - Implement holographic unfolding respecting space/time spectrum
//!
//! These mechanisms replace placeholder implementations with state-sensitive,
//! environment-aware, and evolution-driven dynamic behavior.

pub mod archetype_processor;
pub mod catalyst_generator;
pub mod free_will_engine;
pub mod holographic_unfolding;

// Re-export key types for convenience
pub use archetype_processor::{ArchetypeProcessingResult, ArchetypeProcessor};
pub use catalyst_generator::{CatalystGenerationContext, CatalystGenerator};
pub use free_will_engine::{FreeWillDecision, FreeWillEngine};
pub use holographic_unfolding::{HolographicUnfolding, UnfoldingResult};

/// Dynamic Mechanisms Configuration
///
/// Configuration parameters for all dynamic mechanisms.
#[derive(Debug, Clone)]
pub struct DynamicMechanismsConfig {
    /// Catalyst generation rate (0.0 to 1.0)
    pub catalyst_generation_rate: f64,

    /// Archetype processing speed (0.0 to 1.0)
    pub archetype_processing_speed: f64,

    /// Free will activation threshold (0.0 to 1.0)
    pub free_will_activation_threshold: f64,

    /// Holographic unfolding rate (0.0 to 1.0)
    pub holographic_unfolding_rate: f64,
}

impl Default for DynamicMechanismsConfig {
    fn default() -> Self {
        Self {
            catalyst_generation_rate: 0.3,
            archetype_processing_speed: 0.5,
            free_will_activation_threshold: 0.5,
            holographic_unfolding_rate: 0.2,
        }
    }
}

/// Dynamic Mechanisms State
///
/// Combined state of all dynamic mechanisms.
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct DynamicMechanismsState {
    /// Catalyst generator state
    pub catalyst_generator_state: catalyst_generator::CatalystGeneratorState,

    /// Archetype processor state
    pub archetype_processor_state: archetype_processor::ArchetypeProcessorState,

    /// Free will engine state
    pub free_will_engine_state: free_will_engine::FreeWillEngineState,

    /// Holographic unfolding state
    pub holographic_unfolding_state: holographic_unfolding::HolographicUnfoldingState,
}

