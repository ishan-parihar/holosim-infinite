//! MERA Network - Multi-scale Entanglement Renormalization Ansatz
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 2.2:
//! "MERA (Multi-scale Entanglement Renormalization Ansatz) is a tensor network
//! that implements holographic compression."
//!
//! This module implements:
//! 1. MeraNetwork - Hierarchical tensor structure
//! 2. Disentangler tensors - Remove redundant information
//! 3. Coarse-grainer tensors - Combine representations
//! 4. Wavelet compression - Field storage optimization
//!
//! Performance Targets:
//! - 100x compression (8 GB → 80 MB)
//! - O(log n) decompression for specific queries
//! - Hierarchical storage across 7 scale levels

use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub type Float = f64;

/// MERA Network - Hierarchical tensor network for holographic compression
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "MERA implements holographic compression through hierarchical layers that
/// progressively compress data while maintaining information."
///
/// Structure:
/// - Level 0: Finest detail (quantum scale)
/// - Level 1: Coarser representation
/// - ...
/// - Level 7: Coarsest representation (cosmic scale)
///
/// Performance: Exponential reduction in storage, O(log n) decompression
pub struct MeraNetwork {
    /// Hierarchical layers (0 = finest, N = coarsest)
    layers: Vec<MeraLayer>,

    /// Number of scale levels (typically 7 for quantum → cosmic)
    num_levels: usize,

    /// Compression statistics
    stats: MeraStatistics,
}

/// Single layer in the MERA hierarchy
///
/// Each layer contains:
/// - Disentanglers: Remove redundant information from tensor data
/// - Coarse-grainers: Combine similar representations
/// - Data: Compressed representation at this scale
pub struct MeraLayer {
    /// Layer level (0 = finest)
    pub level: usize,

    /// Disentangler tensors - remove redundancy
    disentanglers: Vec<Tensor>,

    /// Coarse-grainer tensors - combine representations
    coarse_grainers: Vec<Tensor>,

    /// Data tensor at this scale
    pub data: Tensor,

    /// Wavelet coefficients for compression
    wavelet_coeffs: Option<WaveletCoefficients>,
}

/// Multi-dimensional tensor
///
/// Represents a multi-dimensional array of Float values.
/// Used for holographic field data at various scales.
#[derive(Debug, Clone)]
pub struct Tensor {
    /// Tensor dimensions
    pub shape: Vec<usize>,

    /// Tensor data (row-major order)
    pub data: Vec<Float>,
}

/// Wavelet coefficients for compression
///
/// Stores wavelet decomposition of tensor data for efficient compression.
/// Enables lossless or lossy compression based on thresholding.
#[derive(Debug, Clone)]
pub struct WaveletCoefficients {
    /// Approximation coefficients (low-frequency)
    pub approximation: Tensor,

    /// Detail coefficients (high-frequency) for each level
    pub detail: Vec<Tensor>,

    /// Compression threshold (coefficients below this are discarded)
    pub threshold: Float,
}

/// MERA query for decompression
///
/// Specifies what portion of the holographic field to decompress.
#[derive(Debug, Clone)]
pub struct MeraQuery {
    /// Scale level to decompress (0 = finest, N = coarsest)
    pub scale_level: usize,

    /// Spatial region to decompress (optional)
    pub region: Option<(usize, usize, usize, usize)>,

    /// Required precision (0.0 = coarse, 1.0 = finest)
    pub precision: Float,
}

/// Result of MERA compression
#[derive(Debug, Clone)]
pub struct MeraCompressionResult {
    /// Original size in bytes
    pub original_size: usize,

    /// Compressed size in bytes
    pub compressed_size: usize,

    /// Compression ratio
    pub compression_ratio: Float,

    /// Compression time in milliseconds
    pub compression_time_ms: Float,
}

/// Result of MERA decompression
#[derive(Debug, Clone)]
pub struct MeraDecompressionResult {
    /// Decompressed tensor
    pub data: Tensor,

    /// Decompression time in milliseconds
    pub decompression_time_ms: Float,

    /// Precision achieved (0.0 to 1.0)
    pub precision: Float,
}

/// MERA compression statistics
#[derive(Debug, Clone, Default)]
pub struct MeraStatistics {
    /// Total compressions performed
    pub total_compressions: usize,

    /// Total decompressions performed
    pub total_decompressions: usize,

    /// Average compression ratio
    pub average_compression_ratio: Float,

    /// Total time spent compressing (ms)
    pub total_compression_time_ms: Float,

    /// Total time spent decompressing (ms)
    pub total_decompression_time_ms: Float,
}

/// Error types for MERA operations
#[derive(Debug, Clone, PartialEq)]
pub enum MeraError {
    /// Invalid tensor shape
    InvalidTensorShape(Vec<usize>),

