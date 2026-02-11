//! Architecture Tests for Holographic Architecture
//!
//! This module provides comprehensive validation tests for the holographic
//! architecture itself, ensuring that core functionality works correctly.
//!
//! # Tests
//!
//! - Complex vector operations (amplitude, phase, arithmetic)
//! - Interference pattern generation (cross-terms, nodes)
//! - Spatial frequency calculations (Violet → Red gradient)
//! - Phase coherence measurements
//! - Oscillatory synchronization (Kuramoto model)
//! - Holographic field generation
//! - Holographic entity views (Mind, Body, Spirit)
//! - Energy center access
//!
//! # Success Criteria
//!
//! - All architecture tests pass
//! - Complex arithmetic is accurate
//! - Interference patterns are generated correctly
//! - Spatial frequencies match specifications
//! - Phase coherence is measured accurately
//! - Oscillators synchronize correctly
//! - All views are consistent

use super::Float;
use crate::holographic::complex_vectors::{ComplexArchetype, ComplexVector};
use crate::holographic::holographic_entity::HolographicEntity;
use crate::holographic::holographic_field::{HolographicField, InvolutionLayer};
use crate::holographic::interference_pattern::InterferencePattern;
use crate::holographic::oscillator_network::OscillatorNetwork;

/// Validator for holographic architecture tests
#[derive(Debug, Clone)]
pub struct ArchitectureValidator {
    /// Test tolerance for floating point comparisons
    tolerance: Float,
}

impl ArchitectureValidator {
    /// Creates a new architecture validator.
    ///
    /// # Returns
    ///
    /// A new architecture validator with default tolerance
    pub fn new() -> Self {
        ArchitectureValidator { tolerance: 1e-10 }
    }

    /// Runs all architecture validation tests.
    ///
    /// # Returns
    ///
    /// Architecture test result
    pub fn validate_all(&self) -> ArchitectureTestResult {
        let mut tests_passed = 0;
        let mut total_tests = 0;

        // Test 1: Complex vector operations
        total_tests += 1;
        if self.test_complex_vector_arithmetic() {
            tests_passed += 1;
        }

        // Test 2: Interference pattern generation
        total_tests += 1;
        if self.test_interference_pattern_generation() {
            tests_passed += 1;
        }

        // Test 3: Spatial frequency calculations
        total_tests += 1;
        if self.test_spatial_frequency_gradient() {
            tests_passed += 1;
        }

        // Test 4: Phase coherence measurement
        total_tests += 1;
        if self.test_phase_coherence() {
            tests_passed += 1;
        }

        // Test 5: Oscillatory synchronization
        total_tests += 1;
        if self.test_oscillatory_synchronization() {
            tests_passed += 1;
        }

        // Test 6: Holographic field generation
        total_tests += 1;
        if self.test_holographic_field_generation() {
            tests_passed += 1;
        }

        // Test 7: Holographic entity views
        total_tests += 1;
        if self.test_holographic_entity_views() {
            tests_passed += 1;
        }

        // Test 8: Energy center access
        total_tests += 1;
        if self.test_energy_center_access() {
            tests_passed += 1;
        }

        ArchitectureTestResult {
            all_passed: tests_passed == total_tests,
            tests_passed,
            total_tests,
            details: vec![],
        }
    }

    /// Tests complex vector arithmetic operations.
    ///
    /// # Returns
    ///
    /// True if all operations are correct
    fn test_complex_vector_arithmetic(&self) -> Bool {
        use std::f64::consts::PI;

        // Test addition
        let a = ComplexVector::from_amplitude_phase(1.0, 0.0);
        let b = ComplexVector::from_amplitude_phase(1.0, 0.0);
        let c = a.add(&b);
        assert!(
            (c.amplitude() - 2.0).abs() < self.tolerance,
            "Addition failed"
        );

        // Test multiplication
        let a = ComplexVector::from_amplitude_phase(1.0, PI / 4.0);
        let b = ComplexVector::from_amplitude_phase(1.0, PI / 4.0);
        let c = a.multiply(&b);
        assert!(
            (c.amplitude() - 1.0).abs() < self.tolerance,
            "Multiplication amplitude failed"
        );
        assert!(
            (c.phase() - PI / 2.0).abs() < self.tolerance,
            "Multiplication phase failed"
        );

        // Test conjugate
        let cv = ComplexVector::from_amplitude_phase(1.0, PI / 4.0);
        let conj = cv.conjugate();
        assert!(
            (conj.phase() + PI / 4.0).abs() < self.tolerance,
            "Conjugate failed"
        );

        true
    }

