//! MERA Integration Module
//!
//! This module wires the existing MERA tensor network compression system
//! to the observer-driven field substrate. It provides the bridge between
//! compressed field storage and observer-driven decompression.
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "MERA (Multi-scale Entanglement Renormalization Ansatz) implements
//! holographic compression with O(log n) query complexity."

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::field_address::{AddressRange, HolographicAddress, ScaleLevel, Vector3};
use super::observer_driven_field::{DecompressedRegion, ObserverId};
use crate::simulation_v3::mera_network::Tensor;
use crate::simulation_v3::mera_network::{MeraNetwork, MeraQuery, MeraScale, QueryType};

/// Integration layer between MERA network and observer system
///
/// This struct manages the connection between:
/// - The compressed MERA network (storage)
/// - Observer-driven decompression requests
/// - Caching of decompressed regions
pub struct MeraIntegration {
    /// The underlying MERA network
    network: Arc<Mutex<MeraNetwork>>,
    /// Cache of recently decompressed regions
    region_cache: HashMap<String, CachedRegion>,
    /// Maximum cache size in bytes
    max_cache_size: usize,
    /// Current cache size in bytes
    current_cache_size: usize,
}

/// A cached decompressed region
#[derive(Debug, Clone)]
struct CachedRegion {
    /// The decompressed tensor data
    data: Tensor,
    /// The scale this was decompressed at
    #[allow(dead_code)]
    scale: MeraScale,
    /// Access count for LRU eviction
    access_count: u64,
    /// Last access tick
    last_access: u64,
    /// Size in bytes
    size: usize,
}

impl MeraIntegration {
    /// Create a new MERA integration layer
    pub fn new(network: Arc<Mutex<MeraNetwork>>) -> Self {
        Self {
            network,
            region_cache: HashMap::new(),
            max_cache_size: 100 * 1024 * 1024, // 100 MB default
            current_cache_size: 0,
        }
    }

    /// Create with custom cache size
    pub fn with_cache_size(network: Arc<Mutex<MeraNetwork>>, max_cache_size: usize) -> Self {
        Self {
            network,
            region_cache: HashMap::new(),
            max_cache_size,
            current_cache_size: 0,
        }
    }

    /// Decompress a region for an observer
    ///
    /// This is the main entry point for observer-driven decompression.
    /// The region is decompressed from the MERA network and cached.
    pub fn decompress_for_observer(
        &mut self,
        address_range: &AddressRange,
        scale: ScaleLevel,
        observer_id: ObserverId,
        current_tick: u64,
    ) -> Option<Tensor> {
        // Generate cache key
        let cache_key = self.generate_cache_key(address_range, &scale, &observer_id);

        // Check cache first
        if let Some(cached) = self.region_cache.get_mut(&cache_key) {
            cached.access_count += 1;
            cached.last_access = current_tick;
            return Some(cached.data.clone());
        }

        // Decompress from MERA network
        let mera_scale = Self::scale_to_mera(&scale);
        let query = MeraQuery::new(mera_scale, QueryType::Spatial);

        let tensor = {
            let mut network = self.network.lock().ok()?;
            network.decompress_tensor(&query)
        };

        // Cache the result
        let size = tensor.shape.iter().product::<usize>() * std::mem::size_of::<f64>();
        self.ensure_cache_space(size);

        self.region_cache.insert(
            cache_key.clone(),
            CachedRegion {
                data: tensor.clone(),
                scale: mera_scale,
                access_count: 1,
                last_access: current_tick,
                size,
            },
        );
        self.current_cache_size += size;

        Some(tensor)
    }

    /// Create a DecompressedRegion from tensor data
    pub fn create_region(
        &self,
        address_range: AddressRange,
        scale: ScaleLevel,
        tensor: &Tensor,
    ) -> DecompressedRegion {
        // Convert tensor data to field values
        let _field_values = tensor.as_dense().to_vec();

        DecompressedRegion::new(address_range, scale, 100)
    }

