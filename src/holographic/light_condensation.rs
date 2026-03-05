//! Light Condensation Engine
//!
//! This module implements the light condensation process through involution layers.
//! Light descends from the Blue Realm (Law of Light) through Green, Yellow, Orange,
//! and finally condenses into matter at the Red layer.
//!
//! # Core Concepts
//!
//! - **Light Condensation**: The process of spatial frequency reduction (Blue → Red)
//! - **Green-Ray**: Application of dimensionality (Space/Time topology)
//! - **Yellow-Ray**: Application of physical laws (local constraints)
//! - **Orange-Ray**: Application of soul stream (individualization)
//! - **Red-Ray**: Condensation into matter (densest packing)
//!
//! # The Descent Blue → Red
//!
//! The descent is NOT energy decrease, but SPATIAL FREQUENCY REDUCTION:
//! - Blue (2800 lines/mm): Law of Light - where the interference pattern is impressed
//! - Green (2000 lines/mm): Dimensionality applied
//! - Yellow (1400 lines/mm): Physical laws applied
//! - Orange (1000 lines/mm): Soul stream applied
//! - Red (100 lines/mm): Condensed into matter
//!
//! # Archetype Encoding Preservation
//!
//! During condensation, the COMPLETE holographic encoding is preserved.
//! Only spatial frequency changes, not the content of the encoding.
//!
//! # Holographic Architecture
//!
//! "Reality is not constructed; it is Unfolded from a Pre-Existing Whole"

use crate::holographic::complex_vectors::ComplexArchetype;
use crate::holographic::holographic_field::{
    HolographicField, HolographicFieldGenerator, InvolutionLayer,
};
#[cfg(test)]
use crate::types::Float;

/// Result of the light condensation process.
///
/// Contains the holographic fields at each stage of condensation.
#[derive(Clone, Debug)]
pub struct CondensationResult {
    /// Green-Ray field (dimensionality applied)
    pub green_field: HolographicField,

    /// Yellow-Ray field (physical laws applied)
    pub yellow_field: HolographicField,

    /// Orange-Ray field (soul stream applied)
    pub orange_field: HolographicField,

    /// Red-Ray field (condensed into matter)
    pub red_field: HolographicField,
}

impl CondensationResult {
    /// Returns a summary of the condensation result.
    ///
    /// # Returns
    ///
    /// A human-readable summary of the condensation process
    pub fn summary(&self) -> String {
        format!(
            "Light Condensation Result:\n\
             \n\
             Green-Ray (Dimensionality):\n\
             {}\n\
             \n\
             Yellow-Ray (Physical Laws):\n\
             {}\n\
             \n\
             Orange-Ray (Soul Stream):\n\
             {}\n\
             \n\
             Red-Ray (Matter):\n\
             {}",
            self.green_field.summary(),
            self.yellow_field.summary(),
            self.orange_field.summary(),
            self.red_field.summary()
        )
    }

    /// Validates that spatial frequency decreased correctly during condensation.
    ///
    /// # Returns
    ///
    /// True if spatial frequency decreased correctly, false otherwise
    pub fn validate_spatial_frequency_gradient(&self) -> bool {
        self.green_field.spatial_frequency > self.yellow_field.spatial_frequency
            && self.yellow_field.spatial_frequency > self.orange_field.spatial_frequency
            && self.orange_field.spatial_frequency > self.red_field.spatial_frequency
    }

    /// Validates that archetype encoding was preserved during condensation.
    ///
    /// # Returns
    ///
    /// True if archetype encoding is preserved, false otherwise
    pub fn validate_archetype_preservation(&self) -> bool {
        let tolerance = 1e-10;

        for i in 0..22 {
            let green = &self.green_field.archetype_complex_vectors[i];
            let yellow = &self.yellow_field.archetype_complex_vectors[i];
            let orange = &self.orange_field.archetype_complex_vectors[i];
            let red = &self.red_field.archetype_complex_vectors[i];

            // All fields should have the same archetype encoding
            if (green.real - yellow.real).abs() > tolerance
                || (green.imag - yellow.imag).abs() > tolerance
                || (green.real - orange.real).abs() > tolerance
                || (green.imag - orange.imag).abs() > tolerance
                || (green.real - red.real).abs() > tolerance
                || (green.imag - red.imag).abs() > tolerance
            {
                return false;
            }
        }

        true
    }
}

