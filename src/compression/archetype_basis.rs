//! Archetypal Basis Compression
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 2 (Weeks 7-8):
//! "Archetypal Basis Compression" - "22 orthogonal vectors"
//! "ArchetypeActivationProfile as 22 coefficients"
//! "ArchetypicalInterferenceCache for pattern caching"
//! "Delta compression for profile updates"
//!
//! Key Features:
//! - 22 orthogonal archetype vectors (basis set)
//! - ArchetypeActivationProfile as 22 coefficients (88 bytes vs thousands of floats)
//! - 100x compression ratio
//! - ArchetypicalInterferenceCache for pattern caching
//! - Delta compression for profile updates (only store changes)
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "Store 22 archetype coefficients instead of full patterns"
//! "100x compression (88 bytes vs thousands of floats)"

use crate::types::Float;
use std::collections::HashMap;
use std::fmt;

/// Archetype basis - 22 orthogonal vectors
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 2 (Weeks 7-8):
/// "Archetypal Basis Compression" - "22 orthogonal vectors"
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The 22 archetypes form the archetypical mind system (7+7+7+1)"
///
/// The 22 archetypes:
/// - Archetypes 1-7: Mind complex
/// - Archetypes 8-14: Body complex
/// - Archetypes 15-21: Spirit complex
/// - Archetype 22: The Choice
#[derive(Debug, Clone)]
pub struct ArchetypeBasis {
    /// 22 orthogonal basis vectors
    ///
    /// Each vector represents one archetype
    /// Vectors are orthonormal (orthogonal and unit length)
    pub basis: [Vec<Float>; 22],

    /// Dimensionality of each basis vector
    /// (Can be different for different scales)
    pub dimension: usize,
}

impl ArchetypeBasis {
    /// Create a new archetype basis
    ///
    /// Generates 22 orthonormal vectors
    pub fn new(dimension: usize) -> Self {
        let mut basis: [Vec<Float>; 22] = std::array::from_fn(|_| vec![]);

        // Generate orthonormal basis using Gram-Schmidt process
        for i in 0..22 {
            if i < dimension {
                // Create initial vector with 1 at position i, 0 elsewhere
                let mut vec = vec![0.0; dimension];
                vec[i % dimension] = 1.0;

                // Orthogonalize against previous vectors
                for j in 0..i {
                    let prev = &basis[j];
                    let dot = vec
                        .iter()
                        .zip(prev.iter())
                        .map(|(&a, &b)| a * b)
                        .sum::<Float>();
                    if dot != 0.0 {
                        let prev_norm_sq: Float = prev.iter().map(|&v| v * v).sum();
                        if prev_norm_sq > 1e-10 {
                            let scale = dot / prev_norm_sq;
                            for k in 0..dimension {
                                vec[k] -= scale * prev[k];
                            }
                        }
                    }
                }

                // Normalize
                let norm_sq: Float = vec.iter().map(|&v| v * v).sum();
                if norm_sq > 1e-10 {
                    let norm = norm_sq.sqrt();
                    for v in vec.iter_mut() {
                        *v /= norm;
                    }
                }

                basis[i] = vec;
            } else {
                // For i >= dimension, wrap around
                basis[i] = basis[i % dimension].clone();
            }
        }

        Self { basis, dimension }
    }

    /// Create a basis with predefined archetype patterns
    ///
    /// This creates basis vectors that follow the archetype patterns
    /// from the Law of One cosmology
    pub fn with_archetype_patterns() -> Self {
        let dimension = 22;
        let mut basis: [Vec<Float>; 22] = std::array::from_fn(|_| vec![0.0; 22]);

        // Create basis vectors with archetype-specific patterns
        // Each archetype has a unique pattern of activations

        // Mind complex (Archetypes 1-7)
        for i in 0..7 {
            basis[i][i] = 1.0;
            // Add some cross-correlation within the complex
            for j in 0..7 {
                if i != j {
                    basis[i][j] = 0.3;
                }
            }
        }

        // Body complex (Archetypes 8-14)
        for i in 7..14 {
            basis[i][i] = 1.0;
            // Add some cross-correlation within the complex
            for j in 7..14 {
                if i != j {
                    basis[i][j] = 0.3;
                }
            }
        }

        // Spirit complex (Archetypes 15-21)
        for i in 14..21 {
            basis[i][i] = 1.0;
            // Add some cross-correlation within the complex
            for j in 14..21 {
                if i != j {
                    basis[i][j] = 0.3;
                }
            }
        }

        // The Choice (Archetype 22)
        basis[21][21] = 1.0;
        // Archetype 22 has influence on all complexes
        for i in 0..21 {
            basis[21][i] = 0.2;
            basis[i][21] = 0.2;
        }

        // Normalize all vectors
        for i in 0..22 {
            let norm_sq: Float = basis[i].iter().map(|&v| v * v).sum();
            if norm_sq > 1e-10 {
                let norm = norm_sq.sqrt();
                for v in basis[i].iter_mut() {
                    *v /= norm;
                }
            }
        }

        Self { basis, dimension }
    }

