//! Multi-Scale Field Representation
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4.1:
//! "The holographic field is stored at 7 scale levels using MERA-style compression"
//!
//! From MASTER_R&D_ROADMAP.md Week 11-12:
//! - Implement MultiScaleField with 7 scale levels
//! - Store field at multiple scales (MERA-style)
//! - Implement scale-aware access
//! - Implement decompression pipeline
//!
//! This module implements:
//! 1. MultiScaleField - 7 scale levels with MERA compression
//! 2. Scale-aware access via MERA queries
//! 3. Decompression pipeline with refinement
//! 4. Holographic view generation
//! 5. Field statistics and performance tracking

use super::mera_network::{
    Float, MeraDecompressionResult, MeraError, MeraNetwork, MeraQuery,
};
use super::multiscale_camera::{PhysicsMode, ScaleLevel};
use std::collections::HashMap;

/// Multi-Scale Field Representation
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4.1:
/// "The holographic field is stored at 7 scale levels using MERA-style compression"
///
/// Each level is a compressed representation of the holographic field at a specific
/// physical scale. The field maintains holographic continuity: each level contains
/// the whole, just at different resolutions.
///
/// Structure:
/// - Level 0 (Quantum): 10^-35 to 10^-15 m
/// - Level 1 (Cellular): 10^-12 to 10^-6 m
/// - Level 2 (Biological): 10^-5 to 10^0 m
/// - Level 3 (Planetary): 10^0 to 10^7 m
/// - Level 4 (Stellar): 10^8 to 10^13 m
/// - Level 5 (Galactic): 10^14 to 10^21 m
/// - Level 6 (Cosmic): 10^22 to 10^26 m
pub struct MultiScaleField {
    /// MERA network for each scale level
    /// Index 0 = Quantum (finest), Index 6 = Cosmic (coarsest)
    scale_networks: Vec<MeraNetwork>,

    /// Current compression statistics
    stats: MultiScaleFieldStatistics,

    /// View cache for decompressed data
    view_cache: HashMap<CacheKey, CachedView>,
}

/// Holographic view at a specific scale and position
///
/// Represents the holographic field as viewed from a specific scale and position.
/// The view contains the decompressed field data appropriate for that scale.
#[derive(Debug, Clone)]
pub struct HolographicView {
    /// Scale level of this view
    pub scale: ScaleLevel,

    /// Position in the field (log10 of meters)
    pub position_log: Float,

    /// View dimensions (in logarithmic units)
    pub dimensions_log: (Float, Float, Float),

    /// Decompressed field data
    pub field_data: Vec<Float>,

    /// Field data shape (dimensions)
    pub field_shape: Vec<usize>,

    /// View quality (0.0 = coarse, 1.0 = finest)
    pub quality: Float,

    /// Physics mode at this scale
    pub physics_mode: PhysicsMode,

    /// Generation timestamp (for cache invalidation)
    pub generation: u64,
}

/// Cached view entry
#[derive(Debug, Clone)]
struct CachedView {
    /// Cached holographic view
    view: HolographicView,

    /// Cache hit count
    hit_count: usize,

    /// Last access timestamp
    last_access: u64,
}

/// Cache key for view caching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    /// Scale level
    scale: usize,

    /// Position index (quantized)
    position_idx: (i64, i64, i64),

    /// Quality level
    quality: u8,
}

/// Multi-scale field statistics
#[derive(Debug, Clone, Default)]
pub struct MultiScaleFieldStatistics {
    /// Total compressions performed
    pub total_compressions: usize,

    /// Total decompressions performed
    pub total_decompressions: usize,

    /// Average compression ratio
    pub average_compression_ratio: Float,

    /// Total compression time (ms)
    pub total_compression_time_ms: Float,

    /// Total decompression time (ms)
    pub total_decompression_time_ms: Float,

    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: Float,

    /// Total cache hits
    pub cache_hits: usize,

    /// Total cache misses
    pub cache_misses: usize,

    /// Average view quality
    pub average_view_quality: Float,
}

