// Integration Modules
//
// This directory contains integration modules that connect different layers:
// - Dual-dimensional integration (physical/metaphysical)
// - Attractor pattern system
// - Hierarchical refinement
// - Feedback loops

// Re-export from source modules
pub use crate::attractor_pattern_system;
pub use crate::dual_dimensional_integration::{
    DualDimensionalIntegration, EnergyFlow, IntegrationResult, IntegrationState,
    IntegrationStatistics, ValidationResult,
};
pub use crate::feedback_loops;
pub use crate::hierarchical_refinement;