/// Light Condensation Engine
///
/// Simulates the descent of light from the Blue Realm through involution layers,
/// condensing into matter at the Red layer.
///
/// # The Condensation Process
///
/// 1. **Blue Realm (Law of Light)**: The interference pattern is impressed
/// 2. **Green-Ray (Dimensionality)**: Space/Time topology applied
/// 3. **Yellow-Ray (Physical Laws)**: Local constraints applied
/// 4. **Orange-Ray (Soul Stream)**: Individualization applied
/// 5. **Red-Ray (Matter)**: Condensed into densest packing
///
/// # Key Principle
///
/// "Reality is not constructed; it is Unfolded from a Pre-Existing Whole"
///
/// The holographic encoding is PRESERVED during condensation.
/// Only spatial frequency changes, not the content of the encoding.
#[derive(Clone, Debug)]
pub struct LightCondensationEngine {
    /// The holographic field generator for creating fields at different layers
    pub holographic_field_generator: HolographicFieldGenerator,
}

impl LightCondensationEngine {
    /// Creates a new light condensation engine.
    ///
    /// # Arguments
    ///
    /// * `archetypes` - The 22 archetype complex vectors (holographic encoding)
    ///
    /// # Returns
    ///
    /// A new light condensation engine
    pub fn new(archetypes: [ComplexArchetype; 22]) -> Self {
        let mut generator = HolographicFieldGenerator::new();
        generator.set_archetypes(archetypes);

        LightCondensationEngine {
            holographic_field_generator: generator,
        }
    }

    /// Condenses light from the Blue Realm (Law of Light) through involution layers.
    ///
    /// This is the main method that simulates the descent of light:
    /// 1. Apply Dimensionality (Green-Ray)
    /// 2. Apply Physical Laws (Yellow-Ray)
    /// 3. Apply Soul Stream (Orange-Ray)
    /// 4. Condense into Matter (Red-Ray)
    ///
    /// # Returns
    ///
    /// A CondensationResult containing the holographic fields at each stage
    pub fn condense_from_blue_realm(&self) -> CondensationResult {
        println!("Starting light condensation from Blue Realm (Law of Light)");

        // Step 1: Apply Dimensionality (Green-Ray)
        let green_field = self.apply_dimensionality();
        println!(
            "Applied dimensionality - Spatial frequency: {} lines/mm",
            green_field.spatial_frequency
        );

        // Step 2: Apply Physical Laws (Yellow-Ray)
        let yellow_field = self.apply_physical_laws(&green_field);
        println!(
            "Applied physical laws - Spatial frequency: {} lines/mm",
            yellow_field.spatial_frequency
        );

        // Step 3: Apply Soul Stream (Orange-Ray)
        let orange_field = self.apply_soul_stream(&yellow_field);
        println!(
            "Applied soul stream - Spatial frequency: {} lines/mm",
            orange_field.spatial_frequency
        );

        // Step 4: Condense into Matter (Red-Ray)
        let red_field = self.condense_into_matter(&orange_field);
        println!(
            "Condensed into matter - Spatial frequency: {} lines/mm",
            red_field.spatial_frequency
        );

        CondensationResult {
            green_field,
            yellow_field,
            orange_field,
            red_field,
        }
    }

    /// Applies dimensionality (Green-Ray) to the holographic field.
    ///
    /// Green-Ray applies Space/Time topology, reducing spatial frequency
    /// from Blue (2800 lines/mm) to Green (2000 lines/mm).
    ///
    /// # Returns
    ///
    /// A holographic field at the Green layer with dimensionality applied
    fn apply_dimensionality(&self) -> HolographicField {
        // Start with Blue field (Law of Light)
        let blue_field = self
            .holographic_field_generator
            .generate_field(InvolutionLayer::Blue)
            .expect("Blue field not found");

        // Create Green field
        let mut green_field = self
            .holographic_field_generator
            .generate_field(InvolutionLayer::Green)
            .expect("Green field not found");

        // Preserve archetype encoding but reduce spatial frequency
        green_field.archetype_complex_vectors = blue_field.archetype_complex_vectors;

        green_field
    }

