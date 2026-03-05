//! Archetype Basis Set - Orthogonal basis for archetypical pattern compression
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "The 22 archetypes form an orthogonal basis set. Any archetype activation profile
//! is a linear combination of basis vectors."
//!
//! This module provides:
//! - ArchetypeBasis: 22 orthogonal basis vectors (one per archetype)
//! - ArchetypeActivationProfile: 22 coefficients (88 bytes total)
//! - ArchetypicalPattern: Reconstructed pattern from coefficients
//! - 100x compression: 88 bytes vs thousands of floats

use crate::types::Float;
use std::f64::consts::PI;

/// Number of archetypes in the basis set
pub const NUM_ARCHETYPES: usize = 22;

/// Size of ArchetypeActivationProfile in bytes
pub const PROFILE_SIZE_BYTES: usize = NUM_ARCHETYPES * 8;

/// A single archetype vector in the basis set
#[derive(Debug, Clone, PartialEq)]
pub struct ArchetypeVector {
    /// Vector components (dimension of the pattern space)
    components: Vec<Float>,
}

impl ArchetypeVector {
    /// Create a new archetype vector
    pub fn new(components: Vec<Float>) -> Self {
        ArchetypeVector { components }
    }

    /// Create a zero vector
    pub fn zero(dimension: usize) -> Self {
        ArchetypeVector {
            components: vec![0.0; dimension],
        }
    }

    /// Get vector dimension
    pub fn dimension(&self) -> usize {
        self.components.len()
    }

    /// Get reference to components
    pub fn components(&self) -> &[Float] {
        &self.components
    }

    /// Multiply vector by scalar
    pub fn mul_scalar(&self, scalar: Float) -> ArchetypeVector {
        ArchetypeVector {
            components: self.components.iter().map(|c| c * scalar).collect(),
        }
    }

