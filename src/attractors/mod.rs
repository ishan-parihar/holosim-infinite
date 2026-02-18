//! Attractors Module
//!
//! This module contains systems related to attractor fields in the holographic simulation.
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 4.3:
//! "Create emergent attractors from coherence peaks"
//! "Attractors should emerge from field coherence peaks, not pre-defined formulas"

pub mod attractor_fields;
pub mod emergent_attractors;

pub use attractor_fields::DensityAttractorFields;
pub use emergent_attractors::*;