    /// Applies physical laws (Yellow-Ray) to the holographic field.
    ///
    /// Yellow-Ray applies local constraints, reducing spatial frequency
    /// from Green (2000 lines/mm) to Yellow (1400 lines/mm).
    ///
    /// # Arguments
    ///
    /// * `field` - The holographic field at the Green layer
    ///
    /// # Returns
    ///
    /// A holographic field at the Yellow layer with physical laws applied
    fn apply_physical_laws(&self, field: &HolographicField) -> HolographicField {
        // Create Yellow field
        let mut yellow_field = self
            .holographic_field_generator
            .generate_field(InvolutionLayer::Yellow)
            .expect("Yellow field not found");

        // Preserve archetype encoding but reduce spatial frequency
        yellow_field.archetype_complex_vectors = field.archetype_complex_vectors;

        yellow_field
    }

    /// Applies soul stream (Orange-Ray) to the holographic field.
    ///
    /// Orange-Ray applies individualization, reducing spatial frequency
    /// from Yellow (1400 lines/mm) to Orange (1000 lines/mm).
    ///
    /// # Arguments
    ///
    /// * `field` - The holographic field at the Yellow layer
    ///
    /// # Returns
    ///
    /// A holographic field at the Orange layer with soul stream applied
    fn apply_soul_stream(&self, field: &HolographicField) -> HolographicField {
        // Create Orange field
        let mut orange_field = self
            .holographic_field_generator
            .generate_field(InvolutionLayer::Orange)
            .expect("Orange field not found");

        // Preserve archetype encoding but reduce spatial frequency
        orange_field.archetype_complex_vectors = field.archetype_complex_vectors;

        orange_field
    }

    /// Condenses the holographic field into matter (Red-Ray).
    ///
    /// Red-Ray condenses into the densest packing, reducing spatial frequency
    /// from Orange (1000 lines/mm) to Red (100 lines/mm).
    ///
    /// # Arguments
    ///
    /// * `field` - The holographic field at the Orange layer
    ///
    /// # Returns
    ///
    /// A holographic field at the Red layer (condensed into matter)
    fn condense_into_matter(&self, field: &HolographicField) -> HolographicField {
        // Create Red field
        let mut red_field = self
            .holographic_field_generator
            .generate_field(InvolutionLayer::Red)
            .expect("Red field not found");

        // Preserve archetype encoding but reduce spatial frequency
        red_field.archetype_complex_vectors = field.archetype_complex_vectors;

        red_field
    }