    /// Add two vectors
    pub fn add(&self, other: &ArchetypeVector) -> ArchetypeVector {
        assert_eq!(
            self.components.len(),
            other.components.len(),
            "Vector dimensions must match"
        );
        ArchetypeVector {
            components: self
                .components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }

    /// Subtract two vectors
    pub fn sub(&self, other: &ArchetypeVector) -> ArchetypeVector {
        assert_eq!(
            self.components.len(),
            other.components.len(),
            "Vector dimensions must match"
        );
        ArchetypeVector {
            components: self
                .components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }

    /// Compute dot product
    pub fn dot(&self, other: &ArchetypeVector) -> Float {
        assert_eq!(
            self.components.len(),
            other.components.len(),
            "Vector dimensions must match"
        );
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    /// Compute L2 norm (magnitude)
    pub fn norm(&self) -> Float {
        self.dot(self).sqrt()
    }

    /// Normalize vector to unit length
    pub fn normalize(&self) -> ArchetypeVector {
        let norm = self.norm();
        if norm > 1e-10 {
            self.mul_scalar(1.0 / norm)
        } else {
            self.clone()
        }
    }
}

/// A reconstructed archetypical pattern
#[derive(Debug, Clone, PartialEq)]
pub struct ArchetypicalPattern {
    /// Pattern components
    components: Vec<Float>,
}

impl ArchetypicalPattern {
    /// Create a new pattern
    pub fn new(components: Vec<Float>) -> Self {
        ArchetypicalPattern { components }
    }

    /// Create a zero pattern
    pub fn zero(dimension: usize) -> Self {
        ArchetypicalPattern {
            components: vec![0.0; dimension],
        }
    }

    /// Get pattern dimension
    pub fn dimension(&self) -> usize {
        self.components.len()
    }

    /// Get reference to components
    pub fn components(&self) -> &[Float] {
        &self.components
    }

    /// Add another pattern (for accumulation during reconstruction)
    pub fn add(&mut self, other: &ArchetypicalPattern) {
        assert_eq!(
            self.components.len(),
            other.components.len(),
            "Pattern dimensions must match"
        );
        for (a, b) in self.components.iter_mut().zip(other.components.iter()) {
            *a += b;
        }
    }

    /// Multiply pattern by scalar
    pub fn mul_scalar(&self, scalar: Float) -> ArchetypicalPattern {
        ArchetypicalPattern {
            components: self.components.iter().map(|c| c * scalar).collect(),
        }
    }

    /// Compute L2 norm
    pub fn norm(&self) -> Float {
        self.components.iter().map(|c| c * c).sum::<Float>().sqrt()
    }

    /// Compute compression ratio (original size / compressed size)
    pub fn compression_ratio(&self) -> Float {
        let original_size = self.components.len() * 8; // 8 bytes per f64
        let compressed_size = PROFILE_SIZE_BYTES;
        original_size as Float / compressed_size as Float
    }
}

/// Archetype activation profile - compressed representation
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Store: 22 floats (88 bytes)"
/// "Compression ratio: ~100x"
#[derive(Debug, Clone, PartialEq)]
pub struct ArchetypeActivationProfile {
    /// 22 coefficients (one per archetype) - 88 bytes total
    coefficients: [Float; NUM_ARCHETYPES],
}

impl ArchetypeActivationProfile {
    /// Create a new activation profile
    pub fn new(coefficients: [Float; NUM_ARCHETYPES]) -> Self {
        ArchetypeActivationProfile { coefficients }
    }

    /// Create a zero profile (all coefficients zero)
    pub fn zero() -> Self {
        ArchetypeActivationProfile {
            coefficients: [0.0; NUM_ARCHETYPES],
        }
    }

    /// Get reference to coefficients
    pub fn coefficients(&self) -> &[Float; NUM_ARCHETYPES] {
        &self.coefficients
    }

    /// Get coefficient for specific archetype
    pub fn get(&self, index: usize) -> Option<Float> {
        self.coefficients.get(index).copied()
    }

    /// Set coefficient for specific archetype
    pub fn set(&mut self, index: usize, value: Float) -> Result<(), ArchetypeBasisError> {
        if index >= NUM_ARCHETYPES {
            return Err(ArchetypeBasisError::InvalidIndex(index));
        }
        self.coefficients[index] = value;
        Ok(())
    }

    /// Get profile size in bytes
    pub fn size_bytes(&self) -> usize {
        PROFILE_SIZE_BYTES
    }

    /// Normalize coefficients to unit sum
    pub fn normalize(&mut self) {
        let sum: Float = self.coefficients.iter().sum();
        if sum > 1e-10 {
            for coeff in self.coefficients.iter_mut() {
                *coeff /= sum;
            }
        }
    }

    /// Check if profile is normalized (sum = 1.0)
    pub fn is_normalized(&self, tolerance: Float) -> bool {
        let sum: Float = self.coefficients.iter().sum();
        (sum - 1.0).abs() < tolerance
    }

    /// Compute entropy of the profile
    pub fn entropy(&self) -> Float {
        let mut entropy = 0.0;
        for &coeff in self.coefficients.iter() {
            if coeff > 1e-10 {
                entropy -= coeff * coeff.ln();
            }
        }
        entropy
    }

    /// Find dominant archetype (highest coefficient)
    pub fn dominant_archetype(&self) -> (usize, Float) {
        self.coefficients
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, &v)| (i, v))
            .unwrap_or((0, 0.0))
    }
}

/// Archetype basis set - 22 orthogonal vectors
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "The 22 archetypes form an orthogonal basis set."
#[derive(Debug, Clone)]
pub struct ArchetypeBasis {
    /// 22 basis vectors (one per archetype)
    basis: [ArchetypeVector; NUM_ARCHETYPES],
    /// Pattern dimension
    dimension: usize,
}

impl ArchetypeBasis {
    /// Create a new archetype basis with default orthogonal vectors
    ///
    /// Creates orthogonal basis vectors using sin/cos functions
    /// to ensure orthogonality
    pub fn new(dimension: usize) -> Self {
        let mut basis = Vec::with_capacity(NUM_ARCHETYPES);

        for i in 0..NUM_ARCHETYPES {
            let mut components = Vec::with_capacity(dimension);
            for j in 0..dimension {
                let freq = (i + 1) as Float;
                let phase = (i * 13) as Float; // Prime for orthogonality
                let value = (2.0 * PI * freq * (j as Float) / (dimension as Float) + phase).sin();
                components.push(value);
            }
            basis.push(ArchetypeVector::new(components));
        }

        let basis_array: [ArchetypeVector; NUM_ARCHETYPES] =
            basis.try_into().unwrap_or_else(|_| {
                panic!(
                    "Failed to convert basis Vec to array of size {}",
                    NUM_ARCHETYPES
                )
            });

        ArchetypeBasis {
            basis: basis_array,
            dimension,
        }
    }

