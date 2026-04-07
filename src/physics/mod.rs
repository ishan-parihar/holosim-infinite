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
// V5 Phase 4: Quantum Field Implementation
// - QuantumField: Core quantum field managing amplitudes and entanglements
// - QuantumStateSignature: Unique signature for quantum states
// - AttractorField: Stable quantum configurations corresponding to matter
// - Element: Chemical elements emerging from attractor patterns
//
// Knowledge Base References:
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Phase 4, Task 4.1
// - V5_PHASE4_QUANTUM_FIELD_SPEC.md

pub mod archetype_physics;
pub mod dual_physics;
pub mod feedback;
pub mod integrator;
pub mod matter_emergence;
pub mod observation_sync;
pub mod periodic_table;
pub mod physics_world;
pub mod quantum_field;

#[cfg(test)]
mod tests;

// Re-exports for convenience
pub use archetype_physics::{ArchetypePhysicsMapper, Force3D, PhysicalProperties};
pub use dual_physics::{ComparisonReport, DualPhysicsSystem, ParticleProperties, PhysicsMode};
pub use feedback::{
    ArchetypeProfile, FeedbackEvent, FeedbackTrigger, PhysicsFeedback, BODY_END, BODY_START,
    DEFAULT_RESONANCE_THRESHOLD, FREE_WILL_INDEX, HIGH_ACCELERATION_THRESHOLD,
    LOW_VELOCITY_THRESHOLD, MAX_MODIFICATION_PER_TICK, SPIRIT_END, SPIRIT_START,
};
pub use integrator::{RK4Derivative, RK4Integrator, RK4State};
pub use matter_emergence::{
    Atom, MatterEmergenceError, MatterEmergencePipeline, MatterEmergenceStatistics,
    QuantumStateSnapshot,
};
pub use observation_sync::{CollisionEvent, ObservationSync};
pub use periodic_table::{ElementAttractor, PeriodicTable};
pub use physics_world::{EntityPhysicsState, PhysicsEntity, PhysicsWorld};
pub use quantum_field::{
    AttractorField, Element, EntanglementLink, HolographicBlueprint, QuantumField,
    QuantumFieldError, QuantumStateSignature, Spin,
};