    /// Generates test archetypes for unit testing.
    ///
    /// # Returns
    ///
    /// An array of 22 test archetype complex vectors
    #[cfg(test)]
    pub fn generate_test_archetypes() -> [ComplexArchetype; 22] {
        let mut archetypes = Vec::new();

        for i in 0..22 {
            let amplitude = 1.0 + (i as Float) * 0.1;
            let phase = (i as Float) * 0.1 * std::f64::consts::PI;
            archetypes.push(ComplexArchetype::new(amplitude, phase));
        }

        archetypes.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_condensation_engine_creation() {
        let archetypes = LightCondensationEngine::generate_test_archetypes();
        let engine = LightCondensationEngine::new(archetypes);

        // Engine should have a field generator
        assert!(engine.holographic_field_generator.base_archetypes.is_some());
    }

    #[test]
    fn test_light_condensation() {
        let archetypes = LightCondensationEngine::generate_test_archetypes();
        let engine = LightCondensationEngine::new(archetypes);

        let result = engine.condense_from_blue_realm();

        // Check that spatial frequency decreases
        assert!(
            result.green_field.spatial_frequency > result.yellow_field.spatial_frequency,
            "Green spatial frequency should be greater than Yellow"
        );
        assert!(
            result.yellow_field.spatial_frequency > result.orange_field.spatial_frequency,
            "Yellow spatial frequency should be greater than Orange"
        );
        assert!(
            result.orange_field.spatial_frequency > result.red_field.spatial_frequency,
            "Orange spatial frequency should be greater than Red"
        );
    }

    #[test]
    fn test_spatial_frequency_gradient_validation() {
        let archetypes = LightCondensationEngine::generate_test_archetypes();
        let engine = LightCondensationEngine::new(archetypes);

        let result = engine.condense_from_blue_realm();

        // Validate spatial frequency gradient
        assert!(result.validate_spatial_frequency_gradient());
    }

    #[test]
    fn test_archetype_preservation() {
        let archetypes = LightCondensationEngine::generate_test_archetypes();
        let engine = LightCondensationEngine::new(archetypes);

        let result = engine.condense_from_blue_realm();

        // Check that archetype encoding is preserved
        let green_archetypes = result.green_field.archetype_complex_vectors;
        let yellow_archetypes = result.yellow_field.archetype_complex_vectors;
        let orange_archetypes = result.orange_field.archetype_complex_vectors;
        let red_archetypes = result.red_field.archetype_complex_vectors;

        for i in 0..22 {
            // All fields should have the same archetype encoding
            assert!(
                (green_archetypes[i].real - yellow_archetypes[i].real).abs() < 1e-10,
                "Green and Yellow archetype {} should have the same real part",
                i
            );
            assert!(
                (green_archetypes[i].imag - yellow_archetypes[i].imag).abs() < 1e-10,
                "Green and Yellow archetype {} should have the same imaginary part",
                i
            );
            assert!(
                (green_archetypes[i].real - orange_archetypes[i].real).abs() < 1e-10,
                "Green and Orange archetype {} should have the same real part",
                i
            );
            assert!(
                (green_archetypes[i].imag - orange_archetypes[i].imag).abs() < 1e-10,
                "Green and Orange archetype {} should have the same imaginary part",
                i
            );
            assert!(
                (green_archetypes[i].real - red_archetypes[i].real).abs() < 1e-10,
                "Green and Red archetype {} should have the same real part",
                i
            );
            assert!(
                (green_archetypes[i].imag - red_archetypes[i].imag).abs() < 1e-10,
                "Green and Red archetype {} should have the same imaginary part",
                i
            );
        }
    }

    #[test]
    fn test_archetype_preservation_validation() {
        let archetypes = LightCondensationEngine::generate_test_archetypes();
        let engine = LightCondensationEngine::new(archetypes);

        let result = engine.condense_from_blue_realm();

        // Validate archetype preservation
        assert!(result.validate_archetype_preservation());
    }

    #[test]
    fn test_layer_correctness() {
        let archetypes = LightCondensationEngine::generate_test_archetypes();
        let engine = LightCondensationEngine::new(archetypes);

        let result = engine.condense_from_blue_realm();

        // Check that each field is at the correct layer
        assert_eq!(result.green_field.layer, InvolutionLayer::Green);
        assert_eq!(result.yellow_field.layer, InvolutionLayer::Yellow);
        assert_eq!(result.orange_field.layer, InvolutionLayer::Orange);
        assert_eq!(result.red_field.layer, InvolutionLayer::Red);
    }

    #[test]
    fn test_spatial_frequency_values() {
        let archetypes = LightCondensationEngine::generate_test_archetypes();
        let engine = LightCondensationEngine::new(archetypes);

        let result = engine.condense_from_blue_realm();

        // Check spatial frequency values
        assert!((result.green_field.spatial_frequency - 2000.0).abs() < 1e-10);
        assert!((result.yellow_field.spatial_frequency - 1400.0).abs() < 1e-10);
        assert!((result.orange_field.spatial_frequency - 1000.0).abs() < 1e-10);
        assert!((result.red_field.spatial_frequency - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_phase_coherence_preservation() {
        let archetypes = LightCondensationEngine::generate_test_archetypes();
        let engine = LightCondensationEngine::new(archetypes);

        let result = engine.condense_from_blue_realm();

        // Phase coherence should be preserved (all fields have the same encoding)
        let green_coherence = result.green_field.phase_coherence();
        let yellow_coherence = result.yellow_field.phase_coherence();
        let orange_coherence = result.orange_field.phase_coherence();
        let red_coherence = result.red_field.phase_coherence();

        assert!((green_coherence - yellow_coherence).abs() < 1e-10);
        assert!((green_coherence - orange_coherence).abs() < 1e-10);
        assert!((green_coherence - red_coherence).abs() < 1e-10);
    }

    #[test]
    fn test_condensation_result_summary() {
        let archetypes = LightCondensationEngine::generate_test_archetypes();
        let engine = LightCondensationEngine::new(archetypes);

        let result = engine.condense_from_blue_realm();

        let summary = result.summary();

        // Summary should contain layer names
        assert!(summary.contains("Green-Ray"));
        assert!(summary.contains("Yellow-Ray"));
        assert!(summary.contains("Orange-Ray"));
        assert!(summary.contains("Red-Ray"));

        // Summary should contain spatial frequencies
        assert!(summary.contains("2000"));
        assert!(summary.contains("1400"));
        assert!(summary.contains("1000"));
        assert!(summary.contains("100"));
    }
}