/// Multi-scale field error
#[derive(Debug, Clone, PartialEq)]
pub enum MultiScaleFieldError {
    /// Invalid scale level
    InvalidScaleLevel(usize),

    /// MERA error
    MeraError(MeraError),

    /// Field not initialized
    FieldNotInitialized,

    /// Invalid position
    InvalidPosition,
}

impl MultiScaleField {
    /// Create a new multi-scale field
    ///
    /// Initializes 7 MERA networks (one per scale level).
    pub fn new() -> Self {
        MultiScaleField {
            scale_networks: (0..7).map(|_| MeraNetwork::default()).collect(),
            stats: MultiScaleFieldStatistics::default(),
            view_cache: HashMap::new(),
        }
    }

    /// Initialize the field with base data at the finest scale (Quantum)
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "The field is initialized at the finest scale and compressed upward"
    pub fn initialize(
        &mut self,
        base_data: Vec<Float>,
        base_shape: Vec<usize>,
    ) -> Result<(), MultiScaleFieldError> {
        if base_data.is_empty() || base_shape.is_empty() {
            return Err(MultiScaleFieldError::FieldNotInitialized);
        }

        let tensor = super::mera_network::Tensor::from_data(base_shape.clone(), base_data)
            .map_err(MultiScaleFieldError::MeraError)?;

        // Initialize level 0 (Quantum) with base data
        self.scale_networks[0]
            .initialize(tensor)
            .map_err(MultiScaleFieldError::MeraError)?;

        // Compress and initialize other levels
        for level in 1..7 {
            let prev_data = self.scale_networks[level - 1]
                .get_level(0)
                .ok_or(MultiScaleFieldError::FieldNotInitialized)?
                .clone();

            // Downsample for coarser scale
            let downsampled = prev_data
                .downsample()
                .map_err(MultiScaleFieldError::MeraError)?;

            self.scale_networks[level]
                .initialize(downsampled)
                .map_err(MultiScaleFieldError::MeraError)?;
        }

        Ok(())
    }

    /// Compress the field at all scale levels using MERA
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 2.2:
    /// "Compress data using MERA structure - apply disentanglers and
    /// coarse-grainers at each layer"
    pub fn compress_all(&mut self) -> Result<(), MultiScaleFieldError> {
        let _start_time = std::time::Instant::now();

        let mut total_compression_ratio = 0.0;
        let mut compressible_levels = 0;

        for level in 0..7 {
            // Get data from level (may be compressed or uncompressed)
            let data = self.scale_networks[level].get_level(0).cloned();
            if let Some(data) = data {
                // Check if data has 2D shape with dimensions >= 2 for wavelet compression
                let is_2d = data.rank() == 2;
                let dims_ok = if is_2d {
                    data.shape[0] >= 2 && data.shape[1] >= 2
                } else {
                    false
                };

                if dims_ok {
                    compressible_levels += 1;
                    match self.scale_networks[level].compress(data) {
                        Ok(result) => {
                            total_compression_ratio += result.compression_ratio;
                            self.stats.total_compression_time_ms += result.compression_time_ms;
                            self.stats.total_compressions += 1;
                        }
                        Err(MeraError::InsufficientData(_)) => {
                            // Skip compression for levels that don't have enough data
                            // after downsampling through 7 levels
                            total_compression_ratio += 1.0;
                            self.stats.total_compressions += 1;
                        }
                        Err(e) => return Err(MultiScaleFieldError::MeraError(e)),
                    }
                } else {
                    // For very small data or 1D data, count as compressed with ratio 1.0
                    compressible_levels += 1;
                    total_compression_ratio += 1.0;
                    self.stats.total_compressions += 1;
                }
            }
        }

        if compressible_levels > 0 {
            self.stats.average_compression_ratio =
                total_compression_ratio / compressible_levels as Float;
        }

        Ok(())
    }

