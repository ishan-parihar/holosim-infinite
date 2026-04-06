//! Compression Module - Phase 2: Holographic Compression
//!
//! Re-exports from simulation_v3 for backward compatibility.
//! All compression implementations now live in simulation_v3/.

pub mod archetype_basis;
pub mod predictive_cache;
pub mod tensor;

// Re-export MERA from simulation_v3 (canonical implementation)
pub use crate::simulation_v3::mera_network::{
    MeraLayer, MeraNetwork, MeraQuery, MeraScale, QueryType,
};

// Re-export FractalCache from simulation_v3 (canonical implementation)
pub use crate::simulation_v3::fractal_cache::{
    EvictionPolicy as EvictionStrategy, FractalCache, FractalCacheEntry, FractalCacheKey,
    FractalCacheStatistics as FractalCacheStats, FractalData,
};

pub use archetype_basis::{ArchetypeBasis, ArchetypicalInterferenceCache, CompressionStats};
pub use predictive_cache::{
    CacheKey, EntityTrajectory, PredictiveCache, PredictiveCacheConfig, PredictiveCacheStats,
};
pub use tensor::{Tensor, TensorData, TensorShape};