    /// Tests interference pattern generation.
    ///
    /// # Returns
    ///
    /// True if interference pattern is generated correctly
    fn test_interference_pattern_generation(&self) -> Bool {
        let archetypes = self.generate_test_archetypes();
        // Convert ComplexArchetype to ComplexVector
        let archetype_vectors: [ComplexVector; 22] = archetypes.map(|a| a.to_complex_vector());
        let pattern = InterferencePattern::from_archetypes(&archetype_vectors);

        // Check cross-terms
        assert_eq!(
            pattern.cross_terms.matrix.len(),
            22,
            "Cross-term matrix size incorrect"
        );

        // Check constructive and destructive nodes
        assert!(
            pattern.constructive_nodes.len() > 0,
            "No constructive nodes found"
        );
        assert!(
            pattern.destructive_nodes.len() > 0,
            "No destructive nodes found"
        );

        // Check cross-terms are non-zero
        for i in 0..22 {
            for j in 0..22 {
                if i != j {
                    let cross_term = pattern.cross_terms.matrix[i][j];
                    assert!(cross_term.norm_squared() > 0.0, "Cross-term is zero");
                }
            }
        }

        true
    }

    /// Tests spatial frequency calculations.
    ///
    /// # Returns
    ///
    /// True if spatial frequencies are correct
    fn test_spatial_frequency_gradient(&self) -> Bool {
        // Check specific values
        assert_eq!(
            InvolutionLayer::Violet.spatial_frequency(),
            5600.0,
            "Violet frequency incorrect"
        );
        assert_eq!(
            InvolutionLayer::Indigo.spatial_frequency(),
            4000.0,
            "Indigo frequency incorrect"
        );
        assert_eq!(
            InvolutionLayer::Blue.spatial_frequency(),
            2800.0,
            "Blue frequency incorrect"
        );
        assert_eq!(
            InvolutionLayer::Green.spatial_frequency(),
            2000.0,
            "Green frequency incorrect"
        );
        assert_eq!(
            InvolutionLayer::Yellow.spatial_frequency(),
            1400.0,
            "Yellow frequency incorrect"
        );
        assert_eq!(
            InvolutionLayer::Orange.spatial_frequency(),
            1000.0,
            "Orange frequency incorrect"
        );
        assert_eq!(
            InvolutionLayer::Red.spatial_frequency(),
            100.0,
            "Red frequency incorrect"
        );

        // Check gradient (should decrease from Violet to Red)
        let layers = InvolutionLayer::all_layers();
        for i in 0..layers.len() - 1 {
            assert!(
                layers[i].spatial_frequency() > layers[i + 1].spatial_frequency(),
                "Spatial frequency gradient incorrect"
            );
        }

        true
    }

    /// Tests phase coherence measurement.
    ///
    /// # Returns
    ///
    /// True if phase coherence is measured correctly
    fn test_phase_coherence(&self) -> Bool {
        let archetypes = self.generate_test_archetypes();
        // Convert ComplexArchetype to ComplexVector
        let archetype_vectors: [ComplexVector; 22] = archetypes.map(|a| a.to_complex_vector());
        let pattern = InterferencePattern::from_archetypes(&archetype_vectors);

        let coherence = pattern.phase_coherence();

        // Coherence should be between 0.0 and 1.0
        assert!(coherence >= 0.0, "Phase coherence negative");
        assert!(coherence <= 1.0, "Phase coherence > 1.0");

        true
    }

    /// Tests oscillatory synchronization.
    ///
    /// # Returns
    ///
    /// True if oscillators synchronize correctly
    fn test_oscillatory_synchronization(&self) -> Bool {
        let archetypes = self.generate_test_archetypes();
        let mut network = OscillatorNetwork::from_archetypes(&archetypes);

        let old_sync_state = network.synchronization_state();
        network.synchronize();
        let new_sync_state = network.synchronization_state();

        // Check that synchronization state changed
        let state_changed = (old_sync_state.mean_phase - new_sync_state.mean_phase).abs()
            > self.tolerance
            || (old_sync_state.phase_coherence - new_sync_state.phase_coherence).abs()
                > self.tolerance;

        assert!(state_changed, "Synchronization state did not change");

        // Check phase coherence is between 0.0 and 1.0
        assert!(
            new_sync_state.phase_coherence >= 0.0,
            "Phase coherence negative"
        );
        assert!(
            new_sync_state.phase_coherence <= 1.0,
            "Phase coherence > 1.0"
        );

        // Check center activations are between 0.0 and 1.0
        for activation in &new_sync_state.center_activation {
            assert!(*activation >= 0.0, "Center activation negative");
            assert!(*activation <= 1.0, "Center activation > 1.0");
        }

        true
    }