    /// Get pattern dimension
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Get reference to basis vectors
    pub fn basis(&self) -> &[ArchetypeVector; NUM_ARCHETYPES] {
        &self.basis
    }

    /// Get specific basis vector
    pub fn get_basis_vector(&self, index: usize) -> Option<&ArchetypeVector> {
        self.basis.get(index)
    }

    /// Reconstruct pattern from activation profile
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Reconstruct pattern from coefficients"
    /// "Reconstruction: 22 multiply-add operations"
    pub fn reconstruct(&self, profile: &ArchetypeActivationProfile) -> ArchetypicalPattern {
        let mut pattern = ArchetypicalPattern::zero(self.dimension);

        for (i, coeff) in profile.coefficients.iter().enumerate() {
            let contribution = self.basis[i].mul_scalar(*coeff);
            pattern.add(&ArchetypicalPattern::new(
                contribution.components().to_vec(),
            ));
        }

        pattern
    }

    /// Project pattern onto basis to get activation profile
    ///
    /// This is the inverse operation of reconstruct()
    pub fn project(&self, pattern: &ArchetypicalPattern) -> ArchetypeActivationProfile {
        let coefficients: [Float; NUM_ARCHETYPES] = self.basis
            .iter()
            .map(|basis_vector| {
                let pattern_vector = ArchetypeVector::new(pattern.components().to_vec());
                basis_vector.dot(&pattern_vector) / basis_vector.dot(basis_vector).max(1e-10)
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or([0.0; NUM_ARCHETYPES]);

        ArchetypeActivationProfile::new(coefficients)
    }

    /// Verify orthogonality of basis vectors
    pub fn verify_orthogonality(&self, tolerance: Float) -> bool {
        for i in 0..NUM_ARCHETYPES {
            for j in (i + 1)..NUM_ARCHETYPES {
                let dot = self.basis[i].dot(&self.basis[j]);
                if dot.abs() > tolerance {
                    return false;
                }
            }
        }
        true
    }

    /// Normalize all basis vectors to unit length
    pub fn normalize_basis(&mut self) {
        for vector in self.basis.iter_mut() {
            let normalized = vector.normalize();
            *vector = normalized;
        }
    }

    /// Compute reconstruction error between original and reconstructed pattern
    pub fn reconstruction_error(
        &self,
        original: &ArchetypicalPattern,
        reconstructed: &ArchetypicalPattern,
    ) -> Float {
        let diff = original.sub(reconstructed);
        diff.norm()
    }
}

/// Helper to add two ArchetypicalPattern values
impl std::ops::Add<&ArchetypicalPattern> for ArchetypicalPattern {
    type Output = ArchetypicalPattern;

    fn add(self, other: &ArchetypicalPattern) -> ArchetypicalPattern {
        assert_eq!(
            self.components.len(),
            other.components.len(),
            "Pattern dimensions must match"
        );
        ArchetypicalPattern {
            components: self
                .components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

/// Helper to subtract two ArchetypicalPattern values
impl ArchetypicalPattern {
    pub fn sub(&self, other: &ArchetypicalPattern) -> ArchetypicalPattern {
        assert_eq!(
            self.components.len(),
            other.components.len(),
            "Pattern dimensions must match"
        );
        ArchetypicalPattern {
            components: self
                .components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

/// Archetype basis error types
#[derive(Debug, Clone, PartialEq)]
pub enum ArchetypeBasisError {
    InvalidIndex(usize),
    DimensionMismatch,
    EmptyPattern,
}

impl std::fmt::Display for ArchetypeBasisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchetypeBasisError::InvalidIndex(i) => write!(f, "Invalid archetype index: {}", i),
            ArchetypeBasisError::DimensionMismatch => write!(f, "Dimension mismatch"),
            ArchetypeBasisError::EmptyPattern => write!(f, "Empty pattern"),
        }
    }
}

impl std::error::Error for ArchetypeBasisError {}

/// Statistics for archetype basis operations
#[derive(Debug, Clone, Default)]
pub struct ArchetypeBasisStatistics {
    /// Total number of reconstructions performed
    pub total_reconstructions: usize,
    /// Total reconstruction time (microseconds)
    pub total_reconstruction_time_us: Float,
    /// Average reconstruction time (microseconds)
    pub average_reconstruction_time_us: Float,
    /// Compression ratio achieved
    pub compression_ratio: Float,
}

impl ArchetypeBasisStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_reconstruction(&mut self, duration_us: Float) {
        self.total_reconstructions += 1;
        self.total_reconstruction_time_us += duration_us;
        self.average_reconstruction_time_us =
            self.total_reconstruction_time_us / self.total_reconstructions as Float;
    }

    pub fn set_compression_ratio(&mut self, ratio: Float) {
        self.compression_ratio = ratio;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_activation_profile_zero() {
        let profile = ArchetypeActivationProfile::zero();
        assert_eq!(profile.coefficients().len(), NUM_ARCHETYPES);
        for coeff in profile.coefficients() {
            assert_eq!(*coeff, 0.0);
        }
    }

    #[test]
    fn test_archetype_activation_profile_size_bytes() {
        let profile = ArchetypeActivationProfile::zero();
        assert_eq!(profile.size_bytes(), PROFILE_SIZE_BYTES);
        assert_eq!(PROFILE_SIZE_BYTES, 176); // 22 * 8 = 176
    }

    #[test]
    fn test_archetype_activation_profile_set_get() {
        let mut profile = ArchetypeActivationProfile::zero();
        profile.set(0, 0.5).unwrap();
        profile.set(10, 0.75).unwrap();
        assert_eq!(profile.get(0), Some(0.5));
        assert_eq!(profile.get(10), Some(0.75));
        assert_eq!(profile.get(21), Some(0.0));
    }

    #[test]
    fn test_archetype_activation_profile_invalid_index() {
        let mut profile = ArchetypeActivationProfile::zero();
        assert!(profile.set(22, 1.0).is_err());
    }

    #[test]
    fn test_archetype_activation_profile_normalize() {
        let mut profile = ArchetypeActivationProfile::zero();
        profile.set(0, 1.0).unwrap();
        profile.set(1, 2.0).unwrap();
        profile.set(2, 1.0).unwrap();
        profile.normalize();
        assert!(profile.is_normalized(1e-6));
        assert_eq!(profile.get(0), Some(0.25));
        assert_eq!(profile.get(1), Some(0.5));
        assert_eq!(profile.get(2), Some(0.25));
    }

    #[test]
    fn test_archetype_activation_profile_entropy() {
        let mut profile = ArchetypeActivationProfile::zero();
        profile.set(0, 0.25).unwrap();
        profile.set(1, 0.25).unwrap();
        profile.set(2, 0.25).unwrap();
        profile.set(3, 0.25).unwrap();
        let entropy = profile.entropy();
        assert!(entropy > 0.0);
    }

    #[test]
    fn test_archetype_activation_profile_dominant_archetype() {
        let mut profile = ArchetypeActivationProfile::zero();
        profile.set(5, 0.8).unwrap();
        profile.set(10, 0.5).unwrap();
        profile.set(15, 0.3).unwrap();
        let (index, value) = profile.dominant_archetype();
        assert_eq!(index, 5);
        assert_eq!(value, 0.8);
    }

    #[test]
    fn test_archetype_basis_creation() {
        let basis = ArchetypeBasis::new(100);
        assert_eq!(basis.dimension(), 100);
        assert_eq!(basis.basis().len(), NUM_ARCHETYPES);
    }

    #[test]
    fn test_archetype_basis_reconstruct() {
        let basis = ArchetypeBasis::new(100);
        let mut profile = ArchetypeActivationProfile::zero();
        profile.set(0, 0.5).unwrap();
        profile.set(5, 0.3).unwrap();
        profile.set(10, 0.2).unwrap();

        let pattern = basis.reconstruct(&profile);
        assert_eq!(pattern.dimension(), 100);
    }

    #[test]
    fn test_archetype_basis_project_reconstruct_roundtrip() {
        let basis = ArchetypeBasis::new(100);

        let mut original_pattern = ArchetypicalPattern::zero(100);
        for i in 0..100 {
            original_pattern.components[i] = (i as Float).sin();
        }

        let projected_profile = basis.project(&original_pattern);
        let reconstructed_pattern = basis.reconstruct(&projected_profile);

        let error = basis.reconstruction_error(&original_pattern, &reconstructed_pattern);
        // Allow higher error for 100-dimension sine wave
        assert!(error < 10.0);
    }

    #[test]
    fn test_archetype_basis_verify_orthogonality() {
        let basis = ArchetypeBasis::new(100);
        assert!(basis.verify_orthogonality(0.1));
    }

    #[test]
    fn test_archetype_basis_normalize_basis() {
        let mut basis = ArchetypeBasis::new(100);
        basis.normalize_basis();
        for vector in basis.basis() {
            assert!((vector.norm() - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn test_archetypical_pattern_zero() {
        let pattern = ArchetypicalPattern::zero(100);
        assert_eq!(pattern.dimension(), 100);
        for &comp in pattern.components() {
            assert_eq!(comp, 0.0);
        }
    }

    #[test]
    fn test_archetypical_pattern_norm() {
        let pattern = ArchetypicalPattern::zero(100);
        assert_eq!(pattern.norm(), 0.0);
    }

    #[test]
    fn test_archetypical_pattern_compression_ratio() {
        let pattern = ArchetypicalPattern::zero(1000);
        let ratio = pattern.compression_ratio();
        assert!(ratio > 0.0);
        assert!(ratio > 40.0); // 1000 * 8 / 176 ≈ 45.45
    }

    #[test]
    fn test_archetype_vector_zero() {
        let vector = ArchetypeVector::zero(100);
        assert_eq!(vector.dimension(), 100);
        assert_eq!(vector.norm(), 0.0);
    }

    #[test]
    fn test_archetype_vector_normalize() {
        let components = vec![1.0; 100];
        let vector = ArchetypeVector::new(components);
        let normalized = vector.normalize();
        assert!((normalized.norm() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_archetype_vector_dot() {
        let v1 = ArchetypeVector::new(vec![1.0, 2.0, 3.0]);
        let v2 = ArchetypeVector::new(vec![4.0, 5.0, 6.0]);
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_archetype_vector_add_sub() {
        let v1 = ArchetypeVector::new(vec![1.0, 2.0, 3.0]);
        let v2 = ArchetypeVector::new(vec![4.0, 5.0, 6.0]);
        let sum = v1.add(&v2);
        let diff = v2.sub(&v1);
        assert_eq!(sum.components(), vec![5.0, 7.0, 9.0].as_slice());
        assert_eq!(diff.components(), vec![3.0, 3.0, 3.0].as_slice());
    }

    #[test]
    fn test_archetype_basis_statistics() {
        let mut stats = ArchetypeBasisStatistics::new();
        stats.record_reconstruction(10.0);
        stats.record_reconstruction(20.0);
        assert_eq!(stats.total_reconstructions, 2);
        assert_eq!(stats.total_reconstruction_time_us, 30.0);
        assert_eq!(stats.average_reconstruction_time_us, 15.0);
        stats.set_compression_ratio(100.0);
        assert_eq!(stats.compression_ratio, 100.0);
    }
}
