//! Intelligence Module - Active Intelligence Engine
//!
//! From HOLOSIM_INFINITE_RnD_ROADMAP_V5.md Phase 8:
//! "Intelligent Infinity Activation - II as active kernel, not passive field"
//!
//! ## Overview
//!
//! This module transforms Intelligent-Infinity from a passive field emitter
//! to an active, learning intelligence that:
//!
//! 1. **Learns from Outcomes**: Records and analyzes condensation outcomes
//! 2. **Optimizes Emergence**: Finds optimal patterns for target manifestations
//! 3. **Creates New Attractors**: Generates new attractor fields as needed
//!
//! ## Key Components
//!
//! - `ActiveIntelligenceEngine`: Main engine for goal-directed intelligence
//! - `AttractorOptimizer`: Optimization algorithms for emergence patterns
//! - `CondensationOutcome`: Records of matter manifestation events
//! - `EmergenceTarget`: Goals for optimization
//!
//! ## From ROADMAP Phase 8.1
//!
//! ```text
//! ActiveIntelligenceEngine {
//!     feedback_collector: Vec<EntityFeedback>,
//!     condensation_history: Vec<CondensationOutcome>,
//!     attractor_optimizer: AttractorOptimizer,
//! }
//! ```

pub mod active_engine;

// Re-exports for convenience
pub use active_engine::{
    ActiveIntelligenceEngine, AttractorOptimizer, AttractorSpec, CatalystData, CondensationOutcome,
    CondensationType, EmergenceConstraint, EmergenceTarget, EmergenceTargetType, EngineStatistics,
    FieldConfiguration, OptimalPattern, OptimizationRecord, PatternType, PHI,
};
