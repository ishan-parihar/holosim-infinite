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
//! - Predictive Cache
//!   - Predicts field regions needed based on entity movement
//!   - Preloads regions before they're accessed
//!   - Enables 60 FPS with 10,000+ entities
//!
//! - Fractal Cache
//!   - Multi-scale field representation with 8 levels
//!   - O(log n) queries through hierarchical caching
//!   - Interpolation between scale levels
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
//! ## Performance Targets (Phase 5)
//!
//! | Metric | Current | Target | Method |
//! |--------|---------|--------|--------|
//! | Entities | 100-1000 | 10,000+ | MERA compression |
//! | FPS | ~1-5 | 60 | Predictive loading |
//! | Memory | O(n) | O(n^2/3) | Surface area encoding |
//! | Query | O(n) | O(log n) | Fractal caching |

pub mod mera_network;
pub mod tensor;
pub mod archetype_basis;
pub mod predictive_cache;
pub mod fractal_cache;

pub use mera_network::{MeraLayer, MeraNetwork, MeraScale};
pub use tensor::{Tensor, TensorShape, TensorData};
pub use archetype_basis::{ArchetypeBasis, ArchetypicalInterferenceCache, CompressionStats};
pub use predictive_cache::{
    PredictiveCache, 
    PredictiveCacheConfig, 
    CacheKey, 
    EntityTrajectory, 
    PredictiveCacheStats,
};
pub use fractal_cache::{
    FractalCache,
    FractalCacheConfig,
    ScaleLevel,
    FractalCacheKey,
    FractalCacheEntry,
    FractalCacheStats,
    EvictionStrategy,
};