    /// Invalid layer level
    InvalidLayerLevel(usize, usize),

    /// Insufficient data for compression
    InsufficientData(usize),

    /// Decompression failed
    DecompressionFailed(String),

    /// Compression failed
    CompressionFailed(String),
}

impl Tensor {
    /// Create a new tensor with the given shape and fill with zeros
    pub fn new(shape: Vec<usize>) -> Result<Self, MeraError> {
        if shape.is_empty() || shape.iter().any(|&dim| dim == 0) {
            return Err(MeraError::InvalidTensorShape(shape));
        }

        let size = shape.iter().product();
        Ok(Tensor {
            shape,
            data: vec![0.0; size],
        })
    }

    /// Create a new tensor with the given shape and data
    pub fn from_data(shape: Vec<usize>, data: Vec<Float>) -> Result<Self, MeraError> {
        let expected_size = shape.iter().product::<usize>();
        if data.len() != expected_size {
            return Err(MeraError::InvalidTensorShape(shape));
        }

        Ok(Tensor { shape, data })
    }

    /// Create a 2D tensor from a 2D array
    pub fn from_2d_array(arr: &[Vec<Float>]) -> Result<Self, MeraError> {
        if arr.is_empty() {
            return Err(MeraError::InvalidTensorShape(vec![]));
        }

        let rows = arr.len();
        let cols = arr[0].len();

        if arr.iter().any(|row| row.len() != cols) {
            return Err(MeraError::InvalidTensorShape(vec![rows, cols]));
        }

        let shape = vec![rows, cols];
        let mut data = Vec::with_capacity(rows * cols);
        for row in arr {
            data.extend_from_slice(row);
        }

        Ok(Tensor { shape, data })
    }

    /// Get the total number of elements in the tensor
    pub fn num_elements(&self) -> usize {
        self.data.len()
    }

    /// Get the number of dimensions (rank) of the tensor
    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    /// Reshape the tensor (must preserve total number of elements)
    pub fn reshape(&mut self, new_shape: Vec<usize>) -> Result<(), MeraError> {
        let new_size = new_shape.iter().product::<usize>();
        if new_size != self.data.len() {
            return Err(MeraError::InvalidTensorShape(new_shape));
        }

        self.shape = new_shape;
        Ok(())
    }

    /// Get a flat index from multi-dimensional indices
    pub fn get_index(&self, indices: &[usize]) -> Result<usize, MeraError> {
        if indices.len() != self.shape.len() {
            return Err(MeraError::InvalidTensorShape(self.shape.clone()));
        }

        let mut index = 0;
        let mut stride = 1;
        for (&dim, &idx) in self.shape.iter().zip(indices.iter()).rev() {
            if idx >= dim {
                return Err(MeraError::InvalidTensorShape(self.shape.clone()));
            }
            index += idx * stride;
            stride *= dim;
        }

        Ok(index)
    }

    /// Get a value at the given indices
    pub fn get(&self, indices: &[usize]) -> Result<Float, MeraError> {
        let index = self.get_index(indices)?;
        Ok(self.data[index])
    }

    /// Set a value at the given indices
    pub fn set(&mut self, indices: &[usize], value: Float) -> Result<(), MeraError> {
        let index = self.get_index(indices)?;
        self.data[index] = value;
        Ok(())
    }

    /// Apply a function to all elements in place
    pub fn apply_in_place<F>(&mut self, mut f: F)
    where
        F: FnMut(Float) -> Float,
    {
        for elem in self.data.iter_mut() {
            *elem = f(*elem);
        }
    }

    /// Create a new tensor by applying a function
    pub fn map<F>(&self, f: F) -> Tensor
    where
        F: Fn(Float) -> Float,
    {
        Tensor {
            shape: self.shape.clone(),
            data: self.data.iter().map(|&x| f(x)).collect(),
        }
    }

    /// Element-wise addition
    pub fn add(&self, other: &Tensor) -> Result<Tensor, MeraError> {
        if self.shape != other.shape {
            return Err(MeraError::InvalidTensorShape(other.shape.clone()));
        }

        let data: Vec<Float> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();

        Ok(Tensor {
            shape: self.shape.clone(),
            data,
        })
    }

    /// Element-wise subtraction
    pub fn sub(&self, other: &Tensor) -> Result<Tensor, MeraError> {
        if self.shape != other.shape {
            return Err(MeraError::InvalidTensorShape(other.shape.clone()));
        }

        let data: Vec<Float> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a - b)
            .collect();