    /// Tests holographic field generation.
    ///
    /// # Returns
    ///
    /// True if fields are generated correctly
    fn test_holographic_field_generation(&self) -> Bool {
        let archetypes = self.generate_test_archetypes();
        let mut field = HolographicField::new(InvolutionLayer::Violet, archetypes);

        assert_eq!(field.layer, InvolutionLayer::Violet, "Layer incorrect");
        assert_eq!(
            field.spatial_frequency, 5600.0,
            "Spatial frequency incorrect"
        );
        assert_eq!(
            field.archetype_complex_vectors.len(),
            22,
            "Archetype count incorrect"
        );

        // Test effective resolution
        field.set_aperture_size(0.5);
        assert!(
            (field.effective_resolution() - 2800.0).abs() < self.tolerance,
            "Effective resolution incorrect"
        );

        // Test aperture size clamping
        field.set_aperture_size(1.5);
        assert_eq!(field.aperture_size, 1.0, "Aperture size not clamped to 1.0");

        field.set_aperture_size(-0.5);
        assert_eq!(field.aperture_size, 0.0, "Aperture size not clamped to 0.0");

        true
    }

    /// Tests holographic entity views.
    ///
    /// # Returns
    ///
    /// True if all views are consistent
    fn test_holographic_entity_views(&self) -> Bool {
        let archetypes = self.generate_test_archetypes();
        let entity = HolographicEntity::new(archetypes);

        // Test all views exist
        let mind_view = entity.mind_view();
        let body_view = entity.body_view();
        let spirit_view = entity.spirit_view();

        // Check view sizes
        assert_eq!(
            mind_view.information_processing.mind_activations.len(),
            7,
            "Mind view size incorrect"
        );
        assert_eq!(
            body_view.material_manifestation.body_activations.len(),
            7,
            "Body view size incorrect"
        );
        assert_eq!(
            spirit_view.source_connection.spirit_activations.len(),
            7,
            "Spirit view size incorrect"
        );

        // Check phase coherence is consistent
        let entity_coherence = entity.phase_coherence();
        let mind_coherence = mind_view.information_processing.coherence;
        let body_integrity = body_view.material_manifestation.integrity;
        let spirit_connection = spirit_view.source_connection.connection_strength;

        // All should be consistent with entity phase coherence
        assert!(
            (entity_coherence - mind_coherence).abs() < self.tolerance,
            "Mind coherence inconsistent"
        );
        assert!(
            (entity_coherence - body_integrity).abs() < self.tolerance,
            "Body integrity inconsistent"
        );
        assert!(
            (entity_coherence - spirit_connection).abs() < self.tolerance,
            "Spirit connection inconsistent"
        );

        true
    }

    /// Tests energy center access.
    ///
    /// # Returns
    ///
    /// True if energy centers are accessible correctly
    fn test_energy_center_access(&self) -> Bool {
        let archetypes = self.generate_test_archetypes();
        let entity = HolographicEntity::new(archetypes);

        // Test Red center (index 0)
        let red_field = entity.access_energy_center(0);
        assert!(red_field.is_some(), "Red center not accessible");
        assert_eq!(
            red_field.unwrap().layer,
            InvolutionLayer::Red,
            "Red center layer incorrect"
        );

        // Test Violet center (index 6)
        let violet_field = entity.access_energy_center(6);
        assert!(violet_field.is_some(), "Violet center not accessible");
        assert_eq!(
            violet_field.unwrap().layer,
            InvolutionLayer::Violet,
            "Violet center layer incorrect"
        );

        // Test invalid center (index 7)
        let invalid_field = entity.access_energy_center(7);
        assert!(
            invalid_field.is_none(),
            "Invalid center should not be accessible"
        );

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

impl Default for ArchitectureValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Architecture test result
#[derive(Debug, Clone)]
pub struct ArchitectureTestResult {
    /// Whether all tests passed
    pub all_passed: Bool,

    /// Number of tests passed
    pub tests_passed: usize,

    /// Total number of tests
    pub total_tests: usize,

    /// Detailed test results
    pub details: Vec<String>,
}

impl ArchitectureTestResult {
    /// Returns a summary of the test results.
    ///
    /// # Returns
    ///
    /// A human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "Architecture Tests: {} passed / {} total ({:.1}%)",
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
    fn test_architecture_validator_creation() {
        let validator = ArchitectureValidator::new();
        assert_eq!(validator.tolerance, 1e-10);
    }

    #[test]
    fn test_architecture_validator_validate_all() {
        let validator = ArchitectureValidator::new();
        let result = validator.validate_all();

        assert_eq!(result.total_tests, 8);
        assert!(result.tests_passed >= 0);
        assert!(result.tests_passed <= 8);
    }

    #[test]
    fn test_architecture_test_result_summary() {
        let result = ArchitectureTestResult {
            all_passed: true,
            tests_passed: 8,
            total_tests: 8,
            details: vec![],
        };

        let summary = result.summary();
        assert!(summary.contains("Architecture Tests"));
        assert!(summary.contains("8 passed"));
    }

    #[test]
    fn test_architecture_validator_default() {
        let validator = ArchitectureValidator::default();
        assert_eq!(validator.tolerance, 1e-10);
    }
}
