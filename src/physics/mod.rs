// Physics Module - Phase 4: Integration & Migration
//
// This module provides a dual-mode physics system that can switch between
// hardcoded and holographic physics during the migration phase.
//
// Key Components:
// - PhysicsMode: Enum for selecting physics calculation mode
// - DualPhysicsSystem: Main entry point for physics calculations
// - HardcodedPhysicsEngine: Wrapper for existing derive_* functions
// - HolographicPhysicsEngine: Wrapper for holographic discovery
// - ComparisonReport: Report comparing results from both engines
//
// Knowledge Base References:
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Phase 4, Task 4.1

pub mod dual_physics;

// Re-exports for convenience
pub use dual_physics::{ComparisonReport, DualPhysicsSystem, ParticleProperties, PhysicsMode};