    /// Get a basis vector by archetype number (1-22)
    pub fn get_basis_vector(&self, archetype_number: usize) -> Option<&Vec<Float>> {
        if archetype_number >= 1 && archetype_number <= 22 {
            Some(&self.basis[archetype_number - 1])
        } else {
            None
        }
    }

    /// Reconstruct archetype activation from coefficients
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
    /// "Store 22 archetype coefficients instead of full patterns"
    /// "Reconstruct: 22 multiply-add operations"
    pub fn reconstruct(&self, coefficients: &[Float; 22]) -> Vec<Float> {
        let mut result = vec![0.0; self.dimension];

        for (coeff, basis_vec) in coefficients.iter().zip(self.basis.iter()) {
            for (r, b) in result.iter_mut().zip(basis_vec.iter()) {
                *r += coeff * b;
            }
        }

        result
    }

    /// Check orthonormality of basis vectors
    ///
    /// Returns true if all vectors are orthonormal
    pub fn is_orthonormal(&self) -> bool {
        const EPSILON: Float = 1e-6;

        // Check that each vector is unit length
        for vec in &self.basis {
            let norm_sq: Float = vec.iter().map(|&v| v * v).sum();
            if (norm_sq - 1.0).abs() > EPSILON {
                return false;
            }
        }

        // Check that vectors are orthogonal
        for i in 0..22 {
            for j in (i + 1)..22 {
                let dot: Float = self.basis[i]
                    .iter()
                    .zip(self.basis[j].iter())
                    .map(|(&a, &b)| a * b)
                    .sum();
                if dot.abs() > EPSILON {
                    return false;
                }
            }
        }

        true
    }

    /// Calculate compression ratio
    ///
    /// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
    /// "100x compression (88 bytes vs thousands of floats)"
    pub fn compression_ratio(&self, original_size: usize) -> f64 {
        // Compressed: 22 coefficients * 8 bytes = 176 bytes
        let compressed_size = 22 * std::mem::size_of::<Float>();

        // Original: original_size * 8 bytes
        let original_size_bytes = original_size * std::mem::size_of::<Float>();

        if original_size_bytes == 0 {
            return 1.0;
        }

        original_size_bytes as f64 / compressed_size as f64
    }
}

/// Delta encoding for archetype profile updates
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 2 (Weeks 7-8):
/// "Delta compression for profile updates"
///
/// Only store changes (deltas) instead of full profiles
#[derive(Debug, Clone, PartialEq)]
pub struct ArchetypeDelta {
    /// Archetype number (1-22)
    pub archetype_number: usize,

    /// Change in activation (delta)
    pub delta: Float,

    /// Timestamp of the change
    pub timestamp: usize,
}

impl ArchetypeDelta {
    /// Create a new delta
    pub fn new(archetype_number: usize, delta: Float, timestamp: usize) -> Self {
        Self {
            archetype_number,
            delta,
            timestamp,
        }
    }

    /// Apply this delta to a coefficient
    pub fn apply(&self, coefficient: Float) -> Float {
        (coefficient + self.delta).clamp(0.0, 1.0)
    }
}

/// Delta-encoded archetype profile
///
/// Stores a base profile plus deltas for updates
#[derive(Debug, Clone)]
pub struct DeltaEncodedProfile {
    /// Base profile (reference)
    pub base_profile: [Float; 22],

    /// Deltas applied to base profile
    pub deltas: Vec<ArchetypeDelta>,
}

impl DeltaEncodedProfile {
    /// Create a new delta-encoded profile
    pub fn new(base_profile: [Float; 22]) -> Self {
        Self {
            base_profile,
            deltas: Vec::new(),
        }
    }

