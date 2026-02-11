// Three Primal Distortions Validation Tests
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// The Three Primal Distortions are the foundation of all creation. They are applied
// sequentially, each creating the conditions for the next.
//
// 1. First Distortion: Free Will (Indigo-Ray Realm)
//    - Action: The Creator's choice to know Itself
//    - Result: Infinity becomes aware (IntelligentInfinity)
//    - Architectural Artifact: Archetype 22 (The Choice)
//
// 2. Second Distortion: Love/Logos (Blue-Ray Realm)
//    - Action: The focusing of Infinity as an aware or conscious principle
//    - Result: Logos emerges (Creative Principle)
//    - Architectural Artifact: Universal Archetypical Patterns (Foundation)
//
// 3. Third Distortion: Light (Green-Ray Realm)
//    - Action: The manifestation when light has been impressed with love
//    - Result: Light/Love field of potential
//    - Architectural Artifact: The Conditions for the Space/Time and Time/Space Spectrum

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // FIRST DISTORTION: FREE WILL (INDIGO-RAY REALM)
    // ============================================================================

    #[test]
    fn test_first_distortion_creates_intelligent_infinity() {
        // Violet-Ray Realm: Infinity (undifferentiated unity)
        let violet = VioletRealm::new();

        // Apply First Distortion: Free Will
        // This creates IntelligentInfinity (Infinity + Awareness)
        let intelligent_infinity = IntelligentInfinity::new();

        // Verify that IntelligentInfinity contains Violet as source
        assert!(intelligent_infinity.has_awareness());
        assert!(intelligent_infinity.is_infinite());
    }

    #[test]
    fn test_first_distortion_creates_archetype22() {
        // Apply First Distortion to create IntelligentInfinity
        let intelligent_infinity = IntelligentInfinity::new();

        // Archetype 22 emerges as the "permission token" for individuality
        let archetype22 = Archetype22::new(intelligent_infinity);

        // Verify Archetype 22 properties
        assert!(archetype22.is_choice_operator());
        assert!(archetype22.generates_possibility_space());
    }

    #[test]
    fn test_free_will_is_not_random() {
        // Create Free Will kernel with Archetype 22
        let intelligent_infinity = IntelligentInfinity::new();
        let archetype22 = Archetype22::new(intelligent_infinity);
        let mut free_will_kernel = FreeWillKernel::new(archetype22);

        // Create entity state
        let entity_state = EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence: 0.7,
            },
            polarity_state: EntityPolarityState {
                sto_orientation: 0.5,
                sts_orientation: 0.2,
                polarization_strength: 0.5,
            },
            consciousness_level: 0.5,
            karmic_patterns: Vec::new(),
        };

        // Create choice context
        let context = FreeWillChoiceContext {
            polarity_preference: FreeWillPolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        // Make multiple choices with the same state
        let mut choices = Vec::new();
        for _ in 0..20 {
            let result = free_will_kernel.exercise_free_will(&entity_state, &context);
            choices.push(result.choice.selected_index);
        }

        // Verify that choices are not completely random (should show some patterns)
        // but also not deterministic (should show variation)
        let unique_choices: std::collections::HashSet<_> = choices.iter().collect();
        assert!(unique_choices.len() > 1, "Free Will should show variation");
        assert!(
            unique_choices.len() < choices.len(),
            "Free Will should show patterns"
        );
    }

    #[test]
    fn test_possibility_space_generation() {
        // Create Archetype 22
        let intelligent_infinity = IntelligentInfinity::new();
        let archetype22 = Archetype22::new(intelligent_infinity);

        // Create entity state
        let entity_state = EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence: 0.7,
            },
            polarity_state: EntityPolarityState {
                sto_orientation: 0.5,
                sts_orientation: 0.2,
                polarization_strength: 0.5,
            },
            consciousness_level: 0.5,
            karmic_patterns: Vec::new(),
        };

        // Generate possibility space
        let possibility_space = archetype22.generate_possibility_space(&entity_state);

        // Verify possibility space properties
        assert!(!possibility_space.possibilities.is_empty());
        assert!(possibility_space.constraint_factor > 0.0);
        assert!(possibility_space.constraint_factor <= 1.0);

        // All possibilities should have valid probabilities
        for possibility in &possibility_space.possibilities {
            assert!(possibility.probability >= 0.0);
            assert!(possibility.probability <= 1.0);
        }
    }

    // ============================================================================
    // SECOND DISTORTION: LOVE/LOGOS (BLUE-RAY REALM)
    // ============================================================================

    #[test]
    fn test_second_distortion_creates_logos() {
        // IntelligentInfinity (from First Distortion)
        let intelligent_infinity = IntelligentInfinity::new();

        // Apply Second Distortion: Love/Logos
        // This focuses IntelligentInfinity into the Creative Principle
        let logos = Logos::new(intelligent_infinity);

        // Verify Logos properties
        assert!(logos.is_creative_principle());
        assert!(logos.is_aware());
        assert!(logos.has_universal_patterns());
    }

    #[test]
    fn test_second_distortion_includes_first_distortion() {
        // IntelligentInfinity (from First Distortion)
        let intelligent_infinity = IntelligentInfinity::new();

        // Apply Second Distortion
        let logos = Logos::new(intelligent_infinity);

        // Verify that Logos INCLUDES IntelligentInfinity (transcend and include)
        assert!(logos.includes_intelligent_infinity());
    }

    #[test]
    fn test_second_distortion_creates_universal_archetypical_patterns() {
        // IntelligentInfinity (from First Distortion)
        let intelligent_infinity = IntelligentInfinity::new();

        // Apply Second Distortion
        let logos = Logos::new(intelligent_infinity);

        // Verify that Logos creates Universal Archetypical Patterns
        let universal_patterns = logos.get_universal_patterns();
        assert!(universal_patterns.has_structure());
        assert!(universal_patterns.is_universal());
    }

    // ============================================================================
    // THIRD DISTORTION: LIGHT (GREEN-RAY REALM)
    // ============================================================================

    #[test]
    fn test_third_distortion_creates_light_love_field() {
        // Logos (from Second Distortion)
        let intelligent_infinity = IntelligentInfinity::new();
        let logos = Logos::new(intelligent_infinity);

        // Apply Third Distortion: Light
        // This manifests as Light/Love field of potential
        let light_love_field = LightLoveField::new(logos);

        // Verify Light/Love field properties
        assert!(light_love_field.is_light_impressed_with_love());
        assert!(light_love_field.is_field_of_potential());
        assert!(light_love_field.has_holographic_patterns());
    }

    #[test]
    fn test_third_distortion_includes_previous_distortions() {
        // Apply all three distortions sequentially
        let intelligent_infinity = IntelligentInfinity::new();
        let logos = Logos::new(intelligent_infinity);
        let light_love_field = LightLoveField::new(logos);

        // Verify that Light/Love field INCLUDES all previous distortions
        assert!(light_love_field.includes_logos());
        assert!(light_love_field.includes_intelligent_infinity());
        assert!(light_love_field.includes_infinity());
    }

    #[test]
    fn test_third_distortion_creates_conditions_for_spectrum() {
        // Logos (from Second Distortion)
        let intelligent_infinity = IntelligentInfinity::new();
        let logos = Logos::new(intelligent_infinity);

        // Apply Third Distortion
        let light_love_field = LightLoveField::new(logos);

        // Verify that Light/Love field creates conditions for Space/Time and Time/Space spectrum
        assert!(light_love_field.has_spectrum_conditions());
        assert!(light_love_field.has_rhythms_and_fields());
    }

    // ============================================================================
    // SEQUENTIAL APPLICATION OF DISTORTIONS
    // ============================================================================

    #[test]
    fn test_distortions_applied_sequentially() {
        // Apply distortions in sequence
        let violet = VioletRealm::new();

        // First Distortion: Free Will
        let intelligent_infinity = IntelligentInfinity::new();

        // Second Distortion: Love/Logos
        let logos = Logos::new(intelligent_infinity);

        // Third Distortion: Light
        let light_love_field = LightLoveField::new(logos);

        // Verify sequential application
        assert!(light_love_field.includes_logos());
        assert!(light_love_field.includes_intelligent_infinity());
        assert!(light_love_field.includes_infinity());
    }

    #[test]
    fn test_each_distortion_transcends_previous() {
        // Apply distortions in sequence
        let intelligent_infinity = IntelligentInfinity::new();
        let logos = Logos::new(intelligent_infinity);
        let light_love_field = LightLoveField::new(logos);

        // Verify that each distortion TRANSCENDS by adding new capabilities
        // IntelligentInfinity transcends Violet (adds awareness)
        assert!(intelligent_infinity.has_awareness());

        // Logos transcends IntelligentInfinity (adds creative principle)
        assert!(logos.is_creative_principle());

        // Light/Love field transcends Logos (adds manifestation field)
        assert!(light_love_field.is_field_of_potential());
    }

    #[test]
    fn test_each_distortion_creates_attractor_field() {
        // Apply distortions in sequence
        let intelligent_infinity = IntelligentInfinity::new();
        let logos = Logos::new(intelligent_infinity);
        let light_love_field = LightLoveField::new(logos);

        // Verify that each distortion creates attractor-fields
        // IntelligentInfinity creates attractor-field for Logos
        assert!(intelligent_infinity.has_attractor_field());

        // Logos creates attractor-field for Light/Love
        assert!(logos.has_attractor_field());

        // Light/Love creates attractor-field for Dimensions/Spectrum/Veil
        assert!(light_love_field.has_attractor_field());
    }

    // ============================================================================
    // INTEGRATION TESTS
    // ============================================================================

    #[test]
    fn test_complete_three_distortions_flow() {
        // Apply all three distortions in sequence
        let violet = VioletRealm::new();
        let intelligent_infinity = IntelligentInfinity::new();
        let logos = Logos::new(intelligent_infinity);
        let light_love_field = LightLoveField::new(logos);

        // Verify complete flow
        // 1. Violet (Infinity) is the source
        assert!(violet.is_infinite());

        // 2. First Distortion creates IntelligentInfinity
        assert!(intelligent_infinity.is_infinite());
        assert!(intelligent_infinity.has_awareness());

        // 3. Second Distortion creates Logos
        assert!(logos.is_creative_principle());
        assert!(logos.includes_intelligent_infinity());

        // 4. Third Distortion creates Light/Love field
        assert!(light_love_field.is_light_impressed_with_love());
        assert!(light_love_field.includes_logos());
        assert!(light_love_field.includes_intelligent_infinity());

        // 5. All distortions create attractor-fields
        assert!(intelligent_infinity.has_attractor_field());
        assert!(logos.has_attractor_field());
        assert!(light_love_field.has_attractor_field());
    }
}