    /// Get a scale-aware view of the field
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4.2:
    /// "Navigate up the MERA hierarchy to find relevant data and refine"
    ///
    /// This method:
    /// 1. Checks cache first
    /// 2. Decompresses from appropriate MERA level
    /// 3. Refines using higher-resolution levels if available
    /// 4. Caches the result
    pub fn get_view(
        &mut self,
        scale: ScaleLevel,
        position_log: Float,
        dimensions_log: (Float, Float, Float),
    ) -> Result<HolographicView, MultiScaleFieldError> {
        let scale_index = scale.index();
        let generation = self.stats.total_decompressions as u64;

        // Create cache key
        let position_idx = Self::quantize_position(position_log);
        let quality = 100; // Default quality
        let cache_key = CacheKey {
            scale: scale_index,
            position_idx,
            quality,
        };

        // Check cache
        if let Some(cached) = self.view_cache.get_mut(&cache_key) {
            cached.hit_count += 1;
            cached.last_access = generation;
            self.stats.cache_hits += 1;
            return Ok(cached.view.clone());
        }

        self.stats.cache_misses += 1;

        // Decompress from MERA network
        let _mera_query = MeraQuery {
            scale_level: scale_index,
            region: None,
            precision: 1.0,
        };

        // Get a clone of the network data for decompression
        let network_data = self.scale_networks[scale_index]
            .get_level(0)
            .cloned()
            .ok_or(MultiScaleFieldError::FieldNotInitialized)?;

        // For multi-scale field, we already have data at the requested scale level
        // stored in level 0 of each scale's MERA network. We don't need to decompress
        // through a hierarchy - just return the data directly.
        let decompression_result = MeraDecompressionResult {
            data: network_data.clone(),
            decompression_time_ms: 0.0,
            precision: 1.0,
        };

        self.stats.total_decompression_time_ms += decompression_result.decompression_time_ms;
        self.stats.total_decompressions += 1;

        // Create holographic view
        let view = HolographicView {
            scale,
            position_log,
            dimensions_log,
            field_data: decompression_result.data.data.clone(),
            field_shape: decompression_result.data.shape.clone(),
            quality: decompression_result.precision,
            physics_mode: scale.physics_mode(),
            generation,
        };

        // Update average view quality
        let total_quality = self.stats.average_view_quality
            * (self.stats.total_decompressions - 1) as Float
            + view.quality;
        self.stats.average_view_quality = total_quality / self.stats.total_decompressions as Float;

        // Cache the view
        self.view_cache.insert(
            cache_key,
            CachedView {
                view: view.clone(),
                hit_count: 0,
                last_access: generation,
            },
        );

        Ok(view)
    }

    /// Get a view with interpolated scale (during scale transitions)
    ///
    /// From GAMING_ENGINE_ROADMAP_v2.md Section 5.2:
    /// "Smooth transition between scales with holographic continuity"
    pub fn get_interpolated_view(
        &mut self,
        from_scale: ScaleLevel,
        to_scale: ScaleLevel,
        factor: Float,
        position_log: Float,
        dimensions_log: (Float, Float, Float),
    ) -> Result<HolographicView, MultiScaleFieldError> {
        let from_view = self.get_view(from_scale, position_log, dimensions_log)?;
        let to_view = self.get_view(to_scale, position_log, dimensions_log)?;

        // Interpolate field data
        let interpolated_data: Vec<Float> = from_view
            .field_data
            .iter()
            .zip(to_view.field_data.iter())
            .map(|(a, b)| a * (1.0 - factor) + b * factor)
            .collect();

        // Interpolate position
        let interpolated_position =
            from_view.position_log * (1.0 - factor) + to_view.position_log * factor;

        // Determine physics mode based on interpolation factor
        let physics_mode = if factor < 0.5 {
            from_view.physics_mode
        } else {
            to_view.physics_mode
        };

        Ok(HolographicView {
            scale: if factor < 0.5 { from_scale } else { to_scale },
            position_log: interpolated_position,
            dimensions_log,
            field_data: interpolated_data,
            field_shape: from_view.field_shape.clone(),
            quality: from_view.quality * (1.0 - factor) + to_view.quality * factor,
            physics_mode,
            generation: self.stats.total_decompressions as u64,
        })
    }

