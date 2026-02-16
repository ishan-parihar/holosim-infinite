pub mod blue_realm;
pub mod green_realm;
pub mod indigo_realm;
/// Foundation Module - Foundational Layers (Violet → Green)
///
/// This module implements the foundational layers of the cosmological architecture:
/// - Layer 0: Violet-Ray Realm (Infinity as undifferentiated unity)
/// - Layer 1: Indigo-Ray Realm (IntelligentInfinity + Archetype 22)
/// - Layer 2: Blue-Ray Realm (Love/Light + Universal Archetypical Patterns)
/// - Layer 3: Green-Ray Realm (Light/Love field of potential)
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each layer includes all previous development, transcends by adding fundamentally
/// new capabilities, and creates attractor-fields that pull toward the next stage."
///
/// The foundational layers implement the Three Primal Distortions:
/// - First Distortion: Free Will (Indigo-Ray)
/// - Second Distortion: Love/Logos (Blue-Ray)
/// - Third Distortion: Light (Green-Ray)
///
/// These layers provide the foundation for the Space/Time and Time/Space spectrum
/// that emerges at Yellow-Ray.
pub mod transcend_include;
pub mod violet_realm;

// Re-export main types for convenience
pub use blue_realm::Logos;
pub use green_realm::{Field, HolographicPattern, LightLoveField, Rhythm};
pub use indigo_realm::IntelligentInfinity;
pub use transcend_include::{AttractorField, Feature};
pub use violet_realm::VioletRealm;