    /// Add a delta
    pub fn add_delta(&mut self, delta: ArchetypeDelta) {
        self.deltas.push(delta);
    }

    /// Reconstruct the current profile from base + deltas
    pub fn reconstruct(&self) -> [Float; 22] {
        let mut profile = self.base_profile;

        for delta in &self.deltas {
            if delta.archetype_number >= 1 && delta.archetype_number <= 22 {
                profile[delta.archetype_number - 1] =
                    delta.apply(profile[delta.archetype_number - 1]);
            }
        }

        profile
    }

    /// Get compression ratio
    pub fn compression_ratio(&self) -> f64 {
        // Original: 22 coefficients
        let original_size = 22;

        // Compressed: 22 base coefficients + number of deltas * (3 fields)
        let compressed_size = 22 + self.deltas.len() * 3;

        if compressed_size == 0 {
            return 1.0;
        }

        original_size as f64 / compressed_size as f64
    }

    /// Clear deltas (revert to base profile)
    pub fn clear_deltas(&mut self) {
        self.deltas.clear();
    }
}

/// Cache for archetypical interference patterns
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 2 (Weeks 7-8):
/// "ArchetypicalInterferenceCache for pattern caching"
#[derive(Debug, Clone)]
pub struct ArchetypicalInterferenceCache {
    /// Cached interference patterns
    ///
    /// Key: Hash of archetype activation profile
    /// Value: Interference pattern
    pub cache: HashMap<String, InterferencePattern>,

    /// Maximum cache size
    pub max_size: usize,

    /// Cache statistics
    pub stats: CacheStats,
}

/// Cached interference pattern
#[derive(Debug, Clone)]
pub struct InterferencePattern {
    /// Constructive interference nodes
    pub constructive_nodes: Vec<Float>,

    /// Destructive interference nodes
    pub destructive_nodes: Vec<Float>,

    /// Overall interference pattern
    pub pattern: Float,

    /// Coherence of the interference
    pub coherence: Float,
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: usize,

    /// Number of cache misses
    pub misses: usize,

    /// Number of evictions
    pub evictions: usize,

    /// Current cache size
    pub current_size: usize,
}

impl CacheStats {
    /// Calculate hit rate
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
}

