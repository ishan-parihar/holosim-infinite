// Involution Modules - Phase 1 Implementation
//
// This module contains all involution-related functionality for the
// Phase 1 refactor of the Holographic Multi-Scale Cosmic Creation Simulator.
//
// Roadmap Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1
//
// ✅ PHASE 1 COMPLETE: All 8 tasks completed
//
// Completed Tasks:
// - Task 1.1: Density Transition Engine ✅
// - Task 1.2: Energy Densification System ✅
// - Task 1.3: Veil Thickening Mechanism ✅
// - Task 1.4: Memory Fading System ✅
// - Task 1.5: Catalyst Intensity Scaling ✅
// - Task 1.6: Involution Path Tracking ✅
// - Task 1.7: Sub-Density Support ✅
// - Task 1.8: Reversal Point System ✅
//
// Next Phase: Phase 2 - Physical Manifestation

pub mod catalyst_intensity;
pub mod density_transition_engine;
pub mod energy_densification;
pub mod memory_fading;
pub mod path_tracker;
pub mod reversal_point;
pub mod sub_density;

// Re-export density transition engine types for convenience
pub use density_transition_engine::{
    DensityTransitionEngine, DensityTransitionEvent, DensityTransitionState, TransitionStatistics,
};

// Re-export energy densification types for convenience
pub use energy_densification::{
    EnergyDensificationEvent, EnergyDensificationSystem, MatterProperties,
};

// Re-export memory fading types for convenience
pub use memory_fading::{
    MemoryAccessLevel, MemoryCategory, MemoryEntry, MemoryFadingSystem, MemoryStatistics,
};

// Re-export catalyst intensity types for convenience
pub use catalyst_intensity::{
    Catalyst, CatalystIntensityCurve, CatalystIntensitySystem, CatalystStatistics, CatalystType,
    CatalystTypeDistribution,
};

// Re-export path tracker types for convenience
pub use path_tracker::{InvolutionPathTracker, InvolutionStep, PathAnalysis, ReversalPoint};

// Re-export sub-density types for convenience
pub use sub_density::{
    EncodedSubDensity, SubDensity, SubDensityProgression, SubDensityStage, SubDensitySystem,
    SubDensityTransitionResult,
};

// Re-export reversal point types for convenience
pub use reversal_point::{
    ReversalCriteria, ReversalEvent, ReversalEventType, ReversalPointSystem, ReversalReadiness,
    ReversalState,
};
