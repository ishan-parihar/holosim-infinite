// Exploration Module - Phase 6: Novel Discovery
//
// This module implements systems to explore variant holographic architectures
// and discover novel physics beyond the Standard Model.
//
// Key Components:
// - UniverseExplorer: Explores variant architectures
// - HypothesisGenerator: Generates hypotheses from discoveries
// - DiscoveryDatabase: Stores discovered architectures and physics
//
// Knowledge Base References:
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Phase 6

pub mod discovery_database;
pub mod hypothesis_generator;
pub mod universe_explorer;

// Re-exports for convenience
pub use discovery_database::{
    DiscoveryDatabase, DiscoveryQuery, DiscoveryRecord, NoveltyScore, PhysicsConstant, PhysicsLaws,
    StableConfiguration,
};
pub use hypothesis_generator::{
    Hypothesis, HypothesisDatabase, HypothesisGenerator, Pattern, PatternRecognizer,
    TestablePrediction,
};
pub use universe_explorer::{
    ArchitectureVariant, ExplorationResult, UniverseExplorer, VariantComparison,
};
