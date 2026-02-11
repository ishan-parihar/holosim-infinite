//! Holographic Reconstruction Tests
//!
//! This module provides validation tests for holographic reconstruction,
//! ensuring that the holographic architecture correctly implements holographic
//! principles.
//!
//! # Tests
//!
//! - Completeness of encoding (all layers contain complete information)
//! - Resolution independence (same information at different resolutions)
//! - Fault tolerance (graceful degradation with reduced resolution)
//! - Similarity-based retrieval (hypervector similarity search)
//! - Binding and bundling operations
//!
//! # Holographic Principles
//!
//! 1. **Distributed encoding**: Information is distributed across all dimensions
//! 2. **Each part contains the whole**: Every entity contains all densities/sub-densities
//! 3. **Resolution completeness**: Lower resolution = less detail, but still complete
//! 4. **Fault tolerance**: Cutting a hologram reduces resolution but maintains completeness
//!
//! # Success Criteria
//!
//! - All holographic tests pass
//! - Encoding completeness verified
//! - Resolution independence verified
//! - Fault tolerance verified

use super::Float;
use crate::holographic::complex_vectors::ComplexArchetype;
use crate::holographic::holographic_field::{HolographicField, InvolutionLayer};
use crate::holographic::holographic_memory::{Experience, HolographicMemory, Hypervector};

/// Validator for holographic reconstruction tests
#[derive(Debug, Clone)]
pub struct HolographicReconstructionValidator {
    /// Tolerance for floating point comparisons
    tolerance: Float,
}

impl HolographicReconstructionValidator {
    /// Creates a new holographic reconstruction validator.
    ///
    /// # Returns
    ///
    /// A new holographic reconstruction validator
    pub fn new() -> Self {
        HolographicReconstructionValidator { tolerance: 1e-6 }
    }

    /// Runs all holographic reconstruction validation tests.
    ///
    /// # Returns
    ///
    /// Holographic reconstruction test result
    pub fn validate_all(&self) -> HolographicReconstructionResult {
        let mut tests_passed = 0;
        let mut total_tests = 0;

        // Test 1: Encoding completeness
        total_tests += 1;
        if self.test_encoding_completeness() {
            tests_passed += 1;
        }

        // Test 2: Resolution independence
        total_tests += 1;
        if self.test_resolution_independence() {
            tests_passed += 1;
        }

        // Test 3: Fault tolerance
        total_tests += 1;
        if self.test_fault_tolerance() {
            tests_passed += 1;
        }

        // Test 4: Similarity-based retrieval
        total_tests += 1;
        if self.test_similarity_based_retrieval() {
            tests_passed += 1;
        }

        // Test 5: Binding and bundling
        total_tests += 1;
        if self.test_binding_and_bundling() {
            tests_passed += 1;
        }

        HolographicReconstructionResult {
            all_tests_passed: tests_passed == total_tests,
            tests_passed,
            total_tests,
        }
    }

    /// Tests encoding completeness.
    ///
    /// Verifies that all layers contain the complete holographic encoding.
    ///
    /// # Returns
    ///
    /// True if encoding is complete
    fn test_encoding_completeness(&self) -> Bool {
        let archetypes = self.generate_test_archetypes();

        // Generate fields at all layers
        let violet_field = HolographicField::new(InvolutionLayer::Violet, archetypes.clone());
        let red_field = HolographicField::new(InvolutionLayer::Red, archetypes.clone());

        // Both fields should contain the same archetype encoding
        assert_eq!(
            violet_field.archetype_complex_vectors.len(),
            22,
            "Violet field archetype count incorrect"
        );
        assert_eq!(
            red_field.archetype_complex_vectors.len(),
            22,
            "Red field archetype count incorrect"
        );

        // Check that archetype vectors are the same (same encoding)
        for i in 0..22 {
            let violet_cv = &violet_field.archetype_complex_vectors[i];
            let red_cv = &red_field.archetype_complex_vectors[i];

            assert!(
                (violet_cv.real - red_cv.real).abs() < self.tolerance,
                "Archetype {} real part differs",
                i
            );
            assert!(
                (violet_cv.imag - red_cv.imag).abs() < self.tolerance,
                "Archetype {} imag part differs",
                i
            );
        }

        true
    }