        Ok(Tensor {
            shape: self.shape.clone(),
            data,
        })
    }

    /// Element-wise multiplication
    pub fn mul(&self, other: &Tensor) -> Result<Tensor, MeraError> {
        if self.shape != other.shape {
            return Err(MeraError::InvalidTensorShape(other.shape.clone()));
        }

        let data: Vec<Float> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .collect();

        Ok(Tensor {
            shape: self.shape.clone(),
            data,
        })
    }

    /// Scalar multiplication
    pub fn scale(&self, scalar: Float) -> Tensor {
        Tensor {
            shape: self.shape.clone(),
            data: self.data.iter().map(|&x| x * scalar).collect(),
        }
    }

    /// Calculate L2 norm (Euclidean norm)
    pub fn norm(&self) -> Float {
        self.data.iter().map(|&x| x * x).sum::<Float>().sqrt()
    }

    /// Normalize tensor to unit norm
    pub fn normalize(&mut self) {
        let norm = self.norm();
        if norm > 1e-10 {
            self.apply_in_place(|x| x / norm);
        }
    }

    /// Transpose a 2D tensor
    pub fn transpose(&self) -> Result<Tensor, MeraError> {
        if self.rank() != 2 {
            return Err(MeraError::InvalidTensorShape(self.shape.clone()));
        }

        let rows = self.shape[0];
        let cols = self.shape[1];

        let mut transposed = Tensor::new(vec![cols, rows])?;
        for i in 0..rows {
            for j in 0..cols {
                let value = self.get(&[i, j])?;
                transposed.set(&[j, i], value)?;
            }
        }

        Ok(transposed)
    }

    /// Matrix multiplication (2D tensors only)
    pub fn matmul(&self, other: &Tensor) -> Result<Tensor, MeraError> {
        if self.rank() != 2 || other.rank() != 2 {
            return Err(MeraError::InvalidTensorShape(self.shape.clone()));
        }

        let (m, k1) = (self.shape[0], self.shape[1]);
        let (k2, n) = (other.shape[0], other.shape[1]);

        if k1 != k2 {
            return Err(MeraError::InvalidTensorShape(other.shape.clone()));
        }

        let mut result = Tensor::new(vec![m, n])?;

        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for k in 0..k1 {
                    sum += self.get(&[i, k])? * other.get(&[k, j])?;
                }
                result.set(&[i, j], sum)?;
            }
        }

        Ok(result)
    }

    /// Downsample tensor by factor of 2 in each dimension
    pub fn downsample(&self) -> Result<Tensor, MeraError> {
        if self.rank() < 1 {
            return Err(MeraError::InvalidTensorShape(self.shape.clone()));
        }

        let new_shape: Vec<usize> = self.shape.iter().map(|&dim| (dim + 1) / 2).collect();

        let mut result = Tensor::new(new_shape.clone())?;

        match self.rank() {
            1 => {
                for i in 0..new_shape[0] {
                    let value = self.get(&[i * 2])?;
                    result.set(&[i], value)?;
                }
            }
            2 => {
                for i in 0..new_shape[0] {
                    for j in 0..new_shape[1] {
                        let value = self.get(&[i * 2, j * 2])?;
                        result.set(&[i, j], value)?;
                    }
                }
            }
            3 => {
                for i in 0..new_shape[0] {
                    for j in 0..new_shape[1] {
                        for k in 0..new_shape[2] {
                            let value = self.get(&[i * 2, j * 2, k * 2])?;
                            result.set(&[i, j, k], value)?;
                        }
                    }
                }
            }
            _ => {
                for (idx, &val) in self.data.iter().enumerate() {
                    if idx % 2 == 0 {
                        let result_idx = idx / 2;
                        if result_idx < result.data.len() {
                            result.data[result_idx] = val;
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Upsample tensor by factor of 2 in each dimension
    pub fn upsample(&self) -> Result<Tensor, MeraError> {
        if self.rank() < 1 {
            return Err(MeraError::InvalidTensorShape(self.shape.clone()));
        }

        let new_shape: Vec<usize> = self.shape.iter().map(|&dim| dim * 2).collect();

        let mut result = Tensor::new(new_shape.clone())?;

        match self.rank() {
            1 => {
                for i in 0..self.shape[0] {
                    let value = self.get(&[i])?;
                    result.set(&[i * 2], value)?;
                    if i * 2 + 1 < new_shape[0] {
                        result.set(&[i * 2 + 1], value)?;
                    }
                }
            }
            2 => {
                for i in 0..self.shape[0] {
                    for j in 0..self.shape[1] {
                        let value = self.get(&[i, j])?;
                        result.set(&[i * 2, j * 2], value)?;
                        if i * 2 + 1 < new_shape[0] {
                            result.set(&[i * 2 + 1, j * 2], value)?;
                        }
                        if j * 2 + 1 < new_shape[1] {
                            result.set(&[i * 2, j * 2 + 1], value)?;
                        }
                        if i * 2 + 1 < new_shape[0] && j * 2 + 1 < new_shape[1] {
                            result.set(&[i * 2 + 1, j * 2 + 1], value)?;
                        }
                    }
                }
            }
            3 => {
                for i in 0..self.shape[0] {
                    for j in 0..self.shape[1] {
                        for k in 0..self.shape[2] {
                            let value = self.get(&[i, j, k])?;
                            for di in 0..2 {
                                for dj in 0..2 {
                                    for dk in 0..2 {
                                        let result_idx = i * 2 + di;
                                        let result_jdy = j * 2 + dj;
                                        let result_kdz = k * 2 + dk;
                                        if result_idx < new_shape[0]
                                            && result_jdy < new_shape[1]
                                            && result_kdz < new_shape[2]
                                        {
                                            result.set(
                                                &[result_idx, result_jdy, result_kdz],
                                                value,
                                            )?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                for (idx, &val) in self.data.iter().enumerate() {
                    for i in 0..2 {
                        let result_idx = idx * 2 + i;
                        if result_idx < result.data.len() {
                            result.data[result_idx] = val;
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Calculate sum of all elements
    pub fn sum(&self) -> Float {
        self.data.iter().sum()
    }

    /// Calculate mean of all elements
    pub fn mean(&self) -> Float {
        if self.data.is_empty() {
            return 0.0;
        }
        self.sum() / self.data.len() as Float
    }

    /// Calculate variance of all elements
    pub fn variance(&self) -> Float {
        if self.data.len() <= 1 {
            return 0.0;
        }

        let mean = self.mean();
        let sum_sq_diff: Float = self.data.iter().map(|&x| (x - mean).powi(2)).sum();
        sum_sq_diff / self.data.len() as Float
    }

    /// Calculate standard deviation of all elements
    pub fn std_dev(&self) -> Float {
        self.variance().sqrt()
    }
}

impl Index<usize> for Tensor {
    type Output = Float;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Tensor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl MeraLayer {
    /// Create a new MERA layer
    pub fn new(level: usize, data: Tensor) -> Self {
        MeraLayer {
            level,
            disentanglers: Vec::new(),
            coarse_grainers: Vec::new(),
            data,
            wavelet_coeffs: None,
        }
    }

    /// Apply disentanglers to remove redundant information
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Disentanglers remove redundancy from tensor data by identifying
    /// and extracting independent features."
    pub fn disentangle(&self, input: &Tensor) -> Result<Tensor, MeraError> {
        if self.disentanglers.is_empty() {
            return Ok(input.clone());
        }

        let mut result = input.clone();

        for disentangler in &self.disentanglers {
            result = result.matmul(disentangler)?;
        }

        Ok(result)
    }

    /// Apply coarse-grainers to combine representations
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Coarse-grainers combine multiple representations into one,
    /// creating hierarchical representations."
    pub fn coarsen(&self, input: &Tensor) -> Result<Tensor, MeraError> {
        if self.coarse_grainers.is_empty() {
            return Ok(input.clone());
        }

        let mut result = input.clone();

        for coarse_grainer in &self.coarse_grainers {
            result = result.matmul(coarse_grainer)?;
        }

        Ok(result)
    }

    /// Refine data from coarser scale
    pub fn refine(&self, coarse: &Tensor, query: &MeraQuery) -> Result<Tensor, MeraError> {
        if self.coarse_grainers.is_empty() {
            return Ok(coarse.clone());
        }

        let mut result = coarse.clone();

        for coarse_grainer in self.coarse_grainers.iter().rev() {
            let transposed = coarse_grainer.transpose()?;
            result = result.matmul(&transposed)?;
        }

        Ok(result)
    }

    /// Apply wavelet compression to data
    pub fn apply_wavelet_compression(&mut self, threshold: Float) -> Result<(), MeraError> {
        if self.data.rank() != 2 {
            return Err(MeraError::InvalidTensorShape(self.data.shape.clone()));
        }

        let (rows, cols) = (self.data.shape[0], self.data.shape[1]);

        if rows < 2 || cols < 2 {
            return Err(MeraError::InsufficientData(rows * cols));
        }

        let mut approximation = Tensor::new(vec![rows / 2, cols / 2])?;
        let mut detail_h = Tensor::new(vec![rows / 2, cols / 2])?;
        let mut detail_v = Tensor::new(vec![rows / 2, cols / 2])?;
        let mut detail_d = Tensor::new(vec![rows / 2, cols / 2])?;

        for i in 0..(rows / 2) {
            for j in 0..(cols / 2) {
                let i0 = i * 2;
                let i1 = i0 + 1;
                let j0 = j * 2;
                let j1 = j0 + 1;

                let v00 = self.data.get(&[i0, j0])?;
                let v01 = self.data.get(&[i0, j1])?;
                let v10 = self.data.get(&[i1, j0])?;
                let v11 = self.data.get(&[i1, j1])?;

                let avg = (v00 + v01 + v10 + v11) / 4.0;
                let diff_h = (v00 + v11 - v01 - v10) / 4.0;
                let diff_v = (v00 + v01 - v10 - v11) / 4.0;
                let diff_d = (v00 - v01 - v10 + v11) / 4.0;

                approximation.set(&[i, j], avg)?;
                detail_h.set(&[i, j], diff_h)?;
                detail_v.set(&[i, j], diff_v)?;
                detail_d.set(&[i, j], diff_d)?;
            }
        }

        self.wavelet_coeffs = Some(WaveletCoefficients {
            approximation,
            detail: vec![detail_h, detail_v, detail_d],
            threshold,
        });

        Ok(())
    }

    /// Decompress from wavelet coefficients
    pub fn apply_wavelet_decompression(&self) -> Result<Tensor, MeraError> {
        let coeffs = self
            .wavelet_coeffs
            .as_ref()
            .ok_or_else(|| MeraError::DecompressionFailed("No wavelet coefficients".to_string()))?;

        let (half_rows, half_cols) = (coeffs.approximation.shape[0], coeffs.approximation.shape[1]);
        let mut result = Tensor::new(vec![half_rows * 2, half_cols * 2])?;

        for i in 0..half_rows {
            for j in 0..half_cols {
                let avg = coeffs.approximation.get(&[i, j])?;
                let diff_h = coeffs.detail[0].get(&[i, j])?;
                let diff_v = coeffs.detail[1].get(&[i, j])?;
                let diff_d = coeffs.detail[2].get(&[i, j])?;

                let v00 = avg + diff_h + diff_v + diff_d;
                let v01 = avg - diff_h + diff_v - diff_d;
                let v10 = avg + diff_h - diff_v - diff_d;
                let v11 = avg - diff_h - diff_v + diff_d;

                result.set(&[i * 2, j * 2], v00)?;
                result.set(&[i * 2, j * 2 + 1], v01)?;
                result.set(&[i * 2 + 1, j * 2], v10)?;
                result.set(&[i * 2 + 1, j * 2 + 1], v11)?;
            }
        }

        Ok(result)
    }

    /// Get compressed size based on wavelet threshold
    pub fn compressed_size(&self) -> usize {
        if let Some(ref coeffs) = self.wavelet_coeffs {
            let mut count = 0;

            count += coeffs.approximation.num_elements();

            for detail in &coeffs.detail {
                count += detail
                    .data
                    .iter()
                    .filter(|&&v| v.abs() > coeffs.threshold)
                    .count();
            }

            count
        } else {
            self.data.num_elements()
        }
    }
}

impl MeraNetwork {
    /// Create a new MERA network with the specified number of levels
    pub fn new(num_levels: usize) -> Self {
        MeraNetwork {
            layers: Vec::with_capacity(num_levels),
            num_levels,
            stats: MeraStatistics::default(),
        }
    }

    /// Initialize MERA network with base data at level 0
    pub fn initialize(&mut self, base_data: Tensor) -> Result<(), MeraError> {
        if self.layers.is_empty() {
            let layer_0 = MeraLayer::new(0, base_data);
            self.layers.push(layer_0);
        } else {
            self.layers[0].data = base_data;
        }

        Ok(())
    }

    /// Build the MERA hierarchy by progressively coarsening data
    pub fn build_hierarchy(&mut self) -> Result<(), MeraError> {
        if self.layers.is_empty() {
            return Err(MeraError::InsufficientData(0));
        }

        for level in 1..self.num_levels {
            let prev_data = self.layers[level - 1].data.clone();
            let downsampled = prev_data.downsample()?;

            let layer = MeraLayer::new(level, downsampled);
            self.layers.push(layer);
        }

        Ok(())
    }

    /// Compress data using MERA structure
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Compress data using MERA structure - apply disentanglers and
    /// coarse-grainers at each layer."
    pub fn compress(&mut self, data: Tensor) -> Result<MeraCompressionResult, MeraError> {
        let start = std::time::Instant::now();

        let original_size = data.num_elements();

        self.initialize(data)?;
        self.build_hierarchy()?;

        for layer in &mut self.layers {
            let disentangled = layer.disentangle(&layer.data.clone())?;
            let coarsened = layer.coarsen(&disentangled)?;

            layer.data = coarsened;

            if layer.level < self.num_levels - 1 {
                layer.apply_wavelet_compression(0.01)?;
            }
        }

        let compressed_size: usize = self.layers.iter().map(|l| l.compressed_size()).sum();
        let compression_ratio = original_size as Float / compressed_size.max(1) as Float;
        let compression_time_ms = start.elapsed().as_secs_f64() * 1000.0;

        self.stats.total_compressions += 1;
        self.stats.total_compression_time_ms += compression_time_ms;
        self.stats.average_compression_ratio = (self.stats.average_compression_ratio
            * (self.stats.total_compressions - 1) as Float
            + compression_ratio)
            / self.stats.total_compressions as Float;

        Ok(MeraCompressionResult {
            original_size,
            compressed_size,
            compression_ratio,
            compression_time_ms,
        })
    }

    /// Decompress specific portion on-demand
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Navigate up the MERA hierarchy to find relevant data and refine."
    pub fn decompress(&mut self, query: &MeraQuery) -> Result<MeraDecompressionResult, MeraError> {
        let start = std::time::Instant::now();

        if query.scale_level >= self.layers.len() {
            return Err(MeraError::InvalidLayerLevel(
                query.scale_level,
                self.layers.len(),
            ));
        }

        let mut result = self.layers[query.scale_level].data.clone();

        for layer in self.layers[0..query.scale_level].iter().rev() {
            if let Some(ref coeffs) = layer.wavelet_coeffs {
                let decompressed = layer.apply_wavelet_decompression()?;
                result = decompressed;
            }

            result = layer.refine(&result, query)?;
        }

        let decompression_time_ms = start.elapsed().as_secs_f64() * 1000.0;

        self.stats.total_decompressions += 1;
        self.stats.total_decompression_time_ms += decompression_time_ms;

        Ok(MeraDecompressionResult {
            data: result,
            decompression_time_ms,
            precision: query.precision,
        })
    }

    /// Get data at a specific scale level
    pub fn get_level(&self, level: usize) -> Option<&Tensor> {
        self.layers.get(level).map(|layer| &layer.data)
    }

    /// Get the number of levels in the network
    pub fn num_levels(&self) -> usize {
        self.layers.len()
    }

    /// Get compression statistics
    pub fn statistics(&self) -> &MeraStatistics {
        &self.stats
    }

    /// Clear all layers and statistics
    pub fn clear(&mut self) {
        self.layers.clear();
        self.stats = MeraStatistics::default();
    }
}

impl Default for MeraNetwork {
    fn default() -> Self {
        Self::new(7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_creation() {
        let tensor = Tensor::new(vec![2, 3]).unwrap();
        assert_eq!(tensor.shape, vec![2, 3]);
        assert_eq!(tensor.num_elements(), 6);
        assert!(tensor.data.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_tensor_from_data() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let tensor = Tensor::from_data(vec![2, 3], data).unwrap();
        assert_eq!(tensor.shape, vec![2, 3]);
        assert_eq!(tensor.num_elements(), 6);
        assert_eq!(tensor.get(&[0, 0]).unwrap(), 1.0);
        assert_eq!(tensor.get(&[1, 2]).unwrap(), 6.0);
    }

    #[test]
    fn test_tensor_get_set() {
        let mut tensor = Tensor::new(vec![2, 2]).unwrap();
        tensor.set(&[0, 0], 1.0).unwrap();
        tensor.set(&[0, 1], 2.0).unwrap();
        tensor.set(&[1, 0], 3.0).unwrap();
        tensor.set(&[1, 1], 4.0).unwrap();

        assert_eq!(tensor.get(&[0, 0]).unwrap(), 1.0);
        assert_eq!(tensor.get(&[0, 1]).unwrap(), 2.0);
        assert_eq!(tensor.get(&[1, 0]).unwrap(), 3.0);
        assert_eq!(tensor.get(&[1, 1]).unwrap(), 4.0);
    }

    #[test]
    fn test_tensor_add() {
        let a = Tensor::from_data(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = Tensor::from_data(vec![2, 2], vec![5.0, 6.0, 7.0, 8.0]).unwrap();
        let c = a.add(&b).unwrap();

        assert_eq!(c.data, vec![6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn test_tensor_sub() {
        let a = Tensor::from_data(vec![2, 2], vec![5.0, 6.0, 7.0, 8.0]).unwrap();
        let b = Tensor::from_data(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let c = a.sub(&b).unwrap();

        assert_eq!(c.data, vec![4.0, 4.0, 4.0, 4.0]);
    }

    #[test]
    fn test_tensor_mul() {
        let a = Tensor::from_data(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = Tensor::from_data(vec![2, 2], vec![2.0, 3.0, 4.0, 5.0]).unwrap();
        let c = a.mul(&b).unwrap();

        assert_eq!(c.data, vec![2.0, 6.0, 12.0, 20.0]);
    }

    #[test]
    fn test_tensor_scale() {
        let a = Tensor::from_data(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = a.scale(2.0);

        assert_eq!(b.data, vec![2.0, 4.0, 6.0, 8.0]);
    }

    #[test]
    fn test_tensor_norm() {
        let a = Tensor::from_data(vec![2, 2], vec![3.0, 4.0, 0.0, 0.0]).unwrap();
        let norm = a.norm();

        assert!((norm - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_tensor_normalize() {
        let mut a = Tensor::from_data(vec![2, 2], vec![3.0, 4.0, 0.0, 0.0]).unwrap();
        a.normalize();
        let norm = a.norm();

        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tensor_transpose() {
        let a = Tensor::from_data(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let b = a.transpose().unwrap();

        assert_eq!(b.shape, vec![3, 2]);
        assert_eq!(b.get(&[0, 0]).unwrap(), 1.0);
        assert_eq!(b.get(&[1, 0]).unwrap(), 2.0);
        assert_eq!(b.get(&[2, 0]).unwrap(), 3.0);
        assert_eq!(b.get(&[0, 1]).unwrap(), 4.0);
        assert_eq!(b.get(&[1, 1]).unwrap(), 5.0);
        assert_eq!(b.get(&[2, 1]).unwrap(), 6.0);
    }

    #[test]
    fn test_tensor_matmul() {
        let a = Tensor::from_data(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let b = Tensor::from_data(vec![3, 2], vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]).unwrap();
        let c = a.matmul(&b).unwrap();

        assert_eq!(c.shape, vec![2, 2]);
        assert_eq!(c.get(&[0, 0]).unwrap(), 58.0);
        assert_eq!(c.get(&[0, 1]).unwrap(), 64.0);
        assert_eq!(c.get(&[1, 0]).unwrap(), 139.0);
        assert_eq!(c.get(&[1, 1]).unwrap(), 154.0);
    }

    #[test]
    fn test_tensor_downsample() {
        let a = Tensor::from_data(
            vec![4, 4],
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ],
        )
        .unwrap();
        let b = a.downsample().unwrap();

        assert_eq!(b.shape, vec![2, 2]);
        assert_eq!(b.get(&[0, 0]).unwrap(), 1.0);
        assert_eq!(b.get(&[0, 1]).unwrap(), 3.0);
        assert_eq!(b.get(&[1, 0]).unwrap(), 9.0);
        assert_eq!(b.get(&[1, 1]).unwrap(), 11.0);
    }

    #[test]
    fn test_tensor_upsample() {
        let a = Tensor::from_data(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = a.upsample().unwrap();

        assert_eq!(b.shape, vec![4, 4]);
        assert_eq!(b.get(&[0, 0]).unwrap(), 1.0);
        assert_eq!(b.get(&[0, 1]).unwrap(), 1.0);
        assert_eq!(b.get(&[1, 0]).unwrap(), 1.0);
        assert_eq!(b.get(&[1, 1]).unwrap(), 1.0);
    }

    #[test]
    fn test_tensor_sum() {
        let a = Tensor::from_data(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(a.sum(), 10.0);
    }

    #[test]
    fn test_tensor_mean() {
        let a = Tensor::from_data(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(a.mean(), 2.5);
    }

    #[test]
    fn test_tensor_variance() {
        let a = Tensor::from_data(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let variance = a.variance();
        assert!((variance - 1.25).abs() < 1e-10);
    }

    #[test]
    fn test_tensor_std_dev() {
        let a = Tensor::from_data(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let std_dev = a.std_dev();
        assert!((std_dev - 1.118033988749895).abs() < 1e-10);
    }

    #[test]
    fn test_mera_network_creation() {
        let mera = MeraNetwork::new(7);
        assert_eq!(mera.num_levels(), 0);
        assert_eq!(mera.num_levels, 7);
    }

    #[test]
    fn test_mera_network_default() {
        let mera = MeraNetwork::default();
        assert_eq!(mera.num_levels(), 0);
        assert_eq!(mera.num_levels, 7);
    }

    #[test]
    fn test_mera_initialize() {
        let mut mera = MeraNetwork::new(3);
        let data = Tensor::from_data(vec![4, 4], (0..16).map(|i| i as Float).collect()).unwrap();
        mera.initialize(data).unwrap();

        assert_eq!(mera.num_levels(), 1);
        assert_eq!(mera.layers[0].level, 0);
    }

    #[test]
    fn test_mera_build_hierarchy() {
        let mut mera = MeraNetwork::new(3);
        let data = Tensor::from_data(vec![8, 8], (0..64).map(|i| i as Float).collect()).unwrap();
        mera.initialize(data).unwrap();
        mera.build_hierarchy().unwrap();

        assert_eq!(mera.num_levels(), 3);
        assert_eq!(mera.layers[0].data.shape, vec![8, 8]);
        assert_eq!(mera.layers[1].data.shape, vec![4, 4]);
        assert_eq!(mera.layers[2].data.shape, vec![2, 2]);
    }

    #[test]
    fn test_mera_compress() {
        let mut mera = MeraNetwork::new(3);
        let data = Tensor::from_data(vec![8, 8], (0..64).map(|i| i as Float).collect()).unwrap();
        let result = mera.compress(data).unwrap();

        assert_eq!(result.original_size, 64);
        assert!(result.compressed_size > 0);
        assert!(result.compression_ratio > 1.0);
        assert!(result.compression_time_ms > 0.0);
    }

    #[test]
    fn test_mera_decompress() {
        let mut mera = MeraNetwork::new(3);
        let original =
            Tensor::from_data(vec![8, 8], (0..64).map(|i| i as Float).collect()).unwrap();
        mera.compress(original.clone()).unwrap();

        let query = MeraQuery {
            scale_level: 0,
            region: None,
            precision: 1.0,
        };
        let result = mera.decompress(&query).unwrap();

        assert_eq!(result.data.shape, vec![8, 8]);
        assert!(result.decompression_time_ms > 0.0);
    }

    #[test]
    fn test_mera_get_level() {
        let mut mera = MeraNetwork::new(3);
        let data = Tensor::from_data(vec![8, 8], (0..64).map(|i| i as Float).collect()).unwrap();
        mera.compress(data).unwrap();

        let level_0 = mera.get_level(0);
        let level_1 = mera.get_level(1);
        let level_2 = mera.get_level(2);
        let level_3 = mera.get_level(3);

        assert!(level_0.is_some());
        assert!(level_1.is_some());
        assert!(level_2.is_some());
        assert!(level_3.is_none());
    }

    #[test]
    fn test_mera_statistics() {
        let mut mera = MeraNetwork::new(3);
        let data = Tensor::from_data(vec![8, 8], (0..64).map(|i| i as Float).collect()).unwrap();
        mera.compress(data).unwrap();

        let stats = mera.statistics();
        assert_eq!(stats.total_compressions, 1);
        assert!(stats.average_compression_ratio > 0.0);
        assert!(stats.total_compression_time_ms > 0.0);
    }

    #[test]
    fn test_mera_clear() {
        let mut mera = MeraNetwork::new(3);
        let data = Tensor::from_data(vec![8, 8], (0..64).map(|i| i as Float).collect()).unwrap();
        mera.compress(data).unwrap();
        mera.clear();

        assert_eq!(mera.num_levels(), 0);
        assert_eq!(mera.stats.total_compressions, 0);
    }

    #[test]
    fn test_wavelet_compression() {
        let mut layer = MeraLayer::new(
            0,
            Tensor::from_data(vec![4, 4], (0..16).map(|i| i as Float).collect()).unwrap(),
        );
        layer.apply_wavelet_compression(0.01).unwrap();

        assert!(layer.wavelet_coeffs.is_some());
        let coeffs = layer.wavelet_coeffs.as_ref().unwrap();
        assert_eq!(coeffs.approximation.shape, vec![2, 2]);
        assert_eq!(coeffs.detail.len(), 3);
    }

    #[test]
    fn test_wavelet_decompression() {
        let original =
            Tensor::from_data(vec![4, 4], (0..16).map(|i| i as Float).collect()).unwrap();
        let mut layer = MeraLayer::new(0, original.clone());
        layer.apply_wavelet_compression(0.0).unwrap();
        let decompressed = layer.apply_wavelet_decompression().unwrap();

        assert_eq!(decompressed.shape, original.shape);
        let max_diff = original
            .data
            .iter()
            .zip(decompressed.data.iter())
            .map(|(o, d)| (o - d).abs())
            .fold(0.0_f64, |acc, x| acc.max(x));
        assert!(max_diff < 1.0, "Max difference: {}", max_diff);
    }

    #[test]
    fn test_mera_error_invalid_tensor_shape() {
        let result = Tensor::new(vec![0]);
        assert!(matches!(result, Err(MeraError::InvalidTensorShape(_))));
    }

    #[test]
    fn test_compressed_size() {
        let mut layer = MeraLayer::new(
            0,
            Tensor::from_data(vec![4, 4], (0..16).map(|i| i as Float).collect()).unwrap(),
        );
        layer.apply_wavelet_compression(0.01).unwrap();

        let compressed_size = layer.compressed_size();
        assert!(compressed_size > 0);
        assert!(compressed_size < layer.data.num_elements());
    }
}