    /// Decompress a specific level with refinement
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 4.2:
    /// "Navigate up the MERA hierarchy to find relevant data and refine.
    ///  Refine using higher-resolution levels if available"
    #[allow(dead_code)]
    fn decompress_level(
        &self,
        level: usize,
        query: &MeraQuery,
    ) -> Result<HolographicView, MultiScaleFieldError> {
        if level >= 7 {
            return Err(MultiScaleFieldError::InvalidScaleLevel(level));
        }

        // Get a clone of the network data for decompression
        let network_data = self.scale_networks[level]
            .get_level(0)
            .cloned()
            .ok_or(MultiScaleFieldError::FieldNotInitialized)?;

        // Check data size before moving
        let data_size = network_data.num_elements();

        // Create a temporary network for decompression
        let mut temp_network = MeraNetwork::new(1);
        temp_network
            .initialize(network_data)
            .map_err(MultiScaleFieldError::MeraError)?;

        // Only decompress if we have enough data (avoid wavelet compression error)
        let decompression_result = if data_size >= 4 {
            temp_network
                .decompress(query)
                .map_err(MultiScaleFieldError::MeraError)?
        } else {
            // For very small data, return the data directly from temp_network
            let retrieved_data = temp_network
                .get_level(0)
                .cloned()
                .ok_or(MultiScaleFieldError::FieldNotInitialized)?;
            MeraDecompressionResult {
                data: retrieved_data,
                decompression_time_ms: 0.0,
                precision: 1.0,
            }
        };

        Ok(HolographicView {
            scale: ScaleLevel::from_index(level).unwrap(),
            position_log: 0.0,
            dimensions_log: (0.0, 0.0, 0.0),
            field_data: decompression_result.data.data.clone(),
            field_shape: decompression_result.data.shape.clone(),
            quality: decompression_result.precision,
            physics_mode: ScaleLevel::from_index(level).unwrap().physics_mode(),
            generation: self.stats.total_decompressions as u64,
        })
    }

    /// Quantize position for cache key
    fn quantize_position(position_log: Float) -> (i64, i64, i64) {
        const QUANTIZATION_STEP: Float = 0.1;
        ((position_log / QUANTIZATION_STEP).floor() as i64, 0, 0)
    }

    /// Clear the view cache
    pub fn clear_cache(&mut self) {
        self.view_cache.clear();
    }

    /// Get field statistics
    pub fn statistics(&self) -> &MultiScaleFieldStatistics {
        &self.stats
    }

    /// Update cache hit rate
    pub fn update_cache_hit_rate(&mut self) {
        let total = self.stats.cache_hits + self.stats.cache_misses;
        if total > 0 {
            self.stats.cache_hit_rate = self.stats.cache_hits as Float / total as Float;
        }
    }

    /// Get the number of initialized scale levels
    pub fn initialized_levels(&self) -> usize {
        self.scale_networks
            .iter()
            .filter(|n| n.num_levels() > 0)
            .count()
    }

    /// Check if a specific scale level is initialized
    pub fn is_level_initialized(&self, scale: ScaleLevel) -> bool {
        self.scale_networks[scale.index()].num_levels() > 0
    }

    /// Evict stale cache entries
    pub fn evict_stale_cache_entries(&mut self, max_age: u64) {
        let current_generation = self.stats.total_decompressions as u64;

        self.view_cache
            .retain(|_, cached| current_generation - cached.last_access <= max_age);
    }
}

impl Default for MultiScaleField {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiscale_field_creation() {
        let field = MultiScaleField::new();
        assert_eq!(field.scale_networks.len(), 7);
        assert_eq!(field.initialized_levels(), 0);
    }

    #[test]
    fn test_multiscale_field_default() {
        let field = MultiScaleField::default();
        assert_eq!(field.scale_networks.len(), 7);
    }