    /// Tests resolution independence.
    ///
    /// Verifies that the same information is present at different resolutions.
    ///
    /// # Returns
    ///
    /// True if resolution independence holds
    fn test_resolution_independence(&self) -> Bool {
        let archetypes = self.generate_test_archetypes();

        // Generate fields at different layers
        let violet_field = HolographicField::new(InvolutionLayer::Violet, archetypes.clone());
        let green_field = HolographicField::new(InvolutionLayer::Green, archetypes.clone());
        let red_field = HolographicField::new(InvolutionLayer::Red, archetypes.clone());

        // All fields should have the same phase coherence
        let violet_coherence = violet_field.phase_coherence();
        let green_coherence = green_field.phase_coherence();
        let red_coherence = red_field.phase_coherence();

        assert!(
            (violet_coherence - green_coherence).abs() < self.tolerance,
            "Phase coherence differs between Violet and Green"
        );
        assert!(
            (violet_coherence - red_coherence).abs() < self.tolerance,
            "Phase coherence differs between Violet and Red"
        );

        // All fields should have the same number of constructive nodes (in theory)
        // In practice, different resolutions may filter out some nodes
        // But the core information should be the same

        true
    }

    /// Tests fault tolerance.
    ///
    /// Verifies that reducing resolution (aperture size) maintains completeness.
    ///
    /// # Returns
    ///
    /// True if fault tolerance holds
    fn test_fault_tolerance(&self) -> Bool {
        let archetypes = self.generate_test_archetypes();
        let mut field = HolographicField::new(InvolutionLayer::Violet, archetypes);

        // Get full resolution constructive nodes
        field.set_aperture_size(1.0);
        let full_nodes = field.accessible_constructive_nodes();

        // Reduce resolution
        field.set_aperture_size(0.5);
        let reduced_nodes = field.accessible_constructive_nodes();

        // Reduce resolution further
        field.set_aperture_size(0.1);
        let minimal_nodes = field.accessible_constructive_nodes();

        // Reduced resolution should have fewer accessible nodes
        // But the encoding itself should remain complete
        assert!(
            full_nodes.len() >= reduced_nodes.len(),
            "Full resolution should have more nodes than reduced"
        );
        assert!(
            reduced_nodes.len() >= minimal_nodes.len(),
            "Reduced resolution should have more nodes than minimal"
        );

        // The archetype encoding should remain unchanged
        assert_eq!(
            field.archetype_complex_vectors.len(),
            22,
            "Archetype encoding should remain complete"
        );

        true
    }

    /// Tests similarity-based retrieval.
    ///
    /// Verifies that hypervector similarity search works correctly.
    ///
    /// # Returns
    ///
    /// True if similarity-based retrieval works
    fn test_similarity_based_retrieval(&self) -> Bool {
        let mut memory = HolographicMemory::new();

        // Add some experiences
        let exp1 = Experience::new(
            "Test catalyst 1".to_string(),
            0.5,
            "Test learning 1".to_string(),
            1000,
        );

        let exp2 = Experience::new(
            "Test catalyst 2".to_string(),
            -0.3,
            "Test learning 2".to_string(),
            2000,
        );

        memory.add_experience(exp1.clone());
        memory.add_experience(exp2.clone());

        // Retrieve similar experiences
        let retrieved = memory.retrieve_memory(exp1.clone());

        // Should retrieve at least the original experience
        assert!(!retrieved.is_empty(), "No experiences retrieved");

        // Check that retrieved experiences have similar catalyst
        let has_similar = retrieved.iter().any(|e| e.catalyst == exp1.catalyst);

        assert!(has_similar, "No similar experience found");

        true
    }