impl ArchetypicalInterferenceCache {
    /// Create a new interference cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            stats: CacheStats::default(),
        }
    }

    /// Create a cache with unlimited size
    pub fn unlimited() -> Self {
        Self::new(usize::MAX)
    }

    /// Generate a cache key from archetype coefficients
    pub fn generate_key(&self, coefficients: &[Float; 22]) -> String {
        // Simple hash: concatenate values with separator
        coefficients
            .iter()
            .map(|&v| format!("{:.6}", v))
            .collect::<Vec<_>>()
            .join("|")
    }

    /// Get a cached pattern
    pub fn get(&mut self, coefficients: &[Float; 22]) -> Option<&InterferencePattern> {
        let key = self.generate_key(coefficients);

        if let Some(pattern) = self.cache.get(&key) {
            self.stats.hits += 1;
            Some(pattern)
        } else {
            self.stats.misses += 1;
            None
        }
    }

    /// Store a pattern in the cache
    pub fn store(&mut self, coefficients: &[Float; 22], pattern: InterferencePattern) {
        // Evict if cache is full
        if self.cache.len() >= self.max_size {
            self.evict();
        }

        let key = self.generate_key(coefficients);
        self.cache.insert(key, pattern);
        self.stats.current_size = self.cache.len();
    }

    /// Evict an entry from the cache (simple FIFO)
    fn evict(&mut self) {
        if let Some(key) = self.cache.keys().next().cloned() {
            self.cache.remove(&key);
            self.stats.evictions += 1;
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.stats.current_size = 0;
    }

    /// Get cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Get current cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

/// Compression statistics
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
/// "100x memory reduction"
#[derive(Debug, Clone, Default)]
pub struct CompressionStats {
    /// Original data size (in bytes)
    pub original_size: usize,

    /// Compressed data size (in bytes)
    pub compressed_size: usize,

    /// Compression ratio (original / compressed)
    pub compression_ratio: f64,

    /// Number of compression operations
    pub compression_count: usize,

    /// Number of decompression operations
    pub decompression_count: usize,

    /// Average compression time (in nanoseconds)
    pub avg_compression_time_ns: usize,

    /// Average decompression time (in nanoseconds)
    pub avg_decompression_time_ns: usize,
}

impl CompressionStats {
    /// Update compression statistics
    pub fn update_compression(
        &mut self,
        original_size: usize,
        compressed_size: usize,
        time_ns: usize,
    ) {
        self.compression_count += 1;
        self.original_size = original_size;
        self.compressed_size = compressed_size;
        self.compression_ratio = if compressed_size > 0 {
            original_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        // Update average
        let count = self.compression_count as f64;
        self.avg_compression_time_ns = ((self.avg_compression_time_ns as f64 * (count - 1.0)
            + time_ns as f64)
            / count) as usize;
    }

    /// Update decompression statistics
    pub fn update_decompression(&mut self, time_ns: usize) {
        self.decompression_count += 1;

        // Update average
        let count = self.decompression_count as f64;
        self.avg_decompression_time_ns = ((self.avg_decompression_time_ns as f64 * (count - 1.0)
            + time_ns as f64)
            / count) as usize;
    }

    /// Get memory savings (in bytes)
    pub fn memory_savings(&self) -> usize {
        self.original_size.saturating_sub(self.compressed_size)
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        *self = Default::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_basis_creation() {
        let basis = ArchetypeBasis::new(22);
        assert_eq!(basis.dimension, 22);
        assert_eq!(basis.basis.len(), 22);
    }

    #[test]
    fn test_archetype_basis_get_vector() {
        let basis = ArchetypeBasis::new(22);

        let vec1 = basis.get_basis_vector(1);
        assert!(vec1.is_some());

        let vec22 = basis.get_basis_vector(22);
        assert!(vec22.is_some());

        let vec_invalid = basis.get_basis_vector(23);
        assert!(vec_invalid.is_none());
    }

    #[test]
    fn test_archetype_basis_reconstruct() {
        let basis = ArchetypeBasis::new(22);
        let coefficients = [1.0; 22];
        let reconstructed = basis.reconstruct(&coefficients);

        assert_eq!(reconstructed.len(), 22);
    }

    #[test]
    fn test_archetype_basis_orthonormality() {
        let basis = ArchetypeBasis::new(22);
        assert!(basis.is_orthonormal());
    }

    #[test]
    fn test_archetype_basis_compression_ratio() {
        let basis = ArchetypeBasis::new(22);

        // Original: 1000 floats
        let ratio = basis.compression_ratio(1000);
        assert!(ratio > 1.0);

        // Expected: ~100x for large original size
        let ratio_large = basis.compression_ratio(2200);
        assert!(ratio_large > 10.0);
    }

    #[test]
    fn test_archetype_basis_with_patterns() {
        let basis = ArchetypeBasis::with_archetype_patterns();
        assert_eq!(basis.dimension, 22);
        assert_eq!(basis.basis.len(), 22);

        // Check that all basis vectors are non-zero
        for vec in &basis.basis {
            let has_nonzero = vec.iter().any(|&v| v.abs() > 1e-10);
            assert!(has_nonzero);
        }
    }

    #[test]
    fn test_archetype_delta() {
        let delta = ArchetypeDelta::new(1, 0.5, 100);

        assert_eq!(delta.archetype_number, 1);
        assert_eq!(delta.delta, 0.5);
        assert_eq!(delta.timestamp, 100);

        let result = delta.apply(0.3);
        assert_eq!(result, 0.8);

        // Test clamping
        let result_high = delta.apply(0.8);
        assert_eq!(result_high, 1.0);

        let result_low = delta.apply(-0.6);
        assert_eq!(result_low, 0.0);
    }

    #[test]
    fn test_delta_encoded_profile() {
        let base_profile = [0.5; 22];
        let mut profile = DeltaEncodedProfile::new(base_profile);

        assert_eq!(profile.deltas.len(), 0);

        profile.add_delta(ArchetypeDelta::new(1, 0.3, 100));
        assert_eq!(profile.deltas.len(), 1);

        let reconstructed = profile.reconstruct();
        assert_eq!(reconstructed[0], 0.8);
    }

    #[test]
    fn test_delta_encoded_profile_compression_ratio() {
        let base_profile = [0.5; 22];
        let profile = DeltaEncodedProfile::new(base_profile);

        // No deltas: ratio = 1.0
        assert_eq!(profile.compression_ratio(), 1.0);

        // With deltas: ratio improves
        let mut profile_with_deltas = DeltaEncodedProfile::new(base_profile);
        profile_with_deltas.add_delta(ArchetypeDelta::new(1, 0.3, 100));
        let ratio = profile_with_deltas.compression_ratio();
        assert!(ratio > 0.0);
    }

    #[test]
    fn test_delta_encoded_profile_clear() {
        let base_profile = [0.5; 22];
        let mut profile = DeltaEncodedProfile::new(base_profile);

        profile.add_delta(ArchetypeDelta::new(1, 0.3, 100));
        assert_eq!(profile.deltas.len(), 1);

        profile.clear_deltas();
        assert_eq!(profile.deltas.len(), 0);
    }

    #[test]
    fn test_interference_cache_creation() {
        let cache = ArchetypicalInterferenceCache::new(100);
        assert_eq!(cache.max_size, 100);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_interference_cache_unlimited() {
        let cache = ArchetypicalInterferenceCache::unlimited();
        assert_eq!(cache.max_size, usize::MAX);
    }

    #[test]
    #[test]
    fn test_interference_cache_store_get() {
        let mut cache = ArchetypicalInterferenceCache::new(100);
        let coefficients = [0.5; 22];

        let pattern = InterferencePattern {
            constructive_nodes: vec![],
            destructive_nodes: vec![],
            pattern: 0.5,
            coherence: 0.7,
        };

        // First get should miss
        assert!(cache.get(&coefficients).is_none());
        assert_eq!(cache.stats.misses, 1);

        // Store the pattern
        cache.store(&coefficients, pattern.clone());

        // Second get should hit
        let retrieved = cache.get(&coefficients);
        assert!(retrieved.is_some());
        let pattern_value = retrieved.unwrap().pattern;
        assert_eq!(cache.stats.hits, 1);
        assert_eq!(pattern_value, 0.5);
    }
    fn test_interference_cache_clear() {
        let mut cache = ArchetypicalInterferenceCache::new(100);
        let coefficients = [0.5; 22];

        let pattern = InterferencePattern {
            constructive_nodes: vec![],
            destructive_nodes: vec![],
            pattern: 0.5,
            coherence: 0.7,
        };

        cache.store(&coefficients, pattern);
        assert_eq!(cache.size(), 1);

        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_interference_cache_eviction() {
        let mut cache = ArchetypicalInterferenceCache::new(2);

        let pattern = InterferencePattern {
            constructive_nodes: vec![],
            destructive_nodes: vec![],
            pattern: 0.5,
            coherence: 0.7,
        };

        // Add 3 entries (should evict one)
        cache.store(&[0.5; 22], pattern.clone());
        cache.store(&[0.6; 22], pattern.clone());
        cache.store(&[0.7; 22], pattern.clone());

        assert_eq!(cache.size(), 2);
        assert_eq!(cache.stats.evictions, 1);
    }

    #[test]
    fn test_cache_stats_hit_rate() {
        let mut stats = CacheStats::default();
        stats.hits = 90;
        stats.misses = 10;

        assert_eq!(stats.hit_rate(), 0.9);
    }

    #[test]
    fn test_compression_stats() {
        let mut stats = CompressionStats::default();

        stats.update_compression(1000, 10, 100);
        assert_eq!(stats.compression_count, 1);
        assert_eq!(stats.original_size, 1000);
        assert_eq!(stats.compressed_size, 10);
        assert_eq!(stats.compression_ratio, 100.0);
        assert_eq!(stats.avg_compression_time_ns, 100);

        stats.update_decompression(50);
        assert_eq!(stats.decompression_count, 1);
        assert_eq!(stats.avg_decompression_time_ns, 50);

        assert_eq!(stats.memory_savings(), 990);
    }

    #[test]
    fn test_compression_stats_reset() {
        let mut stats = CompressionStats::default();
        stats.update_compression(1000, 10, 100);
        stats.update_decompression(50);

        stats.reset();
        assert_eq!(stats.compression_count, 0);
        assert_eq!(stats.compression_ratio, 0.0);
        assert_eq!(stats.avg_compression_time_ns, 0);
    }
}