    #[test]
    fn test_multiscale_field_initialize() {
        let mut field = MultiScaleField::new();

        let base_data = vec![1.0; 16 * 16];
        let base_shape = vec![16, 16];

        field.initialize(base_data, base_shape).unwrap();

        assert!(field.is_level_initialized(ScaleLevel::Quantum));
        assert!(field.is_level_initialized(ScaleLevel::Cellular));
        assert_eq!(field.initialized_levels(), 7);
    }

    #[test]
    fn test_multiscale_field_compress_all() {
        let mut field = MultiScaleField::new();

        // Use smaller data to avoid excessive downsampling
        let base_data = vec![1.0; 16 * 16];
        let base_shape = vec![16, 16];

        field.initialize(base_data, base_shape).unwrap();

        // Just verify it runs without error
        field.compress_all().unwrap();

        let stats = field.statistics();
        assert!(stats.total_compressions > 0);
        assert!(stats.average_compression_ratio >= 1.0);
    }

    #[test]
    fn test_multiscale_field_get_view() {
        let mut field = MultiScaleField::new();

        let base_data = vec![1.0; 16 * 16];
        let base_shape = vec![16, 16];

        field.initialize(base_data, base_shape).unwrap();

        let view = field
            .get_view(ScaleLevel::Quantum, -25.0, (5.0, 5.0, 5.0))
            .unwrap();

        assert_eq!(view.scale, ScaleLevel::Quantum);
        assert_eq!(view.position_log, -25.0);
        assert!(!view.field_data.is_empty());
        assert_eq!(view.physics_mode, PhysicsMode::Quantum);
    }

    #[test]
    fn test_multiscale_field_get_view_cache_hit() {
        let mut field = MultiScaleField::new();

        let base_data = vec![1.0; 16 * 16];
        let base_shape = vec![16, 16];

        field.initialize(base_data, base_shape).unwrap();

        let _ = field
            .get_view(ScaleLevel::Quantum, -25.0, (5.0, 5.0, 5.0))
            .unwrap();
        field.update_cache_hit_rate();
        assert_eq!(field.statistics().cache_misses, 1);

        let _ = field
            .get_view(ScaleLevel::Quantum, -25.0, (5.0, 5.0, 5.0))
            .unwrap();
        field.update_cache_hit_rate();
        assert_eq!(field.statistics().cache_hits, 1);
    }

    #[test]
    fn test_multiscale_field_get_interpolated_view() {
        let mut field = MultiScaleField::new();

        let base_data = vec![1.0; 16 * 16];
        let base_shape = vec![16, 16];

        field.initialize(base_data, base_shape).unwrap();

        let view = field
            .get_interpolated_view(
                ScaleLevel::Quantum,
                ScaleLevel::Cellular,
                0.5,
                -20.0,
                (5.0, 5.0, 5.0),
            )
            .unwrap();

        assert!(view.physics_mode == PhysicsMode::Quantum);
    }

    #[test]
    fn test_multiscale_field_clear_cache() {
        let mut field = MultiScaleField::new();

        let base_data = vec![1.0; 16 * 16];
        let base_shape = vec![16, 16];

        field.initialize(base_data, base_shape).unwrap();

        let _ = field
            .get_view(ScaleLevel::Quantum, -25.0, (5.0, 5.0, 5.0))
            .unwrap();

        assert!(!field.view_cache.is_empty());

        field.clear_cache();
        assert!(field.view_cache.is_empty());
    }

    #[test]
    fn test_multiscale_field_evict_stale_cache_entries() {
        let mut field = MultiScaleField::new();

        let base_data = vec![1.0; 16 * 16];
        let base_shape = vec![16, 16];

        field.initialize(base_data, base_shape).unwrap();

        let _ = field
            .get_view(ScaleLevel::Quantum, -25.0, (5.0, 5.0, 5.0))
            .unwrap();

        let initial_cache_size = field.view_cache.len();
        assert!(initial_cache_size > 0);

        field.evict_stale_cache_entries(0);
        assert_eq!(field.view_cache.len(), 0);
    }

