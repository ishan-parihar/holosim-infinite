//! Compression Module - Phase 2: Holographic Compression
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 2 (Weeks 5-8):
//! "Implement MERA-style compression for exponential performance"
//!
//! This module provides:
//! - MERA Tensor Network (Multi-scale Entanglement Renormalization Ansatz)
//!   - Hierarchical compression across 7 scales (quantum → cosmic)
//!   - Disentangler tensors (remove redundancy)
//!   - Coarse-grainer tensors (combine similar representations)
//!   - Compression: O(n) → O(log n)
//!   - Decompression: O(log n) for specific queries
//!
//! - Archetypal Basis Compression
//!   - 22 orthogonal archetype vectors
//!   - ArchetypeActivationProfile as 22 coefficients
//!   - ArchetypicalInterferenceCache for pattern caching
//!   - Delta compression for profile updates
//!   - 100x compression (88 bytes vs thousands of floats)
//!
//! ## Key Principles
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "Each part contains the whole" = efficient compression through self-similarity
//! "MERA networks implement this mathematically"
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
//! - Memory: 100x reduction (store compressed, reconstruct as needed)
//! - Scale transition: 100,000x faster (just change view, no loading)
//! - Density transition: 10,000x faster (modify profile, not reload)

pub mod mera_network;
pub mod tensor;
pub mod archetype_basis;

pub use mera_network::{MeraLayer, MeraNetwork, MeraScale};
pub use tensor::{Tensor, TensorShape, TensorData};
pub use archetype_basis::{ArchetypeBasis, ArchetypicalInterferenceCache, CompressionStats};