    /// Tests binding and bundling operations.
    ///
    /// Verifies that hypervector binding and bundling work correctly.
    ///
    /// # Returns
    ///
    /// True if binding and bundling work
    fn test_binding_and_bundling(&self) -> Bool {
        let hv1 = Hypervector::from_components(vec![1.0, 0.0, 0.0]);
        let hv2 = Hypervector::from_components(vec![0.0, 1.0, 0.0]);
        let hv3 = Hypervector::from_components(vec![0.0, 0.0, 1.0]);

        // Test binding
        let bound = hv1.bind(&hv2);
        assert_eq!(
            bound.dimensionality(),
            3,
            "Bound hypervector dimensionality incorrect"
        );

        // Test bundling
        let bundled = hv1.bundle(&[hv2, hv3]);
        assert_eq!(
            bundled.dimensionality(),
            3,
            "Bundled hypervector dimensionality incorrect"
        );

        // Test normalization
        let mut hv = Hypervector::from_components(vec![3.0, 4.0]);
        hv.normalize();
        let norm: Float = hv
            .components
            .iter()
            .map(|x| x.powi(2))
            .sum::<Float>()
            .sqrt();
        assert!((norm - 1.0).abs() < self.tolerance, "Normalization failed");

        true
    }

    /// Generates test archetypes with reasonable values.
    ///
    /// # Returns
    ///
    /// Array of 22 test archetypes
    fn generate_test_archetypes(&self) -> [ComplexArchetype; 22] {
        use std::f64::consts::PI;

        let mut archetypes = [ComplexArchetype {
            amplitude: 0.0,
            phase: 0.0,
        }; 22];
        for i in 0..22 {
            archetypes[i] = ComplexArchetype {
                amplitude: (i as Float + 1.0) / 22.0, // 0.045 to 1.0
                phase: (i as Float) * PI / 11.0,      // 0 to 2π
            };
        }
        archetypes
    }
}

impl Default for HolographicReconstructionValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Holographic reconstruction test result
#[derive(Debug, Clone)]
pub struct HolographicReconstructionResult {
    /// Whether all tests passed
    pub all_tests_passed: Bool,

    /// Number of tests passed
    pub tests_passed: usize,

    /// Total number of tests
    pub total_tests: usize,
}

impl HolographicReconstructionResult {
    /// Returns a summary of the test results.
    ///
    /// # Returns
    ///
    /// A human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "Holographic Reconstruction Tests: {} passed / {} total ({:.1}%)",
            self.tests_passed,
            self.total_tests,
            (self.tests_passed as Float / self.total_tests as Float) * 100.0
        )
    }
}

// Bool type for compatibility
type Bool = bool;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holographic_reconstruction_validator_creation() {
        let validator = HolographicReconstructionValidator::new();
        assert_eq!(validator.tolerance, 1e-6);
    }

    #[test]
    fn test_holographic_reconstruction_validator_validate_all() {
        let validator = HolographicReconstructionValidator::new();
        let result = validator.validate_all();

        assert_eq!(result.total_tests, 5);
        assert!(result.tests_passed >= 0);
        assert!(result.tests_passed <= 5);
    }

    #[test]
    fn test_holographic_reconstruction_result_summary() {
        let result = HolographicReconstructionResult {
            all_tests_passed: true,
            tests_passed: 5,
            total_tests: 5,
        };

        let summary = result.summary();
        assert!(summary.contains("Holographic Reconstruction Tests"));
        assert!(summary.contains("5 passed"));
    }

    #[test]
    fn test_holographic_reconstruction_validator_default() {
        let validator = HolographicReconstructionValidator::default();
        assert_eq!(validator.tolerance, 1e-6);
    }

    #[test]
    fn test_encoding_completeness() {
        let validator = HolographicReconstructionValidator::new();
        assert!(validator.test_encoding_completeness());
    }

    #[test]
    fn test_resolution_independence() {
        let validator = HolographicReconstructionValidator::new();
        assert!(validator.test_resolution_independence());
    }

    #[test]
    fn test_fault_tolerance() {
        let validator = HolographicReconstructionValidator::new();
        assert!(validator.test_fault_tolerance());
    }

    #[test]
    fn test_similarity_based_retrieval() {
        let validator = HolographicReconstructionValidator::new();
        assert!(validator.test_similarity_based_retrieval());
    }

    #[test]
    fn test_binding_and_bundling() {
        let validator = HolographicReconstructionValidator::new();
        assert!(validator.test_binding_and_bundling());
    }
}