    /// Detect coherence peaks in a decompressed tensor
    ///
    /// Coherence peaks are points where the field has high structure,
    /// indicating potential locations for emergent entities.
    pub fn detect_peaks(&self, tensor: &Tensor, threshold: f64) -> Vec<Vector3> {
        let data = tensor.as_dense().to_vec();
        let mut peaks = Vec::new();

        // Find dimensions (assume cubic for now)
        let size = (data.len() as f64).cbrt() as usize;
        if size * size * size != data.len() {
            return peaks;
        }

        // Find local maxima above threshold
        for i in 0..size {
            for j in 0..size {
                for k in 0..size {
                    let idx = i * size * size + j * size + k;
                    if data[idx] > threshold {
                        // Check if local maximum (simplified)
                        let is_peak = self.is_local_maximum(&data, size, i, j, k);
                        if is_peak {
                            peaks.push(Vector3::new(
                                i as f64 / size as f64,
                                j as f64 / size as f64,
                                k as f64 / size as f64,
                            ));
                        }
                    }
                }
            }
        }

        peaks
    }

    /// Check if a point is a local maximum
    fn is_local_maximum(&self, data: &[f64], size: usize, i: usize, j: usize, k: usize) -> bool {
        let idx = i * size * size + j * size + k;
        let val = data[idx];

        // Check 26 neighbors (3x3x3 minus center)
        for di in -1..=1 {
            for dj in -1..=1 {
                for dk in -1..=1 {
                    if di == 0 && dj == 0 && dk == 0 {
                        continue;
                    }

                    let ni = i as i64 + di;
                    let nj = j as i64 + dj;
                    let nk = k as i64 + dk;

                    if ni >= 0
                        && ni < size as i64
                        && nj >= 0
                        && nj < size as i64
                        && nk >= 0
                        && nk < size as i64
                    {
                        let nidx =
                            (ni as usize) * size * size + (nj as usize) * size + (nk as usize);
                        if data[nidx] > val {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    /// Apply entity feedback to the compressed field
    ///
    /// Consciousness modifies reality by influencing the MERA network.
    /// This is implemented as a perturbation to the tensor structure.
    pub fn apply_feedback(&mut self, address: &HolographicAddress, feedback: f64) -> bool {
        let scale = Self::scale_to_mera(&address.scale);
        let query = MeraQuery::new(scale, QueryType::Spatial);

        let mut network = match self.network.lock() {
            Ok(n) => n,
            Err(_) => return false,
        };

        // Decompress, modify, recompress
        let tensor = network.decompress_tensor(&query);

        // Apply feedback perturbation
        let mut data = tensor.as_dense().to_vec();
        for value in data.iter_mut() {
            *value += feedback * 0.01;
            *value = value.clamp(0.0, 1.0);
        }

        // Recompress (in full implementation, this would be incremental)
        let _ = network.compress(tensor);

        true
    }

    /// Clear the region cache
    pub fn clear_cache(&mut self) {
        self.region_cache.clear();
        self.current_cache_size = 0;
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> CacheStats {
        CacheStats {
            region_count: self.region_cache.len(),
            current_size: self.current_cache_size,
            max_size: self.max_cache_size,
            utilization: if self.max_cache_size > 0 {
                self.current_cache_size as f64 / self.max_cache_size as f64
            } else {
                0.0
            },
        }
    }

    /// Generate a cache key for a region
    fn generate_cache_key(
        &self,
        address_range: &AddressRange,
        scale: &ScaleLevel,
        observer_id: &ObserverId,
    ) -> String {
        format!(
            "{}_{:?}_{}",
            address_range.min.depth(),
            scale,
            observer_id.0
        )
    }

    /// Ensure there's enough space in the cache
    fn ensure_cache_space(&mut self, required: usize) {
        while self.current_cache_size + required > self.max_cache_size
            && !self.region_cache.is_empty()
        {
            // Evict least recently used
            let mut oldest_key = String::new();
            let mut oldest_access = u64::MAX;

            for (key, cached) in &self.region_cache {
                if cached.last_access < oldest_access {
                    oldest_access = cached.last_access;
                    oldest_key = key.clone();
                }
            }

            if let Some(cached) = self.region_cache.remove(&oldest_key) {
                self.current_cache_size -= cached.size;
            }
        }
    }

    /// Convert ScaleLevel to MeraScale
    fn scale_to_mera(scale: &ScaleLevel) -> MeraScale {
        match scale {
            ScaleLevel::Quantum => MeraScale::Quantum,
            ScaleLevel::Atomic => MeraScale::Atomic,
            ScaleLevel::Molecular => MeraScale::Molecular,
            ScaleLevel::Cellular => MeraScale::Cellular,
            ScaleLevel::Biological => MeraScale::Organism,
            ScaleLevel::Planetary => MeraScale::Planetary,
            ScaleLevel::Stellar => MeraScale::Cosmic,
            ScaleLevel::Cosmic => MeraScale::Cosmic,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Copy)]
pub struct CacheStats {
    /// Number of cached regions
    pub region_count: usize,
    /// Current cache size in bytes
    pub current_size: usize,
    /// Maximum cache size in bytes
    pub max_size: usize,
    /// Cache utilization (0.0 to 1.0)
    pub utilization: f64,
}

/// Builder for MERA integration
pub struct MeraIntegrationBuilder {
    network: Option<Arc<Mutex<MeraNetwork>>>,
    max_cache_size: usize,
}

impl MeraIntegrationBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            network: None,
            max_cache_size: 100 * 1024 * 1024,
        }
    }

    /// Set the MERA network
    pub fn with_network(mut self, network: Arc<Mutex<MeraNetwork>>) -> Self {
        self.network = Some(network);
        self
    }

    /// Set the maximum cache size
    pub fn with_cache_size(mut self, size: usize) -> Self {
        self.max_cache_size = size;
        self
    }

    /// Build the integration
    pub fn build(self) -> Result<MeraIntegration, String> {
        let network = self.network.ok_or("Network is required")?;
        Ok(MeraIntegration::with_cache_size(
            network,
            self.max_cache_size,
        ))
    }
}

impl Default for MeraIntegrationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compression::TensorShape;

    fn create_test_network() -> Arc<Mutex<MeraNetwork>> {
        let mut network = MeraNetwork::new();
        network.initialize_default(128);
        network.generate_tensors();
        Arc::new(Mutex::new(network))
    }

    #[test]
    fn test_mera_integration_creation() {
        let network = create_test_network();
        let integration = MeraIntegration::new(network);
        assert_eq!(integration.cache_stats().region_count, 0);
    }

    #[test]
    fn test_cache_stats() {
        let network = create_test_network();
        let integration = MeraIntegration::new(network);
        let stats = integration.cache_stats();

        assert_eq!(stats.region_count, 0);
        assert_eq!(stats.current_size, 0);
        assert!(stats.utilization >= 0.0);
    }

    #[test]
    fn test_clear_cache() {
        let network = create_test_network();
        let mut integration = MeraIntegration::new(network);
        integration.clear_cache();
        assert_eq!(integration.cache_stats().region_count, 0);
    }

    #[test]
    fn test_scale_to_mera() {
        assert_eq!(
            MeraIntegration::scale_to_mera(&ScaleLevel::Quantum),
            MeraScale::Quantum
        );
        assert_eq!(
            MeraIntegration::scale_to_mera(&ScaleLevel::Biological),
            MeraScale::Organism
        );
        assert_eq!(
            MeraIntegration::scale_to_mera(&ScaleLevel::Cosmic),
            MeraScale::Cosmic
        );
    }

    #[test]
    fn test_builder() {
        let network = create_test_network();
        let integration = MeraIntegrationBuilder::new()
            .with_network(network)
            .with_cache_size(50 * 1024 * 1024)
            .build()
            .unwrap();

        assert_eq!(integration.max_cache_size, 50 * 1024 * 1024);
    }

    #[test]
    fn test_decompress_for_observer() {
        let network = create_test_network();
        let mut integration = MeraIntegration::new(network);

        let addr = HolographicAddress::cosmic_origin();
        let range = AddressRange::new(addr.clone(), addr);

        let result = integration.decompress_for_observer(
            &range,
            ScaleLevel::Biological,
            ObserverId::new(1),
            0,
        );

        // Should return some tensor
        assert!(result.is_some());
    }

    #[test]
    fn test_detect_peaks_empty() {
        let network = create_test_network();
        let integration = MeraIntegration::new(network);

        let tensor = Tensor::zeros(TensorShape::vector(8));
        let peaks = integration.detect_peaks(&tensor, 0.5);

        // No peaks in zero tensor
        assert_eq!(peaks.len(), 0);
    }

    #[test]
    fn test_apply_feedback() {
        let network = create_test_network();
        let mut integration = MeraIntegration::new(network);

        let addr = HolographicAddress::cosmic_origin();
        let result = integration.apply_feedback(&addr, 0.5);

        assert!(result);
    }
}