// Type aliases for consistency with layer naming
pub type BlueRealm = Logos;
pub type GreenRealm = LightLoveField;
pub type IndigoRealm = IntelligentInfinity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foundation_layers_transcend_include() {
        // Test the sequential creation process through the foundational layers

        // Layer 0: Violet-Ray (Infinity)
        let violet = VioletRealm::new();
        assert!(violet.is_undifferentiated());

        // Layer 1: Indigo-Ray (IntelligentInfinity) - First Distortion
        let (violet_included, first_distortion, archetype22) = violet.apply_first_distortion();
        assert_eq!(violet_included.unity, 1.0);
        assert_eq!(first_distortion.name, "First Distortion: Free Will");
        assert_eq!(archetype22.name, "Archetype 22: The Choice");

        let intelligent = IntelligentInfinity::from_violet(violet);
        assert!(intelligent.is_aware());
        assert_eq!(intelligent.awareness(), 1.0);

        // Layer 2: Blue-Ray (Logos) - Second Distortion
        let (intelligent_included, second_distortion, universal_patterns) =
            intelligent.apply_second_distortion();
        assert_eq!(intelligent_included.awareness(), 1.0);
        assert_eq!(second_distortion.name, "Second Distortion: Love/Logos");
        assert_eq!(universal_patterns.name, "Universal Archetypical Patterns");

        let logos = Logos::from_intelligent_infinity(intelligent);
        assert!(logos.is_focused());
        assert_eq!(logos.focusing_strength(), 1.0);

        // Layer 3: Green-Ray (Light/Love field) - Third Distortion
        let (logos_included, third_distortion, light_love_field) = logos.apply_third_distortion();
        assert_eq!(logos_included.focusing_strength(), 1.0);
        assert_eq!(third_distortion.name, "Third Distortion: Light");
        assert_eq!(light_love_field.name, "Light/Love Field of Potential");

        let mut field = LightLoveField::from_logos(logos);
        assert_eq!(field.potential_strength(), 1.0);

        // Populate the field with holographic patterns, rhythms, and fields
        // to enable spectrum conditions
        field.add_holographic_pattern(HolographicPattern::new(1.0, [1.0, 0.0, 0.0], 1.0));
        field.add_rhythm(Rhythm::new(0.5, 1.0, 0.0));
        field.add_field(Field::new(1.0, 1.0, "Spectrum Field"));

        // Verify "transcend and include" at each layer
        // Each layer INCLUDES all previous development
        assert_eq!(field.logos.intelligent_infinity.violet_realm.unity, 1.0);
        assert_eq!(field.logos.intelligent_infinity.awareness(), 1.0);
        assert_eq!(field.logos.focusing_strength(), 1.0);

        // Each layer TRANSCENDS by adding new development
        assert!(violet_included.is_undifferentiated()); // Violet: Unity only
        assert!(intelligent_included.is_aware()); // Indigo: + Awareness
        assert!(logos_included.is_focused()); // Blue: + Focus
        assert!(field.has_spectrum_conditions()); // Green: + Spectrum conditions (when populated)

        // Each layer EVOLVES INTO attractor-fields
        assert_eq!(
            archetype22.target,
            "IntelligentInfinity - Awareness emerges"
        );
        assert_eq!(
            universal_patterns.target,
            "Love/Light - The Creative Principle emerges"
        );
        assert_eq!(
            light_love_field.target,
            "Conditions for Space/Time and Time/Space Spectrum emerge"
        );
    }

    #[test]
    fn test_three_primal_distortions() {
        // Test that the Three Primal Distortions are correctly implemented

        // First Distortion: Free Will
        let violet = VioletRealm::new();
        let (_, first_distortion, _) = violet.apply_first_distortion();
        assert_eq!(first_distortion.name, "First Distortion: Free Will");
        assert_eq!(first_distortion.strength, 1.0);

        // Second Distortion: Love/Logos
        let intelligent = IntelligentInfinity::from_violet(violet);
        let (_, second_distortion, _) = intelligent.apply_second_distortion();
        assert_eq!(second_distortion.name, "Second Distortion: Love/Logos");
        assert_eq!(second_distortion.strength, 1.0);

        // Third Distortion: Light
        let logos = Logos::from_intelligent_infinity(intelligent);
        let (_, third_distortion, _) = logos.apply_third_distortion();
        assert_eq!(third_distortion.name, "Third Distortion: Light");
        assert_eq!(third_distortion.strength, 1.0);
    }

    #[test]
    fn test_archetype22_at_indigo_realm() {
        // Test that Archetype 22 is correctly implemented at Indigo-Ray

        let violet = VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);

        // Archetype 22 should be present at Indigo-Ray
        assert_eq!(intelligent.archetype22.strength, 1.0);
        assert_eq!(intelligent.archetype22.choice_operator, "Choice Operator");

        // Archetype 22 should generate possibility space
        use crate::entity_layer7::layer7::{EntityState, PolarityState, VibrationalState};
        use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};

        let entity_state = EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence: 0.5,
                density: Density::First(Density1SubLevel::Planetary),
                kinetic_energy: 0.5,
                potential_energy: 0.5,
            },
            polarity_state: PolarityState {
                polarity_bias: 0.0,
                polarization_strength: 0.0,
            },
            consciousness_level: 0.5,
            experience_accumulation: 10.0,
            learning_progress: 0.5,
        };

        let possibility_space =
            intelligent
                .archetype22
                .generate_possibility_space(&entity_state, 0.5, 0.5);
        // From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0: "Generate 3-5 possibilities based on entity state"
        // For unpolarized entity (polarity_bias = 0.0), generates 3 possibilities: STO, STS, Neutral
        assert_eq!(possibility_space.count(), 3);

        // Archetype 22 should make choices
        let choice = intelligent
            .archetype22
            .make_choice(&possibility_space, &entity_state);
        // PolarityChoice is an enum (ServiceToOthers, ServiceToSelf, Neutral), not an Option
        assert_ne!(choice, indigo_realm::PolarityChoice::Neutral);
    }

    #[test]
    fn test_universal_patterns_at_blue_realm() {
        // Test that Universal Archetypical Patterns are correctly implemented at Blue-Ray

        let violet = VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);

        // Universal patterns should be present at Blue-Ray
        assert_eq!(logos.universal_patterns.coherence, 1.0);

        // Universal patterns should be different from 22-archetype system
        // (22-archetype system is a Solar-Logos feature at Red-Ray)
        let desc = logos.universal_patterns.description();
        assert!(desc.contains("UNKNOWN"));
        assert!(desc.contains("NOT the 22-archetype system"));

        // Logos should refine the cosmic mind
        let mut logos_refined = logos.clone();
        logos_refined.refine_cosmic_mind("Test bias");
        assert_eq!(logos_refined.universal_patterns.count(), 1);
    }

    #[test]
    fn test_light_love_field_at_green_realm() {
        // Test that Light/Love field is correctly implemented at Green-Ray

        let violet = VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let mut field = LightLoveField::from_logos(logos);

        // Field should have potential strength
        assert_eq!(field.potential_strength(), 1.0);

        // Field should include all previous layers
        assert_eq!(field.logos.intelligent_infinity.violet_realm.unity, 1.0);
        assert_eq!(field.logos.intelligent_infinity.awareness(), 1.0);
        assert_eq!(field.logos.focusing_strength(), 1.0);

        // Field should support holographic patterns
        field.add_holographic_pattern(HolographicPattern::new(0.8, [1.0, 0.0, 0.0], 0.7));
        assert_eq!(field.holographic_pattern_count(), 1);

        // Field should support rhythms
        field.add_rhythm(Rhythm::new(0.5, 0.8, 0.0));
        assert_eq!(field.rhythm_count(), 1);

        // Field should support fields
        field.add_field(Field::new(0.9, 0.85, "Test Field"));
        assert_eq!(field.field_count(), 1);

        // Field should have conditions for spectrum when populated
        assert!(field.has_spectrum_conditions());
    }

    #[test]
    fn test_mysterious_emergence_at_green_realm() {
        // Test that the mysterious emergence is correctly implemented at Green-Ray

        let violet = VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let field = LightLoveField::from_logos(logos);

        // Green-Ray should apply mysterious emergence to transition to Yellow-Ray
        let (_, mysterious_emergence, dimensions_spectrum_veil) =
            field.apply_mysterious_emergence();

        assert_eq!(mysterious_emergence.name, "The Mysterious Emergence");
        assert_eq!(mysterious_emergence.strength, 1.0);

        assert_eq!(dimensions_spectrum_veil.name, "Dimensions/Spectrum/Veil");
        assert_eq!(
            dimensions_spectrum_veil.target,
            "Space/Time and Time/Space Spectrum emerges"
        );
    }
}
