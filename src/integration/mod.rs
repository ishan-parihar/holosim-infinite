// Integration Modules - V5 Phase 6 Implementation
//
// This directory contains integration modules that connect different layers:
// - Entity-Planet Bridge - Connects entities to their environment
// - Teleological Guidance - Provides purposeful evolution direction
// - Dual-dimensional integration (physical/metaphysical)
// - Attractor pattern system
// - Hierarchical refinement
// - Feedback loops
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "All aspects of reality are interconnected through the holographic principle.
// Integration is the recognition that separation is an illusion."

// Phase 6: New Integration Modules
pub mod entity_planet_bridge;
pub mod teleological_guidance;

// Re-export from source modules
pub use crate::attractor_pattern_system;
pub use crate::dual_dimensional_integration::{
    DualDimensionalIntegration, EnergyFlow, IntegrationResult, IntegrationState,
    IntegrationStatistics, ValidationResult,
};
pub use crate::feedback_loops;
pub use crate::hierarchical_refinement;

// Phase 6: Entity-Planet Bridge exports
pub use entity_planet_bridge::{
    EntityPlanetBridge, PlanetData, PlanetId,
    EnvironmentExperience, WeatherState, TerrainType, Season,
    BridgeStats,
};

// Phase 6: Teleological Guidance exports
pub use teleological_guidance::{
    TeleologicalGuidance, TeleologicalProgress,
    AdaptiveAttractor, EntityFeedback, EvolutionOption,
    GuidanceStats,
};