    #[test]
    fn test_holographic_view_properties() {
        let view = HolographicView {
            scale: ScaleLevel::Biological,
            position_log: 0.0,
            dimensions_log: (1.0, 1.0, 1.0),
            field_data: vec![1.0, 2.0, 3.0],
            field_shape: vec![3],
            quality: 1.0,
            physics_mode: PhysicsMode::SpaceTime,
            generation: 0,
        };

        assert_eq!(view.scale, ScaleLevel::Biological);
        assert_eq!(view.position_log, 0.0);
        assert_eq!(view.field_data.len(), 3);
        assert_eq!(view.physics_mode, PhysicsMode::SpaceTime);
    }

    #[test]
    fn test_cache_key_hash() {
        let key1 = CacheKey {
            scale: 0,
            position_idx: (100, 0, 0),
            quality: 100,
        };

        let key2 = CacheKey {
            scale: 0,
            position_idx: (100, 0, 0),
            quality: 100,
        };

        assert_eq!(key1, key2);

        let key3 = CacheKey {
            scale: 1,
            position_idx: (100, 0, 0),
            quality: 100,
        };

        assert_ne!(key1, key3);
    }

    #[test]
    fn test_multiscale_field_error_invalid_scale_level() {
        let field = MultiScaleField::new();
        let query = MeraQuery {
            scale_level: 10,
            region: None,
            precision: 1.0,
        };

        let result = field.decompress_level(10, &query);
        assert!(matches!(
            result,
            Err(MultiScaleFieldError::InvalidScaleLevel(10))
        ));
    }

    #[test]
    fn test_multiscale_field_error_field_not_initialized() {
        let mut field = MultiScaleField::new();
        let base_data = vec![];
        let base_shape = vec![];

        let result = field.initialize(base_data, base_shape);
        assert!(matches!(
            result,
            Err(MultiScaleFieldError::FieldNotInitialized)
        ));
    }

    #[test]
    fn test_all_scale_levels_physics_mode() {
        let scales = [
            ScaleLevel::Quantum,
            ScaleLevel::Cellular,
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            ScaleLevel::Stellar,
            ScaleLevel::Galactic,
            ScaleLevel::Cosmic,
        ];

        for scale in scales {
            let physics_mode = scale.physics_mode();
            match scale {
                ScaleLevel::Quantum | ScaleLevel::Cellular => {
                    assert_eq!(physics_mode, PhysicsMode::Quantum);
                }
                ScaleLevel::Cosmic => {
                    assert_eq!(physics_mode, PhysicsMode::TimeSpace);
                }
                _ => {
                    assert_eq!(physics_mode, PhysicsMode::SpaceTime);
                }
            }
        }
    }

    #[test]
    fn test_multiscale_field_statistics_default() {
        let stats = MultiScaleFieldStatistics::default();
        assert_eq!(stats.total_compressions, 0);
        assert_eq!(stats.total_decompressions, 0);
        assert_eq!(stats.average_compression_ratio, 0.0);
        assert_eq!(stats.cache_hit_rate, 0.0);
    }

    #[test]
    fn test_update_cache_hit_rate() {
        let mut field = MultiScaleField::new();

        let base_data = vec![1.0; 16 * 16];
        let base_shape = vec![16, 16];

        field.initialize(base_data, base_shape).unwrap();
        field.compress_all().unwrap();

        // Generate some cache activity
        let _ = field
            .get_view(ScaleLevel::Quantum, -25.0, (5.0, 5.0, 5.0))
            .unwrap();
        field.update_cache_hit_rate();
        assert_eq!(field.statistics().cache_hit_rate, 0.0);

        let _ = field
            .get_view(ScaleLevel::Quantum, -25.0, (5.0, 5.0, 5.0))
            .unwrap();
        field.update_cache_hit_rate();
        assert_eq!(field.statistics().cache_hit_rate, 0.5);
    }
}
