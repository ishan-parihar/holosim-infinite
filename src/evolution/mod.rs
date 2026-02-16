//! Evolution Module - Intelligent Evolution System
//!
//! This module implements intelligent evolution for HoloSim_Infinite, replacing mechanical
//! probabilistic evolution with directed, purposeful evolution.
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3 (Weeks 9-12):
//! "The critical phase that addresses the audit's main finding: evolution is mechanical, not intelligent."
//!
//! ## Architecture
//!
//! The evolution module consists of three components:
//!
//! 1. **Adaptive Attractor Fields** (`adaptive_attractor.rs`):
//!    - Dynamic attractor fields that adjust based on entity feedback
//!    - Learning system that strengthens/weakens attractors based on effectiveness
//!    - Feedback loop: entities → attractors → entities
//!
//! 2. **Teleological Progress** (`teleological.rs`):
//!    - Purpose tracking toward return to Intelligent-Infinity
//!    - Progress measurement: alignment, coherence, service orientation, wisdom
//!    - Meaningful choices that align with purpose
//!
//! 3. **Enhanced Intelligent-Infinity** (`intelligent_infinity.rs`):
//!    - Active feedback collector from entities
//!    - Pattern analysis of entity behavior
//!    - Teleological pull toward source
//!
//! ## Key Principles
//!
//! - **Feedback-Based Learning**: Attractors adapt based on entity response
//! - **Teleological Direction**: Evolution has purpose (return to source, having served)
//! - **Meaningful Choices**: Not random, but aligned with purpose
//! - **Coherence Matters**: Alignment with purpose affects evolution rate
//!
//! ## Usage
//!
//! ```rust
//! use holonic_realms::evolution::{
//!     AdaptiveAttractorField, EntityFeedback,
//!     TeleologicalProgress, evaluate_purpose,
//!     IntelligentInfinity
//! };
//!
//! // Create adaptive attractor field
//! let mut attractor = AdaptiveAttractorField::new(Density::Third, 0.3);
//!
//! // Entity provides feedback
//! let feedback = EntityFeedback {
//!     entity_id: EntityId::new("entity-1".to_string()),
//!     attractor_pull: 0.7,
//!     evolution_progress: 0.5,
//!     alignment_with_attractor: 0.8,
//!     timestamp: 12345,
//! };
//! attractor.receive_feedback(feedback);
//! attractor.adjust_strength();
//!
//! // Evaluate entity's purpose alignment
//! let entity = /* ... */;
//! let teleological = evaluate_purpose(&entity);
//!
//! // Enhanced Intelligent-Infinity collects feedback
//! let mut ii = IntelligentInfinity::new();
//! ii.receive_entity_feedback(feedback);
//! let analysis = ii.analyze_feedback_patterns();
//! ```

pub mod adaptive_attractor;
pub mod intelligent_infinity;
pub mod teleological;

pub use adaptive_attractor::{
    AdaptiveAttractorField, EntityFeedback, FeedbackEffectiveness,
};
pub use intelligent_infinity::{FeedbackAnalysis, IntelligentInfinity};
pub use teleological::{evaluate_purpose, TeleologicalProgress};